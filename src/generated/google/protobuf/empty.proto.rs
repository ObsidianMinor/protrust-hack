//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/empty.proto


#[derive(Clone, PartialEq)]
pub struct Empty {
    _unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Empty {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                _ => { }
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> std::option::Option<i32> {
        let mut size = 0i32;
        std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Empty {
    fn new() -> Self {
        Self {
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
    }
}
impl crate::Message for self::Empty {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Empty {
}