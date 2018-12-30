//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/any.proto

#[derive(Clone, Debug, PartialEq)]
pub struct Any {
    pub type_url: std::string::String,
    pub value: std::vec::Vec<u8>,
    _unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Any {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.type_url = input.read_string()?,
                18 => self.value = input.read_bytes()?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> std::option::Option<i32> {
        let mut size = 0i32;
        let type_url = &self.type_url;
        if type_url != Self::TYPE_URL_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(type_url)?)?;
        }
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bytes(value)?)?;
        }
        size = size.checked_add(self._unknown_fields.calculate_size()?)?;
        std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let type_url = &self.type_url;
        if type_url != Self::TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(type_url)?;
        }
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[18])?;
            output.write_bytes(value)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Any {
    fn new() -> Self {
        Self {
            type_url: std::string::String::new(),
            value: std::vec::Vec::new(),
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.type_url = other.type_url.clone();
        self.value = other.value.clone();
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::Any {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Any {
    /// Gets the field number of the 'type_url' field
    pub const TYPE_URL_FIELD_NUMBER: i32 = 1;
    pub const TYPE_URL_DEFAULT_VALUE: &'static str = "";
    pub fn type_url(&self) -> &std::string::String {
        &self.type_url
    }
    pub fn type_url_mut(&mut self) -> &mut std::string::String {
        &mut self.type_url
    }
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 2;
    pub const VALUE_DEFAULT_VALUE: &'static [u8] = &[];
    pub fn value(&self) -> &std::vec::Vec<u8> {
        &self.value
    }
    pub fn value_mut(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
}