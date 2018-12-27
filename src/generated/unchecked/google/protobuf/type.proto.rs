//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/type.proto


#[derive(Clone, PartialEq)]
pub struct Type {
    pub name: std::string::String,
    pub fields: crate::collections::RepeatedField<std::boxed::Box<self::Field>>,
    pub oneofs: crate::collections::RepeatedField<std::string::String>,
    pub options: crate::collections::RepeatedField<std::boxed::Box<self::Option>>,
    pub source_context: std::option::Option<std::boxed::Box<super::google_protobuf_source_context_proto::SourceContext>>,
    pub syntax: crate::EnumValue<self::Syntax>,
    _unknown_fields: crate::UnknownFieldSet
}
static TYPE_NAME_DEFAULT_VALUE: &'static str = "";
static TYPE_FIELDS_CODEC: crate::Codec<std::boxed::Box<self::Field>> = crate::Codec::message(18);
static TYPE_ONEOFS_CODEC: crate::Codec<std::string::String> = crate::Codec::string(26);
static TYPE_OPTIONS_CODEC: crate::Codec<std::boxed::Box<self::Option>> = crate::Codec::message(34);
static TYPE_SYNTAX_DEFAULT_VALUE: crate::EnumValue<self::Syntax> = crate::EnumValue::Defined(self::Syntax::Proto2);
impl crate::CodedMessage for self::Type {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.fields.add_entries(tag.get(), input, &TYPE_FIELDS_CODEC)?,
                26 => self.oneofs.add_entries(tag.get(), input, &TYPE_ONEOFS_CODEC)?,
                34 => self.options.add_entries(tag.get(), input, &TYPE_OPTIONS_CODEC)?,
                42 => input.read_message(self.source_context.get_or_insert_with(crate::LiteMessage::new))?,
                48 => self.syntax = input.read_enum_value()?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != TYPE_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        size += self.fields.calculate_size(&TYPE_FIELDS_CODEC);
        size += self.oneofs.calculate_size(&TYPE_ONEOFS_CODEC);
        size += self.options.calculate_size(&TYPE_OPTIONS_CODEC);
        let source_context = &self.source_context;
        if let std::option::Option::Some(source_context) = source_context {
            size += 1;
            size += crate::io::sizes::message(source_context);
        }
        let syntax = self.syntax;
        if syntax != TYPE_SYNTAX_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::enum_value(syntax);
        }
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != TYPE_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        self.fields.write_to(output, &TYPE_FIELDS_CODEC)?;
        self.oneofs.write_to(output, &TYPE_ONEOFS_CODEC)?;
        self.options.write_to(output, &TYPE_OPTIONS_CODEC)?;
        let source_context = &self.source_context;
        if let std::option::Option::Some(source_context) = source_context {
            output.write_raw_tag_bytes(&[42])?;
            output.write_message(source_context)?;
        }
        let syntax = self.syntax;
        if syntax != TYPE_SYNTAX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[48])?;
            output.write_enum_value(syntax)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Type {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            fields: crate::collections::RepeatedField::new(),
            oneofs: crate::collections::RepeatedField::new(),
            options: crate::collections::RepeatedField::new(),
            source_context: std::option::Option::None,
            syntax: TYPE_SYNTAX_DEFAULT_VALUE,
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.fields.merge(&other.fields);
        self.oneofs.merge(&other.oneofs);
        self.options.merge(&other.options);
        if let std::option::Option::Some(source_context) = &other.source_context {
            self.source_context.get_or_insert_with(crate::LiteMessage::new).merge(source_context);
        }
        self.syntax = other.syntax;
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::Type {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Type {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'fields' field
    pub const FIELDS_FIELD_NUMBER: i32 = 2;
    /// Gets the field number of the 'oneofs' field
    pub const ONEOFS_FIELD_NUMBER: i32 = 3;
    /// Gets the field number of the 'options' field
    pub const OPTIONS_FIELD_NUMBER: i32 = 4;
    /// Gets the field number of the 'source_context' field
    pub const SOURCE_CONTEXT_FIELD_NUMBER: i32 = 5;
    /// Gets the field number of the 'syntax' field
    pub const SYNTAX_FIELD_NUMBER: i32 = 6;
}
#[derive(Clone, PartialEq)]
pub struct Field {
    pub kind: crate::EnumValue<self::Field_Kind>,
    pub cardinality: crate::EnumValue<self::Field_Cardinality>,
    pub number: i32,
    pub name: std::string::String,
    pub type_url: std::string::String,
    pub oneof_index: i32,
    pub packed: bool,
    pub options: crate::collections::RepeatedField<std::boxed::Box<self::Option>>,
    pub json_name: std::string::String,
    pub default_value: std::string::String,
    _unknown_fields: crate::UnknownFieldSet
}
static FIELD_KIND_DEFAULT_VALUE: crate::EnumValue<self::Field_Kind> = crate::EnumValue::Defined(self::Field_Kind::TypeUnknown);
static FIELD_CARDINALITY_DEFAULT_VALUE: crate::EnumValue<self::Field_Cardinality> = crate::EnumValue::Defined(self::Field_Cardinality::Unknown);
static FIELD_NUMBER_DEFAULT_VALUE: i32 = 0;
static FIELD_NAME_DEFAULT_VALUE: &'static str = "";
static FIELD_TYPE_URL_DEFAULT_VALUE: &'static str = "";
static FIELD_ONEOF_INDEX_DEFAULT_VALUE: i32 = 0;
static FIELD_PACKED_DEFAULT_VALUE: bool = false;
static FIELD_OPTIONS_CODEC: crate::Codec<std::boxed::Box<self::Option>> = crate::Codec::message(74);
static FIELD_JSON_NAME_DEFAULT_VALUE: &'static str = "";
static FIELD_DEFAULT_VALUE_DEFAULT_VALUE: &'static str = "";
impl crate::CodedMessage for self::Field {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.kind = input.read_enum_value()?,
                16 => self.cardinality = input.read_enum_value()?,
                24 => self.number = input.read_int32()?,
                34 => self.name = input.read_string()?,
                50 => self.type_url = input.read_string()?,
                56 => self.oneof_index = input.read_int32()?,
                64 => self.packed = input.read_bool()?,
                74 => self.options.add_entries(tag.get(), input, &FIELD_OPTIONS_CODEC)?,
                82 => self.json_name = input.read_string()?,
                90 => self.default_value = input.read_string()?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let kind = self.kind;
        if kind != FIELD_KIND_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::enum_value(kind);
        }
        let cardinality = self.cardinality;
        if cardinality != FIELD_CARDINALITY_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::enum_value(cardinality);
        }
        let number = self.number;
        if number != FIELD_NUMBER_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int32(number);
        }
        let name = &self.name;
        if name != FIELD_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        let type_url = &self.type_url;
        if type_url != FIELD_TYPE_URL_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(type_url);
        }
        let oneof_index = self.oneof_index;
        if oneof_index != FIELD_ONEOF_INDEX_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int32(oneof_index);
        }
        let packed = self.packed;
        if packed != FIELD_PACKED_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::bool(packed);
        }
        size += self.options.calculate_size(&FIELD_OPTIONS_CODEC);
        let json_name = &self.json_name;
        if json_name != FIELD_JSON_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(json_name);
        }
        let default_value = &self.default_value;
        if default_value != FIELD_DEFAULT_VALUE_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(default_value);
        }
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let kind = self.kind;
        if kind != FIELD_KIND_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_enum_value(kind)?;
        }
        let cardinality = self.cardinality;
        if cardinality != FIELD_CARDINALITY_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[16])?;
            output.write_enum_value(cardinality)?;
        }
        let number = self.number;
        if number != FIELD_NUMBER_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[24])?;
            output.write_int32(number)?;
        }
        let name = &self.name;
        if name != FIELD_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(name)?;
        }
        let type_url = &self.type_url;
        if type_url != FIELD_TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[50])?;
            output.write_string(type_url)?;
        }
        let oneof_index = self.oneof_index;
        if oneof_index != FIELD_ONEOF_INDEX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[56])?;
            output.write_int32(oneof_index)?;
        }
        let packed = self.packed;
        if packed != FIELD_PACKED_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[64])?;
            output.write_bool(packed)?;
        }
        self.options.write_to(output, &FIELD_OPTIONS_CODEC)?;
        let json_name = &self.json_name;
        if json_name != FIELD_JSON_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[82])?;
            output.write_string(json_name)?;
        }
        let default_value = &self.default_value;
        if default_value != FIELD_DEFAULT_VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[90])?;
            output.write_string(default_value)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Field {
    fn new() -> Self {
        Self {
            kind: FIELD_KIND_DEFAULT_VALUE,
            cardinality: FIELD_CARDINALITY_DEFAULT_VALUE,
            number: FIELD_NUMBER_DEFAULT_VALUE,
            name: std::string::String::new(),
            type_url: std::string::String::new(),
            oneof_index: FIELD_ONEOF_INDEX_DEFAULT_VALUE,
            packed: FIELD_PACKED_DEFAULT_VALUE,
            options: crate::collections::RepeatedField::new(),
            json_name: std::string::String::new(),
            default_value: std::string::String::new(),
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.kind = other.kind;
        self.cardinality = other.cardinality;
        self.number = other.number;
        self.name = other.name.clone();
        self.type_url = other.type_url.clone();
        self.oneof_index = other.oneof_index;
        self.packed = other.packed;
        self.options.merge(&other.options);
        self.json_name = other.json_name.clone();
        self.default_value = other.default_value.clone();
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::Field {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Field {
    /// Gets the field number of the 'kind' field
    pub const KIND_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'cardinality' field
    pub const CARDINALITY_FIELD_NUMBER: i32 = 2;
    /// Gets the field number of the 'number' field
    pub const NUMBER_FIELD_NUMBER: i32 = 3;
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 4;
    /// Gets the field number of the 'type_url' field
    pub const TYPE_URL_FIELD_NUMBER: i32 = 6;
    /// Gets the field number of the 'oneof_index' field
    pub const ONEOF_INDEX_FIELD_NUMBER: i32 = 7;
    /// Gets the field number of the 'packed' field
    pub const PACKED_FIELD_NUMBER: i32 = 8;
    /// Gets the field number of the 'options' field
    pub const OPTIONS_FIELD_NUMBER: i32 = 9;
    /// Gets the field number of the 'json_name' field
    pub const JSON_NAME_FIELD_NUMBER: i32 = 10;
    /// Gets the field number of the 'default_value' field
    pub const DEFAULT_VALUE_FIELD_NUMBER: i32 = 11;
}
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Field_Kind {
    TypeUnknown = 0,
    TypeDouble = 1,
    TypeFloat = 2,
    TypeInt64 = 3,
    TypeUint64 = 4,
    TypeInt32 = 5,
    TypeFixed64 = 6,
    TypeFixed32 = 7,
    TypeBool = 8,
    TypeString = 9,
    TypeGroup = 10,
    TypeMessage = 11,
    TypeBytes = 12,
    TypeUint32 = 13,
    TypeEnum = 14,
    TypeSfixed32 = 15,
    TypeSfixed64 = 16,
    TypeSint32 = 17,
    TypeSint64 = 18,
}
impl std::convert::TryFrom<i32> for self::Field_Kind {
    type Error = crate::VariantUndefinedError;
    
    fn try_from(value: i32) -> std::result::Result<Self, crate::VariantUndefinedError> {
        match value {
            0 => std::result::Result::Ok(self::Field_Kind::TypeUnknown),
            1 => std::result::Result::Ok(self::Field_Kind::TypeDouble),
            2 => std::result::Result::Ok(self::Field_Kind::TypeFloat),
            3 => std::result::Result::Ok(self::Field_Kind::TypeInt64),
            4 => std::result::Result::Ok(self::Field_Kind::TypeUint64),
            5 => std::result::Result::Ok(self::Field_Kind::TypeInt32),
            6 => std::result::Result::Ok(self::Field_Kind::TypeFixed64),
            7 => std::result::Result::Ok(self::Field_Kind::TypeFixed32),
            8 => std::result::Result::Ok(self::Field_Kind::TypeBool),
            9 => std::result::Result::Ok(self::Field_Kind::TypeString),
            10 => std::result::Result::Ok(self::Field_Kind::TypeGroup),
            11 => std::result::Result::Ok(self::Field_Kind::TypeMessage),
            12 => std::result::Result::Ok(self::Field_Kind::TypeBytes),
            13 => std::result::Result::Ok(self::Field_Kind::TypeUint32),
            14 => std::result::Result::Ok(self::Field_Kind::TypeEnum),
            15 => std::result::Result::Ok(self::Field_Kind::TypeSfixed32),
            16 => std::result::Result::Ok(self::Field_Kind::TypeSfixed64),
            17 => std::result::Result::Ok(self::Field_Kind::TypeSint32),
            18 => std::result::Result::Ok(self::Field_Kind::TypeSint64),
            _ => std::result::Result::Err(crate::VariantUndefinedError)
        }
    }
}
impl std::convert::From<self::Field_Kind> for i32 {
    fn from(value: self::Field_Kind) -> i32 {
        value as i32
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Field_Cardinality {
    Unknown = 0,
    Optional = 1,
    Required = 2,
    Repeated = 3,
}
impl std::convert::TryFrom<i32> for self::Field_Cardinality {
    type Error = crate::VariantUndefinedError;
    
    fn try_from(value: i32) -> std::result::Result<Self, crate::VariantUndefinedError> {
        match value {
            0 => std::result::Result::Ok(self::Field_Cardinality::Unknown),
            1 => std::result::Result::Ok(self::Field_Cardinality::Optional),
            2 => std::result::Result::Ok(self::Field_Cardinality::Required),
            3 => std::result::Result::Ok(self::Field_Cardinality::Repeated),
            _ => std::result::Result::Err(crate::VariantUndefinedError)
        }
    }
}
impl std::convert::From<self::Field_Cardinality> for i32 {
    fn from(value: self::Field_Cardinality) -> i32 {
        value as i32
    }
}
#[derive(Clone, PartialEq)]
pub struct Enum {
    pub name: std::string::String,
    pub enumvalue: crate::collections::RepeatedField<std::boxed::Box<self::EnumValue>>,
    pub options: crate::collections::RepeatedField<std::boxed::Box<self::Option>>,
    pub source_context: std::option::Option<std::boxed::Box<super::google_protobuf_source_context_proto::SourceContext>>,
    pub syntax: crate::EnumValue<self::Syntax>,
    _unknown_fields: crate::UnknownFieldSet
}
static ENUM_NAME_DEFAULT_VALUE: &'static str = "";
static ENUM_ENUMVALUE_CODEC: crate::Codec<std::boxed::Box<self::EnumValue>> = crate::Codec::message(18);
static ENUM_OPTIONS_CODEC: crate::Codec<std::boxed::Box<self::Option>> = crate::Codec::message(26);
static ENUM_SYNTAX_DEFAULT_VALUE: crate::EnumValue<self::Syntax> = crate::EnumValue::Defined(self::Syntax::Proto2);
impl crate::CodedMessage for self::Enum {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.enumvalue.add_entries(tag.get(), input, &ENUM_ENUMVALUE_CODEC)?,
                26 => self.options.add_entries(tag.get(), input, &ENUM_OPTIONS_CODEC)?,
                34 => input.read_message(self.source_context.get_or_insert_with(crate::LiteMessage::new))?,
                40 => self.syntax = input.read_enum_value()?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != ENUM_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        size += self.enumvalue.calculate_size(&ENUM_ENUMVALUE_CODEC);
        size += self.options.calculate_size(&ENUM_OPTIONS_CODEC);
        let source_context = &self.source_context;
        if let std::option::Option::Some(source_context) = source_context {
            size += 1;
            size += crate::io::sizes::message(source_context);
        }
        let syntax = self.syntax;
        if syntax != ENUM_SYNTAX_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::enum_value(syntax);
        }
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != ENUM_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        self.enumvalue.write_to(output, &ENUM_ENUMVALUE_CODEC)?;
        self.options.write_to(output, &ENUM_OPTIONS_CODEC)?;
        let source_context = &self.source_context;
        if let std::option::Option::Some(source_context) = source_context {
            output.write_raw_tag_bytes(&[34])?;
            output.write_message(source_context)?;
        }
        let syntax = self.syntax;
        if syntax != ENUM_SYNTAX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[40])?;
            output.write_enum_value(syntax)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Enum {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            enumvalue: crate::collections::RepeatedField::new(),
            options: crate::collections::RepeatedField::new(),
            source_context: std::option::Option::None,
            syntax: ENUM_SYNTAX_DEFAULT_VALUE,
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.enumvalue.merge(&other.enumvalue);
        self.options.merge(&other.options);
        if let std::option::Option::Some(source_context) = &other.source_context {
            self.source_context.get_or_insert_with(crate::LiteMessage::new).merge(source_context);
        }
        self.syntax = other.syntax;
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::Enum {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Enum {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'enumvalue' field
    pub const ENUMVALUE_FIELD_NUMBER: i32 = 2;
    /// Gets the field number of the 'options' field
    pub const OPTIONS_FIELD_NUMBER: i32 = 3;
    /// Gets the field number of the 'source_context' field
    pub const SOURCE_CONTEXT_FIELD_NUMBER: i32 = 4;
    /// Gets the field number of the 'syntax' field
    pub const SYNTAX_FIELD_NUMBER: i32 = 5;
}
#[derive(Clone, PartialEq)]
pub struct EnumValue {
    pub name: std::string::String,
    pub number: i32,
    pub options: crate::collections::RepeatedField<std::boxed::Box<self::Option>>,
    _unknown_fields: crate::UnknownFieldSet
}
static ENUM_VALUE_NAME_DEFAULT_VALUE: &'static str = "";
static ENUM_VALUE_NUMBER_DEFAULT_VALUE: i32 = 0;
static ENUM_VALUE_OPTIONS_CODEC: crate::Codec<std::boxed::Box<self::Option>> = crate::Codec::message(26);
impl crate::CodedMessage for self::EnumValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                16 => self.number = input.read_int32()?,
                26 => self.options.add_entries(tag.get(), input, &ENUM_VALUE_OPTIONS_CODEC)?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != ENUM_VALUE_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        let number = self.number;
        if number != ENUM_VALUE_NUMBER_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int32(number);
        }
        size += self.options.calculate_size(&ENUM_VALUE_OPTIONS_CODEC);
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != ENUM_VALUE_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let number = self.number;
        if number != ENUM_VALUE_NUMBER_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[16])?;
            output.write_int32(number)?;
        }
        self.options.write_to(output, &ENUM_VALUE_OPTIONS_CODEC)?;
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::EnumValue {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            number: ENUM_VALUE_NUMBER_DEFAULT_VALUE,
            options: crate::collections::RepeatedField::new(),
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.number = other.number;
        self.options.merge(&other.options);
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::EnumValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::EnumValue {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'number' field
    pub const NUMBER_FIELD_NUMBER: i32 = 2;
    /// Gets the field number of the 'options' field
    pub const OPTIONS_FIELD_NUMBER: i32 = 3;
}
#[derive(Clone, PartialEq)]
pub struct Option {
    pub name: std::string::String,
    pub value: std::option::Option<std::boxed::Box<super::google_protobuf_any_proto::Any>>,
    _unknown_fields: crate::UnknownFieldSet
}
static OPTION_NAME_DEFAULT_VALUE: &'static str = "";
impl crate::CodedMessage for self::Option {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => input.read_message(self.value.get_or_insert_with(crate::LiteMessage::new))?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != OPTION_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        let value = &self.value;
        if let std::option::Option::Some(value) = value {
            size += 1;
            size += crate::io::sizes::message(value);
        }
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != OPTION_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let value = &self.value;
        if let std::option::Option::Some(value) = value {
            output.write_raw_tag_bytes(&[18])?;
            output.write_message(value)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Option {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            value: std::option::Option::None,
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        if let std::option::Option::Some(value) = &other.value {
            self.value.get_or_insert_with(crate::LiteMessage::new).merge(value);
        }
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::Option {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Option {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 2;
}
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Syntax {
    Proto2 = 0,
    Proto3 = 1,
}
impl std::convert::TryFrom<i32> for self::Syntax {
    type Error = crate::VariantUndefinedError;
    
    fn try_from(value: i32) -> std::result::Result<Self, crate::VariantUndefinedError> {
        match value {
            0 => std::result::Result::Ok(self::Syntax::Proto2),
            1 => std::result::Result::Ok(self::Syntax::Proto3),
            _ => std::result::Result::Err(crate::VariantUndefinedError)
        }
    }
}
impl std::convert::From<self::Syntax> for i32 {
    fn from(value: self::Syntax) -> i32 {
        value as i32
    }
}