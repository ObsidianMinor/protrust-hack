#![allow(dead_code)]

#[allow(non_camel_case_types, dead_code, non_snake_case)]
pub mod gen;
pub mod util;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;
