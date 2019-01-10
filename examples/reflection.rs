#![feature(try_from)]

#[rustfmt::skip]
#[allow(unused_variables, dead_code, non_camel_case_types, non_snake_case)]
mod gen;

fn main() {
    let descriptor = crate::gen::addressbook_proto::file();
    println!("{:#?}", descriptor);
}
