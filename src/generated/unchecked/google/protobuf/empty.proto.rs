//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//!
//! Source: google/protobuf/empty.proto

#[derive(Debug, PartialEq)]
pub struct Empty {
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Empty {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Empty {
    fn new() -> Self {
        Self {
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Empty {
    fn clone(&self) -> Self {
        Self {
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Empty {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Empty {
}