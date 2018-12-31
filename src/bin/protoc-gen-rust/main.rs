mod generators;
mod names;
mod printer;

use protrust::plugin;
use protrust::prelude::*;
use protrust::reflect;
use std::fmt::Write;
use std::io::{stdin, stdout};

pub struct Options {
    /// Allows users to change the name of the crate for referencing the codegen modules.
    ///
    /// The default is 'protrust'
    pub crate_name: String,
    /// Allows users to make the compiler not generate JSON trait implementations, even for proto3 files
    pub no_json: bool,
    /// Sets all generated fields to public and doesn't generate accessors
    ///
    /// Static default values will also be set to public
    pub pub_fields: bool,
    /// Uses checked addition in CodedMessage::calculate_size. Must be used with the checked_size feature
    pub size_checks: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            crate_name: "::protrust".to_string(),
            no_json: false,
            pub_fields: false,
            size_checks: false,
        }
    }
}

fn main() {
    match plugin::CodeGeneratorRequest::read_new(&mut stdin()) {
        Ok(request) => run(request)
            .write(&mut stdout())
            .expect("Could not write response to stdout!"),
        Err(e) => panic!("{:?}", e),
    }
}

fn run(request: plugin::CodeGeneratorRequest) -> plugin::CodeGeneratorResponse {
    fn error(
        mut response: plugin::CodeGeneratorResponse,
        msg: String,
    ) -> plugin::CodeGeneratorResponse {
        response.file.clear();
        response.error = Some(msg);
        response
    }

    let mut response = plugin::CodeGeneratorResponse::new();
    let options = match parse_options(request.parameter.as_ref()) {
        Ok(k) => k,
        Err(s) => return error(response, s),
    };

    let mut mod_file_content =
        "#![allow(unused_variables, dead_code, non_camel_case_types)]\n\n".to_string();
    let pool = reflect::DescriptorPool::build_from_files(request.proto_file.as_slice());
    for file in request.file_to_generate.iter() {
        let descriptor: &reflect::FileDescriptor = pool
            .find_file_by_name(file)
            .expect("proto_file did not contain file to generate");

        let mut printer = printer::Printer::new(String::new());
        let mut generator = generators::Generator::<reflect::FileDescriptor, _>::new(
            &mut printer,
            descriptor,
            &options,
        );
        match generator.generate() {
            Ok(()) => {
                let mut gen_file = plugin::CodeGeneratorResponse_File::new();
                gen_file.name = Some(names::get_rust_file_name(descriptor));
                gen_file.content = Some(printer.into_inner());

                response.file.push(gen_file);
            }
            Err(err) => return error(response, format!("{:?}", err)),
        }

        writeln!(
            mod_file_content,
            "#[path = \"{}\"]",
            names::get_rust_file_name(descriptor)
        )
        .expect("Could not format generated mod file"); // write the path override
        writeln!(
            mod_file_content,
            "pub mod {};",
            names::get_rust_file_mod_name(descriptor)
        )
        .expect("Could not format generated mod file"); // write the mod definition
    }

    let mut mod_file = plugin::CodeGeneratorResponse_File::new();
    mod_file.content = Some(mod_file_content);
    mod_file.name = Some("mod.rs".to_string());

    response.file.push(mod_file);
    response
}

fn parse_options(params: Option<&String>) -> Result<Options, String> {
    let mut options = Options::default();

    if let Some(params) = params {
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
                ("pub_fields", None) => options.pub_fields = true,
                ("checked_size", None) => options.size_checks = true,
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
