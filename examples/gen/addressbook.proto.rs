// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: addressbook.proto


pub fn file() -> &'static ::protrust::reflect::FileDescriptor {
    super::pool().find_file_by_name("addressbook.proto").unwrap()
}

///  [START messages]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Person {
    name: ::std::string::String,
    id: i32,
    email: ::std::string::String,
    phones: ::protrust::collections::RepeatedField<self::person::PhoneNumber>,
    last_updated: ::std::option::Option<::std::boxed::Box<self::super::google_protobuf_timestamp_proto::Timestamp>>,
    unknown_fields: ::protrust::UnknownFieldSet,
}
static PERSON_PHONES_CODEC: ::protrust::Codec<self::person::PhoneNumber> = ::protrust::Codec::message(34);
impl ::protrust::CodedMessage for self::Person {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                16 | 18 => self.id = input.read_int32()?,
                26 => self.email = input.read_string()?,
                34 => self.phones.add_entries(input, &PERSON_PHONES_CODEC)?,
                42 => input.read_message(&mut **self.last_updated.get_or_insert_with(|| ::std::boxed::Box::new(::protrust::LiteMessage::new())))?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::string(name);
        }
        let id = self.id;
        if id != Self::ID_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::int32(id);
        }
        let email = &self.email;
        if email != Self::EMAIL_DEFAULT_VALUE {
            size += 1;
            size += ::protrust::io::sizes::string(email);
        }
        size += self.phones.calculate_size(&PERSON_PHONES_CODEC);
        let last_updated = &self.last_updated;
        if let ::std::option::Option::Some(last_updated) = last_updated {
            size += 1;
            size += ::protrust::io::sizes::message(&**last_updated);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut ::protrust::io::CodedOutput) -> ::protrust::io::OutputResult {
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let id = self.id;
        if id != Self::ID_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[16])?;
            output.write_int32(id)?;
        }
        let email = &self.email;
        if email != Self::EMAIL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[26])?;
            output.write_string(email)?;
        }
        self.phones.write_to(output, &PERSON_PHONES_CODEC)?;
        let last_updated = &self.last_updated;
        if let ::std::option::Option::Some(last_updated) = last_updated {
            output.write_raw_tag_bytes(&[42])?;
            output.write_message(&**last_updated)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl ::protrust::LiteMessage for self::Person {
    fn new() -> Self {
        Self {
            name: ::std::string::String::new(),
            id: Self::ID_DEFAULT_VALUE,
            email: ::std::string::String::new(),
            phones: ::protrust::collections::RepeatedField::new(),
            last_updated: ::std::option::Option::None,
            unknown_fields: ::protrust::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.id = other.id;
        self.email = other.email.clone();
        self.phones.merge(&other.phones);
        if let ::std::option::Option::Some(last_updated) = &other.last_updated {
            self.last_updated.get_or_insert_with(|| ::std::boxed::Box::new(::protrust::LiteMessage::new())).merge(last_updated);
        }
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl ::protrust::Message for self::Person {
    fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Person {
    /// Gets the field number of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    pub fn name(&self) -> &::std::string::String {
        &self.name
    }
    /// Returns a unique reference to the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
    /// Gets the field number of the [`id`] field
    ///
    /// [`id`]: #method.id
    pub const ID_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`id`] field
    ///
    /// [`id`]: #method.id
    pub const ID_DEFAULT_VALUE: i32 = 0;
    ///  Unique ID number for this person.
    pub fn id(&self) -> i32 {
        self.id
    }
    /// Returns a unique reference to the [`id`] field
    ///
    /// [`id`]: #method.id
    pub fn id_mut(&mut self) -> &mut i32 {
        &mut self.id
    }
    /// Gets the field number of the [`email`] field
    ///
    /// [`email`]: #method.email
    pub const EMAIL_FIELD_NUMBER: i32 = 3;
    /// A constant value representing the default value of the [`email`] field
    ///
    /// [`email`]: #method.email
    pub const EMAIL_DEFAULT_VALUE: &'static str = "";
    pub fn email(&self) -> &::std::string::String {
        &self.email
    }
    /// Returns a unique reference to the [`email`] field
    ///
    /// [`email`]: #method.email
    pub fn email_mut(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }
    /// Gets the field number of the [`phones`] field
    ///
    /// [`phones`]: #method.phones
    pub const PHONES_FIELD_NUMBER: i32 = 4;
    pub fn phones(&self) -> &::protrust::collections::RepeatedField<self::person::PhoneNumber> {
        &self.phones
    }
    /// Returns a unique reference to the [`phones`] field
    ///
    /// [`phones`]: #method.phones
    pub fn phones_mut(&mut self) -> &mut ::protrust::collections::RepeatedField<self::person::PhoneNumber> {
        &mut self.phones
    }
    /// Gets the field number of the [`last_updated`] field
    ///
    /// [`last_updated`]: #method.last_updated
    pub const LAST_UPDATED_FIELD_NUMBER: i32 = 5;
    pub fn last_updated(&self) -> &::std::option::Option<::std::boxed::Box<self::super::google_protobuf_timestamp_proto::Timestamp>> {
        &self.last_updated
    }
    /// Returns a unique reference to the [`last_updated`] field
    ///
    /// [`last_updated`]: #method.last_updated
    pub fn last_updated_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<self::super::google_protobuf_timestamp_proto::Timestamp>> {
        &mut self.last_updated
    }
}
///  [START messages]
pub mod person {
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct PhoneNumber {
        number: ::std::string::String,
        r#type: ::protrust::EnumValue<self::PhoneType>,
        unknown_fields: ::protrust::UnknownFieldSet,
    }
    impl ::protrust::CodedMessage for self::PhoneNumber {
        fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
            while let ::std::option::Option::Some(tag) = input.read_tag()? {
                match tag.get() {
                    10 => self.number = input.read_string()?,
                    16 | 18 => self.r#type = input.read_enum_value()?,
                    _ => self.unknown_fields.merge_from(tag, input)?
                }
            }
            ::std::result::Result::Ok(())
        }
        fn calculate_size(&self) -> i32 {
            let mut size = 0i32;
            let number = &self.number;
            if number != Self::NUMBER_DEFAULT_VALUE {
                size += 1;
                size += ::protrust::io::sizes::string(number);
            }
            let r#type = self.r#type;
            if r#type != Self::TYPE_DEFAULT_VALUE {
                size += 1;
                size += ::protrust::io::sizes::enum_value(r#type);
            }
            size += self.unknown_fields.calculate_size();
            size
        }
        fn write_to(&self, output: &mut ::protrust::io::CodedOutput) -> ::protrust::io::OutputResult {
            let number = &self.number;
            if number != Self::NUMBER_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[10])?;
                output.write_string(number)?;
            }
            let r#type = self.r#type;
            if r#type != Self::TYPE_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[16])?;
                output.write_enum_value(r#type)?;
            }
            self.unknown_fields.write_to(output)?;
            ::std::result::Result::Ok(())
        }
    }
    impl ::protrust::LiteMessage for self::PhoneNumber {
        fn new() -> Self {
            Self {
                number: ::std::string::String::new(),
                r#type: Self::TYPE_DEFAULT_VALUE,
                unknown_fields: ::protrust::UnknownFieldSet::new(),
            }
        }
        fn merge(&mut self, other: &Self) {
            self.number = other.number.clone();
            self.r#type = other.r#type;
            self.unknown_fields.merge(&other.unknown_fields);
        }
    }
    impl ::protrust::Message for self::PhoneNumber {
        fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
            &self::super::file().messages()[0].messages()[0]
        }
    }
    impl self::PhoneNumber {
        /// Gets the field number of the [`number`] field
        ///
        /// [`number`]: #method.number
        pub const NUMBER_FIELD_NUMBER: i32 = 1;
        /// A constant value representing the default value of the [`number`] field
        ///
        /// [`number`]: #method.number
        pub const NUMBER_DEFAULT_VALUE: &'static str = "";
        pub fn number(&self) -> &::std::string::String {
            &self.number
        }
        /// Returns a unique reference to the [`number`] field
        ///
        /// [`number`]: #method.number
        pub fn number_mut(&mut self) -> &mut ::std::string::String {
            &mut self.number
        }
        /// Gets the field number of the [`type`] field
        ///
        /// [`type`]: #method.type
        pub const TYPE_FIELD_NUMBER: i32 = 2;
        /// A constant value representing the default value of the [`type`] field
        ///
        /// [`type`]: #method.type
        pub const TYPE_DEFAULT_VALUE: ::protrust::EnumValue<self::PhoneType> = ::protrust::EnumValue::Defined(self::PhoneType::Mobile);
        pub fn r#type(&self) -> ::protrust::EnumValue<self::PhoneType> {
            self.r#type
        }
        /// Returns a unique reference to the [`type`] field
        ///
        /// [`type`]: #method.type
        pub fn type_mut(&mut self) -> &mut ::protrust::EnumValue<self::PhoneType> {
            &mut self.r#type
        }
    }
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum PhoneType {
        Mobile,
        Home,
        Work,
    }
    impl ::protrust::Enum for self::PhoneType {
        fn descriptor() -> &'static ::protrust::reflect::EnumDescriptor {
            &self::super::file().messages()[0].enums()[0]
        }
    }
    impl ::std::convert::TryFrom<i32> for self::PhoneType {
        type Error = ::protrust::VariantUndefinedError;
        fn try_from(value: i32) -> ::std::result::Result<Self, ::protrust::VariantUndefinedError> {
            #[allow(unreachable_patterns)]
            match value {
                0 => ::std::result::Result::Ok(self::PhoneType::Mobile),
                1 => ::std::result::Result::Ok(self::PhoneType::Home),
                2 => ::std::result::Result::Ok(self::PhoneType::Work),
                _ => ::std::result::Result::Err(::protrust::VariantUndefinedError)
            }
        }
    }
    impl ::std::convert::From<self::PhoneType> for i32 {
        fn from(value: self::PhoneType) -> i32 {
            match value {
                PhoneType::Mobile => 0,
                PhoneType::Home => 1,
                PhoneType::Work => 2,
            }
        }
    }
}
///  Our address book file is just one of these.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddressBook {
    people: ::protrust::collections::RepeatedField<self::Person>,
    unknown_fields: ::protrust::UnknownFieldSet,
}
static ADDRESS_BOOK_PEOPLE_CODEC: ::protrust::Codec<self::Person> = ::protrust::Codec::message(10);
impl ::protrust::CodedMessage for self::AddressBook {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.people.add_entries(input, &ADDRESS_BOOK_PEOPLE_CODEC)?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.people.calculate_size(&ADDRESS_BOOK_PEOPLE_CODEC);
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut ::protrust::io::CodedOutput) -> ::protrust::io::OutputResult {
        self.people.write_to(output, &ADDRESS_BOOK_PEOPLE_CODEC)?;
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl ::protrust::LiteMessage for self::AddressBook {
    fn new() -> Self {
        Self {
            people: ::protrust::collections::RepeatedField::new(),
            unknown_fields: ::protrust::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.people.merge(&other.people);
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl ::protrust::Message for self::AddressBook {
    fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
        &self::file().messages()[1]
    }
}
impl self::AddressBook {
    /// Gets the field number of the [`people`] field
    ///
    /// [`people`]: #method.people
    pub const PEOPLE_FIELD_NUMBER: i32 = 1;
    pub fn people(&self) -> &::protrust::collections::RepeatedField<self::Person> {
        &self.people
    }
    /// Returns a unique reference to the [`people`] field
    ///
    /// [`people`]: #method.people
    pub fn people_mut(&mut self) -> &mut ::protrust::collections::RepeatedField<self::Person> {
        &mut self.people
    }
}