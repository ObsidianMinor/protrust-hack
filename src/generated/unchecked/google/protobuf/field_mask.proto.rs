//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/field_mask.proto

#[derive(Clone, PartialEq)]
pub struct FieldMask {
    pub paths: crate::collections::RepeatedField<std::string::String>,
    _unknown_fields: crate::UnknownFieldSet
}
static FIELD_MASK_PATHS_CODEC: crate::Codec<std::string::String> = crate::Codec::string(10);
impl crate::CodedMessage for self::FieldMask {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.paths.add_entries(tag.get(), input, &FIELD_MASK_PATHS_CODEC)?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.paths.calculate_size(&FIELD_MASK_PATHS_CODEC);
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.paths.write_to(output, &FIELD_MASK_PATHS_CODEC)?;
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::FieldMask {
    fn new() -> Self {
        Self {
            paths: crate::collections::RepeatedField::new(),
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.paths.merge(&other.paths);
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::FieldMask {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::FieldMask {
    /// Gets the field number of the 'paths' field
    pub const PATHS_FIELD_NUMBER: i32 = 1;
    pub fn paths(&self) -> &crate::collections::RepeatedField<std::string::String> {
        &self.paths
    }
    pub fn paths_mut(&mut self) -> &mut crate::collections::RepeatedField<std::string::String> {
        &mut self.paths
    }
}