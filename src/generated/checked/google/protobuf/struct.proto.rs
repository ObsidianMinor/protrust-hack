// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/struct.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 0]> = ::std::option::Option::None;
static FILE_BINARY: &'static [u8] = &[
    10, 28, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 115, 116, 
    114, 117, 99, 116, 46, 112, 114, 111, 116, 111, 18, 15, 103, 111, 111, 103, 108, 101, 46, 112, 
    114, 111, 116, 111, 98, 117, 102, 34, 152, 1, 10, 6, 83, 116, 114, 117, 99, 116, 18, 59, 
    10, 6, 102, 105, 101, 108, 100, 115, 24, 1, 32, 3, 40, 11, 50, 35, 46, 103, 111, 111, 
    103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 83, 116, 114, 117, 99, 116, 46, 
    70, 105, 101, 108, 100, 115, 69, 110, 116, 114, 121, 82, 6, 102, 105, 101, 108, 100, 115, 26, 
    81, 10, 11, 70, 105, 101, 108, 100, 115, 69, 110, 116, 114, 121, 18, 16, 10, 3, 107, 101, 
    121, 24, 1, 32, 1, 40, 9, 82, 3, 107, 101, 121, 18, 44, 10, 5, 118, 97, 108, 117, 
    101, 24, 2, 32, 1, 40, 11, 50, 22, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 
    116, 111, 98, 117, 102, 46, 86, 97, 108, 117, 101, 82, 5, 118, 97, 108, 117, 101, 58, 2, 
    56, 1, 34, 166, 2, 10, 5, 86, 97, 108, 117, 101, 18, 57, 10, 10, 110, 117, 108, 108, 
    95, 118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 14, 50, 26, 46, 103, 111, 111, 103, 108, 
    101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 78, 117, 108, 108, 86, 97, 108, 117, 101, 
    82, 9, 110, 117, 108, 108, 86, 97, 108, 117, 101, 18, 33, 10, 12, 110, 117, 109, 98, 101, 
    114, 95, 118, 97, 108, 117, 101, 24, 2, 32, 1, 40, 1, 82, 11, 110, 117, 109, 98, 101, 
    114, 86, 97, 108, 117, 101, 18, 33, 10, 12, 115, 116, 114, 105, 110, 103, 95, 118, 97, 108, 
    117, 101, 24, 3, 32, 1, 40, 9, 82, 11, 115, 116, 114, 105, 110, 103, 86, 97, 108, 117, 
    101, 18, 29, 10, 10, 98, 111, 111, 108, 95, 118, 97, 108, 117, 101, 24, 4, 32, 1, 40, 
    8, 82, 9, 98, 111, 111, 108, 86, 97, 108, 117, 101, 18, 58, 10, 12, 115, 116, 114, 117, 
    99, 116, 95, 118, 97, 108, 117, 101, 24, 5, 32, 1, 40, 11, 50, 23, 46, 103, 111, 111, 
    103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 83, 116, 114, 117, 99, 116, 82, 
    11, 115, 116, 114, 117, 99, 116, 86, 97, 108, 117, 101, 18, 57, 10, 10, 108, 105, 115, 116, 
    95, 118, 97, 108, 117, 101, 24, 6, 32, 1, 40, 11, 50, 26, 46, 103, 111, 111, 103, 108, 
    101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 76, 105, 115, 116, 86, 97, 108, 117, 101, 
    82, 9, 108, 105, 115, 116, 86, 97, 108, 117, 101, 66, 6, 10, 4, 107, 105, 110, 100, 34, 
    59, 10, 9, 76, 105, 115, 116, 86, 97, 108, 117, 101, 18, 46, 10, 6, 118, 97, 108, 117, 
    101, 115, 24, 1, 32, 3, 40, 11, 50, 22, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 
    111, 116, 111, 98, 117, 102, 46, 86, 97, 108, 117, 101, 82, 6, 118, 97, 108, 117, 101, 115, 
    42, 25, 10, 9, 78, 117, 108, 108, 86, 97, 108, 117, 101, 18, 12, 10, 10, 78, 85, 76, 
    76, 95, 86, 65, 76, 85, 69, 66, 129, 1, 10, 19, 99, 111, 109, 46, 103, 111, 111, 103, 
    108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 66, 11, 83, 116, 114, 117, 99, 116, 80, 
    114, 111, 116, 111, 80, 1, 90, 49, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 103, 
    111, 108, 97, 110, 103, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 112, 116, 121, 112, 101, 
    115, 47, 115, 116, 114, 117, 99, 116, 59, 115, 116, 114, 117, 99, 116, 112, 98, 248, 1, 1, 
    162, 2, 3, 71, 80, 66, 170, 2, 30, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 
    111, 98, 117, 102, 46, 87, 101, 108, 108, 75, 110, 111, 119, 110, 84, 121, 112, 101, 115, 98, 
    6, 112, 114, 111, 116, 111, 51, 
];

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut FILE_BINARY.as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap(),
            crate::reflect::GeneratedCodeInfo {
                structs: ::std::option::Option::Some(::std::boxed::Box::new([
                    crate::reflect::GeneratedStructInfo {
                        new: || ::std::boxed::Box::new(<self::Struct as crate::LiteMessage>::new()),
                        structs: ::std::option::Option::Some(::std::boxed::Box::new([
                        ])),
                    },
                    crate::reflect::GeneratedStructInfo {
                        new: || ::std::boxed::Box::new(<self::Value as crate::LiteMessage>::new()),
                        structs: ::std::option::Option::None,
                    },
                    crate::reflect::GeneratedStructInfo {
                        new: || ::std::boxed::Box::new(<self::ListValue as crate::LiteMessage>::new()),
                        structs: ::std::option::Option::None,
                    },
                ])),
            }
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/struct.proto").unwrap());
    }
}

