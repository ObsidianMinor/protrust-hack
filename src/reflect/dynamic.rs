use crate::io::{self, Tag, FieldNumber, WireType};
use crate::reflect::access::{self, FieldAccessor, FieldAccessError::{self, InvalidMessage}};
use crate::reflect::{
    AnyEnum, AnyMessage, AnyValue, CompositeScope, DynamicType, Descriptor, Syntax, EnumDescriptor, EnumValueDescriptor,
    FieldDescriptor, FieldLabel, FieldScope, FieldType, MessageDescriptor, OneofDescriptor, ValueType,
};
use crate::{CodedMessage, ExtensionRegistry, UnknownFieldSet};
use std::any::TypeId;
use hashbrown::{HashMap, hash_map::{Entry, RawEntryMut}};
use std::fmt::{self, Debug, Formatter};
use std::hash::{BuildHasher, Hasher, Hash};

fn new_default_value<'a>(message: &'a MessageDescriptor<'a>, field: &FieldDescriptor<'a>) -> Box<dyn AnyValue<'a> + 'a> {
    match message.find_field_by_number(field.number()).unwrap().field_type() {
        FieldType::Message(m) | FieldType::Group(m) => m.new_instance().unwrap(),
        FieldType::Enum(e) => e.new_from(0),
        FieldType::Bool => Box::new(false),
        FieldType::Bytes => Box::new(Vec::new()),
        FieldType::Double => Box::new(0.0f64),
        FieldType::Fixed32 | FieldType::Uint32 => Box::new(0u32),
        FieldType::Fixed64 | FieldType::Uint64 => Box::new(0u64),
        FieldType::Float => Box::new(0.0f32),
        FieldType::Int32 | FieldType::Sfixed32 | FieldType::Sint32 => Box::new(0i32),
        FieldType::Int64 | FieldType::Sfixed64 | FieldType::Sint64 => Box::new(0i64),
        FieldType::String => Box::new(String::new())
    }
}

fn check_value<'a>(field: &FieldDescriptor<'a>, value: Box<dyn AnyValue<'a> + 'a>) -> Result<Box<dyn AnyValue<'a> + 'a>, Box<dyn AnyValue<'a> + 'a>> {
    match (field.field_type(), value.type_id()) {
        (FieldType::Bool, ValueType::Static(t)) if t == TypeId::of::<bool>() => Ok(value),
        (FieldType::Bytes, ValueType::Static(t)) if t == TypeId::of::<Vec<u8>>() => Ok(value),
        (FieldType::Double, ValueType::Static(t)) if t == TypeId::of::<f64>() => Ok(value),
        (FieldType::Enum(e), ValueType::Dynamic(DynamicType::Enum)) if value.as_enum().unwrap().descriptor() == &**e => Ok(value),
        (FieldType::Fixed32, ValueType::Static(t)) | (FieldType::Uint32, ValueType::Static(t)) if t == TypeId::of::<u32>() => Ok(value),
        (FieldType::Fixed64, ValueType::Static(t)) | (FieldType::Uint64, ValueType::Static(t)) if t == TypeId::of::<u64>() => Ok(value),
        (FieldType::Float, ValueType::Static(t)) if t == TypeId::of::<f32>() => Ok(value),
        (FieldType::Group(m), ValueType::Dynamic(DynamicType::Message)) | (FieldType::Message(m), ValueType::Dynamic(DynamicType::Message)) if value.as_message().unwrap().descriptor() == &**m => Ok(value),
        (FieldType::Int32, ValueType::Static(t)) | (FieldType::Sfixed32, ValueType::Static(t)) | (FieldType::Sint32, ValueType::Static(t)) if t == TypeId::of::<i32>() => Ok(value),
        (FieldType::Int64, ValueType::Static(t)) | (FieldType::Sfixed64, ValueType::Static(t)) | (FieldType::Sint64, ValueType::Static(t)) if t == TypeId::of::<i64>() => Ok(value),
        (FieldType::String, ValueType::Static(t)) if t == TypeId::of::<String>() => Ok(value),
        _ => Err(value)
    }
}

#[derive(Clone, PartialEq)]
pub struct DynamicMessage<'a> {
    descriptor: &'a MessageDescriptor<'a>,
    fields: HashMap<FieldNumber, DynamicFieldValue<'a>>,
    oneofs: HashMap<usize, (usize, DynamicFieldValue<'a>)>,
    unknown_fields: UnknownFieldSet,
}

impl<'a> DynamicMessage<'a> {
    pub fn new(descriptor: &'a MessageDescriptor<'a>) -> DynamicMessage<'a> {
        match descriptor.file().syntax() {
            Syntax::Proto3 => { // proto3 says no to field presence, so everything that can have a "default" not null is set
                DynamicMessage {
                    descriptor,
                    fields: descriptor.message_fields()
                                .iter()
                                .filter(|f| !f.field_type().is_message() && !f.field_type().is_group() && f.label() == FieldLabel::Optional)
                                .map(|f| (f.number(), DynamicFieldValue::new(f)))
                                .collect(),
                    oneofs: HashMap::new(),
                    unknown_fields: UnknownFieldSet::new(),
                }
            },
            _ => {
                DynamicMessage {
                    descriptor,
                    fields: HashMap::new(),
                    oneofs: HashMap::new(),
                    unknown_fields: UnknownFieldSet::new(),
                }
            }
        }
    }

    pub fn downcast_any_ref<'b>(message: &'b dyn AnyMessage<'a>) -> Option<&'b DynamicMessage<'a>> {
        if message.type_id() == ValueType::Dynamic(DynamicType::Message) {
            unsafe { Some(&*(message as *const dyn AnyMessage<'a> as *const DynamicMessage<'a>)) }
        } else {
            None
        }
    }

    pub fn downcast_any_mut<'b>(message: &'b mut dyn AnyMessage<'a>) -> Option<&'b mut DynamicMessage<'a>> {
        if message.type_id() == ValueType::Dynamic(DynamicType::Message) {
            unsafe { Some(&mut *(message as *mut dyn AnyMessage<'a> as *mut DynamicMessage<'a>)) }
        } else {
            None
        }
    }

    pub fn get_set_case<'b>(&self, oneof: &'b OneofDescriptor<'a>) -> Option<&'b FieldDescriptor<'a>> {
        self.oneofs.get(&oneof.message_index()).map(|(i, _)| &*oneof.fields()[*i])
    }

    fn get_end_group_tag(&self) -> Option<Tag> {
        match self.descriptor.scope() {
            CompositeScope::Message(m) => {
                for field in m.fields() {
                    match field.field_type() {
                        FieldType::Group(m) if &**m == self.descriptor => {
                            return Some(Tag::new(field.number(), WireType::EndGroup))
                        }
                        _ => {}
                    }
                }
                for extension in m.extensions() {
                    match extension.field_type() {
                        FieldType::Group(m) if &**m == self.descriptor => {
                            return Some(Tag::new(extension.number(), WireType::EndGroup))
                        }
                        _ => {}
                    }
                }
                None
            }
            CompositeScope::File(f) => {
                for extension in f.extensions() {
                    match extension.field_type() {
                        FieldType::Group(m) if &**m == self.descriptor => {
                            return Some(Tag::new(extension.number(), WireType::EndGroup))
                        }
                        _ => {}
                    }
                }
                None
            }
        }
    }
}

