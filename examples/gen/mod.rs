/* generated by protoc-gen-rust */

mod externals {
    pub(super) use protrust::generated::*;
}
use self::externals::google_protobuf_timestamp_proto;
static mut EXTERNAL_REGISTRIES: ::std::option::Option<[&'static ::protrust::ExtensionRegistry; 1]> =
    ::std::option::Option::None;
static mut EXTENSIONS_REGISTRY: ::std::option::Option<::protrust::ExtensionRegistry> =
    ::std::option::Option::None;
static EXTENSIONS_INIT: ::std::sync::Once = ::std::sync::Once::new();
fn extensions_init() {
    unsafe {
        self::EXTERNAL_REGISTRIES =
            ::std::option::Option::Some([::protrust::generated::extensions()]);
        self::EXTENSIONS_REGISTRY = ::std::option::Option::Some(
            ::protrust::ExtensionRegistry::new(self::EXTERNAL_REGISTRIES.as_ref().unwrap(), &[]),
        );
    }
}
pub fn extensions() -> &'static ::protrust::ExtensionRegistry {
    unsafe {
        self::EXTENSIONS_INIT.call_once(extensions_init);
        self::EXTENSIONS_REGISTRY.as_ref().unwrap()
    }
}
static mut EXTERNAL_DEPS: ::std::option::Option<
    [&'static ::protrust::reflect::DescriptorPool<'static>; 1],
> = ::std::option::Option::None;
static mut FILES: ::std::option::Option<[::protrust::descriptor::FileDescriptorProto; 1]> =
    ::std::option::Option::None;
static mut POOL: ::std::option::Option<::protrust::reflect::DescriptorPool<'static>> =
    ::std::option::Option::None;
