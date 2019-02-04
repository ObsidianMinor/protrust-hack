# protrust (WIP) - A fully featured protobuf library for Rust

## Proto3 basic usage example
```rust
let mut person = Person::new();
*person.id_mut() = 1;
*person.name_mut() = "Foo".to_string();
*person.email_mut() = "foo@bar".to_string();

let mut phone = PhoneNumber::new();
*phone.number_mut() = "555-1212".to_string();
person.phones_mut().push(phone);

let person_data = person.write_to_vec()?;

let person_copy: Person = LiteMessage::read_new(&mut person_data.as_slice())?;

let mut address_book = AddressBook::new();
address_book.people_mut().push(person_copy);

let address_book_data = address_book.write_to_vec()?;

let address_book: AddressBook = LiteMessage::read_new(&mut address_book_data.as_slice())?;

assert_eq!(address_book.people().len(), 1);
assert_eq!(*address_book.people()[0], person);
```

## [Contributing](CONTRIBUTING.md)

## Roadmap

 1. Extensions
 2. Full reflection
 3. Dynamic messages
 4. JSON support
 5. Conformance
 6. Code size optimization
 7. Finish markdown filter
 8. Plugin module
 9. Docs everywhere
 10. Tests everywhere