impl Debug for DynamicMessage<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut strct = f.debug_struct(self.descriptor().name());
        for field in self.descriptor().fields() {
            strct.field(field.name(), &FieldDebugFormatter(field.accessor().unwrap(), self));
        }
        strct.finish()
    }
}

static FAILED_SELF_ACCESS_ERROR: &str = "couldn't access field on self";
static MISREPORTED_LEN: &str = "repeated field accessor misreported its length";

struct FieldDebugFormatter<'a, 'b, 'c>(FieldAccessor<'a, 'b>, &'c dyn AnyMessage<'a>);

impl Debug for FieldDebugFormatter<'_, '_, '_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let accessor = self.0;
        let message = self.1;

        match accessor {
            FieldAccessor::Single(single) => {
                if let Some(value) = single.get(message).expect(FAILED_SELF_ACCESS_ERROR) {
                    value.fmt(f)?;
                }
                Ok(())
            },
            FieldAccessor::Repeated(repeated) => {
                let mut list = f.debug_list();
                for i in 0..repeated.len(message).expect(FAILED_SELF_ACCESS_ERROR) {
                    let value = repeated.get(message, i).expect(FAILED_SELF_ACCESS_ERROR).unwrap();
                    list.entry(&value);
                }
                list.finish()
            },
            FieldAccessor::Map(map) => {
                let mut dbg_map = f.debug_map();
                for (key, value) in map.iter(message).expect(FAILED_SELF_ACCESS_ERROR) {
                    dbg_map.entry(&key, &value);
                }
                dbg_map.finish()
            }
        }
    }
}

struct MutAnyMessageWrapper<'a, 'b>(&'a mut dyn AnyMessage<'b>);

impl CodedMessage for MutAnyMessageWrapper<'_, '_> {
    fn merge_from(&mut self, input: &mut io::CodedInput) -> io::InputResult<()> {
        self.0.merge_from(input)
    }
    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        self.0.calculate_size()
    }
    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        self.0.calculate_size()
    }
    fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult {
        self.0.write_to(output)
    }
    fn is_initialized(&self) -> bool {
        self.0.is_initialized()
    }
}

