#![feature(try_from)]

#[rustfmt::skip]
#[allow(dead_code)]
mod gen;

fn main() {
    let descriptor = crate::gen::addressbook_proto::file();
    println!("{:#?}", descriptor);
}
