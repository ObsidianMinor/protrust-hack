// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/source_context.proto


pub fn file() -> &'static crate::reflect::FileDescriptor {
    super::pool().find_file_by_name("google/protobuf/source_context.proto").unwrap()
}

/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceContext {
    file_name: ::std::string::String,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::SourceContext {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.file_name = input.read_string()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let file_name = &self.file_name;
        if file_name != Self::FILE_NAME_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(file_name));
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let file_name = &self.file_name;
        if file_name != Self::FILE_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(file_name)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::SourceContext {
    fn new() -> Self {
        Self {
            file_name: ::std::string::String::new(),
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.file_name = other.file_name.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::SourceContext {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::SourceContext {
    /// Gets the field number of the [`file_name`] field
    ///
    /// [`file_name`]: #method.file_name
    pub const FILE_NAME_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`file_name`] field
    ///
    /// [`file_name`]: #method.file_name
    pub const FILE_NAME_DEFAULT_VALUE: &'static str = "";
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    pub fn file_name(&self) -> &::std::string::String {
        &self.file_name
    }
    /// Returns a unique reference to the [`file_name`] field
    ///
    /// [`file_name`]: #method.file_name
    pub fn file_name_mut(&mut self) -> &mut ::std::string::String {
        &mut self.file_name
    }
}