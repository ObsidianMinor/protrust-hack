//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//!
//! Source: conformance.proto

#[derive(Debug, PartialEq)]
pub struct ConformanceRequest {
    pub requested_output_format: ::protrust::EnumValue<self::WireFormat>,
    pub message_type: ::std::string::String,
    pub test_category: ::protrust::EnumValue<self::TestCategory>,
    pub jspb_encoding_options: ::std::option::Option<::std::boxed::Box<self::JspbEncodingConfig>>,
    pub payload: ConformanceRequest_Payload,
    unknown_fields: ::protrust::UnknownFieldSet
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConformanceRequest_Payload {
    None,
    ProtobufPayload(::std::vec::Vec<u8>),
    JsonPayload(::std::string::String),
    JspbPayload(::std::string::String),
}
impl ::protrust::CodedMessage for self::ConformanceRequest {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.payload = self::ConformanceRequest_Payload::ProtobufPayload(input.read_bytes()?),
                18 => self.payload = self::ConformanceRequest_Payload::JsonPayload(input.read_string()?),
                58 => self.payload = self::ConformanceRequest_Payload::JspbPayload(input.read_string()?),
                24 => self.requested_output_format = input.read_enum_value()?,
                34 => self.message_type = input.read_string()?,
                40 => self.test_category = input.read_enum_value()?,
                50 => input.read_message(self.jspb_encoding_options.get_or_insert_with(::protrust::LiteMessage::new))?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        if let self::ConformanceRequest_Payload::ProtobufPayload(payload) = &self.payload {
            size += 1;
            size += ::protrust::io::sizes::bytes(payload);
        }
        if let self::ConformanceRequest_Payload::JsonPayload(payload) = &self.payload {
            size += 1;
            size += ::protrust::io::sizes::string(payload);
        }
        if let self::ConformanceRequest_Payload::JspbPayload(payload) = &self.payload {
            size += 1;
            size += ::protrust::io::sizes::string(payload);
        }
        let requested_output_format = self.requested_output_format;
        if requested_output_format != Self::REQUESTED_OUTPUT_FORMAT_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::enum_value(requested_output_format);
        }
        let message_type = &self.message_type;
        if message_type != Self::MESSAGE_TYPE_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::string(message_type);
        }
        let test_category = self.test_category;
        if test_category != Self::TEST_CATEGORY_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::enum_value(test_category);
        }
        let jspb_encoding_options = &self.jspb_encoding_options;
        if let ::std::option::Option::Some(jspb_encoding_options) = jspb_encoding_options {
            size += 1;
            size += ::protrust::io::sizes::message(jspb_encoding_options);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut ::protrust::io::CodedOutput) -> ::protrust::io::OutputResult {
        if let self::ConformanceRequest_Payload::ProtobufPayload(payload) = &self.payload {
            output.write_raw_tag_bytes(&[10])?;
            output.write_bytes(payload)?;
        }
        if let self::ConformanceRequest_Payload::JsonPayload(payload) = &self.payload {
            output.write_raw_tag_bytes(&[18])?;
            output.write_string(payload)?;
        }
        if let self::ConformanceRequest_Payload::JspbPayload(payload) = &self.payload {
            output.write_raw_tag_bytes(&[58])?;
            output.write_string(payload)?;
        }
        let requested_output_format = self.requested_output_format;
        if requested_output_format != Self::REQUESTED_OUTPUT_FORMAT_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[24])?;
            output.write_enum_value(requested_output_format)?;
        }
        let message_type = &self.message_type;
        if message_type != Self::MESSAGE_TYPE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(message_type)?;
        }
        let test_category = self.test_category;
        if test_category != Self::TEST_CATEGORY_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[40])?;
            output.write_enum_value(test_category)?;
        }
        let jspb_encoding_options = &self.jspb_encoding_options;
        if let ::std::option::Option::Some(jspb_encoding_options) = jspb_encoding_options {
            output.write_raw_tag_bytes(&[50])?;
            output.write_message(jspb_encoding_options)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl ::protrust::LiteMessage for self::ConformanceRequest {
    fn new() -> Self {
        Self {
            requested_output_format: Self::REQUESTED_OUTPUT_FORMAT_DEFAULT_VALUE,
            message_type: ::std::string::String::new(),
            test_category: Self::TEST_CATEGORY_DEFAULT_VALUE,
            jspb_encoding_options: ::std::option::Option::None,
            payload: self::ConformanceRequest_Payload::None,
            unknown_fields: ::protrust::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::ConformanceRequest {
    fn clone(&self) -> Self {
        Self {
            requested_output_format: self.requested_output_format.clone(),
            message_type: self.message_type.clone(),
            test_category: self.test_category.clone(),
            jspb_encoding_options: self.jspb_encoding_options.clone(),
            payload: self.payload.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        if let self::ConformanceRequest_Payload::ProtobufPayload(payload) = &other.payload {
            self.payload = self::ConformanceRequest_Payload::ProtobufPayload(payload.clone());
        }
        if let self::ConformanceRequest_Payload::JsonPayload(payload) = &other.payload {
            self.payload = self::ConformanceRequest_Payload::JsonPayload(payload.clone());
        }
        if let self::ConformanceRequest_Payload::JspbPayload(payload) = &other.payload {
            self.payload = self::ConformanceRequest_Payload::JspbPayload(payload.clone());
        }
        self.requested_output_format = other.requested_output_format;
        self.message_type = other.message_type.clone();
        self.test_category = other.test_category;
        if let ::std::option::Option::Some(jspb_encoding_options) = &other.jspb_encoding_options {
            self.jspb_encoding_options.get_or_insert_with(::protrust::LiteMessage::new).clone_from(jspb_encoding_options);
        }
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl ::protrust::Message for self::ConformanceRequest {
    fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::ConformanceRequest {
    /// Gets the field number of the 'requested_output_format' field
    pub const REQUESTED_OUTPUT_FORMAT_FIELD_NUMBER: i32 = 3;
    pub const REQUESTED_OUTPUT_FORMAT_DEFAULT_VALUE: ::protrust::EnumValue<self::WireFormat> = ::protrust::EnumValue::Defined(self::WireFormat::Unspecified);
    /// Gets the field number of the 'message_type' field
    pub const MESSAGE_TYPE_FIELD_NUMBER: i32 = 4;
    pub const MESSAGE_TYPE_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'test_category' field
    pub const TEST_CATEGORY_FIELD_NUMBER: i32 = 5;
    pub const TEST_CATEGORY_DEFAULT_VALUE: ::protrust::EnumValue<self::TestCategory> = ::protrust::EnumValue::Defined(self::TestCategory::UnspecifiedTest);
    /// Gets the field number of the 'jspb_encoding_options' field
    pub const JSPB_ENCODING_OPTIONS_FIELD_NUMBER: i32 = 6;
    pub fn payload(&self) -> &ConformanceRequest_Payload {
        &self.payload
    }
    pub fn payload_mut(&mut self) -> &mut ConformanceRequest_Payload {
        &mut self.payload
    }
}
#[derive(Debug, PartialEq)]
pub struct ConformanceResponse {
    pub result: ConformanceResponse_Result,
    unknown_fields: ::protrust::UnknownFieldSet
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConformanceResponse_Result {
    None,
    ParseError(::std::string::String),
    SerializeError(::std::string::String),
    RuntimeError(::std::string::String),
    ProtobufPayload(::std::vec::Vec<u8>),
    JsonPayload(::std::string::String),
    Skipped(::std::string::String),
    JspbPayload(::std::string::String),
}
impl ::protrust::CodedMessage for self::ConformanceResponse {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.result = self::ConformanceResponse_Result::ParseError(input.read_string()?),
                50 => self.result = self::ConformanceResponse_Result::SerializeError(input.read_string()?),
                18 => self.result = self::ConformanceResponse_Result::RuntimeError(input.read_string()?),
                26 => self.result = self::ConformanceResponse_Result::ProtobufPayload(input.read_bytes()?),
                34 => self.result = self::ConformanceResponse_Result::JsonPayload(input.read_string()?),
                42 => self.result = self::ConformanceResponse_Result::Skipped(input.read_string()?),
                58 => self.result = self::ConformanceResponse_Result::JspbPayload(input.read_string()?),
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        if let self::ConformanceResponse_Result::ParseError(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::string(result);
        }
        if let self::ConformanceResponse_Result::SerializeError(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::string(result);
        }
        if let self::ConformanceResponse_Result::RuntimeError(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::string(result);
        }
        if let self::ConformanceResponse_Result::ProtobufPayload(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::bytes(result);
        }
        if let self::ConformanceResponse_Result::JsonPayload(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::string(result);
        }
        if let self::ConformanceResponse_Result::Skipped(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::string(result);
        }
        if let self::ConformanceResponse_Result::JspbPayload(result) = &self.result {
            size += 1;
            size += ::protrust::io::sizes::string(result);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut ::protrust::io::CodedOutput) -> ::protrust::io::OutputResult {
        if let self::ConformanceResponse_Result::ParseError(result) = &self.result {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(result)?;
        }
        if let self::ConformanceResponse_Result::SerializeError(result) = &self.result {
            output.write_raw_tag_bytes(&[50])?;
            output.write_string(result)?;
        }
        if let self::ConformanceResponse_Result::RuntimeError(result) = &self.result {
            output.write_raw_tag_bytes(&[18])?;
            output.write_string(result)?;
        }
        if let self::ConformanceResponse_Result::ProtobufPayload(result) = &self.result {
            output.write_raw_tag_bytes(&[26])?;
            output.write_bytes(result)?;
        }
        if let self::ConformanceResponse_Result::JsonPayload(result) = &self.result {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(result)?;
        }
        if let self::ConformanceResponse_Result::Skipped(result) = &self.result {
            output.write_raw_tag_bytes(&[42])?;
            output.write_string(result)?;
        }
        if let self::ConformanceResponse_Result::JspbPayload(result) = &self.result {
            output.write_raw_tag_bytes(&[58])?;
            output.write_string(result)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl ::protrust::LiteMessage for self::ConformanceResponse {
    fn new() -> Self {
        Self {
            result: self::ConformanceResponse_Result::None,
            unknown_fields: ::protrust::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::ConformanceResponse {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        if let self::ConformanceResponse_Result::ParseError(result) = &other.result {
            self.result = self::ConformanceResponse_Result::ParseError(result.clone());
        }
        if let self::ConformanceResponse_Result::SerializeError(result) = &other.result {
            self.result = self::ConformanceResponse_Result::SerializeError(result.clone());
        }
        if let self::ConformanceResponse_Result::RuntimeError(result) = &other.result {
            self.result = self::ConformanceResponse_Result::RuntimeError(result.clone());
        }
        if let self::ConformanceResponse_Result::ProtobufPayload(result) = &other.result {
            self.result = self::ConformanceResponse_Result::ProtobufPayload(result.clone());
        }
        if let self::ConformanceResponse_Result::JsonPayload(result) = &other.result {
            self.result = self::ConformanceResponse_Result::JsonPayload(result.clone());
        }
        if let self::ConformanceResponse_Result::Skipped(result) = &other.result {
            self.result = self::ConformanceResponse_Result::Skipped(result.clone());
        }
        if let self::ConformanceResponse_Result::JspbPayload(result) = &other.result {
            self.result = self::ConformanceResponse_Result::JspbPayload(result.clone());
        }
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl ::protrust::Message for self::ConformanceResponse {
    fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::ConformanceResponse {
    pub fn result(&self) -> &ConformanceResponse_Result {
        &self.result
    }
    pub fn result_mut(&mut self) -> &mut ConformanceResponse_Result {
        &mut self.result
    }
}
#[derive(Debug, PartialEq)]
pub struct JspbEncodingConfig {
    pub use_jspb_array_any_format: bool,
    unknown_fields: ::protrust::UnknownFieldSet
}
impl ::protrust::CodedMessage for self::JspbEncodingConfig {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.use_jspb_array_any_format = input.read_bool()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let use_jspb_array_any_format = self.use_jspb_array_any_format;
        if use_jspb_array_any_format != Self::USE_JSPB_ARRAY_ANY_FORMAT_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::bool(use_jspb_array_any_format);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut ::protrust::io::CodedOutput) -> ::protrust::io::OutputResult {
        let use_jspb_array_any_format = self.use_jspb_array_any_format;
        if use_jspb_array_any_format != Self::USE_JSPB_ARRAY_ANY_FORMAT_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_bool(use_jspb_array_any_format)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl ::protrust::LiteMessage for self::JspbEncodingConfig {
    fn new() -> Self {
        Self {
            use_jspb_array_any_format: Self::USE_JSPB_ARRAY_ANY_FORMAT_DEFAULT_VALUE,
            unknown_fields: ::protrust::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::JspbEncodingConfig {
    fn clone(&self) -> Self {
        Self {
            use_jspb_array_any_format: self.use_jspb_array_any_format.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.use_jspb_array_any_format = other.use_jspb_array_any_format;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl ::protrust::Message for self::JspbEncodingConfig {
    fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::JspbEncodingConfig {
    /// Gets the field number of the 'use_jspb_array_any_format' field
    pub const USE_JSPB_ARRAY_ANY_FORMAT_FIELD_NUMBER: i32 = 1;
    pub const USE_JSPB_ARRAY_ANY_FORMAT_DEFAULT_VALUE: bool = false;
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WireFormat {
    Unspecified = 0,
    Protobuf = 1,
    Json = 2,
    Jspb = 3,
}
impl ::std::convert::TryFrom<i32> for self::WireFormat {
    type Error = ::protrust::VariantUndefinedError;
    fn try_from(value: i32) -> ::std::result::Result<Self, ::protrust::VariantUndefinedError> {
        match value {
            0 => ::std::result::Result::Ok(self::WireFormat::Unspecified),
            1 => ::std::result::Result::Ok(self::WireFormat::Protobuf),
            2 => ::std::result::Result::Ok(self::WireFormat::Json),
            3 => ::std::result::Result::Ok(self::WireFormat::Jspb),
            _ => ::std::result::Result::Err(::protrust::VariantUndefinedError)
        }
    }
}
impl ::std::convert::From<self::WireFormat> for i32 {
    fn from(value: self::WireFormat) -> i32 {
        value as i32
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestCategory {
    UnspecifiedTest = 0,
    BinaryTest = 1,
    JsonTest = 2,
    JsonIgnoreUnknownParsingTest = 3,
    JspbTest = 4,
}
impl ::std::convert::TryFrom<i32> for self::TestCategory {
    type Error = ::protrust::VariantUndefinedError;
    fn try_from(value: i32) -> ::std::result::Result<Self, ::protrust::VariantUndefinedError> {
        match value {
            0 => ::std::result::Result::Ok(self::TestCategory::UnspecifiedTest),
            1 => ::std::result::Result::Ok(self::TestCategory::BinaryTest),
            2 => ::std::result::Result::Ok(self::TestCategory::JsonTest),
            3 => ::std::result::Result::Ok(self::TestCategory::JsonIgnoreUnknownParsingTest),
            4 => ::std::result::Result::Ok(self::TestCategory::JspbTest),
            _ => ::std::result::Result::Err(::protrust::VariantUndefinedError)
        }
    }
}
impl ::std::convert::From<self::TestCategory> for i32 {
    fn from(value: self::TestCategory) -> i32 {
        value as i32
    }
}