/// Gets the pool containing all the symbols in this proto file and its dependencies
pub fn pool() -> &'static crate::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
/// Gets the file descriptor representing the proto that created this generated file
pub fn file() -> &'static crate::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
/// 
/// The JSON representation for `Struct` is JSON object.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    fields: crate::collections::MapField<::std::string::String, self::Value>,
    unknown_fields: crate::UnknownFieldSet
}
static STRUCT_FIELDS_CODEC: crate::collections::MapCodec<::std::string::String, self::Value> = crate::collections::MapCodec::new(crate::Codec::string(10), crate::Codec::message(18), 10);
impl crate::CodedMessage for self::Struct {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.fields.add_entries(input, &STRUCT_FIELDS_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        size = size.checked_add(self.fields.calculate_size(&STRUCT_FIELDS_CODEC)?)?;
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.fields.write_to(output, &STRUCT_FIELDS_CODEC)?;
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Struct {
    fn new() -> Self {
        Self {
            fields: crate::collections::MapField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.fields.merge(&other.fields);
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Struct {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Struct {
    /// Gets the field number of the [`fields`] field
    ///
    /// [`fields`]: #method.fields
    pub const FIELDS_FIELD_NUMBER: i32 = 1;
        /// Unordered map of dynamically typed values.
        /// 
    pub fn fields(&self) -> &crate::collections::MapField<::std::string::String, self::Value> {
        &self.fields
    }
    /// Returns a unique reference to the [`fields`] field
    ///
    /// [`fields`]: #method.fields
    pub fn fields_mut(&mut self) -> &mut crate::collections::MapField<::std::string::String, self::Value> {
        &mut self.fields
    }
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.
/// 
/// The JSON representation for `Value` is JSON value.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    kind: self::Value_Kind,
    unknown_fields: crate::UnknownFieldSet
}
/// The kind of value.
/// 
#[derive(Clone, Debug, PartialEq)]
pub enum Value_Kind {
    /// No value
    None,
        /// Represents a null value.
        /// 
    NullValue(crate::EnumValue<self::NullValue>),
        /// Represents a double value.
        /// 
    NumberValue(f64),
        /// Represents a string value.
        /// 
    StringValue(::std::string::String),
        /// Represents a boolean value.
        /// 
    BoolValue(bool),
        /// Represents a structured value.
        /// 
    StructValue(::std::boxed::Box<self::Struct>),
        /// Represents a repeated `Value`.
        /// 
    ListValue(::std::boxed::Box<self::ListValue>),
}
impl crate::CodedMessage for self::Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.kind = self::Value_Kind::NullValue(input.read_enum_value()?),
                17 => self.kind = self::Value_Kind::NumberValue(input.read_double()?),
                26 => self.kind = self::Value_Kind::StringValue(input.read_string()?),
                32 => self.kind = self::Value_Kind::BoolValue(input.read_bool()?),
                42 => 
                    if let self::Value_Kind::StructValue(kind) = &mut self.kind {
                        kind.merge_from(input)?;
                    } else {
                        let mut kind = ::std::boxed::Box::new(<self::Struct as crate::LiteMessage>::new());
                        kind.merge_from(input)?;
                        self.kind = self::Value_Kind::StructValue(kind)
                    },
                50 => 
                    if let self::Value_Kind::ListValue(kind) = &mut self.kind {
                        kind.merge_from(input)?;
                    } else {
                        let mut kind = ::std::boxed::Box::new(<self::ListValue as crate::LiteMessage>::new());
                        kind.merge_from(input)?;
                        self.kind = self::Value_Kind::ListValue(kind)
                    },
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        if let self::Value_Kind::NullValue(kind) = self.kind {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::enum_value(kind));
        }
        if let self::Value_Kind::NumberValue(kind) = self.kind {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::double(kind));
        }
        if let self::Value_Kind::StringValue(kind) = &self.kind {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(kind));
        }
        if let self::Value_Kind::BoolValue(kind) = self.kind {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bool(kind));
        }
        if let self::Value_Kind::StructValue(kind) = &self.kind {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::message(&**kind));
        }
        if let self::Value_Kind::ListValue(kind) = &self.kind {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::message(&**kind));
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
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
            output.write_message(&**kind)?;
        }
        if let self::Value_Kind::ListValue(kind) = &self.kind {
            output.write_raw_tag_bytes(&[50])?;
            output.write_message(&**kind)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Value {
    fn new() -> Self {
        Self {
            kind: self::Value_Kind::None,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
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
                existing.merge(kind);
            } else {
                self.kind = self::Value_Kind::StructValue(kind.clone());
            }
        }
        if let self::Value_Kind::ListValue(kind) = &other.kind {
            if let self::Value_Kind::ListValue(existing) = &mut self.kind {
                existing.merge(kind);
            } else {
                self.kind = self::Value_Kind::ListValue(kind.clone());
            }
        }
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[1]
    }
}
impl self::Value {
    /// Gets a shared reference to the [`kind`] oneof field
    ///
    /// [`kind`]: enum.Value_Kind.html
    pub fn kind(&self) -> &self::Value_Kind {
        &self.kind
    }
    /// Gets a unique reference to the [`kind`] oneof field
    ///
    /// [`kind`]: enum.Value_Kind.html
    pub fn kind_mut(&mut self) -> &mut self::Value_Kind {
        &mut self.kind
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
/// 
/// The JSON representation for `ListValue` is JSON array.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct ListValue {
    values: crate::collections::RepeatedField<self::Value>,
    unknown_fields: crate::UnknownFieldSet
}
static LIST_VALUE_VALUES_CODEC: crate::Codec<self::Value> = crate::Codec::message(10);
impl crate::CodedMessage for self::ListValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.values.add_entries(tag.get(), input, &LIST_VALUE_VALUES_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        size = size.checked_add(self.values.calculate_size(&LIST_VALUE_VALUES_CODEC)?)?;
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.values.write_to(output, &LIST_VALUE_VALUES_CODEC)?;
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::ListValue {
    fn new() -> Self {
        Self {
            values: crate::collections::RepeatedField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.values.merge(&other.values);
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::ListValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[2]
    }
}
impl self::ListValue {
    /// Gets the field number of the [`values`] field
    ///
    /// [`values`]: #method.values
    pub const VALUES_FIELD_NUMBER: i32 = 1;
        /// Repeated field of dynamically typed values.
        /// 
    pub fn values(&self) -> &crate::collections::RepeatedField<self::Value> {
        &self.values
    }
    /// Returns a unique reference to the [`values`] field
    ///
    /// [`values`]: #method.values
    pub fn values_mut(&mut self) -> &mut crate::collections::RepeatedField<self::Value> {
        &mut self.values
    }
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
/// 
/// The JSON representation for `NullValue` is JSON `null`.
/// 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NullValue {
        /// Null value.
        /// 
    NullValue,
}
impl ::std::convert::TryFrom<i32> for self::NullValue {
    type Error = crate::VariantUndefinedError;
    fn try_from(value: i32) -> ::std::result::Result<Self, crate::VariantUndefinedError> {
        match value {
            0 => ::std::result::Result::Ok(self::NullValue::NullValue),
            _ => ::std::result::Result::Err(crate::VariantUndefinedError)
        }
    }
}
impl ::std::convert::From<self::NullValue> for i32 {
    fn from(value: self::NullValue) -> i32 {
        match value {
            NullValue::NullValue => 0,
        }
    }
}