//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/struct.proto

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub fields: crate::collections::MapField<std::string::String, self::Value>,
    unknown_fields: crate::UnknownFieldSet
}
static STRUCT_FIELDS_CODEC: crate::collections::MapCodec<std::string::String, self::Value> = crate::collections::MapCodec::new(crate::Codec::string(10), crate::Codec::message(18), 10);
impl crate::CodedMessage for self::Struct {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.fields.add_entries(input, &STRUCT_FIELDS_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.fields.calculate_size(&STRUCT_FIELDS_CODEC);
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.fields.write_to(output, &STRUCT_FIELDS_CODEC)?;
        self.unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Struct {
    fn new() -> Self {
        Self {
            fields: crate::collections::MapField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::Struct {
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.fields.clone_from(&other.fields);
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Struct {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Struct {
    /// Gets the field number of the 'fields' field
    pub const FIELDS_FIELD_NUMBER: i32 = 1;
}
#[derive(Debug, PartialEq)]
pub struct Value {
    pub kind: Value_Kind,
    unknown_fields: crate::UnknownFieldSet
}
#[derive(Clone, Debug, PartialEq)]
pub enum Value_Kind {
    None,
    NullValue(crate::EnumValue<self::NullValue>),
    NumberValue(f64),
    StringValue(std::string::String),
    BoolValue(bool),
    StructValue(std::boxed::Box<self::Struct>),
    ListValue(std::boxed::Box<self::ListValue>),
}

impl crate::CodedMessage for self::Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.kind = self::Value_Kind::NullValue(input.read_enum_value()?),
                17 => self.kind = self::Value_Kind::NumberValue(input.read_double()?),
                26 => self.kind = self::Value_Kind::StringValue(input.read_string()?),
                32 => self.kind = self::Value_Kind::BoolValue(input.read_bool()?),
                42 => 
                    if let self::Value_Kind::StructValue(kind) = &mut self.kind {
                        kind.merge_from(input)?;
                    } else {
                        
                        let mut kind = std::boxed::Box::new(<self::Struct as crate::LiteMessage>::new());
                        kind.merge_from(input)?;
                        self.kind = self::Value_Kind::StructValue(kind)
                    },
                50 => 
                    if let self::Value_Kind::ListValue(kind) = &mut self.kind {
                        kind.merge_from(input)?;
                    } else {
                        
                        let mut kind = std::boxed::Box::new(<self::ListValue as crate::LiteMessage>::new());
                        kind.merge_from(input)?;
                        self.kind = self::Value_Kind::ListValue(kind)
                    },
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        if let self::Value_Kind::NullValue(kind) = self.kind {
            size += 1;
            size += crate::io::sizes::enum_value(kind);
        }
        if let self::Value_Kind::NumberValue(kind) = self.kind {
            size += 1;
            size += crate::io::sizes::double(kind);
        }
        if let self::Value_Kind::StringValue(kind) = &self.kind {
            size += 1;
            size += crate::io::sizes::string(kind);
        }
        if let self::Value_Kind::BoolValue(kind) = self.kind {
            size += 1;
            size += crate::io::sizes::bool(kind);
        }
        if let self::Value_Kind::StructValue(kind) = &self.kind {
            size += 1;
            size += crate::io::sizes::message(kind);
        }
        if let self::Value_Kind::ListValue(kind) = &self.kind {
            size += 1;
            size += crate::io::sizes::message(kind);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        if let self::Value_Kind::NullValue(kind) = self.kind {
            output.write_raw_tag_bytes(&[8])?;
            output.write_enum_value(kind)?;
        }
        if let self::Value_Kind::NumberValue(kind) = self.kind {
            output.write_raw_tag_bytes(&[17])?;
            output.write_double(kind)?;
        }
        if let self::Value_Kind::StringValue(kind) = &self.kind {
            output.write_raw_tag_bytes(&[26])?;
            output.write_string(kind)?;
        }
        if let self::Value_Kind::BoolValue(kind) = self.kind {
            output.write_raw_tag_bytes(&[32])?;
            output.write_bool(kind)?;
        }
        if let self::Value_Kind::StructValue(kind) = &self.kind {
            output.write_raw_tag_bytes(&[42])?;
            output.write_message(kind)?;
        }
        if let self::Value_Kind::ListValue(kind) = &self.kind {
            output.write_raw_tag_bytes(&[50])?;
            output.write_message(kind)?;
        }
        self.unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Value {
    fn new() -> Self {
        Self {
            kind: self::Value_Kind::None,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::Value {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        if let self::Value_Kind::NullValue(kind) = other.kind {
            self.kind = self::Value_Kind::NullValue(kind);
        }
        if let self::Value_Kind::NumberValue(kind) = other.kind {
            self.kind = self::Value_Kind::NumberValue(kind);
        }
        if let self::Value_Kind::StringValue(kind) = &other.kind {
            self.kind = self::Value_Kind::StringValue(kind.clone());
        }
        if let self::Value_Kind::BoolValue(kind) = other.kind {
            self.kind = self::Value_Kind::BoolValue(kind);
        }
        if let self::Value_Kind::StructValue(kind) = &other.kind {
            if let self::Value_Kind::StructValue(existing) = &mut self.kind {
                existing.clone_from(kind);
            } else {
                self.kind = self::Value_Kind::StructValue(kind.clone());
            }
        }
        if let self::Value_Kind::ListValue(kind) = &other.kind {
            if let self::Value_Kind::ListValue(existing) = &mut self.kind {
                existing.clone_from(kind);
            } else {
                self.kind = self::Value_Kind::ListValue(kind.clone());
            }
        }
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Value {
}
#[derive(Debug, PartialEq)]
pub struct ListValue {
    pub values: crate::collections::RepeatedField<self::Value>,
    unknown_fields: crate::UnknownFieldSet
}
static LIST_VALUE_VALUES_CODEC: crate::Codec<self::Value> = crate::Codec::message(10);
impl crate::CodedMessage for self::ListValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.values.add_entries(tag.get(), input, &LIST_VALUE_VALUES_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.values.calculate_size(&LIST_VALUE_VALUES_CODEC);
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.values.write_to(output, &LIST_VALUE_VALUES_CODEC)?;
        self.unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::ListValue {
    fn new() -> Self {
        Self {
            values: crate::collections::RepeatedField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::ListValue {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.values.clone_from(&other.values);
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::ListValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::ListValue {
    /// Gets the field number of the 'values' field
    pub const VALUES_FIELD_NUMBER: i32 = 1;
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NullValue {
    NullValue = 0,
}
impl std::convert::TryFrom<i32> for self::NullValue {
    type Error = crate::VariantUndefinedError;
    
    fn try_from(value: i32) -> std::result::Result<Self, crate::VariantUndefinedError> {
        match value {
            0 => std::result::Result::Ok(self::NullValue::NullValue),
            _ => std::result::Result::Err(crate::VariantUndefinedError)
        }
    }
}
impl std::convert::From<self::NullValue> for i32 {
    fn from(value: self::NullValue) -> i32 {
        value as i32
    }
}