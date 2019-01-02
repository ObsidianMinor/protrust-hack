//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//!
//! Source: google/protobuf/any.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 0]> = ::std::option::Option::None;

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut [
            10, 25, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 97, 110, 
            121, 46, 112, 114, 111, 116, 111, 18, 15, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 
            111, 98, 117, 102, 34, 54, 10, 3, 65, 110, 121, 18, 25, 10, 8, 116, 121, 112, 101, 95, 
            117, 114, 108, 24, 1, 32, 1, 40, 9, 82, 7, 116, 121, 112, 101, 85, 114, 108, 18, 20, 
            10, 5, 118, 97, 108, 117, 101, 24, 2, 32, 1, 40, 12, 82, 5, 118, 97, 108, 117, 101, 
            66, 111, 10, 19, 99, 111, 109, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 
            98, 117, 102, 66, 8, 65, 110, 121, 80, 114, 111, 116, 111, 80, 1, 90, 37, 103, 105, 116, 
            104, 117, 98, 46, 99, 111, 109, 47, 103, 111, 108, 97, 110, 103, 47, 112, 114, 111, 116, 111, 
            98, 117, 102, 47, 112, 116, 121, 112, 101, 115, 47, 97, 110, 121, 162, 2, 3, 71, 80, 66, 
            170, 2, 30, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 111, 98, 117, 102, 46, 87, 
            101, 108, 108, 75, 110, 111, 119, 110, 84, 121, 112, 101, 115, 98, 6, 112, 114, 111, 116, 111, 
            51, 
        ].as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap()
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/any.proto").unwrap());
    }
}

/// Gets the pool containing all the symbols in this proto file and its dependencies
pub fn pool() -> &'static crate::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
/// Gets the file descriptor representing the proto that created this generated file
pub fn file() -> &'static crate::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
#[derive(Debug, PartialEq)]
pub struct Any {
    pub type_url: ::std::string::String,
    pub value: ::std::vec::Vec<u8>,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Any {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.type_url = input.read_string()?,
                18 => self.value = input.read_bytes()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
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
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
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
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Any {
    fn new() -> Self {
        Self {
            type_url: ::std::string::String::new(),
            value: ::std::vec::Vec::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Any {
    fn clone(&self) -> Self {
        Self {
            type_url: self.type_url.clone(),
            value: self.value.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.type_url = other.type_url.clone();
        self.value = other.value.clone();
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Any {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Any {
    /// Gets the field number of the 'type_url' field
    pub const TYPE_URL_FIELD_NUMBER: i32 = 1;
    pub const TYPE_URL_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'value' field
    pub const VALUE_FIELD_NUMBER: i32 = 2;
    pub const VALUE_DEFAULT_VALUE: &'static [u8] = &[];
}