//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/field_mask.proto

#[derive(Debug, PartialEq)]
pub struct FieldMask {
    pub paths: crate::collections::RepeatedField<std::string::String>,
    unknown_fields: crate::UnknownFieldSet
}
static FIELD_MASK_PATHS_CODEC: crate::Codec<std::string::String> = crate::Codec::string(10);
impl crate::CodedMessage for self::FieldMask {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.paths.add_entries(tag.get(), input, &FIELD_MASK_PATHS_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> std::option::Option<i32> {
        let mut size = 0i32;
        size = size.checked_add(self.paths.calculate_size(&FIELD_MASK_PATHS_CODEC)?)?;
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.paths.write_to(output, &FIELD_MASK_PATHS_CODEC)?;
        self.unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::FieldMask {
    fn new() -> Self {
        Self {
            paths: crate::collections::RepeatedField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::FieldMask {
    fn clone(&self) -> Self {
        Self {
            paths: self.paths.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.paths.clone_from(&other.paths);
        self.unknown_fields.clone_from(&other.unknown_fields);
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
}