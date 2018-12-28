//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/source_context.proto


#[derive(Clone, PartialEq)]
pub struct SourceContext {
    pub file_name: std::string::String,
    _unknown_fields: crate::UnknownFieldSet
}
static SOURCE_CONTEXT_FILE_NAME_DEFAULT_VALUE: &'static str = "";
impl crate::CodedMessage for self::SourceContext {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.file_name = input.read_string()?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let file_name = &self.file_name;
        if file_name != SOURCE_CONTEXT_FILE_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(file_name);
        }
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let file_name = &self.file_name;
        if file_name != SOURCE_CONTEXT_FILE_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(file_name)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::SourceContext {
    fn new() -> Self {
        Self {
            file_name: std::string::String::new(),
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.file_name = other.file_name.clone();
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::SourceContext {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::SourceContext {
    /// Gets the field number of the 'file_name' field
    pub const FILE_NAME_FIELD_NUMBER: i32 = 1;
}