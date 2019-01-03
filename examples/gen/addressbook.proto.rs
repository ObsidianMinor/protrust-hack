// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: addressbook.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<::protrust::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[::protrust::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static ::protrust::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static ::protrust::reflect::DescriptorPool<'static>; 1]> = ::std::option::Option::None;

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([::protrust::LiteMessage::read_new(&mut [
            10, 17, 97, 100, 100, 114, 101, 115, 115, 98, 111, 111, 107, 46, 112, 114, 111, 116, 111, 18, 
            8, 116, 117, 116, 111, 114, 105, 97, 108, 26, 31, 103, 111, 111, 103, 108, 101, 47, 112, 114, 
            111, 116, 111, 98, 117, 102, 47, 116, 105, 109, 101, 115, 116, 97, 109, 112, 46, 112, 114, 111, 
            116, 111, 34, 185, 2, 10, 6, 80, 101, 114, 115, 111, 110, 18, 18, 10, 4, 110, 97, 109, 
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
            80, 104, 111, 110, 101, 84, 121, 112, 101, 82, 4, 116, 121, 112, 101, 34, 41, 10, 9, 80, 
            104, 111, 110, 101, 84, 121, 112, 101, 18, 8, 10, 6, 77, 79, 66, 73, 76, 69, 18, 8, 
            10, 4, 72, 79, 77, 69, 16, 1, 18, 8, 10, 4, 87, 79, 82, 75, 16, 2, 34, 55, 
            10, 11, 65, 100, 100, 114, 101, 115, 115, 66, 111, 111, 107, 18, 40, 10, 6, 112, 101, 111, 
            112, 108, 101, 24, 1, 32, 3, 40, 11, 50, 16, 46, 116, 117, 116, 111, 114, 105, 97, 108, 
            46, 80, 101, 114, 115, 111, 110, 82, 6, 112, 101, 111, 112, 108, 101, 66, 80, 10, 20, 99, 
            111, 109, 46, 101, 120, 97, 109, 112, 108, 101, 46, 116, 117, 116, 111, 114, 105, 97, 108, 66, 
            17, 65, 100, 100, 114, 101, 115, 115, 66, 111, 111, 107, 80, 114, 111, 116, 111, 115, 170, 2, 
            36, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 111, 98, 117, 102, 46, 69, 120, 97, 
            109, 112, 108, 101, 115, 46, 65, 100, 100, 114, 101, 115, 115, 66, 111, 111, 107, 98, 6, 112, 
            114, 111, 116, 111, 51, 
        ].as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([::protrust::wkt::timestamp::pool(), ]);
        FILE_POOL = ::std::option::Option::Some(::protrust::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap()
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("addressbook.proto").unwrap());
    }
}

/// Gets the pool containing all the symbols in this proto file and its dependencies
pub fn pool() -> &'static ::protrust::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
/// Gets the file descriptor representing the proto that created this generated file
pub fn file() -> &'static ::protrust::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
/// [START messages]
#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: ::std::string::String,
    pub id: i32,
    pub email: ::std::string::String,
    pub phones: ::protrust::collections::RepeatedField<self::Person_PhoneNumber>,
    pub last_updated: ::std::option::Option<::std::boxed::Box<::protrust::wkt::timestamp::Timestamp>>,
    unknown_fields: ::protrust::UnknownFieldSet
}
static PERSON_PHONES_CODEC: ::protrust::Codec<self::Person_PhoneNumber> = ::protrust::Codec::message(34);
impl ::protrust::CodedMessage for self::Person {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                16 => self.id = input.read_int32()?,
                26 => self.email = input.read_string()?,
                34 => self.phones.add_entries(tag.get(), input, &PERSON_PHONES_CODEC)?,
                42 => input.read_message(self.last_updated.get_or_insert_with(::protrust::LiteMessage::new))?,
                tag => self.unknown_fields.merge_from(tag, input)?
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
            size += ::protrust::io::sizes::message(last_updated);
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
            output.write_message(last_updated)?;
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
            unknown_fields: ::protrust::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Person {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            id: self.id.clone(),
            email: self.email.clone(),
            phones: self.phones.clone(),
            last_updated: self.last_updated.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.id = other.id;
        self.email = other.email.clone();
        self.phones.clone_from(&other.phones);
        if let ::std::option::Option::Some(last_updated) = &other.last_updated {
            self.last_updated.get_or_insert_with(::protrust::LiteMessage::new).clone_from(last_updated);
        }
        self.unknown_fields.clone_from(&other.unknown_fields);
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
    /// Unique ID number for this person.
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
    pub fn phones(&self) -> &::protrust::collections::RepeatedField<self::Person_PhoneNumber> {
        &self.phones
    }
    /// Returns a unique reference to the [`phones`] field
    ///
    /// [`phones`]: #method.phones
    pub fn phones_mut(&mut self) -> &mut ::protrust::collections::RepeatedField<self::Person_PhoneNumber> {
        &mut self.phones
    }
    /// Gets the field number of the [`last_updated`] field
    ///
    /// [`last_updated`]: #method.last_updated
    pub const LAST_UPDATED_FIELD_NUMBER: i32 = 5;
    pub fn last_updated(&self) -> &::std::option::Option<::std::boxed::Box<::protrust::wkt::timestamp::Timestamp>> {
        &self.last_updated
    }
    /// Returns a unique reference to the [`last_updated`] field
    ///
    /// [`last_updated`]: #method.last_updated
    pub fn last_updated_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<::protrust::wkt::timestamp::Timestamp>> {
        &mut self.last_updated
    }
}
#[derive(Debug, PartialEq)]
pub struct Person_PhoneNumber {
    pub number: ::std::string::String,
    pub r#type: ::protrust::EnumValue<self::Person_PhoneType>,
    unknown_fields: ::protrust::UnknownFieldSet
}
impl ::protrust::CodedMessage for self::Person_PhoneNumber {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.number = input.read_string()?,
                16 => self.r#type = input.read_enum_value()?,
                tag => self.unknown_fields.merge_from(tag, input)?
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
impl ::protrust::LiteMessage for self::Person_PhoneNumber {
    fn new() -> Self {
        Self {
            number: ::std::string::String::new(),
            r#type: Self::TYPE_DEFAULT_VALUE,
            unknown_fields: ::protrust::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Person_PhoneNumber {
    fn clone(&self) -> Self {
        Self {
            number: self.number.clone(),
            r#type: self.r#type.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.number = other.number.clone();
        self.r#type = other.r#type;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl ::protrust::Message for self::Person_PhoneNumber {
    fn descriptor() -> &'static ::protrust::reflect::MessageDescriptor {
        &self::file().messages()[0].messages()[0]
    }
}
impl self::Person_PhoneNumber {
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
    pub const TYPE_DEFAULT_VALUE: ::protrust::EnumValue<self::Person_PhoneType> = ::protrust::EnumValue::Defined(self::Person_PhoneType::Mobile);
    pub fn r#type(&self) -> ::protrust::EnumValue<self::Person_PhoneType> {
        self.r#type
    }
    /// Returns a unique reference to the [`type`] field
    ///
    /// [`type`]: #method.type
    pub fn type_mut(&mut self) -> &mut ::protrust::EnumValue<self::Person_PhoneType> {
        &mut self.r#type
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Person_PhoneType {
    Mobile = 0,
    Home = 1,
    Work = 2,
}
impl ::std::convert::TryFrom<i32> for self::Person_PhoneType {
    type Error = ::protrust::VariantUndefinedError;
    fn try_from(value: i32) -> ::std::result::Result<Self, ::protrust::VariantUndefinedError> {
        match value {
            0 => ::std::result::Result::Ok(self::Person_PhoneType::Mobile),
            1 => ::std::result::Result::Ok(self::Person_PhoneType::Home),
            2 => ::std::result::Result::Ok(self::Person_PhoneType::Work),
            _ => ::std::result::Result::Err(::protrust::VariantUndefinedError)
        }
    }
}
impl ::std::convert::From<self::Person_PhoneType> for i32 {
    fn from(value: self::Person_PhoneType) -> i32 {
        value as i32
    }
}
/// Our address book file is just one of these.
#[derive(Debug, PartialEq)]
pub struct AddressBook {
    pub people: ::protrust::collections::RepeatedField<self::Person>,
    unknown_fields: ::protrust::UnknownFieldSet
}
static ADDRESS_BOOK_PEOPLE_CODEC: ::protrust::Codec<self::Person> = ::protrust::Codec::message(10);
impl ::protrust::CodedMessage for self::AddressBook {
    fn merge_from(&mut self, input: &mut ::protrust::io::CodedInput) -> ::protrust::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.people.add_entries(tag.get(), input, &ADDRESS_BOOK_PEOPLE_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
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
            unknown_fields: ::protrust::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::AddressBook {
    fn clone(&self) -> Self {
        Self {
            people: self.people.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.people.clone_from(&other.people);
        self.unknown_fields.clone_from(&other.unknown_fields);
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