static POOL_INIT: ::std::sync::Once = ::std::sync::Once::new();
fn pool_init() {
    unsafe {
        self::EXTERNAL_DEPS = ::std::option::Option::Some([::protrust::generated::pool()]);
        self::FILES = ::std::option::Option::Some([::protrust::LiteMessage::read_new_from_input(
            &mut ::protrust::io::CodedInput::new(&mut ADDRESSBOOK_PROTO_BINARY.as_ref())
                .with_registry(::std::option::Option::Some(self::extensions())),
        )
        .expect("couldn't read file descriptor")]);
        self :: POOL = :: std :: option :: Option :: Some ( :: protrust :: reflect :: DescriptorPool :: build_from_generated_code ( self :: FILES . as_ref ( ) . unwrap ( ) . as_ref ( ) , self :: EXTERNAL_DEPS . as_ref ( ) . unwrap ( ) , :: std :: boxed :: Box :: new ( [ :: protrust :: reflect :: GeneratedCodeInfo { structs : :: std :: option :: Option :: Some ( :: std :: boxed :: Box :: new ( [ :: protrust :: reflect :: GeneratedStructInfo { new : || :: std :: boxed :: Box :: new ( < self :: addressbook_proto :: Person as :: protrust :: LiteMessage > :: new ( ) ) , structs : :: std :: option :: Option :: Some ( :: std :: boxed :: Box :: new ( [ :: protrust :: reflect :: GeneratedStructInfo { new : || :: std :: boxed :: Box :: new ( < self :: addressbook_proto :: person :: PhoneNumber as :: protrust :: LiteMessage > :: new ( ) ) , structs : :: std :: option :: Option :: None , fields : :: std :: option :: Option :: Some ( :: std :: boxed :: Box :: new ( [ :: protrust :: reflect :: access :: FieldAccessor :: Single ( & self :: addressbook_proto :: person :: phone_number :: NUMBER_REFLECTOR ) , :: protrust :: reflect :: access :: FieldAccessor :: Single ( & self :: addressbook_proto :: person :: phone_number :: TYPE_REFLECTOR ) , ] ) ) , extensions : :: std :: option :: Option :: None , } , ] ) ) , fields : :: std :: option :: Option :: Some ( :: std :: boxed :: Box :: new ( [ :: protrust :: reflect :: access :: FieldAccessor :: Single ( & self :: addressbook_proto :: person :: NAME_REFLECTOR ) , :: protrust :: reflect :: access :: FieldAccessor :: Single ( & self :: addressbook_proto :: person :: ID_REFLECTOR ) , :: protrust :: reflect :: access :: FieldAccessor :: Single ( & self :: addressbook_proto :: person :: EMAIL_REFLECTOR ) , :: protrust :: reflect :: access :: FieldAccessor :: Repeated ( & self :: addressbook_proto :: person :: PHONES_REFLECTOR ) , :: protrust :: reflect :: access :: FieldAccessor :: Single ( & self :: addressbook_proto :: person :: LAST_UPDATED_REFLECTOR ) , ] ) ) , extensions : :: std :: option :: Option :: None , } , :: protrust :: reflect :: GeneratedStructInfo { new : || :: std :: boxed :: Box :: new ( < self :: addressbook_proto :: AddressBook as :: protrust :: LiteMessage > :: new ( ) ) , structs : :: std :: option :: Option :: None , fields : :: std :: option :: Option :: Some ( :: std :: boxed :: Box :: new ( [ :: protrust :: reflect :: access :: FieldAccessor :: Repeated ( & self :: addressbook_proto :: address_book :: PEOPLE_REFLECTOR ) , ] ) ) , extensions : :: std :: option :: Option :: None , } , ] ) ) , extensions : :: std :: option :: Option :: None , } , ] ) ) ) ;
    }
}
pub fn pool() -> &'static ::protrust::reflect::DescriptorPool<'static> {
    unsafe {
        self::POOL_INIT.call_once(pool_init);
        self::POOL.as_ref().unwrap()
    }
}
static ADDRESSBOOK_PROTO_BINARY: &'static [u8] = &[
    10, 17, 97, 100, 100, 114, 101, 115, 115, 98, 111, 111, 107, 46, 112, 114, 111, 116, 111, 18,
    8, 116, 117, 116, 111, 114, 105, 97, 108, 26, 31, 103, 111, 111, 103, 108, 101, 47, 112, 114,
    111, 116, 111, 98, 117, 102, 47, 116, 105, 109, 101, 115, 116, 97, 109, 112, 46, 112, 114, 111,
    116, 111, 34, 187, 2, 10, 6, 80, 101, 114, 115, 111, 110, 18, 18, 10, 4, 110, 97, 109, 101, 24,
    1, 32, 1, 40, 9, 82, 4, 110, 97, 109, 101, 18, 14, 10, 2, 105, 100, 24, 2, 32, 1, 40, 5, 82, 2,
    105, 100, 18, 20, 10, 5, 101, 109, 97, 105, 108, 24, 3, 32, 1, 40, 9, 82, 5, 101, 109, 97, 105,
    108, 18, 52, 10, 6, 112, 104, 111, 110, 101, 115, 24, 4, 32, 3, 40, 11, 50, 28, 46, 116, 117,
    116, 111, 114, 105, 97, 108, 46, 80, 101, 114, 115, 111, 110, 46, 80, 104, 111, 110, 101, 78,
    117, 109, 98, 101, 114, 82, 6, 112, 104, 111, 110, 101, 115, 18, 61, 10, 12, 108, 97, 115, 116,
    95, 117, 112, 100, 97, 116, 101, 100, 24, 5, 32, 1, 40, 11, 50, 26, 46, 103, 111, 111, 103,
    108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 84, 105, 109, 101, 115, 116, 97, 109,
    112, 82, 11, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 26, 85, 10, 11, 80, 104, 111,
    110, 101, 78, 117, 109, 98, 101, 114, 18, 22, 10, 6, 110, 117, 109, 98, 101, 114, 24, 1, 32, 1,
    40, 9, 82, 6, 110, 117, 109, 98, 101, 114, 18, 46, 10, 4, 116, 121, 112, 101, 24, 2, 32, 1, 40,
    14, 50, 26, 46, 116, 117, 116, 111, 114, 105, 97, 108, 46, 80, 101, 114, 115, 111, 110, 46, 80,
    104, 111, 110, 101, 84, 121, 112, 101, 82, 4, 116, 121, 112, 101, 34, 43, 10, 9, 80, 104, 111,
    110, 101, 84, 121, 112, 101, 18, 10, 10, 6, 77, 79, 66, 73, 76, 69, 16, 0, 18, 8, 10, 4, 72,
    79, 77, 69, 16, 1, 18, 8, 10, 4, 87, 79, 82, 75, 16, 2, 34, 55, 10, 11, 65, 100, 100, 114, 101,
    115, 115, 66, 111, 111, 107, 18, 40, 10, 6, 112, 101, 111, 112, 108, 101, 24, 1, 32, 3, 40, 11,
    50, 16, 46, 116, 117, 116, 111, 114, 105, 97, 108, 46, 80, 101, 114, 115, 111, 110, 82, 6, 112,
    101, 111, 112, 108, 101, 66, 80, 10, 20, 99, 111, 109, 46, 101, 120, 97, 109, 112, 108, 101,
    46, 116, 117, 116, 111, 114, 105, 97, 108, 66, 17, 65, 100, 100, 114, 101, 115, 115, 66, 111,
    111, 107, 80, 114, 111, 116, 111, 115, 170, 2, 36, 71, 111, 111, 103, 108, 101, 46, 80, 114,
    111, 116, 111, 98, 117, 102, 46, 69, 120, 97, 109, 112, 108, 101, 115, 46, 65, 100, 100, 114,
    101, 115, 115, 66, 111, 111, 107, 98, 6, 112, 114, 111, 116, 111, 51,
];
pub mod addressbook_proto {
    pub fn file() -> &'static ::protrust::reflect::FileDescriptor<'static> {
        super::pool()
            .find_file_by_name("addressbook.proto")
            .unwrap()
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct Person {
        name: ::std::string::String,
        id: i32,
        email: ::std::string::String,
        phones: ::protrust::collections::RepeatedField<self::person::PhoneNumber>,
        last_updated: ::std::option::Option<
            ::std::boxed::Box<self::super::google_protobuf_timestamp_proto::Timestamp>,
        >,
        unknown_fields: ::protrust::UnknownFieldSet,
    }
    impl ::protrust::CodedMessage for self::Person {
        fn merge_from(
            &mut self,
            input: &mut ::protrust::io::CodedInput,
        ) -> ::protrust::io::InputResult<()> {
            while let ::std::option::Option::Some(tag) = input.read_tag()? {
                match tag.get() {
                    10 => *self.name_mut() = input.read_string()?,
                    16 | 18 => *self.id_mut() = input.read_int32()?,
                    26 => *self.email_mut() = input.read_string()?,
                    34 => self
                        .phones
                        .add_entries(input, &self::person::PHONES_CODEC)?,
                    42 => input.read_message(self.last_updated_mut())?,
                    _ => self.unknown_fields.merge_from(tag, input)?,
                }
            }
            ::std::result::Result::Ok(())
        }
        fn calculate_size(&self) -> i32 {
            let mut size = 0i32;
            if **self.name() != *Self::NAME_DEFAULT_VALUE {
                size += 1;
                size += ::protrust::io::sizes::string(self.name());
            }
            if *self.id() != Self::ID_DEFAULT_VALUE {
                size += 1;
                size += ::protrust::io::sizes::int32(*self.id());
            }
            if **self.email() != *Self::EMAIL_DEFAULT_VALUE {
                size += 1;
                size += ::protrust::io::sizes::string(self.email());
            }
            size += self.phones().calculate_size(&self::person::PHONES_CODEC);
            if let ::std::option::Option::Some(last_updated) = self.last_updated() {
                size += 1;
                size += ::protrust::io::sizes::message(last_updated);
            }
            size += self.unknown_fields.calculate_size();
            size
        }
        fn write_to(
            &self,
            output: &mut ::protrust::io::CodedOutput,
        ) -> ::protrust::io::OutputResult {
            if **self.name() != *Self::NAME_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[10])?;
                output.write_string(self.name())?;
            }
            if *self.id() != Self::ID_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[16])?;
                output.write_int32(*self.id())?;
            }
            if **self.email() != *Self::EMAIL_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[26])?;
                output.write_string(self.email())?;
            }
            self.phones()
                .write_to(output, &self::person::PHONES_CODEC)?;
            if let ::std::option::Option::Some(last_updated) = self.last_updated() {
                output.write_raw_tag_bytes(&[42])?;
                output.write_message(last_updated)?;
            }
            self.unknown_fields.write_to(output)?;
            ::std::result::Result::Ok(())
        }
        fn is_initialized(&self) -> bool {
            if !self.phones.is_initialized() {
                return false;
            }
            if let ::std::option::Option::Some(last_updated) = self.last_updated() {
                if !::protrust::CodedMessage::is_initialized(last_updated) {
                    return false;
                }
            }
            true
        }
    }
    impl ::protrust::LiteMessage for self::Person {
        fn merge(&mut self, other: &Self) {
            if other.name.len() != 0 {
                *self.name_mut() = ::std::clone::Clone::clone(other.name());
            }
            if *other.id() != Self::ID_DEFAULT_VALUE {
                *self.id_mut() = *other.id();
            }
            if other.email.len() != 0 {
                *self.email_mut() = ::std::clone::Clone::clone(other.email());
            }
            self.phones.merge(&other.phones);
            if let ::std::option::Option::Some(last_updated) = &other.last_updated() {
                ::protrust::LiteMessage::merge(self.last_updated_mut(), last_updated);
            }
            self.unknown_fields.merge(&other.unknown_fields);
        }
    }
    impl ::protrust::Message for self::Person {
        fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor<'static> {
            &self::file().messages()[0]
        }
    }
    impl self::Person {
        pub const NAME_FIELD_NUMBER: i32 = 1;
        pub const NAME_DEFAULT_VALUE: &'static str = "";
        pub fn name(&self) -> &::std::string::String {
            &self.name
        }
        pub fn name_mut(&mut self) -> &mut ::std::string::String {
            &mut self.name
        }
        pub const ID_FIELD_NUMBER: i32 = 2;
        pub const ID_DEFAULT_VALUE: i32 = 0;
        pub fn id(&self) -> &i32 {
            &self.id
        }
        pub fn id_mut(&mut self) -> &mut i32 {
            &mut self.id
        }
        pub const EMAIL_FIELD_NUMBER: i32 = 3;
        pub const EMAIL_DEFAULT_VALUE: &'static str = "";
        pub fn email(&self) -> &::std::string::String {
            &self.email
        }
        pub fn email_mut(&mut self) -> &mut ::std::string::String {
            &mut self.email
        }
        pub const PHONES_FIELD_NUMBER: i32 = 4;
        pub fn phones(&self) -> &::protrust::collections::RepeatedField<self::person::PhoneNumber> {
            &self.phones
        }
        pub fn phones_mut(
            &mut self,
        ) -> &mut ::protrust::collections::RepeatedField<self::person::PhoneNumber> {
            &mut self.phones
        }
        pub const LAST_UPDATED_FIELD_NUMBER: i32 = 5;
        pub fn last_updated(
            &self,
        ) -> ::std::option::Option<&self::super::google_protobuf_timestamp_proto::Timestamp>
        {
            self.last_updated.as_ref().map(|v| &**v)
        }
        pub fn last_updated_mut(
            &mut self,
        ) -> &mut self::super::google_protobuf_timestamp_proto::Timestamp {
            self.last_updated
                .get_or_insert_with(::std::default::Default::default)
        }
        pub fn has_last_updated(&self) -> bool {
            self.last_updated.is_some()
        }
        pub fn set_last_updated(
            &mut self,
            value: self::super::google_protobuf_timestamp_proto::Timestamp,
        ) {
            self.last_updated = ::std::option::Option::Some(::std::convert::From::from(value))
        }
        pub fn take_last_updated(
            &mut self,
        ) -> ::std::option::Option<self::super::google_protobuf_timestamp_proto::Timestamp>
        {
            self.last_updated.take().map(|v| *v)
        }
        pub fn clear_last_updated(&mut self) {
            self.last_updated = ::std::option::Option::None;
        }
    }
    pub mod person {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum PhoneType {
            Mobile,
            Home,
            Work,
        }
        impl ::protrust::Enum for self::PhoneType {
            fn descriptor() -> &'static ::protrust::reflect::EnumDescriptor<'static> {
                &<self::super::Person as ::protrust::Message>::descriptor().enums()[0]
            }
        }
        impl ::std::convert::TryFrom<i32> for self::PhoneType {
            type Error = ::protrust::VariantUndefinedError;
            fn try_from(
                value: i32,
            ) -> ::std::result::Result<Self, ::protrust::VariantUndefinedError> {
                #[allow(unreachable_patterns)]
                match value {
                    0 => ::std::result::Result::Ok(self::PhoneType::Mobile),
                    1 => ::std::result::Result::Ok(self::PhoneType::Home),
                    2 => ::std::result::Result::Ok(self::PhoneType::Work),
                    _ => ::std::result::Result::Err(::protrust::VariantUndefinedError),
                }
            }
        }
        impl ::std::convert::From<self::PhoneType> for i32 {
            fn from(value: self::PhoneType) -> i32 {
                match value {
                    self::PhoneType::Mobile => 0,
                    self::PhoneType::Home => 1,
                    self::PhoneType::Work => 2,
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Default)]
        pub struct PhoneNumber {
            number: ::std::string::String,
            r#type: ::protrust::EnumValue<self::PhoneType>,
            unknown_fields: ::protrust::UnknownFieldSet,
        }
        impl ::protrust::CodedMessage for self::PhoneNumber {
            fn merge_from(
                &mut self,
                input: &mut ::protrust::io::CodedInput,
            ) -> ::protrust::io::InputResult<()> {
                while let ::std::option::Option::Some(tag) = input.read_tag()? {
                    match tag.get() {
                        10 => *self.number_mut() = input.read_string()?,
                        16 | 18 => *self.type_mut() = input.read_enum_value()?,
                        _ => self.unknown_fields.merge_from(tag, input)?,
                    }
                }
                ::std::result::Result::Ok(())
            }
            fn calculate_size(&self) -> i32 {
                let mut size = 0i32;
                if **self.number() != *Self::NUMBER_DEFAULT_VALUE {
                    size += 1;
                    size += ::protrust::io::sizes::string(self.number());
                }
                if *self.r#type() != Self::TYPE_DEFAULT_VALUE {
                    size += 1;
                    size += ::protrust::io::sizes::enum_value(*self.r#type());
                }
                size += self.unknown_fields.calculate_size();
                size
            }
            fn write_to(
                &self,
                output: &mut ::protrust::io::CodedOutput,
            ) -> ::protrust::io::OutputResult {
                if **self.number() != *Self::NUMBER_DEFAULT_VALUE {
                    output.write_raw_tag_bytes(&[10])?;
                    output.write_string(self.number())?;
                }
                if *self.r#type() != Self::TYPE_DEFAULT_VALUE {
                    output.write_raw_tag_bytes(&[16])?;
                    output.write_enum_value(*self.r#type())?;
                }
                self.unknown_fields.write_to(output)?;
                ::std::result::Result::Ok(())
            }
            fn is_initialized(&self) -> bool {
                true
            }
        }
        impl ::protrust::LiteMessage for self::PhoneNumber {
            fn merge(&mut self, other: &Self) {
                if other.number.len() != 0 {
                    *self.number_mut() = ::std::clone::Clone::clone(other.number());
                }
                if *other.r#type() != Self::TYPE_DEFAULT_VALUE {
                    *self.type_mut() = *other.r#type();
                }
                self.unknown_fields.merge(&other.unknown_fields);
            }
        }
        impl ::protrust::Message for self::PhoneNumber {
            fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor<'static> {
                &<self::super::Person as ::protrust::Message>::descriptor().messages()[0]
            }
        }
        impl self::PhoneNumber {
            pub const NUMBER_FIELD_NUMBER: i32 = 1;
            pub const NUMBER_DEFAULT_VALUE: &'static str = "";
            pub fn number(&self) -> &::std::string::String {
                &self.number
            }
            pub fn number_mut(&mut self) -> &mut ::std::string::String {
                &mut self.number
            }
            pub const TYPE_FIELD_NUMBER: i32 = 2;
            pub const TYPE_DEFAULT_VALUE: ::protrust::EnumValue<self::PhoneType> =
                ::protrust::EnumValue::Defined(self::PhoneType::Mobile);
            pub fn r#type(&self) -> &::protrust::EnumValue<self::PhoneType> {
                &self.r#type
            }
            pub fn type_mut(&mut self) -> &mut ::protrust::EnumValue<self::PhoneType> {
                &mut self.r#type
            }
        }
        pub(in super::super::super) mod phone_number {
            pub(in super::super::super) static NUMBER_REFLECTOR:
                ::protrust::reflect::access::SimpleFieldAccessor<
                    self::super::PhoneNumber,
                    ::std::string::String,
                > = ::protrust::reflect::access::SimpleFieldAccessor {
                get: self::super::PhoneNumber::number,
                get_mut: self::super::PhoneNumber::number_mut,
            };
            pub(in super::super::super) static TYPE_REFLECTOR:
                ::protrust::reflect::access::SimpleFieldAccessor<
                    self::super::PhoneNumber,
                    ::protrust::EnumValue<self::super::PhoneType>,
                > = ::protrust::reflect::access::SimpleFieldAccessor {
                get: self::super::PhoneNumber::r#type,
                get_mut: self::super::PhoneNumber::type_mut,
            };
        }
        pub(super) static PHONES_CODEC: ::protrust::Codec<self::PhoneNumber> =
            ::protrust::Codec::message(34);
        pub(in super::super) static NAME_REFLECTOR:
            ::protrust::reflect::access::SimpleFieldAccessor<
                self::super::Person,
                ::std::string::String,
            > = ::protrust::reflect::access::SimpleFieldAccessor {
            get: self::super::Person::name,
            get_mut: self::super::Person::name_mut,
        };
        pub(in super::super) static ID_REFLECTOR: ::protrust::reflect::access::SimpleFieldAccessor<
            self::super::Person,
            i32,
        > = ::protrust::reflect::access::SimpleFieldAccessor {
            get: self::super::Person::id,
            get_mut: self::super::Person::id_mut,
        };
        pub(in super::super) static EMAIL_REFLECTOR:
            ::protrust::reflect::access::SimpleFieldAccessor<
                self::super::Person,
                ::std::string::String,
            > = ::protrust::reflect::access::SimpleFieldAccessor {
            get: self::super::Person::email,
            get_mut: self::super::Person::email_mut,
        };
        pub(in super::super) static PHONES_REFLECTOR:
            ::protrust::reflect::access::SimpleFieldAccessor<
                self::super::Person,
                ::protrust::collections::RepeatedField<self::PhoneNumber>,
            > = ::protrust::reflect::access::SimpleFieldAccessor {
            get: self::super::Person::phones,
            get_mut: self::super::Person::phones_mut,
        };
        pub(in super::super) static LAST_UPDATED_REFLECTOR:
            ::protrust::reflect::access::VerboseFieldAccessor<
                self::super::Person,
                self::super::super::google_protobuf_timestamp_proto::Timestamp,
            > = ::protrust::reflect::access::VerboseFieldAccessor {
            get_option: self::super::Person::last_updated,
            get_mut: self::super::Person::last_updated_mut,
            set: self::super::Person::set_last_updated,
            take: self::super::Person::take_last_updated,
            clear: self::super::Person::clear_last_updated,
        };
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct AddressBook {
        people: ::protrust::collections::RepeatedField<self::Person>,
        unknown_fields: ::protrust::UnknownFieldSet,
    }
    impl ::protrust::CodedMessage for self::AddressBook {
        fn merge_from(
            &mut self,
            input: &mut ::protrust::io::CodedInput,
        ) -> ::protrust::io::InputResult<()> {
            while let ::std::option::Option::Some(tag) = input.read_tag()? {
                match tag.get() {
                    10 => self
                        .people
                        .add_entries(input, &self::address_book::PEOPLE_CODEC)?,
                    _ => self.unknown_fields.merge_from(tag, input)?,
                }
            }
            ::std::result::Result::Ok(())
        }
        fn calculate_size(&self) -> i32 {
            let mut size = 0i32;
            size += self
                .people()
                .calculate_size(&self::address_book::PEOPLE_CODEC);
            size += self.unknown_fields.calculate_size();
            size
        }
        fn write_to(
            &self,
            output: &mut ::protrust::io::CodedOutput,
        ) -> ::protrust::io::OutputResult {
            self.people()
                .write_to(output, &self::address_book::PEOPLE_CODEC)?;
            self.unknown_fields.write_to(output)?;
            ::std::result::Result::Ok(())
        }
        fn is_initialized(&self) -> bool {
            if !self.people.is_initialized() {
                return false;
            }
            true
        }
    }
    impl ::protrust::LiteMessage for self::AddressBook {
        fn merge(&mut self, other: &Self) {
            self.people.merge(&other.people);
            self.unknown_fields.merge(&other.unknown_fields);
        }
    }
    impl ::protrust::Message for self::AddressBook {
        fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor<'static> {
            &self::file().messages()[1]
        }
    }
    impl self::AddressBook {
        pub const PEOPLE_FIELD_NUMBER: i32 = 1;
        pub fn people(&self) -> &::protrust::collections::RepeatedField<self::Person> {
            &self.people
        }
        pub fn people_mut(&mut self) -> &mut ::protrust::collections::RepeatedField<self::Person> {
            &mut self.people
        }
    }
    pub(in super::super) mod address_book {
        pub(super) static PEOPLE_CODEC: ::protrust::Codec<self::super::Person> =
            ::protrust::Codec::message(10);
        pub(in super::super) static PEOPLE_REFLECTOR:
            ::protrust::reflect::access::SimpleFieldAccessor<
                self::super::AddressBook,
                ::protrust::collections::RepeatedField<self::super::Person>,
            > = ::protrust::reflect::access::SimpleFieldAccessor {
            get: self::super::AddressBook::people,
            get_mut: self::super::AddressBook::people_mut,
        };
    }
}
