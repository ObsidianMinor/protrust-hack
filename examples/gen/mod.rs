mod externals {
    pub(super) use ::protrust::generated::*;
}
use externals::google_protobuf_timestamp_proto;
#[path = "addressbook.proto.rs"]
pub mod addressbook_proto;
static ADDRESSBOOK_PROTO_BINARY: &'static [u8] = &[
    10, 17, 97, 100, 100, 114, 101, 115, 115, 98, 111, 111, 107, 46, 112, 114, 111, 116, 111, 18, 
    8, 116, 117, 116, 111, 114, 105, 97, 108, 26, 31, 103, 111, 111, 103, 108, 101, 47, 112, 114, 
    111, 116, 111, 98, 117, 102, 47, 116, 105, 109, 101, 115, 116, 97, 109, 112, 46, 112, 114, 111, 
    116, 111, 34, 187, 2, 10, 6, 80, 101, 114, 115, 111, 110, 18, 18, 10, 4, 110, 97, 109, 
    101, 24, 1, 32, 1, 40, 9, 82, 4, 110, 97, 109, 101, 18, 14, 10, 2, 105, 100, 24, 
    2, 32, 1, 40, 5, 82, 2, 105, 100, 18, 20, 10, 5, 101, 109, 97, 105, 108, 24, 3, 
    32, 1, 40, 9, 82, 5, 101, 109, 97, 105, 108, 18, 52, 10, 6, 112, 104, 111, 110, 101, 
    115, 24, 4, 32, 3, 40, 11, 50, 28, 46, 116, 117, 116, 111, 114, 105, 97, 108, 46, 80, 
    101, 114, 115, 111, 110, 46, 80, 104, 111, 110, 101, 78, 117, 109, 98, 101, 114, 82, 6, 112, 
    104, 111, 110, 101, 115, 18, 61, 10, 12, 108, 97, 115, 116, 95, 117, 112, 100, 97, 116, 101, 
    100, 24, 5, 32, 1, 40, 11, 50, 26, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 
    116, 111, 98, 117, 102, 46, 84, 105, 109, 101, 115, 116, 97, 109, 112, 82, 11, 108, 97, 115, 
    116, 85, 112, 100, 97, 116, 101, 100, 26, 85, 10, 11, 80, 104, 111, 110, 101, 78, 117, 109, 
    98, 101, 114, 18, 22, 10, 6, 110, 117, 109, 98, 101, 114, 24, 1, 32, 1, 40, 9, 82, 
    6, 110, 117, 109, 98, 101, 114, 18, 46, 10, 4, 116, 121, 112, 101, 24, 2, 32, 1, 40, 
    14, 50, 26, 46, 116, 117, 116, 111, 114, 105, 97, 108, 46, 80, 101, 114, 115, 111, 110, 46, 
    80, 104, 111, 110, 101, 84, 121, 112, 101, 82, 4, 116, 121, 112, 101, 34, 43, 10, 9, 80, 
    104, 111, 110, 101, 84, 121, 112, 101, 18, 10, 10, 6, 77, 79, 66, 73, 76, 69, 16, 0, 
    18, 8, 10, 4, 72, 79, 77, 69, 16, 1, 18, 8, 10, 4, 87, 79, 82, 75, 16, 2, 
    34, 55, 10, 11, 65, 100, 100, 114, 101, 115, 115, 66, 111, 111, 107, 18, 40, 10, 6, 112, 
    101, 111, 112, 108, 101, 24, 1, 32, 3, 40, 11, 50, 16, 46, 116, 117, 116, 111, 114, 105, 
    97, 108, 46, 80, 101, 114, 115, 111, 110, 82, 6, 112, 101, 111, 112, 108, 101, 66, 80, 10, 
    20, 99, 111, 109, 46, 101, 120, 97, 109, 112, 108, 101, 46, 116, 117, 116, 111, 114, 105, 97, 
    108, 66, 17, 65, 100, 100, 114, 101, 115, 115, 66, 111, 111, 107, 80, 114, 111, 116, 111, 115, 
    170, 2, 36, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 111, 98, 117, 102, 46, 69, 
    120, 97, 109, 112, 108, 101, 115, 46, 65, 100, 100, 114, 101, 115, 115, 66, 111, 111, 107, 98, 
    6, 112, 114, 111, 116, 111, 51, 
];
static mut EXTERNAL_REGISTRIES: ::std::option::Option<[&'static ::protrust::ExtensionRegistry; 1]> = ::std::option::Option::None;
static mut EXTENSIONS_REGISTRY: ::std::option::Option<::protrust::ExtensionRegistry> = ::std::option::Option::None;
static EXTENSIONS_INIT: ::std::sync::Once = ::std::sync::Once::new();
fn extensions_init() {
    unsafe {
        self::EXTERNAL_REGISTRIES = ::std::option::Option::Some([
            ::protrust::generated::extensions(),
        ]);
        self::EXTENSIONS_REGISTRY = ::std::option::Option::Some(::protrust::ExtensionRegistry::new(self::EXTERNAL_REGISTRIES.as_ref().unwrap(), &[
        ]));
    }
}

/// Gets the extension registry containing all the extensions contained in this generated code module
pub fn extensions() -> &'static ::protrust::ExtensionRegistry {
    unsafe {
        EXTENSIONS_INIT.call_once(extensions_init);
        EXTENSIONS_REGISTRY.as_ref().unwrap()
    }
}
static mut EXTERNAL_DEPS: ::std::option::Option<[&'static ::protrust::reflect::DescriptorPool<'static>; 1]> = ::std::option::Option::None;
static mut FILES: ::std::option::Option<[::protrust::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut POOL: ::std::option::Option<::protrust::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static POOL_INIT: ::std::sync::Once = ::std::sync::Once::new();
fn pool_init() {
    unsafe {
        self::EXTERNAL_DEPS = ::std::option::Option::Some([
            ::protrust::generated::pool(),
        ]);
        self::FILES = ::std::option::Option::Some([
            ::protrust::LiteMessage::read_new_from_input(&mut ::protrust::io::CodedInput::new(&mut ADDRESSBOOK_PROTO_BINARY.as_ref()).with_registry(::std::option::Option::Some(self::extensions()))).expect("Could not read file descriptor"),
        ]);
        self::POOL = ::std::option::Option::Some(::protrust::reflect::DescriptorPool::build_from_generated_code(self::FILES.as_ref().unwrap().as_ref(), self::EXTERNAL_DEPS.as_ref().unwrap(), ::std::boxed::Box::new([
            ::protrust::reflect::GeneratedCodeInfo {
                structs: ::std::option::Option::Some(::std::boxed::Box::new([
                    ::protrust::reflect::GeneratedStructInfo {
                        new: || ::std::boxed::Box::new(<self::addressbook_proto::Person as ::protrust::LiteMessage>::new()),
                        structs: ::std::option::Option::Some(::std::boxed::Box::new([
                            ::protrust::reflect::GeneratedStructInfo {
                                new: || ::std::boxed::Box::new(<self::addressbook_proto::person::PhoneNumber as ::protrust::LiteMessage>::new()),
                                structs: ::std::option::Option::None,
                            },
                        ])),
                    },
                    ::protrust::reflect::GeneratedStructInfo {
                        new: || ::std::boxed::Box::new(<self::addressbook_proto::AddressBook as ::protrust::LiteMessage>::new()),
                        structs: ::std::option::Option::None,
                    },
                ])),
            },
        ])));
    }
}

/// Gets the descriptor pool containing all the reflection information contained in this generated code module
pub fn pool() -> &'static ::protrust::reflect::DescriptorPool<'static> {
    unsafe {
        POOL_INIT.call_once(pool_init);
        POOL.as_ref().unwrap()
    }
}