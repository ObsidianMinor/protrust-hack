//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//!
//! Source: google/protobuf/source_context.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 0]> = ::std::option::Option::None;

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut [
            10, 36, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 115, 111, 
            117, 114, 99, 101, 95, 99, 111, 110, 116, 101, 120, 116, 46, 112, 114, 111, 116, 111, 18, 15, 
            103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 34, 44, 10, 13, 83, 
            111, 117, 114, 99, 101, 67, 111, 110, 116, 101, 120, 116, 18, 27, 10, 9, 102, 105, 108, 101, 
            95, 110, 97, 109, 101, 24, 1, 32, 1, 40, 9, 82, 8, 102, 105, 108, 101, 78, 97, 109, 
            101, 66, 149, 1, 10, 19, 99, 111, 109, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 
            116, 111, 98, 117, 102, 66, 18, 83, 111, 117, 114, 99, 101, 67, 111, 110, 116, 101, 120, 116, 
            80, 114, 111, 116, 111, 80, 1, 90, 65, 103, 111, 111, 103, 108, 101, 46, 103, 111, 108, 97, 
            110, 103, 46, 111, 114, 103, 47, 103, 101, 110, 112, 114, 111, 116, 111, 47, 112, 114, 111, 116, 
            111, 98, 117, 102, 47, 115, 111, 117, 114, 99, 101, 95, 99, 111, 110, 116, 101, 120, 116, 59, 
            115, 111, 117, 114, 99, 101, 95, 99, 111, 110, 116, 101, 120, 116, 162, 2, 3, 71, 80, 66, 
            170, 2, 30, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 111, 98, 117, 102, 46, 87, 
            101, 108, 108, 75, 110, 111, 119, 110, 84, 121, 112, 101, 115, 74, 208, 16, 10, 6, 18, 4, 
            30, 0, 47, 1, 10, 204, 12, 10, 1, 12, 18, 3, 30, 0, 18, 50, 193, 12, 32, 80, 
            114, 111, 116, 111, 99, 111, 108, 32, 66, 117, 102, 102, 101, 114, 115, 32, 45, 32, 71, 111, 
            111, 103, 108, 101, 39, 115, 32, 100, 97, 116, 97, 32, 105, 110, 116, 101, 114, 99, 104, 97, 
            110, 103, 101, 32, 102, 111, 114, 109, 97, 116, 10, 32, 67, 111, 112, 121, 114, 105, 103, 104, 
            116, 32, 50, 48, 48, 56, 32, 71, 111, 111, 103, 108, 101, 32, 73, 110, 99, 46, 32, 32, 
            65, 108, 108, 32, 114, 105, 103, 104, 116, 115, 32, 114, 101, 115, 101, 114, 118, 101, 100, 46, 
            10, 32, 104, 116, 116, 112, 115, 58, 47, 47, 100, 101, 118, 101, 108, 111, 112, 101, 114, 115, 
            46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111, 99, 111, 108, 
            45, 98, 117, 102, 102, 101, 114, 115, 47, 10, 10, 32, 82, 101, 100, 105, 115, 116, 114, 105, 
            98, 117, 116, 105, 111, 110, 32, 97, 110, 100, 32, 117, 115, 101, 32, 105, 110, 32, 115, 111, 
            117, 114, 99, 101, 32, 97, 110, 100, 32, 98, 105, 110, 97, 114, 121, 32, 102, 111, 114, 109, 
            115, 44, 32, 119, 105, 116, 104, 32, 111, 114, 32, 119, 105, 116, 104, 111, 117, 116, 10, 32, 
            109, 111, 100, 105, 102, 105, 99, 97, 116, 105, 111, 110, 44, 32, 97, 114, 101, 32, 112, 101, 
            114, 109, 105, 116, 116, 101, 100, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 116, 104, 97, 
            116, 32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 99, 111, 110, 100, 
            105, 116, 105, 111, 110, 115, 32, 97, 114, 101, 10, 32, 109, 101, 116, 58, 10, 10, 32, 32, 
            32, 32, 32, 42, 32, 82, 101, 100, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111, 110, 115, 
            32, 111, 102, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 100, 101, 32, 109, 117, 115, 116, 
            32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 97, 98, 111, 118, 101, 32, 99, 111, 
            112, 121, 114, 105, 103, 104, 116, 10, 32, 110, 111, 116, 105, 99, 101, 44, 32, 116, 104, 105, 
            115, 32, 108, 105, 115, 116, 32, 111, 102, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 115, 
            32, 97, 110, 100, 32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 100, 
            105, 115, 99, 108, 97, 105, 109, 101, 114, 46, 10, 32, 32, 32, 32, 32, 42, 32, 82, 101, 
            100, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111, 110, 115, 32, 105, 110, 32, 98, 105, 110, 
            97, 114, 121, 32, 102, 111, 114, 109, 32, 109, 117, 115, 116, 32, 114, 101, 112, 114, 111, 100, 
            117, 99, 101, 32, 116, 104, 101, 32, 97, 98, 111, 118, 101, 10, 32, 99, 111, 112, 121, 114, 
            105, 103, 104, 116, 32, 110, 111, 116, 105, 99, 101, 44, 32, 116, 104, 105, 115, 32, 108, 105, 
            115, 116, 32, 111, 102, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 115, 32, 97, 110, 100, 
            32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 100, 105, 115, 99, 108, 
            97, 105, 109, 101, 114, 10, 32, 105, 110, 32, 116, 104, 101, 32, 100, 111, 99, 117, 109, 101, 
            110, 116, 97, 116, 105, 111, 110, 32, 97, 110, 100, 47, 111, 114, 32, 111, 116, 104, 101, 114, 
            32, 109, 97, 116, 101, 114, 105, 97, 108, 115, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 
            119, 105, 116, 104, 32, 116, 104, 101, 10, 32, 100, 105, 115, 116, 114, 105, 98, 117, 116, 105, 
            111, 110, 46, 10, 32, 32, 32, 32, 32, 42, 32, 78, 101, 105, 116, 104, 101, 114, 32, 116, 
            104, 101, 32, 110, 97, 109, 101, 32, 111, 102, 32, 71, 111, 111, 103, 108, 101, 32, 73, 110, 
            99, 46, 32, 110, 111, 114, 32, 116, 104, 101, 32, 110, 97, 109, 101, 115, 32, 111, 102, 32, 
            105, 116, 115, 10, 32, 99, 111, 110, 116, 114, 105, 98, 117, 116, 111, 114, 115, 32, 109, 97, 
            121, 32, 98, 101, 32, 117, 115, 101, 100, 32, 116, 111, 32, 101, 110, 100, 111, 114, 115, 101, 
            32, 111, 114, 32, 112, 114, 111, 109, 111, 116, 101, 32, 112, 114, 111, 100, 117, 99, 116, 115, 
            32, 100, 101, 114, 105, 118, 101, 100, 32, 102, 114, 111, 109, 10, 32, 116, 104, 105, 115, 32, 
            115, 111, 102, 116, 119, 97, 114, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 115, 112, 101, 
            99, 105, 102, 105, 99, 32, 112, 114, 105, 111, 114, 32, 119, 114, 105, 116, 116, 101, 110, 32, 
            112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 46, 10, 10, 32, 84, 72, 73, 83, 32, 83, 
            79, 70, 84, 87, 65, 82, 69, 32, 73, 83, 32, 80, 82, 79, 86, 73, 68, 69, 68, 32, 
            66, 89, 32, 84, 72, 69, 32, 67, 79, 80, 89, 82, 73, 71, 72, 84, 32, 72, 79, 76, 
            68, 69, 82, 83, 32, 65, 78, 68, 32, 67, 79, 78, 84, 82, 73, 66, 85, 84, 79, 82, 
            83, 10, 32, 34, 65, 83, 32, 73, 83, 34, 32, 65, 78, 68, 32, 65, 78, 89, 32, 69, 
            88, 80, 82, 69, 83, 83, 32, 79, 82, 32, 73, 77, 80, 76, 73, 69, 68, 32, 87, 65, 
            82, 82, 65, 78, 84, 73, 69, 83, 44, 32, 73, 78, 67, 76, 85, 68, 73, 78, 71, 44, 
            32, 66, 85, 84, 32, 78, 79, 84, 10, 32, 76, 73, 77, 73, 84, 69, 68, 32, 84, 79, 
            44, 32, 84, 72, 69, 32, 73, 77, 80, 76, 73, 69, 68, 32, 87, 65, 82, 82, 65, 78, 
            84, 73, 69, 83, 32, 79, 70, 32, 77, 69, 82, 67, 72, 65, 78, 84, 65, 66, 73, 76, 
            73, 84, 89, 32, 65, 78, 68, 32, 70, 73, 84, 78, 69, 83, 83, 32, 70, 79, 82, 10, 
            32, 65, 32, 80, 65, 82, 84, 73, 67, 85, 76, 65, 82, 32, 80, 85, 82, 80, 79, 83, 
            69, 32, 65, 82, 69, 32, 68, 73, 83, 67, 76, 65, 73, 77, 69, 68, 46, 32, 73, 78, 
            32, 78, 79, 32, 69, 86, 69, 78, 84, 32, 83, 72, 65, 76, 76, 32, 84, 72, 69, 32, 
            67, 79, 80, 89, 82, 73, 71, 72, 84, 10, 32, 79, 87, 78, 69, 82, 32, 79, 82, 32, 
            67, 79, 78, 84, 82, 73, 66, 85, 84, 79, 82, 83, 32, 66, 69, 32, 76, 73, 65, 66, 
            76, 69, 32, 70, 79, 82, 32, 65, 78, 89, 32, 68, 73, 82, 69, 67, 84, 44, 32, 73, 
            78, 68, 73, 82, 69, 67, 84, 44, 32, 73, 78, 67, 73, 68, 69, 78, 84, 65, 76, 44, 
            10, 32, 83, 80, 69, 67, 73, 65, 76, 44, 32, 69, 88, 69, 77, 80, 76, 65, 82, 89, 
            44, 32, 79, 82, 32, 67, 79, 78, 83, 69, 81, 85, 69, 78, 84, 73, 65, 76, 32, 68, 
            65, 77, 65, 71, 69, 83, 32, 40, 73, 78, 67, 76, 85, 68, 73, 78, 71, 44, 32, 66, 
            85, 84, 32, 78, 79, 84, 10, 32, 76, 73, 77, 73, 84, 69, 68, 32, 84, 79, 44, 32, 
            80, 82, 79, 67, 85, 82, 69, 77, 69, 78, 84, 32, 79, 70, 32, 83, 85, 66, 83, 84, 
            73, 84, 85, 84, 69, 32, 71, 79, 79, 68, 83, 32, 79, 82, 32, 83, 69, 82, 86, 73, 
            67, 69, 83, 59, 32, 76, 79, 83, 83, 32, 79, 70, 32, 85, 83, 69, 44, 10, 32, 68, 
            65, 84, 65, 44, 32, 79, 82, 32, 80, 82, 79, 70, 73, 84, 83, 59, 32, 79, 82, 32, 
            66, 85, 83, 73, 78, 69, 83, 83, 32, 73, 78, 84, 69, 82, 82, 85, 80, 84, 73, 79, 
            78, 41, 32, 72, 79, 87, 69, 86, 69, 82, 32, 67, 65, 85, 83, 69, 68, 32, 65, 78, 
            68, 32, 79, 78, 32, 65, 78, 89, 10, 32, 84, 72, 69, 79, 82, 89, 32, 79, 70, 32, 
            76, 73, 65, 66, 73, 76, 73, 84, 89, 44, 32, 87, 72, 69, 84, 72, 69, 82, 32, 73, 
            78, 32, 67, 79, 78, 84, 82, 65, 67, 84, 44, 32, 83, 84, 82, 73, 67, 84, 32, 76, 
            73, 65, 66, 73, 76, 73, 84, 89, 44, 32, 79, 82, 32, 84, 79, 82, 84, 10, 32, 40, 
            73, 78, 67, 76, 85, 68, 73, 78, 71, 32, 78, 69, 71, 76, 73, 71, 69, 78, 67, 69, 
            32, 79, 82, 32, 79, 84, 72, 69, 82, 87, 73, 83, 69, 41, 32, 65, 82, 73, 83, 73, 
            78, 71, 32, 73, 78, 32, 65, 78, 89, 32, 87, 65, 89, 32, 79, 85, 84, 32, 79, 70, 
            32, 84, 72, 69, 32, 85, 83, 69, 10, 32, 79, 70, 32, 84, 72, 73, 83, 32, 83, 79, 
            70, 84, 87, 65, 82, 69, 44, 32, 69, 86, 69, 78, 32, 73, 70, 32, 65, 68, 86, 73, 
            83, 69, 68, 32, 79, 70, 32, 84, 72, 69, 32, 80, 79, 83, 83, 73, 66, 73, 76, 73, 
            84, 89, 32, 79, 70, 32, 83, 85, 67, 72, 32, 68, 65, 77, 65, 71, 69, 46, 10, 10, 
            8, 10, 1, 2, 18, 3, 32, 8, 23, 10, 8, 10, 1, 8, 18, 3, 34, 0, 59, 10, 
            9, 10, 2, 8, 37, 18, 3, 34, 0, 59, 10, 8, 10, 1, 8, 18, 3, 35, 0, 44, 
            10, 9, 10, 2, 8, 1, 18, 3, 35, 0, 44, 10, 8, 10, 1, 8, 18, 3, 36, 0, 
            51, 10, 9, 10, 2, 8, 8, 18, 3, 36, 0, 51, 10, 8, 10, 1, 8, 18, 3, 37, 
            0, 34, 10, 9, 10, 2, 8, 10, 18, 3, 37, 0, 34, 10, 8, 10, 1, 8, 18, 3, 
            38, 0, 33, 10, 9, 10, 2, 8, 36, 18, 3, 38, 0, 33, 10, 8, 10, 1, 8, 18, 
            3, 39, 0, 88, 10, 9, 10, 2, 8, 11, 18, 3, 39, 0, 88, 10, 131, 1, 10, 2, 
            4, 0, 18, 4, 43, 0, 47, 1, 26, 119, 32, 96, 83, 111, 117, 114, 99, 101, 67, 111, 
            110, 116, 101, 120, 116, 96, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 115, 32, 105, 110, 
            102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 97, 98, 111, 117, 116, 32, 116, 104, 101, 32, 
            115, 111, 117, 114, 99, 101, 32, 111, 102, 32, 97, 10, 32, 112, 114, 111, 116, 111, 98, 117, 
            102, 32, 101, 108, 101, 109, 101, 110, 116, 44, 32, 108, 105, 107, 101, 32, 116, 104, 101, 32, 
            102, 105, 108, 101, 32, 105, 110, 32, 119, 104, 105, 99, 104, 32, 105, 116, 32, 105, 115, 32, 
            100, 101, 102, 105, 110, 101, 100, 46, 10, 10, 10, 10, 3, 4, 0, 1, 18, 3, 43, 8, 
            21, 10, 163, 1, 10, 4, 4, 0, 2, 0, 18, 3, 46, 2, 23, 26, 149, 1, 32, 84, 
            104, 101, 32, 112, 97, 116, 104, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 110, 97, 
            109, 101, 32, 111, 102, 32, 116, 104, 101, 32, 46, 112, 114, 111, 116, 111, 32, 102, 105, 108, 
            101, 32, 116, 104, 97, 116, 32, 99, 111, 110, 116, 97, 105, 110, 101, 100, 32, 116, 104, 101, 
            32, 97, 115, 115, 111, 99, 105, 97, 116, 101, 100, 10, 32, 112, 114, 111, 116, 111, 98, 117, 
            102, 32, 101, 108, 101, 109, 101, 110, 116, 46, 32, 32, 70, 111, 114, 32, 101, 120, 97, 109, 
            112, 108, 101, 58, 32, 96, 34, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 
            117, 102, 47, 115, 111, 117, 114, 99, 101, 95, 99, 111, 110, 116, 101, 120, 116, 46, 112, 114, 
            111, 116, 111, 34, 96, 46, 10, 10, 13, 10, 5, 4, 0, 2, 0, 4, 18, 4, 46, 2, 
            43, 23, 10, 12, 10, 5, 4, 0, 2, 0, 5, 18, 3, 46, 2, 8, 10, 12, 10, 5, 
            4, 0, 2, 0, 1, 18, 3, 46, 9, 18, 10, 12, 10, 5, 4, 0, 2, 0, 3, 18, 
            3, 46, 21, 22, 98, 6, 112, 114, 111, 116, 111, 51, 
        ].as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap()
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/source_context.proto").unwrap());
    }
}

pub fn pool() -> &'static crate::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
pub fn file() -> &'static crate::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
#[derive(Debug, PartialEq)]
pub struct SourceContext {
    pub file_name: ::std::string::String,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::SourceContext {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.file_name = input.read_string()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let file_name = &self.file_name;
        if file_name != Self::FILE_NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(file_name);
        }
        size += self.unknown_fields.calculate_size();
        size
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
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::SourceContext {
    fn clone(&self) -> Self {
        Self {
            file_name: self.file_name.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.file_name = other.file_name.clone();
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::SourceContext {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::SourceContext {
    /// Gets the field number of the 'file_name' field
    pub const FILE_NAME_FIELD_NUMBER: i32 = 1;
    pub const FILE_NAME_DEFAULT_VALUE: &'static str = "";
}