#![feature(result_map_or_else)]

mod generators;
mod names;
mod printer;

use protrust::plugin::{CodeGeneratorRequest, CodeGeneratorResponse};
use protrust::prelude::*;
use std::convert::identity as id;
use std::io::{stdin, stdout};

pub struct Options {
    /// Allows users to change the name of the crate for referencing the codegen modules.
    ///
    /// The default is 'protrust'
    pub crate_name: String,
    /// Allows users to make the compiler not generate JSON trait implementations, even for proto3 files
    pub no_json: bool,
    /// Uses checked addition in CodedMessage::calculate_size. Must be used with the checked_size feature
    pub size_checks: bool,
    /// Includes the specified modules in a generated code module
    pub external_modules: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            crate_name: "::protrust".to_string(),
            no_json: false,
            size_checks: false,
            external_modules: Vec::new(),
        }
    }
}

fn main() {
    match CodeGeneratorRequest::read_new(&mut stdin()) {
        Ok(request) => parse_options(request.parameter())
            .and_then(|options| {
                let mut response = CodeGeneratorResponse::new();
                generators::Generator::<CodeGeneratorRequest, CodeGeneratorResponse>::new(
                    &mut response,
                    &request,
                    &options,
                )
                .generate()
                .map(|_| response)
                .map_err(|e| format!("{:?}", e))
            })
            .map_or_else(error, id)
            .write(&mut stdout())
            .expect("Could not write response to stdout!"),
        Err(e) => panic!("{:?}", e),
    }
}

fn error(msg: String) -> CodeGeneratorResponse {
    let mut response = CodeGeneratorResponse::new();
    response.set_error(msg);
    response
}

fn parse_options(params: &str) -> Result<Options, String> {
    let mut options = Options::default();

    if !params.is_empty() {
        for option in params.split(',') {
            match split_option(option) {
                ("crate_name", Some(value)) => {
                    if value != "crate" {
                        options.crate_name = format!("::{}", value)
                    } else {
                        options.crate_name = value.to_string()
                    }
                }
                ("no_json", None) => options.no_json = true,
                ("checked_size", None) => options.size_checks = true,
                ("external_modules", Some(value)) => value
                    .split('+')
                    .for_each(|s| options.external_modules.push(s.to_string())),
                (k, v) => return Err(format!("Unknown option: {}={:#?}", k, v)),
            }
        }
    }

    Ok(options)
}

fn split_option(value: &str) -> (&str, Option<&str>) {
    let mut iter = value.splitn(2, '=');

    let name = iter.next().expect("splitn returned nothing"); // splitn returns at least one item
    let value = iter.next();
    (name, value)
}