impl<'a> CodedMessage for DynamicMessage<'a> {
    fn merge_from(&mut self, input: &mut io::CodedInput) -> io::InputResult<()> {
        let descriptor = self.descriptor();
        let end_tag = self.get_end_group_tag();
        while let Some(tag) = input.read_tag()? {
            if let Some(end_tag) = end_tag {
                if tag == end_tag {
                    break;
                }
            }
            if let Some(field) = descriptor.find_any_field_by_number(tag.number()) {
                match field.accessor().unwrap() {
                    FieldAccessor::Single(single) => {
                        match field.field_type() {
                            FieldType::Bool => single.set(self, Box::new(input.read_bool()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Bytes => single.set(self, Box::new(input.read_bytes()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Double => single.set(self, Box::new(input.read_double()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Enum(e) => single.set(self, e.new_from(input.read_int32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Fixed32 => single.set(self, Box::new(input.read_fixed32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Fixed64 => single.set(self, Box::new(input.read_fixed64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Float => single.set(self, Box::new(input.read_float()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Int32 => single.set(self, Box::new(input.read_int32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Int64 => single.set(self, Box::new(input.read_int64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Message(_) | FieldType::Group(_) => {
                                input.read_message(&mut MutAnyMessageWrapper(single.get_mut(self).expect(FAILED_SELF_ACCESS_ERROR).as_message_mut().unwrap()))?
                            },
                            FieldType::Sfixed32 => single.set(self, Box::new(input.read_sfixed32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Sfixed64 => single.set(self, Box::new(input.read_sfixed64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Sint32 => single.set(self, Box::new(input.read_sint32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Sint64 => single.set(self, Box::new(input.read_sint64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::String => single.set(self, Box::new(input.read_string()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Uint32 => single.set(self, Box::new(input.read_uint32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                            FieldType::Uint64 => single.set(self, Box::new(input.read_uint64()?)).expect(FAILED_SELF_ACCESS_ERROR)
                        }
                    },
                    FieldAccessor::Repeated(repeated) => {
                        if tag.wire_type() == WireType::LengthDelimited && field.wire_type().is_packable() {
                            let new_limit = input.read_length()?;
                            let old = input.push_limit(new_limit);
                            while !input.reached_limit() {
                                match field.field_type() {
                                    FieldType::Bool => repeated.push(self, Box::new(input.read_bool()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Double => repeated.push(self, Box::new(input.read_double()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Enum(e) => repeated.push(self, e.new_from(input.read_int32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Fixed32 => repeated.push(self, Box::new(input.read_fixed32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Fixed64 => repeated.push(self, Box::new(input.read_fixed64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Float => repeated.push(self, Box::new(input.read_float()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Int32 => repeated.push(self, Box::new(input.read_int32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Int64 => repeated.push(self, Box::new(input.read_int64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Sfixed32 => repeated.push(self, Box::new(input.read_sfixed32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Sfixed64 => repeated.push(self, Box::new(input.read_sfixed64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Sint32 => repeated.push(self, Box::new(input.read_sint32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Sint64 => repeated.push(self, Box::new(input.read_sint64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Uint32 => repeated.push(self, Box::new(input.read_uint32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    FieldType::Uint64 => repeated.push(self, Box::new(input.read_uint64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                    _ => unreachable!("field type is unusable in packed fields")
                                }
                            }
                            input.pop_limit(old);
                        } else {
                            match field.field_type() {
                                FieldType::Bool => repeated.push(self, Box::new(input.read_bool()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Bytes => repeated.push(self, Box::new(input.read_bytes()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Double => repeated.push(self, Box::new(input.read_double()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Enum(e) => repeated.push(self, e.new_from(input.read_int32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Fixed32 => repeated.push(self, Box::new(input.read_fixed32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Fixed64 => repeated.push(self, Box::new(input.read_fixed64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Float => repeated.push(self, Box::new(input.read_float()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Int32 => repeated.push(self, Box::new(input.read_int32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Int64 => repeated.push(self, Box::new(input.read_int64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Message(m) | FieldType::Group(m) => {
                                    let mut value = m.new_instance().unwrap();
                                    input.read_message(&mut MutAnyMessageWrapper(value.as_message_mut().unwrap()))?;
                                    repeated.push(self, value).expect(FAILED_SELF_ACCESS_ERROR)
                                },
                                FieldType::Sfixed32 => repeated.push(self, Box::new(input.read_sfixed32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Sfixed64 => repeated.push(self, Box::new(input.read_sfixed64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Sint32 => repeated.push(self, Box::new(input.read_sint32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Sint64 => repeated.push(self, Box::new(input.read_sint64()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::String => repeated.push(self, Box::new(input.read_string()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Uint32 => repeated.push(self, Box::new(input.read_uint32()?)).expect(FAILED_SELF_ACCESS_ERROR),
                                FieldType::Uint64 => repeated.push(self, Box::new(input.read_uint64()?)).expect(FAILED_SELF_ACCESS_ERROR)
                            }
                        }
                    },
                    FieldAccessor::Map(map) => {
                        let entry = 
                            match field.field_type() {
                                FieldType::Message(m) => m,
                                _ => unreachable!("map accessor requires field type of message")
                            };

                        let key_field = &entry.fields()[0];
                        let key_tag = Tag::new(key_field.number(), key_field.wire_type());
                        let value_field = &entry.fields()[1];
                        let value_tag = Tag::new(value_field.number(), value_field.wire_type());

                        let new_limit = input.read_length()?;
                        let old = input.push_limit(new_limit);
                        while !input.reached_limit() {
                            let mut key: Option<Box<dyn AnyValue<'a>>> = None;
                            let mut value: Option<Box<dyn AnyValue<'a>>> = None;
                            while let Some(tag) = input.read_tag()? {
                                match tag {
                                    tag if tag == key_tag => {
                                        key = 
                                            Some(match key_field.field_type() {
                                                FieldType::Bool => Box::new(input.read_bool()?),
                                                FieldType::Fixed32 => Box::new(input.read_fixed32()?),
                                                FieldType::Fixed64 => Box::new(input.read_fixed64()?),
                                                FieldType::Int32 => Box::new(input.read_int32()?),
                                                FieldType::Int64 => Box::new(input.read_int64()?),
                                                FieldType::Sfixed32 => Box::new(input.read_sfixed32()?),
                                                FieldType::Sfixed64 => Box::new(input.read_sfixed64()?),
                                                FieldType::Sint32 => Box::new(input.read_sint32()?),
                                                FieldType::Sint64 => Box::new(input.read_sint64()?),
                                                FieldType::Uint32 => Box::new(input.read_uint32()?),
                                                FieldType::Uint64 => Box::new(input.read_uint64()?),
                                                _ => unreachable!("invalid field type for map key")
                                            });
                                    },
                                    tag if tag == value_tag => {
                                        value = 
                                            Some(match value_field.field_type() {
                                                FieldType::Bool => Box::new(input.read_bool()?),
                                                FieldType::Fixed32 => Box::new(input.read_fixed32()?),
                                                FieldType::Fixed64 => Box::new(input.read_fixed64()?),
                                                FieldType::Int32 => Box::new(input.read_int32()?),
                                                FieldType::Int64 => Box::new(input.read_int64()?),
                                                FieldType::Sfixed32 => Box::new(input.read_sfixed32()?),
                                                FieldType::Sfixed64 => Box::new(input.read_sfixed64()?),
                                                FieldType::Sint32 => Box::new(input.read_sint32()?),
                                                FieldType::Sint64 => Box::new(input.read_sint64()?),
                                                FieldType::Uint32 => Box::new(input.read_uint32()?),
                                                FieldType::Uint64 => Box::new(input.read_uint64()?),
                                                FieldType::Bytes => Box::new(input.read_bytes()?),
                                                FieldType::Double => Box::new(input.read_double()?),
                                                FieldType::Enum(e) => e.new_from(input.read_int32()?),
                                                FieldType::Float => Box::new(input.read_float()?),
                                                FieldType::Group(m) | FieldType::Message(m) => {
                                                    let mut value = m.new_instance().unwrap();
                                                    input.read_message(&mut MutAnyMessageWrapper(value.as_message_mut().unwrap()))?;
                                                    value
                                                },
                                                FieldType::String => Box::new(input.read_string()?)
                                            });
                                    },
                                    tag => input.skip(tag)?
                                }
                            }
                            map.insert(
                                self, 
                                key.unwrap_or_else(|| new_default_value(descriptor, key_field)), 
                                value.unwrap_or_else(|| new_default_value(descriptor, value_field))).expect(FAILED_SELF_ACCESS_ERROR);
                        }
                        input.pop_limit(old);
                    }
                }
            } else {
                self.unknown_fields.merge_from(tag, input)?;
            }
        }
        Ok(())
    }
    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        unimplemented!()
    }
    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        if self.descriptor().file().syntax() == Syntax::Proto3 {
            return true;
        }

        for field in self.descriptor().fields() {
            match field.accessor().unwrap() {
                FieldAccessor::Single(single) => {
                    match single.get(self).expect(FAILED_SELF_ACCESS_ERROR) {
                        Some(value) if field.field_type().is_message() || field.field_type().is_group() => {
                            if !value.as_message().unwrap().is_initialized() {
                                return false;
                            }
                        },
                        None if field.label() == FieldLabel::Required => return false,
                        _ => { },
                    }
                },
                FieldAccessor::Repeated(repeated) => {
                    if field.field_type().is_message() || field.field_type().is_group() {
                        for i in 0..repeated.len(self).expect(FAILED_SELF_ACCESS_ERROR) {
                            let value = repeated.get(self, i).expect(FAILED_SELF_ACCESS_ERROR).expect(MISREPORTED_LEN);
                            if !value.as_message().unwrap().is_initialized() {
                                return false;
                            }
                        }
                    }
                },
                FieldAccessor::Map(map) => {
                    match field.field_type() {
                        FieldType::Message(m) => {
                            if m.fields()[1].field_type().is_message() {
                                for (_, value) in map.iter(self).expect(FAILED_SELF_ACCESS_ERROR) {
                                    if !value.as_message().unwrap().is_initialized() {
                                        return false;
                                    }
                                }
                            }
                        },
                        _ => unreachable!()
                    }
                }
            }
        }

        if self.descriptor().file().syntax() == Syntax::Proto2 && self.descriptor().proto().extension_range().len() != 0 {
            for field in self.fields.keys().filter_map(|n| self.descriptor().find_extension_by_number(*n)) {
                match field.accessor().unwrap() {
                    FieldAccessor::Single(single) => {
                        match single.get(self).expect(FAILED_SELF_ACCESS_ERROR) {
                            Some(value) if field.field_type().is_message() || field.field_type().is_group() => {
                                if !value.as_message().unwrap().is_initialized() {
                                    return false;
                                }
                            },
                            _ => { },
                        }
                    },
                    FieldAccessor::Repeated(repeated) => {
                        if field.field_type().is_message() || field.field_type().is_group() {
                            for i in 0..repeated.len(self).expect(FAILED_SELF_ACCESS_ERROR) {
                                let value = repeated.get(self, i).expect(FAILED_SELF_ACCESS_ERROR).expect(MISREPORTED_LEN);
                                if !value.as_message().unwrap().is_initialized() {
                                    return false;
                                }
                            }
                        }
                    },
                    FieldAccessor::Map(_) => unreachable!("extension fields cannot be map fields"),
                }
            }
        }

        true
    }
}

impl<'a> AnyValue<'a> for DynamicMessage<'a> {
    fn clone(&self) -> Box<dyn AnyValue<'a>> {
        Box::new(Clone::clone(self))
    }
    fn eq(&self, other: &dyn AnyValue<'a>) -> bool {
        match other.type_id() {
            ValueType::Dynamic(DynamicType::Message) => {
                let other = unsafe { &*(other as *const dyn AnyValue as *const DynamicMessage<'a>) };
                PartialEq::eq(self, other)
            }
            _ => false,
        }
    }

    fn type_id(&self) -> ValueType {
        ValueType::Dynamic(DynamicType::Message)
    }
    fn as_enum(&self) -> Option<&dyn AnyEnum<'a>> {
        None
    }
    fn as_enum_mut(&mut self) -> Option<&mut dyn AnyEnum<'a>> {
        None
    }
    fn as_message(&self) -> Option<&dyn AnyMessage<'a>> {
        Some(self)
    }
    fn as_message_mut(&mut self) -> Option<&mut dyn AnyMessage<'a>> {
        Some(self)
    }
}

impl<'a> AnyMessage<'a> for DynamicMessage<'a> {
    fn merge(&mut self, other: &dyn AnyMessage<'a>) {
        let other = 
            match DynamicMessage::downcast_any_ref(other) {
                Some(other) if self.descriptor() == other.descriptor() => other,
                _ => return,
            };

        let syntax = self.descriptor().file().syntax();

        for field in self.descriptor().fields() {
            match field.accessor().unwrap() {
                FieldAccessor::Single(single) => {
                    match field.field_type() {
                        FieldType::Message(_) | FieldType::Group(_) => {
                            if let Some(value) = single.get(other).expect(FAILED_SELF_ACCESS_ERROR) {
                                single
                                    .get_mut(self)
                                    .expect(FAILED_SELF_ACCESS_ERROR)
                                    .as_message_mut()
                                    .unwrap()
                                    .merge(value.as_message().unwrap());
                            }
                        },
                        FieldType::String => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<String>)) {
                                (Syntax::Proto3, Some(value)) if value.len() != 0 => {
                                    single.set(self, Box::new(Clone::clone(value))).expect(FAILED_SELF_ACCESS_ERROR);
                                },
                                (Syntax::Proto2, Some(value)) => {
                                    single.set(self, Box::new(Clone::clone(value))).expect(FAILED_SELF_ACCESS_ERROR);
                                },
                                _ => { }
                            }
                        },
                        FieldType::Bytes => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<Vec<u8>>)) {
                                (Syntax::Proto3, Some(value)) if value.len() != 0 => {
                                    single.set(self, Box::new(Clone::clone(value))).expect(FAILED_SELF_ACCESS_ERROR);
                                },
                                (Syntax::Proto2, Some(value)) => {
                                    single.set(self, Box::new(Clone::clone(value))).expect(FAILED_SELF_ACCESS_ERROR);
                                },
                                _ => { }
                            }
                        },
                        FieldType::Bool => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<bool>)) {
                                (Syntax::Proto3, Some(true)) => single.set(self, Box::new(true)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Enum(_) => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::as_enum)) {
                                (Syntax::Proto3, Some(value)) if value.get_i32() != 0 => single.set(self, value.clone()).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, value.clone()).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Double => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<f64>)) {
                                (Syntax::Proto3, Some(value)) if *value != 0.0 => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Float => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<f32>)) {
                                (Syntax::Proto3, Some(value)) if *value != 0.0 => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Fixed32 | FieldType::Uint32 => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<u32>)) {
                                (Syntax::Proto3, Some(value)) if *value != 0 => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Fixed64 | FieldType::Uint64 => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<u64>)) {
                                (Syntax::Proto3, Some(value)) if *value != 0 => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Int32 | FieldType::Sfixed32 | FieldType::Sint32 => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<i32>)) {
                                (Syntax::Proto3, Some(value)) if *value != 0 => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                        FieldType::Int64 | FieldType::Sfixed64 | FieldType::Sint64 => {
                            match (syntax, single.get(other).expect(FAILED_SELF_ACCESS_ERROR).and_then(AnyValue::downcast_ref::<i64>)) {
                                (Syntax::Proto3, Some(value)) if *value != 0 => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                (Syntax::Proto2, Some(value)) => single.set(self, Box::new(*value)).expect(FAILED_SELF_ACCESS_ERROR),
                                _ => { }
                            }
                        },
                    }
                },
                FieldAccessor::Repeated(repeated) => {
                    for i in 0..repeated.len(other).expect(FAILED_SELF_ACCESS_ERROR) {
                        let value = repeated.get(other, i).expect(FAILED_SELF_ACCESS_ERROR).expect(MISREPORTED_LEN);
                        repeated.push(self, value.clone()).expect(FAILED_SELF_ACCESS_ERROR);
                    }
                },
                FieldAccessor::Map(map) => {
                    for (key, value) in map.iter(other).expect(FAILED_SELF_ACCESS_ERROR) {
                        map.insert(self, key.clone(), value.clone()).expect(FAILED_SELF_ACCESS_ERROR);
                    }
                }
            }
        }

        if syntax == Syntax::Proto2 && self.descriptor().proto().extension_range().len() != 0 {
            for field in other.fields.keys().filter_map(|n| other.descriptor().find_extension_by_number(*n)) {
                match field.accessor().unwrap() {
                    FieldAccessor::Single(single) => {
                        match field.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                if let Some(value) = single.get(other).expect(FAILED_SELF_ACCESS_ERROR) {
                                    single
                                        .get_mut(self)
                                        .expect(FAILED_SELF_ACCESS_ERROR)
                                        .as_message_mut()
                                        .unwrap()
                                        .merge(value.as_message().unwrap());
                                }
                            },
                            _ => {
                                if let Some(value) = single.get(other).expect(FAILED_SELF_ACCESS_ERROR) {
                                    single.set(self, value.clone()).expect(FAILED_SELF_ACCESS_ERROR);
                                }
                            }
                        }
                    },
                    FieldAccessor::Repeated(repeated) => {
                        for i in 0..repeated.len(other).expect(FAILED_SELF_ACCESS_ERROR) {
                            let value = repeated.get(other, i).expect(FAILED_SELF_ACCESS_ERROR).expect(MISREPORTED_LEN);
                            repeated.push(self, value.clone()).expect(FAILED_SELF_ACCESS_ERROR);
                        }
                    },
                    FieldAccessor::Map(_) => unreachable!("extension fields cannot be map fields"),
                }
            }
        }

        self.unknown_fields.merge(&other.unknown_fields);
    }
    fn descriptor(&self) -> &'a MessageDescriptor<'a> {
        self.descriptor
    }
    fn registry(&self) -> Option<&'static ExtensionRegistry> {
        None
    }
    fn replace_registry(
        &mut self,
        _: Option<&'static ExtensionRegistry>,
    ) -> Option<&'static ExtensionRegistry> {
        None
    }
}

impl<'a> access::SingleFieldAccessor<'a> for FieldDescriptor<'a> {
    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
    ) -> access::Result<'a, Option<&'b dyn AnyValue<'a>>> {
        let instance = DynamicMessage::downcast_any_ref(instance).ok_or(InvalidMessage)?;
        match self.scope() {
            FieldScope::Oneof(o) => {
                match instance.oneofs.get(&o.message_index()) {
                    Some((index, value)) if *index == self.composite_scope_index() => {
                        match value {
                            DynamicFieldValue::Single(value) => Ok(Some(value.as_ref())),
                            _ => unreachable!()
                        }
                    },
                    _ => Ok(None)
                }
            },
            _ => {
                Ok(instance.fields.get(&self.number()).map(|value| {
                    match value {
                        DynamicFieldValue::Single(value) => value.as_ref(),
                        _ => unreachable!()
                    }
                }))
            }
        }
    }
    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
    ) -> access::Result<'a, &'b mut dyn AnyValue<'a>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        let descriptor = instance.descriptor;
        let value = 
            match self.scope() {
                FieldScope::Oneof(o) => {
                    &mut instance.oneofs
                        .entry(o.message_index())
                        .and_modify(|e| {
                            if e.0 != self.composite_scope_index() {
                                *e = (self.composite_scope_index(), DynamicFieldValue::Single(new_default_value(descriptor, self)))
                            }
                        })
                        .or_insert_with(|| (self.composite_scope_index(), DynamicFieldValue::Single(new_default_value(descriptor, self))))
                        .1
                },
                _ => instance.fields.entry(self.number()).or_insert_with(|| DynamicFieldValue::Single(new_default_value(descriptor, self)))
            };
        match value {
            DynamicFieldValue::Single(value) => Ok(value.as_mut()),
            _ => unreachable!()
        }
    }
    fn set(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        value: Box<dyn AnyValue<'a> + 'a>,
    ) -> access::Result<'a, ()> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        let value = check_value(self, value).map_err(|e| FieldAccessError::InvalidValue(e))?;
        match self.scope() {
            FieldScope::Oneof(o) => {
                instance.oneofs.insert(o.message_index(), (self.composite_scope_index(), DynamicFieldValue::Single(value)));
            }
            _ => {
                instance.fields.insert(self.number(), DynamicFieldValue::Single(value));
            }
        }
        Ok(())
    }
    fn take(
        &self,
        instance: &mut dyn AnyMessage<'a>,
    ) -> access::Result<'a, Option<Box<dyn AnyValue<'a> + 'a>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        match self.scope() {
            FieldScope::Oneof(o) => {
                if let Entry::Occupied(o) = instance.oneofs.entry(o.message_index()) {
                    if o.get().0 == self.composite_scope_index() {
                        return Ok(Some(match o.remove().1 {
                            DynamicFieldValue::Single(v) => v,
                            _ => unreachable!()
                        }))
                    }
                }
                Ok(None)
            },
            _ => {
                Ok(instance.fields.remove(&self.number()).map(|value| {
                    match value {
                        DynamicFieldValue::Single(v) => v,
                        _ => unreachable!()
                    }
                }))
            }
        }
    }
    fn clear(&self, instance: &mut dyn AnyMessage<'a>) -> access::Result<'a, ()> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        match self.scope() {
            FieldScope::Oneof(o) => {
                if let Entry::Occupied(o) = instance.oneofs.entry(o.message_index()) {
                    if o.get().0 == self.composite_scope_index() {
                        o.remove();
                    }
                }
            },
            _ => {
                instance.fields.remove(&self.number());
            }
        }
        Ok(())
    }
}

impl<'a> access::RepeatedFieldAccessor<'a> for FieldDescriptor<'a> {
    fn len(&self, instance: &dyn AnyMessage<'a>) -> access::Result<'a, usize> {
        let instance = DynamicMessage::downcast_any_ref(instance).ok_or(InvalidMessage)?;
        Ok(instance.fields.get(&self.number()).map_or(0, |value| {
            match value {
                DynamicFieldValue::Repeated(value) => value.len(),
                _ => unreachable!()
            }
        }))
    }

    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
        index: usize,
    ) -> access::Result<'a, Option<&'b dyn AnyValue<'a>>> {
        let instance = DynamicMessage::downcast_any_ref(instance).ok_or(InvalidMessage)?;
        Ok(instance.fields.get(&self.number()).and_then(|value| {
            match value {
                DynamicFieldValue::Repeated(value) => value.get(index).map(|b| b.as_ref()),
                _ => unreachable!()
            }
        }))
    }
    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
        index: usize,
    ) -> access::Result<'a, Option<&'b mut dyn AnyValue<'a>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        Ok(instance.fields.get_mut(&self.number()).and_then(|value| {
            match value {
                DynamicFieldValue::Repeated(value) => value.get_mut(index).map(|b| b.as_mut()),
                _ => unreachable!()
            }
        }))
    }
    fn push(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        value: Box<dyn AnyValue<'a>>,
    ) -> access::Result<'a, ()> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        let value = check_value(self, value).map_err(|v| FieldAccessError::InvalidValue(v))?;
        match instance.fields.entry(self.number()).or_insert_with(|| DynamicFieldValue::Repeated(Vec::new())) {
            DynamicFieldValue::Repeated(v) => v.push(value),
            _ => unreachable!()
        };
        Ok(())
    }
    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        index: usize,
        value: Box<dyn AnyValue<'a>>,
    ) -> access::Result<'a, ()> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        let value = check_value(self, value).map_err(|v| FieldAccessError::InvalidValue(v))?;
        match instance.fields.entry(self.number()).or_insert_with(|| DynamicFieldValue::Repeated(Vec::new())) {
            DynamicFieldValue::Repeated(v) => v.insert(index, value),
            _ => unreachable!()
        };
        Ok(())
    }
    fn pop(
        &self,
        instance: &mut dyn AnyMessage<'a>,
    ) -> access::Result<'a, Option<Box<dyn AnyValue<'a>>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        match instance.fields.entry(self.number()) {
            Entry::Occupied(mut o) => {
                let value = 
                    match o.get_mut() {
                        DynamicFieldValue::Repeated(v) => (v.pop(), v.is_empty()),
                        _ => unreachable!()
                    };
                if value.1 {
                    o.remove();
                }
                Ok(value.0)
            },
            Entry::Vacant(_) => Ok(None)
        }
    }
    fn remove(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        index: usize,
    ) -> access::Result<'a, Box<dyn AnyValue<'a>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        match instance.fields.entry(self.number()) {
            Entry::Occupied(mut o) => {
                let value = 
                    match o.get_mut() {
                        DynamicFieldValue::Repeated(v) => (v.remove(index), v.is_empty()),
                        _ => unreachable!()
                    };
                if value.1 {
                    o.remove();
                }
                Ok(value.0)
            },
            Entry::Vacant(_) => panic!("assertion failed: index < len")
        }
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'a>) -> access::Result<'a, ()> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        instance.fields.remove(&self.number());
        Ok(())
    }
}

impl<'a> access::MapFieldAccessor<'a> for FieldDescriptor<'a> {
    fn len(&self, instance: &dyn AnyMessage<'a>) -> access::Result<'a, usize> {
        let instance = DynamicMessage::downcast_any_ref(instance).ok_or(InvalidMessage)?;
        Ok(instance.fields.get(&self.number()).map_or(0, |field| {
            match field {
                DynamicFieldValue::Map(field) => field.len(),
                _ => unreachable!()
            }
        }))
    }

    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
        key: &dyn AnyValue<'a>,
    ) -> access::Result<'a, Option<&'b dyn AnyValue<'a>>> {
        let instance = DynamicMessage::downcast_any_ref(instance).ok_or(InvalidMessage)?;
        if let Some(v) = instance.fields.get(&self.number()) {
            match v {
                DynamicFieldValue::Map(v) => {
                    let entry =
                        match key.type_id() {
                            ValueType::Static(t) if t == TypeId::of::<i32>() => {
                                let key = key.downcast_ref::<i32>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Int32(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<i64>() => {
                                let key = key.downcast_ref::<i64>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Int64(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<u32>() => {
                                let key = key.downcast_ref::<u32>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Uint32(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<u64>() => {
                                let key = key.downcast_ref::<u64>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Uint64(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<bool>() => {
                                let key = key.downcast_ref::<bool>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Bool(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<String>() => {
                                let key = key.downcast_ref::<String>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::String(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            _ => unreachable!()
                        };
                    Ok(entry.map(|t| t.1.as_ref()))
                },
                _ => unreachable!()
            }
        } else {
            Ok(None)
        }
    }
    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
        key: &dyn AnyValue<'a>,
    ) -> access::Result<'a, Option<&'b mut dyn AnyValue<'a>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        if let Some(v) = instance.fields.get_mut(&self.number()) {
            match v {
                DynamicFieldValue::Map(v) => {
                    let entry =
                        match key.type_id() {
                            ValueType::Static(t) if t == TypeId::of::<i32>() => {
                                let key = key.downcast_ref::<i32>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry_mut().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Int32(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<i64>() => {
                                let key = key.downcast_ref::<i64>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry_mut().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Int64(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<u32>() => {
                                let key = key.downcast_ref::<u32>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry_mut().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Uint32(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<u64>() => {
                                let key = key.downcast_ref::<u64>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry_mut().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Uint64(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<bool>() => {
                                let key = key.downcast_ref::<bool>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry_mut().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::Bool(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            ValueType::Static(t) if t == TypeId::of::<String>() => {
                                let key = key.downcast_ref::<String>().unwrap();

                                let mut hasher = v.hasher().build_hasher();
                                key.hash(&mut hasher);
                                let hash = hasher.finish();

                                v.raw_entry_mut().from_hash(hash, |k| {
                                    match k {
                                        DynamicKey::String(i) => key == i,
                                        _ => unreachable!()
                                    }
                                })
                            },
                            _ => unreachable!()
                        };
                    match entry {
                        RawEntryMut::Occupied(o) => Ok(Some(o.into_mut().as_mut())),
                        RawEntryMut::Vacant(_) => Ok(None)
                    }
                },
                _ => unreachable!()
            }
        } else {
            Ok(None)
        }
    }

    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        key: Box<dyn AnyValue<'a> + 'a>,
        value: Box<dyn AnyValue<'a> + 'a>,
    ) -> access::Result<'a, Option<Box<dyn AnyValue<'a> + 'a>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        let key = match check_value(self, key) {
            Ok(k) => k,
            Err(e) => return Err(FieldAccessError::InvalidEntry(e, value)),
        };
        let value = match check_value(self, value) {
            Ok(v) => v,
            Err(e) => return Err(FieldAccessError::InvalidEntry(key, e)),
        };
        match instance.fields.entry(self.number()).or_insert_with(|| DynamicFieldValue::Map(HashMap::new())) {
            DynamicFieldValue::Map(v) => {
                let key =
                    match key.type_id() {
                        ValueType::Static(t) if t == TypeId::of::<i32>() => DynamicKey::Int32(*key.downcast::<i32>().unwrap()),
                        ValueType::Static(t) if t == TypeId::of::<i64>() => DynamicKey::Int64(*key.downcast::<i64>().unwrap()),
                        ValueType::Static(t) if t == TypeId::of::<u32>() => DynamicKey::Uint32(*key.downcast::<u32>().unwrap()),
                        ValueType::Static(t) if t == TypeId::of::<u64>() => DynamicKey::Uint64(*key.downcast::<u64>().unwrap()),
                        ValueType::Static(t) if t == TypeId::of::<bool>() => DynamicKey::Bool(*key.downcast::<bool>().unwrap()),
                        ValueType::Static(t) if t == TypeId::of::<String>() => DynamicKey::String(*key.downcast::<String>().unwrap()),
                        _ => unreachable!()
                    };
                Ok(v.insert(key, value))
            },
            _ => unreachable!()
        }
    }
    fn remove(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        key: &dyn AnyValue<'a>,
    ) -> access::Result<'a, Option<Box<dyn AnyValue<'a> + 'a>>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        match instance.fields.entry(self.number()) {
            Entry::Occupied(mut o) => {
                match o.get_mut() {
                    DynamicFieldValue::Map(v) => {
                        let entry =
                            match key.type_id() {
                                ValueType::Static(t) if t == TypeId::of::<i32>() => {
                                    let key = key.downcast_ref::<i32>().unwrap();

                                    let mut hasher = v.hasher().build_hasher();
                                    key.hash(&mut hasher);
                                    let hash = hasher.finish();

                                    v.raw_entry_mut().from_hash(hash, |k| {
                                        match k {
                                            DynamicKey::Int32(i) => key == i,
                                            _ => unreachable!()
                                        }
                                    })
                                },
                                ValueType::Static(t) if t == TypeId::of::<i64>() => {
                                    let key = key.downcast_ref::<i64>().unwrap();

                                    let mut hasher = v.hasher().build_hasher();
                                    key.hash(&mut hasher);
                                    let hash = hasher.finish();

                                    v.raw_entry_mut().from_hash(hash, |k| {
                                        match k {
                                            DynamicKey::Int64(i) => key == i,
                                            _ => unreachable!()
                                        }
                                    })
                                },
                                ValueType::Static(t) if t == TypeId::of::<u32>() => {
                                    let key = key.downcast_ref::<u32>().unwrap();

                                    let mut hasher = v.hasher().build_hasher();
                                    key.hash(&mut hasher);
                                    let hash = hasher.finish();

                                    v.raw_entry_mut().from_hash(hash, |k| {
                                        match k {
                                            DynamicKey::Uint32(i) => key == i,
                                            _ => unreachable!()
                                        }
                                    })
                                },
                                ValueType::Static(t) if t == TypeId::of::<u64>() => {
                                    let key = key.downcast_ref::<u64>().unwrap();

                                    let mut hasher = v.hasher().build_hasher();
                                    key.hash(&mut hasher);
                                    let hash = hasher.finish();

                                    v.raw_entry_mut().from_hash(hash, |k| {
                                        match k {
                                            DynamicKey::Uint64(i) => key == i,
                                            _ => unreachable!()
                                        }
                                    })
                                },
                                ValueType::Static(t) if t == TypeId::of::<bool>() => {
                                    let key = key.downcast_ref::<bool>().unwrap();

                                    let mut hasher = v.hasher().build_hasher();
                                    key.hash(&mut hasher);
                                    let hash = hasher.finish();

                                    v.raw_entry_mut().from_hash(hash, |k| {
                                        match k {
                                            DynamicKey::Bool(i) => key == i,
                                            _ => unreachable!()
                                        }
                                    })
                                },
                                ValueType::Static(t) if t == TypeId::of::<String>() => {
                                    let key = key.downcast_ref::<String>().unwrap();

                                    let mut hasher = v.hasher().build_hasher();
                                    key.hash(&mut hasher);
                                    let hash = hasher.finish();

                                    v.raw_entry_mut().from_hash(hash, |k| {
                                        match k {
                                            DynamicKey::String(i) => key == i,
                                            _ => unreachable!()
                                        }
                                    })
                                },
                                _ => unreachable!()
                            };
                        match entry {
                            RawEntryMut::Occupied(o) => Ok(Some(o.remove())),
                            RawEntryMut::Vacant(_) => Ok(None)
                        }
                    },
                    _ => unreachable!()
                }
            },
            _ => Ok(None)
        }
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'a>) -> access::Result<'a, ()> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        instance.fields.remove(&self.number());
        Ok(())
    }

    fn iter<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
    ) -> access::Result<'a, Box<dyn Iterator<Item = (&'b dyn AnyValue<'a>, &'b dyn AnyValue<'a>)> + 'b>> {
        let instance = DynamicMessage::downcast_any_ref(instance).ok_or(InvalidMessage)?;
        match instance.fields.get(&self.number()) {
            Some(DynamicFieldValue::Map(v)) => Ok(Box::new(v.iter().map(|(k, v)| (k.as_any(), v.as_ref())))),
            None => Ok(Box::new(std::iter::empty())),
            _ => unreachable!()
        }
    }
    fn iter_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
    ) -> access::Result<'a, Box<dyn Iterator<Item = (&'b dyn AnyValue<'a>, &'b mut dyn AnyValue<'a>)> + 'b>> {
        let instance = DynamicMessage::downcast_any_mut(instance).ok_or(InvalidMessage)?;
        match instance.fields.get_mut(&self.number()) {
            Some(DynamicFieldValue::Map(v)) => Ok(Box::new(v.iter_mut().map(|(k, v)| (k.as_any(), v.as_mut())))),
            None => Ok(Box::new(std::iter::empty())),
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
enum DynamicFieldValue<'a> {
    Single(Box<dyn AnyValue<'a>>),
    Repeated(Vec<Box<dyn AnyValue<'a>>>),
    Map(HashMap<DynamicKey, Box<dyn AnyValue<'a>>>),
}

#[derive(Clone, PartialEq, Eq)]
enum DynamicKey {
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Bool(bool),
    String(String)
}

impl<'a> DynamicKey {
    fn as_any(&self) -> &dyn AnyValue<'a> {
        match self {
            DynamicKey::Int32(ref v) => v,
            DynamicKey::Int64(ref v) => v,
            DynamicKey::Uint32(ref v) => v,
            DynamicKey::Uint64(ref v) => v,
            DynamicKey::Bool(ref v) => v,
            DynamicKey::String(ref v) => v,
        }
    }
}

// don't use the default since it uses discriminants
impl Hash for DynamicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DynamicKey::Int32(v) => v.hash(state),
            DynamicKey::Int64(v) => v.hash(state),
            DynamicKey::Uint32(v) => v.hash(state),
            DynamicKey::Uint64(v) => v.hash(state),
            DynamicKey::Bool(v) => v.hash(state),
            DynamicKey::String(v) => v.hash(state)
        }
    }
}

impl PartialEq for DynamicFieldValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DynamicFieldValue::Single(v), DynamicFieldValue::Single(v2)) => v == v2,
            (DynamicFieldValue::Repeated(v), DynamicFieldValue::Repeated(v2)) => v == v2,
            (DynamicFieldValue::Map(v), DynamicFieldValue::Map(v2)) => v == v2,
            _ => false
        }
    }
}

impl<'a> DynamicFieldValue<'a> {
    fn new(field: &'a FieldDescriptor<'a>) -> DynamicFieldValue<'a> {
        match (field.label(), field.field_type()) {
            (FieldLabel::Optional, _) | (FieldLabel::Required, _) => {
                DynamicFieldValue::Single(new_default_value(field.message(), field))
            },
            (FieldLabel::Repeated, FieldType::Message(m)) if m.is_map_entry() => {
                DynamicFieldValue::Map(HashMap::new())
            },
            (FieldLabel::Repeated, _) => {
                DynamicFieldValue::Repeated(Vec::new())
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DynamicEnum<'a> {
    descriptor: &'a EnumDescriptor<'a>,
    value: DynamicEnumValue<'a>,
}

impl<'a> DynamicEnum<'a> {
    pub fn new(descriptor: &'a EnumDescriptor<'a>, value: i32) -> DynamicEnum<'a> {
        let value = 
            descriptor
                .values()
                .iter()
                .find(move |r| r.number() == value)
                .map(|r| DynamicEnumValue::Defined(r))
                .unwrap_or(DynamicEnumValue::Undefined(value));
        DynamicEnum { descriptor, value }
    }
}

#[derive(Clone, PartialEq)]
enum DynamicEnumValue<'a> {
    Defined(&'a EnumValueDescriptor<'a>),
    Undefined(i32),
}

impl Debug for DynamicEnum<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.value {
            DynamicEnumValue::Defined(defined) => {
                f.debug_tuple("Defined").field(&defined.name()).finish()
            }
            DynamicEnumValue::Undefined(undefined) => {
                f.debug_tuple("Undefined").field(&undefined).finish()
            }
        }
    }
}

impl<'a> AnyValue<'a> for DynamicEnum<'a> {
    fn clone(&self) -> Box<dyn AnyValue<'a>> {
        Box::new(Clone::clone(self))
    }
    fn eq(&self, other: &dyn AnyValue<'a>) -> bool {
        match other.type_id() {
            ValueType::Dynamic(DynamicType::Enum) => {
                let other = unsafe { &*(other as *const dyn AnyValue as *const DynamicEnum<'a>) };
                PartialEq::eq(self, other)
            }
            _ => false,
        }
    }

    fn type_id(&self) -> ValueType {
        ValueType::Dynamic(DynamicType::Enum)
    }
    fn as_enum(&self) -> Option<&dyn AnyEnum<'a>> {
        Some(self)
    }
    fn as_enum_mut(&mut self) -> Option<&mut dyn AnyEnum<'a>> {
        Some(self)
    }
    fn as_message(&self) -> Option<&dyn AnyMessage<'a>> {
        None
    }
    fn as_message_mut(&mut self) -> Option<&mut dyn AnyMessage<'a>> {
        None
    }
}

impl<'a> AnyEnum<'a> for DynamicEnum<'a> {
    fn descriptor(&self) -> &'a EnumDescriptor<'a> {
        self.descriptor
    }

    fn get(&self) -> Option<&'a EnumValueDescriptor<'a>> {
        match self.value {
            DynamicEnumValue::Defined(d) => Some(d),
            DynamicEnumValue::Undefined(_) => None,
        }
    }

    fn get_i32(&self) -> i32 {
        match self.value {
            DynamicEnumValue::Defined(d) => d.number(),
            DynamicEnumValue::Undefined(u) => u,
        }
    }

    fn set(&mut self, value: &'a EnumValueDescriptor<'a>) -> bool {
        if self.descriptor() != value.enum_type() {
            false
        } else {
            self.value = DynamicEnumValue::Defined(value);
            true
        }
    }

    fn set_i32(&mut self, value: i32) {
        self.value = self
            .descriptor
            .values()
            .iter()
            .find(move |r| r.number() == value)
            .map(|r| DynamicEnumValue::Defined(r))
            .unwrap_or(DynamicEnumValue::Undefined(value))
    }
}