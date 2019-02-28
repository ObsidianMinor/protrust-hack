#![feature(try_from)]

#[rustfmt::skip]
#[allow(dead_code)]
mod gen;

use crate::gen::addressbook_proto::person::PhoneNumber;
use crate::gen::addressbook_proto::*;
use protrust::prelude::*;

fn main() {
    let mut person = Person::new();
    *person.id_mut() = 1;
    *person.name_mut() = "Foo".to_string();
    *person.email_mut() = "foo@bar".to_string();

    let mut phone = PhoneNumber::new();
    *phone.number_mut() = "555-1212".to_string();
    person.phones_mut().push(phone);

    let person_data = person.write_to_vec().expect("Could not write proto to vec");

    let person_copy: Person = LiteMessage::read_new(&mut person_data.as_slice())
        .expect("Could not read proto back from vec");

    let mut address_book = AddressBook::new();
    address_book.people_mut().push(person_copy);

    let address_book_data = address_book
        .write_to_vec()
        .expect("Could not write proto to vec");

    let address_book: AddressBook = LiteMessage::read_new(&mut address_book_data.as_slice())
        .expect("Could not read proto back from vec");

    assert_eq!(address_book.people().len(), 1);
    assert_eq!(address_book.people()[0], person);
}
