#![feature(try_from)]

#[rustfmt::skip]
mod gen;

fn main() {
    let descriptor = crate::gen::addressbook_proto::file();
    println!("{:#?}", descriptor);
}
