//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: addressbook.proto

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: std::string::String,
    pub id: i32,
    pub email: std::string::String,
    pub phones: protrust::collections::RepeatedField<self::Person_PhoneNumber>,
    pub last_updated: std::option::Option<std::boxed::Box<protrust::wkt::timestamp::Timestamp>>,
    unknown_fields: protrust::UnknownFieldSet
}
static PERSON_PHONES_CODEC: protrust::Codec<self::Person_PhoneNumber> = protrust::Codec::message(34);
impl protrust::CodedMessage for self::Person {
    fn merge_from(&mut self, input: &mut protrust::io::CodedInput) -> protrust::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                16 => self.id = input.read_int32()?,
                26 => self.email = input.read_string()?,
                34 => self.phones.add_entries(tag.get(), input, &PERSON_PHONES_CODEC)?,
                42 => input.read_message(self.last_updated.get_or_insert_with(protrust::LiteMessage::new))?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            size += 1;
            size += protrust::io::sizes::string(name);
        }
        let id = self.id;
        if id != Self::ID_DEFAULT_VALUE {
            size += 1;
            size += protrust::io::sizes::int32(id);
        }
        let email = &self.email;
        if email != Self::EMAIL_DEFAULT_VALUE {
            size += 1;
            size += protrust::io::sizes::string(email);
        }
        size += self.phones.calculate_size(&PERSON_PHONES_CODEC);
        let last_updated = &self.last_updated;
        if let std::option::Option::Some(last_updated) = last_updated {
            size += 1;
            size += protrust::io::sizes::message(last_updated);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut protrust::io::CodedOutput) -> protrust::io::OutputResult {
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
        if let std::option::Option::Some(last_updated) = last_updated {
            output.write_raw_tag_bytes(&[42])?;
            output.write_message(last_updated)?;
        }
        self.unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl protrust::LiteMessage for self::Person {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            id: Self::ID_DEFAULT_VALUE,
            email: std::string::String::new(),
            phones: protrust::collections::RepeatedField::new(),
            last_updated: std::option::Option::None,
            unknown_fields: protrust::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::Person {
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
        if let std::option::Option::Some(last_updated) = &other.last_updated {
            self.last_updated.get_or_insert_with(protrust::LiteMessage::new).clone_from(last_updated);
        }
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl protrust::Message for self::Person {
    fn descriptor() -> &'static protrust::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Person {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'id' field
    pub const ID_FIELD_NUMBER: i32 = 2;
    pub const ID_DEFAULT_VALUE: i32 = 0;
    /// Gets the field number of the 'email' field
    pub const EMAIL_FIELD_NUMBER: i32 = 3;
    pub const EMAIL_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'phones' field
    pub const PHONES_FIELD_NUMBER: i32 = 4;
    /// Gets the field number of the 'last_updated' field
    pub const LAST_UPDATED_FIELD_NUMBER: i32 = 5;
}
#[derive(Debug, PartialEq)]
pub struct Person_PhoneNumber {
    pub number: std::string::String,
    pub r#type: protrust::EnumValue<self::Person_PhoneType>,
    unknown_fields: protrust::UnknownFieldSet
}
impl protrust::CodedMessage for self::Person_PhoneNumber {
    fn merge_from(&mut self, input: &mut protrust::io::CodedInput) -> protrust::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.number = input.read_string()?,
                16 => self.r#type = input.read_enum_value()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let number = &self.number;
        if number != Self::NUMBER_DEFAULT_VALUE {
            size += 1;
            size += protrust::io::sizes::string(number);
        }
        let r#type = self.r#type;
        if r#type != Self::TYPE_DEFAULT_VALUE {
            size += 1;
            size += protrust::io::sizes::enum_value(r#type);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut protrust::io::CodedOutput) -> protrust::io::OutputResult {
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
        std::result::Result::Ok(())
    }
}
impl protrust::LiteMessage for self::Person_PhoneNumber {
    fn new() -> Self {
        Self {
            number: std::string::String::new(),
            r#type: Self::TYPE_DEFAULT_VALUE,
            unknown_fields: protrust::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::Person_PhoneNumber {
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
impl protrust::Message for self::Person_PhoneNumber {
    fn descriptor() -> &'static protrust::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Person_PhoneNumber {
    /// Gets the field number of the 'number' field
    pub const NUMBER_FIELD_NUMBER: i32 = 1;
    pub const NUMBER_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'type' field
    pub const TYPE_FIELD_NUMBER: i32 = 2;
    pub const TYPE_DEFAULT_VALUE: protrust::EnumValue<self::Person_PhoneType> = protrust::EnumValue::Defined(self::Person_PhoneType::Mobile);
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Person_PhoneType {
    Mobile = 0,
    Home = 1,
    Work = 2,
}
impl std::convert::TryFrom<i32> for self::Person_PhoneType {
    type Error = protrust::VariantUndefinedError;
    
    fn try_from(value: i32) -> std::result::Result<Self, protrust::VariantUndefinedError> {
        match value {
            0 => std::result::Result::Ok(self::Person_PhoneType::Mobile),
            1 => std::result::Result::Ok(self::Person_PhoneType::Home),
            2 => std::result::Result::Ok(self::Person_PhoneType::Work),
            _ => std::result::Result::Err(protrust::VariantUndefinedError)
        }
    }
}
impl std::convert::From<self::Person_PhoneType> for i32 {
    fn from(value: self::Person_PhoneType) -> i32 {
        value as i32
    }
}
#[derive(Debug, PartialEq)]
pub struct AddressBook {
    pub people: protrust::collections::RepeatedField<self::Person>,
    unknown_fields: protrust::UnknownFieldSet
}
static ADDRESS_BOOK_PEOPLE_CODEC: protrust::Codec<self::Person> = protrust::Codec::message(10);
impl protrust::CodedMessage for self::AddressBook {
    fn merge_from(&mut self, input: &mut protrust::io::CodedInput) -> protrust::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.people.add_entries(tag.get(), input, &ADDRESS_BOOK_PEOPLE_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.people.calculate_size(&ADDRESS_BOOK_PEOPLE_CODEC);
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut protrust::io::CodedOutput) -> protrust::io::OutputResult {
        self.people.write_to(output, &ADDRESS_BOOK_PEOPLE_CODEC)?;
        self.unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl protrust::LiteMessage for self::AddressBook {
    fn new() -> Self {
        Self {
            people: protrust::collections::RepeatedField::new(),
            unknown_fields: protrust::UnknownFieldSet::new()
        }
    }
}
impl std::clone::Clone for self::AddressBook {
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
impl protrust::Message for self::AddressBook {
    fn descriptor() -> &'static protrust::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::AddressBook {
    /// Gets the field number of the 'people' field
    pub const PEOPLE_FIELD_NUMBER: i32 = 1;
}