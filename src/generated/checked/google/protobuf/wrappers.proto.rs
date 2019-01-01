//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//!
//! Source: google/protobuf/wrappers.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 0]> = ::std::option::Option::None;

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut [
            10, 30, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 119, 114, 
            97, 112, 112, 101, 114, 115, 46, 112, 114, 111, 116, 111, 18, 15, 103, 111, 111, 103, 108, 101, 
            46, 112, 114, 111, 116, 111, 98, 117, 102, 34, 35, 10, 11, 68, 111, 117, 98, 108, 101, 86, 
            97, 108, 117, 101, 18, 20, 10, 5, 118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 1, 82, 
            5, 118, 97, 108, 117, 101, 34, 34, 10, 10, 70, 108, 111, 97, 116, 86, 97, 108, 117, 101, 
            18, 20, 10, 5, 118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 2, 82, 5, 118, 97, 108, 
            117, 101, 34, 34, 10, 10, 73, 110, 116, 54, 52, 86, 97, 108, 117, 101, 18, 20, 10, 5, 
            118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 3, 82, 5, 118, 97, 108, 117, 101, 34, 35, 
            10, 11, 85, 73, 110, 116, 54, 52, 86, 97, 108, 117, 101, 18, 20, 10, 5, 118, 97, 108, 
            117, 101, 24, 1, 32, 1, 40, 4, 82, 5, 118, 97, 108, 117, 101, 34, 34, 10, 10, 73, 
            110, 116, 51, 50, 86, 97, 108, 117, 101, 18, 20, 10, 5, 118, 97, 108, 117, 101, 24, 1, 
            32, 1, 40, 5, 82, 5, 118, 97, 108, 117, 101, 34, 35, 10, 11, 85, 73, 110, 116, 51, 
            50, 86, 97, 108, 117, 101, 18, 20, 10, 5, 118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 
            13, 82, 5, 118, 97, 108, 117, 101, 34, 33, 10, 9, 66, 111, 111, 108, 86, 97, 108, 117, 
            101, 18, 20, 10, 5, 118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 8, 82, 5, 118, 97, 
            108, 117, 101, 34, 35, 10, 11, 83, 116, 114, 105, 110, 103, 86, 97, 108, 117, 101, 18, 20, 
            10, 5, 118, 97, 108, 117, 101, 24, 1, 32, 1, 40, 9, 82, 5, 118, 97, 108, 117, 101, 
            34, 34, 10, 10, 66, 121, 116, 101, 115, 86, 97, 108, 117, 101, 18, 20, 10, 5, 118, 97, 
            108, 117, 101, 24, 1, 32, 1, 40, 12, 82, 5, 118, 97, 108, 117, 101, 66, 124, 10, 19, 
            99, 111, 109, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 66, 
            13, 87, 114, 97, 112, 112, 101, 114, 115, 80, 114, 111, 116, 111, 80, 1, 90, 42, 103, 105, 
            116, 104, 117, 98, 46, 99, 111, 109, 47, 103, 111, 108, 97, 110, 103, 47, 112, 114, 111, 116, 
            111, 98, 117, 102, 47, 112, 116, 121, 112, 101, 115, 47, 119, 114, 97, 112, 112, 101, 114, 115, 
            248, 1, 1, 162, 2, 3, 71, 80, 66, 170, 2, 30, 71, 111, 111, 103, 108, 101, 46, 80, 
            114, 111, 116, 111, 98, 117, 102, 46, 87, 101, 108, 108, 75, 110, 111, 119, 110, 84, 121, 112, 
            101, 115, 98, 6, 112, 114, 111, 116, 111, 51, 
        ].as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap()
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/wrappers.proto").unwrap());
    }
}

pub fn pool() -> &'static crate::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
pub fn file() -> &'static crate::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
#[derive(Debug, PartialEq)]
pub struct DoubleValue {
    pub value: f64,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::DoubleValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                9 => self.value = input.read_double()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::double(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[9])?;
            output.write_double(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::DoubleValue {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::DoubleValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::DoubleValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::DoubleValue {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: f64 = 0.0;
}
#[derive(Debug, PartialEq)]
pub struct FloatValue {
    pub value: f32,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::FloatValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                13 => self.value = input.read_float()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::float(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[13])?;
            output.write_float(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::FloatValue {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::FloatValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::FloatValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[1]
    }
}
impl self::FloatValue {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: f32 = 0.0;
}
#[derive(Debug, PartialEq)]
pub struct Int64Value {
    pub value: i64,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Int64Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.value = input.read_int64()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::int64(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_int64(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Int64Value {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Int64Value {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Int64Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[2]
    }
}
impl self::Int64Value {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: i64 = 0;
}
#[derive(Debug, PartialEq)]
pub struct UInt64Value {
    pub value: u64,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::UInt64Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.value = input.read_uint64()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::uint64(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_uint64(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::UInt64Value {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::UInt64Value {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::UInt64Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[3]
    }
}
impl self::UInt64Value {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: u64 = 0;
}
#[derive(Debug, PartialEq)]
pub struct Int32Value {
    pub value: i32,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Int32Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.value = input.read_int32()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::int32(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_int32(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Int32Value {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Int32Value {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Int32Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[4]
    }
}
impl self::Int32Value {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: i32 = 0;
}
#[derive(Debug, PartialEq)]
pub struct UInt32Value {
    pub value: u32,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::UInt32Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.value = input.read_uint32()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::uint32(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_uint32(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::UInt32Value {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::UInt32Value {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::UInt32Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[5]
    }
}
impl self::UInt32Value {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: u32 = 0;
}
#[derive(Debug, PartialEq)]
pub struct BoolValue {
    pub value: bool,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::BoolValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.value = input.read_bool()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bool(value))?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_bool(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::BoolValue {
    fn new() -> Self {
        Self {
            value: Self::VALUE_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::BoolValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::BoolValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[6]
    }
}
impl self::BoolValue {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: bool = false;
}
#[derive(Debug, PartialEq)]
pub struct StringValue {
    pub value: ::std::string::String,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::StringValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.value = input.read_string()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = &self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(value)?)?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = &self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::StringValue {
    fn new() -> Self {
        Self {
            value: ::std::string::String::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::StringValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value.clone();
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::StringValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[7]
    }
}
impl self::StringValue {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: &'static str = "";
}
#[derive(Debug, PartialEq)]
pub struct BytesValue {
    pub value: ::std::vec::Vec<u8>,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::BytesValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.value = input.read_bytes()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bytes(value)?)?;
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_bytes(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::BytesValue {
    fn new() -> Self {
        Self {
            value: ::std::vec::Vec::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::BytesValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.value = other.value.clone();
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::BytesValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[8]
    }
}
impl self::BytesValue {
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    pub const VALUE_DEFAULT_VALUE: &'static [u8] = &[];
}