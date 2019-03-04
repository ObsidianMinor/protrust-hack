// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/wrappers.proto


pub fn file() -> &'static crate::reflect::FileDescriptor {
    super::pool().find_file_by_name("google/protobuf/wrappers.proto").unwrap()
}

/// Wrapper message for `double`.
/// 
/// The JSON representation for `DoubleValue` is JSON number.
#[derive(Clone, Debug, PartialEq)]
pub struct DoubleValue {
    value: f64,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::DoubleValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                9 | 10 => self.value = input.read_double()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::double(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::DoubleValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::DoubleValue {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: f64 = 0.0;
    /// The double value.
    pub fn value(&self) -> f64 {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut f64 {
        &mut self.value
    }
}
/// Wrapper message for `float`.
/// 
/// The JSON representation for `FloatValue` is JSON number.
#[derive(Clone, Debug, PartialEq)]
pub struct FloatValue {
    value: f32,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::FloatValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                13 | 10 => self.value = input.read_float()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::float(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::FloatValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[1]
    }
}
impl self::FloatValue {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: f32 = 0.0;
    /// The float value.
    pub fn value(&self) -> f32 {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}
/// Wrapper message for `int64`.
/// 
/// The JSON representation for `Int64Value` is JSON string.
#[derive(Clone, Debug, PartialEq)]
pub struct Int64Value {
    value: i64,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::Int64Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 | 10 => self.value = input.read_int64()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::int64(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Int64Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[2]
    }
}
impl self::Int64Value {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: i64 = 0;
    /// The int64 value.
    pub fn value(&self) -> i64 {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut i64 {
        &mut self.value
    }
}
/// Wrapper message for `uint64`.
/// 
/// The JSON representation for `UInt64Value` is JSON string.
#[derive(Clone, Debug, PartialEq)]
pub struct UInt64Value {
    value: u64,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::UInt64Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 | 10 => self.value = input.read_uint64()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::uint64(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::UInt64Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[3]
    }
}
impl self::UInt64Value {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: u64 = 0;
    /// The uint64 value.
    pub fn value(&self) -> u64 {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
}
/// Wrapper message for `int32`.
/// 
/// The JSON representation for `Int32Value` is JSON number.
#[derive(Clone, Debug, PartialEq)]
pub struct Int32Value {
    value: i32,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::Int32Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 | 10 => self.value = input.read_int32()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::int32(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Int32Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[4]
    }
}
impl self::Int32Value {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: i32 = 0;
    /// The int32 value.
    pub fn value(&self) -> i32 {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}
/// Wrapper message for `uint32`.
/// 
/// The JSON representation for `UInt32Value` is JSON number.
#[derive(Clone, Debug, PartialEq)]
pub struct UInt32Value {
    value: u32,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::UInt32Value {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 | 10 => self.value = input.read_uint32()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::uint32(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::UInt32Value {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[5]
    }
}
impl self::UInt32Value {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: u32 = 0;
    /// The uint32 value.
    pub fn value(&self) -> u32 {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}
/// Wrapper message for `bool`.
/// 
/// The JSON representation for `BoolValue` is JSON `true` and `false`.
#[derive(Clone, Debug, PartialEq)]
pub struct BoolValue {
    value: bool,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::BoolValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 | 10 => self.value = input.read_bool()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bool(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::BoolValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[6]
    }
}
impl self::BoolValue {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: bool = false;
    /// The bool value.
    pub fn value(&self) -> bool {
        self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}
/// Wrapper message for `string`.
/// 
/// The JSON representation for `StringValue` is JSON string.
#[derive(Clone, Debug, PartialEq)]
pub struct StringValue {
    value: ::std::string::String,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::StringValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.value = input.read_string()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = &self.value;
        if value != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::StringValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[7]
    }
}
impl self::StringValue {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: &'static str = "";
    /// The string value.
    pub fn value(&self) -> &::std::string::String {
        &self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}
/// Wrapper message for `bytes`.
/// 
/// The JSON representation for `BytesValue` is JSON string.
#[derive(Clone, Debug, PartialEq)]
pub struct BytesValue {
    value: ::std::vec::Vec<u8>,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::BytesValue {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.value = input.read_bytes()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bytes(value));
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
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.value = other.value.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::BytesValue {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[8]
    }
}
impl self::BytesValue {
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: &'static [u8] = &[];
    /// The bytes value.
    pub fn value(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}