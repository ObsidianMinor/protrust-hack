use crate::{
    names,
    printer::{self, Printer},
    Options,
};
use protrust::descriptor::FileOptions_OptimizeMode as OptimizeMode;
use protrust::io::{self, WireType};
use protrust::plugin::{
    CodeGeneratorRequest, CodeGeneratorResponse, CodeGeneratorResponse_File as File,
};
use protrust::prelude::*;
use protrust::reflect::*;
use pulldown_cmark::{Event, Parser, Tag};
use std::collections::HashMap;
use std::fmt::Write;

macro_rules! var {
    ($target:expr, $var:expr) => {
        $target
            .get(stringify!($var))
            .ok_or_else(|| Error::MissingVariable(line!(), column!(), stringify!($var)))?
    };
}

macro_rules! gen {
    ($target:expr; $fmt:expr => $vars:expr, $($arg:ident),*) => (write!($target, $fmt, $($arg = var!($vars, $arg)),*)?);
    ($dst:expr, $($arg:tt)*) => (write!($dst, $($arg)*)?);
}

macro_rules! genln {
    ($target:expr; $fmt:expr => $vars:expr, $($arg:ident),*) => {
        {
            writeln!($target)?;
            write!($target, $fmt, $($arg = var!($vars, $arg)),*)?;
        }
    };
    ($dst:expr, $($arg:tt)*) => {
        {
            writeln!($dst)?;
            write!($dst, $($arg)*)?;
        }
    };
    ($dst:expr) => (writeln!($dst)?)
}

macro_rules! indent {
    ($target:expr, $block:block) => {
        $target.indent();
        $block
        $target.unindent();
    };
}

macro_rules! generator_new {
    ($type:ty, $p:ident, $o:ident; $($key:expr, $value:expr),*) => {
        impl<'a, W> Generator<'a, $type, W> {
            pub fn new(output: &'a mut W, $p: &'a $type, $o: &'a Options) -> Generator<'a, $type, W> {
                let mut generator = Generator {
                    output,
                    input: $p,
                    options: $o,
                    vars: HashMap::new()
                };
                $(
                    generator.vars.insert($key, $value);
                )*
                generator
            }

            #[allow(dead_code)]
            pub fn from_other<'b, T>(other: &'a mut Generator<'b, T, W>, proto: &'b $type) -> Generator<'a, $type, W> {
                Self::new(&mut other.output, proto, &other.options)
            }
        }
    };
}

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    FormatError,
    MissingInputFile(String),
    MissingVariable(u32, u32, &'static str),
}

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Error {
        Error::FormatError
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::FormatError => write!(f, "An error occured while generating the result"),
            Error::MissingInputFile(i) => write!(f, "Could not find file to generate '{}'", i),
            Error::MissingVariable(line, column, var) => write!(
                f,
                "Could not find var named '{}' on line {}, column {}",
                var, line, column
            ),
        }
    }
}

pub struct Generator<'a, T, U> {
    vars: HashMap<&'static str, String>,
    output: &'a mut U,
    input: &'a T,
    options: &'a Options,
}

generator_new!(CodeGeneratorRequest, request, options;
    "crate_name", options.crate_name.clone(),
    "file_count", request.file_to_generate().len().to_string());

impl Generator<'_, CodeGeneratorRequest, CodeGeneratorResponse> {
    pub fn generate(&mut self) -> Result {
        let pool = DescriptorPool::build_from_files(self.input.proto_file());
        let mut mod_file = File::new();
        mod_file.set_name("mod.rs".to_string());
        let mut printer = Printer::new(mod_file.content_mut());

        let files = {
            let mut files = Vec::with_capacity(self.input.file_to_generate().len());
            for file in self.input.file_to_generate().iter().map(|file| {
                pool.find_file_by_name(file)
                    .ok_or_else(|| Error::MissingInputFile(file.clone()))
            }) {
                files.push(file?);
            }
            files
        };

        for file in &files {
            Generator::<FileDescriptor, _>::new(&mut printer, file, self.options)
                .generate_mod_info()?;

            let mut code_file = File::new();
            code_file.set_name(names::get_rust_file_name(file));
            Generator::<FileDescriptor, _>::new(
                &mut Printer::new(code_file.content_mut()),
                file,
                self.options,
            )
            .generate()?;

            self.output.file_mut().push(code_file);
        }

        self.generate_extension_registry(&files, &mut printer)?;
        self.generate_pool(&files, &mut printer)?;

        self.output.file_mut().push(mod_file);

        Ok(())
    }

    pub fn generate_extension_registry<W: Write>(
        &mut self,
        files: &[&FileDescriptor],
        mod_file: &mut Printer<W>,
    ) -> Result {
        Ok(())
    }

    pub fn generate_pool<W: Write>(
        &mut self,
        files: &[&FileDescriptor],
        mod_file: &mut Printer<W>,
    ) -> Result {
        let contains_included_files = files
            .iter()
            .flat_map(|f| f.dependencies()) // include the direct dependencies
            .chain(files.iter().flat_map(|f| {
                f.public_dependencies()
                    .iter()
                    .flat_map(|p| p.dependencies()) // include the direct dependencies of our dependents public dependencies
            }))
            .map(|r| &**r) // make them standard shared references
            .collect::<std::collections::HashSet<_>>() // remove equal items
            .difference(&files.iter().map(|r| *r).collect::<std::collections::HashSet<_>>()) // get the difference (the depended items not generated)
            .any(|f| match f.name() {
                "google/protobuf/descriptor.proto"
                | "google/protobuf/compiler/plugin.proto"
                | "google/protobuf/any.proto"
                | "google/protobuf/api.proto"
                | "google/protobuf/duration.proto"
                | "google/protobuf/empty.proto"
                | "google/protobuf/field_mask.proto"
                | "google/protobuf/source_context.proto"
                | "google/protobuf/struct.proto"
                | "google/protobuf/timestamp.proto"
                | "google/protobuf/type.proto"
                | "google/protobuf/wrappers.proto" => true,
                _ => false,
            });
        let mut dep_count = 0;
        if contains_included_files {
            dep_count += 1;
        }

        genln!(mod_file; "static mut FILES: ::std::option::Option<[{crate_name}::descriptor::FileDescriptorProto; {file_count}]> = ::std::option::Option::None;" => self.vars, crate_name, file_count);
        genln!(mod_file, "static mut EXTERNAL_DEPS: ::std::option::Option<[&'static {}::reflect::DescriptorPool<'static>; {}]> = ::std::option::Option::None;", var!(self.vars, crate_name), dep_count);
        genln!(mod_file; "static mut POOL: ::std::option::Option<{crate_name}::reflect::DescriptorPool<'static>> = ::std::option::Option::None;" => self.vars, crate_name);
        genln!(
            mod_file,
            "static POOL_INIT: ::std::sync::Once = ::std::sync::Once::new();"
        );

        genln!(mod_file, "fn pool_init() {{");
        indent!(mod_file, {
            genln!(mod_file, "unsafe {{");
            indent!(mod_file, {
                genln!(mod_file, "self::FILES = ::std::option::Option::Some([");
                indent!(mod_file, {
                    for file in files {
                        Generator::<FileDescriptor, _>::new(mod_file, file, self.options)
                            .generate_blob_read()?;
                    }
                });
                genln!(mod_file, "]);");
                genln!(mod_file, "self::EXTERNAL_DEPS = ::std::option::Option::Some([");
                indent!(mod_file, {
                    if contains_included_files {
                        genln!(mod_file; "{}::pool()," => self.vars, crate_name);
                    }
                });
                genln!(mod_file, "]);");
                genln!(mod_file; "self::POOL = ::std::option::Option::Some({}::reflect::DescriptorPool::build_from_generated_code(self::FILES.as_ref().unwrap().as_ref(), self::EXTERNAL_DEPS.as_ref().unwrap(), ::std::boxed::Box::new([" => self.vars, crate_name);
                indent!(mod_file, {
                    for file in files {
                        Generator::<FileDescriptor, _>::new(mod_file, file, self.options)
                            .generate_code_info()?;
                    }
                });
                genln!(mod_file, "])));");
            });
            genln!(mod_file, "}}");
        });
        genln!(mod_file, "}}");

        genln!(mod_file; "pub fn pool() -> &'static {}::reflect::DescriptorPool<'static> {{" => self.vars, crate_name);
        indent!(mod_file, {
            genln!(mod_file, "unsafe {{");
            indent!(mod_file, {
                genln!(mod_file, "POOL_INIT.call_once(pool_init);");
                genln!(mod_file, "POOL.as_ref().unwrap()");
            });
            genln!(mod_file, "}}");
        });
        genln!(mod_file, "}}");
        Ok(())
    }
}

generator_new!(FileDescriptor, proto, options;
    "file", proto.name().to_string(),
    "file_path", names::get_rust_file_name(proto),
    "file_mod_name", names::get_rust_file_mod_name(proto),
    "file_blob_name", names::get_rust_file_mod_name(proto).to_uppercase() + "_BINARY",
    "crate_name", options.crate_name.clone(),
    "dep_count", proto.dependencies().len().to_string());

impl<W: Write> Generator<'_, FileDescriptor, Printer<W>> {
    pub fn generate(&mut self) -> Result {
        gen!(self.output, "// DO NOT EDIT!");
        genln!(
            self.output,
            "// Generated by protoc-gen-rust, part of the protrust crate."
        );
        genln!(self.output, "//");
        genln!(self.output; "// Source: {file}\n" => self.vars, file);
        genln!(self.output);
        genln!(self.output; "pub fn file() -> &'static {}::reflect::FileDescriptor {{" => self.vars, crate_name);
        indent!(self.output, {
            genln!(self.output; "super::pool().find_file_by_name(\"{}\").unwrap()" => self.vars, file);
        });
        genln!(self.output, "}}");
        genln!(self.output);

        // extensions
        //for _extension in self.input.extensions() {
        //
        //}

        // messages
        for message in self.input.messages() {
            Generator::<MessageDescriptor, _>::from_other(self, message).generate()?;
        }

        // enums
        for enum_type in self.input.enums() {
            Generator::<EnumDescriptor, _>::from_other(self, enum_type).generate()?;
        }

        Ok(())
    }

    pub fn generate_mod_info(&mut self) -> Result {
        genln!(self.output; "#[path = \"{}\"]" => self.vars, file_path);
        genln!(self.output; "pub mod {};" => self.vars, file_mod_name);
        genln!(self.output; "static {}: &'static [u8] = &[" => self.vars, file_blob_name);
        indent!(self.output, {
            let mut new_proto = self.input.proto().clone();
            new_proto.clear_source_code_info();
            let vec = new_proto.write_to_vec().unwrap();
            for chunk in vec.chunks(20) {
                genln!(self.output);
                for byte in chunk {
                    gen!(self.output, "{}, ", byte);
                }
            }
        });
        genln!(self.output, "];");
        Ok(())
    }

    pub fn generate_code_info(&mut self) -> Result {
        genln!(self.output; "{crate_name}::reflect::GeneratedCodeInfo {{" => self.vars, crate_name);
        indent!(self.output, {
            if self.input.messages().len() == 0 {
                genln!(self.output, "structs: ::std::option::Option::None,");
            } else {
                genln!(
                    self.output,
                    "structs: ::std::option::Option::Some(::std::boxed::Box::new(["
                );
                indent!(self.output, {
                    for message in self.input.messages().iter().filter(|m| !m.map_entry()) {
                        Generator::<MessageDescriptor, _>::from_other(self, message)
                            .generate_struct_info()?;
                    }
                });
                genln!(self.output, "])),");
            }
        });
        genln!(self.output, "}},");

        Ok(())
    }

    pub fn generate_blob_read(&mut self) -> Result {
        genln!(self.output; "{}::LiteMessage::read_new(&mut {}.as_ref()).expect(\"Could not read file descriptor\")," => self.vars, crate_name, file_blob_name);
        Ok(())
    }
}

generator_new!(MessageDescriptor, proto, options;
    "type_name", names::get_message_type_name(proto),
    "full_type_name", names::get_full_message_type_name(proto, Some(proto.file()), &options.crate_name),
    "full_type_mod_name", names::get_full_message_type_name(proto, None, &options.crate_name),
    "crate_name", options.crate_name.clone());

impl<W: Write> Generator<'_, MessageDescriptor, Printer<W>> {
    pub fn generate_rustdoc_comments(&mut self) -> Result {
        if let Some(source_info) = self.input.source_code_info() {
            generate_rustdoc_comments(self.output, source_info)?;
        }

        Ok(())
    }

    pub fn generate_struct_info(&mut self) -> Result {
        genln!(self.output; "{crate_name}::reflect::GeneratedStructInfo {{" => self.vars, crate_name);
        indent!(self.output, {
            genln!(self.output; "new: || ::std::boxed::Box::new(<{full_type_mod_name} as {crate_name}::LiteMessage>::new())," => self.vars, full_type_mod_name, crate_name);
            if self.input.messages().len() == 0 {
                genln!(self.output, "structs: ::std::option::Option::None,");
            } else {
                genln!(
                    self.output,
                    "structs: ::std::option::Option::Some(::std::boxed::Box::new(["
                );
                indent!(self.output, {
                    for message in self.input.messages().iter().filter(|m| !m.map_entry()) {
                        Generator::<MessageDescriptor, _>::from_other(self, message)
                            .generate_struct_info()?;
                    }
                });
                genln!(self.output, "])),");
            }
        });
        genln!(self.output, "}},");
        Ok(())
    }

    pub fn generate(&mut self) -> Result {
        self.generate_rustdoc_comments()?;
        genln!(self.output, "#[derive(Clone, Debug, PartialEq)]");
        genln!(self.output; "pub struct {type_name} {{" => self.vars, type_name);
        indent!(self.output, {
            for field in self.input.message_fields() {
                Generator::<FieldDescriptor, _>::from_other(self, field).generate_struct_field()?;
            }

            for oneof in self.input.oneofs() {
                Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_struct_field()?;
            }

            genln!(self.output; "unknown_fields: {crate_name}::UnknownFieldSet" => self.vars, crate_name);
        });
        genln!(self.output, "}}");

        for field in self.input.fields() {
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_codec()?;
        }

        for oneof in self.input.oneofs() {
            Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_type()?;
        }

        self.generate_coded_message_impl()?;
        self.generate_lite_message_impl()?;

        if self.input.file().options().map(|o| o.optimize_for())
            != Some(EnumValue::Defined(OptimizeMode::LiteRuntime))
        {
            self.generate_message_impl()?;
        }

        self.generate_struct_impl()?;

        for nested in self.input.messages().iter().filter(|m| !m.map_entry()) {
            Generator::<MessageDescriptor, _>::from_other(self, nested).generate()?;
        }

        for nested in self.input.enums() {
            Generator::<EnumDescriptor, _>::from_other(self, nested).generate()?;
        }

        Ok(())
    }

    pub fn generate_coded_message_impl(&mut self) -> Result {
        genln!(self.output; "impl {crate_name}::CodedMessage for {full_type_name} {{" => self.vars, crate_name, full_type_name);
        indent!(self.output, {
            genln!(self.output; "fn merge_from(&mut self, input: &mut {crate_name}::io::CodedInput) -> {crate_name}::io::InputResult<()> {{" => self.vars, crate_name);
            indent!(self.output, {
                genln!(
                    self.output,
                    "while let ::std::option::Option::Some(tag) = input.read_tag()? {{"
                );
                indent!(self.output, {
                    genln!(self.output, "match tag.get() {{");
                    indent!(self.output, {
                        for field in self.input.fields() {
                            Generator::<FieldDescriptor, _>::from_other(self, field)
                                .generate_merge_arm()?;
                        }
                        genln!(
                            self.output,
                            "_ => self.unknown_fields.merge_from(tag, input)?"
                        );
                    });
                    genln!(self.output, "}}");
                });
                genln!(self.output, "}}");
                genln!(self.output, "::std::result::Result::Ok(())");
            });
            genln!(self.output, "}}");

            if self.options.size_checks {
                genln!(
                    self.output,
                    "fn calculate_size(&self) -> ::std::option::Option<i32> {{"
                );
            } else {
                genln!(self.output, "fn calculate_size(&self) -> i32 {{");
            }
            indent!(self.output, {
                genln!(self.output, "let mut size = 0i32;");
                for field in self.input.fields() {
                    Generator::<FieldDescriptor, _>::from_other(self, field)
                        .generate_size_calculator()?;
                }
                if self.options.size_checks {
                    genln!(
                        self.output,
                        "size = size.checked_add(self.unknown_fields.calculate_size()?)?;"
                    );
                    genln!(self.output, "::std::option::Option::Some(size)");
                } else {
                    genln!(self.output, "size += self.unknown_fields.calculate_size();");
                    genln!(self.output, "size");
                }
            });
            genln!(self.output, "}}");

            genln!(self.output; "fn write_to(&self, output: &mut {crate_name}::io::CodedOutput) -> {crate_name}::io::OutputResult {{" => self.vars, crate_name);
            indent!(self.output, {
                for field in self.input.fields() {
                    Generator::<FieldDescriptor, _>::from_other(self, field).generate_writer()?;
                }
                genln!(self.output, "self.unknown_fields.write_to(output)?;");
                genln!(self.output, "::std::result::Result::Ok(())");
            });
            genln!(self.output, "}}");

            if self.input.file().syntax() == Syntax::Proto2
                && self.input.fields().iter().any(|i| {
                    i.label() == FieldLabel::Required
                        || i.label() == FieldLabel::Repeated
                        || i.field_type().is_message()
                        || i.field_type().is_group()
                })
            {
                genln!(self.output, "fn is_initialized(&self) -> bool {{");
                indent!(self.output, {
                    for field in self.input.fields().iter().filter(|i| {
                        i.label() == FieldLabel::Required
                            || i.field_type().is_message()
                            || i.field_type().is_group()
                    }) {
                        Generator::<FieldDescriptor, _>::from_other(self, field)
                            .generate_is_initialized()?;
                    }
                    genln!(self.output, "true");
                });
                genln!(self.output, "}}");
            }
        });
        genln!(self.output, "}}");

        Ok(())
    }

    pub fn generate_lite_message_impl(&mut self) -> Result {
        genln!(self.output; "impl {crate_name}::LiteMessage for {full_type_name} {{" => self.vars, crate_name, full_type_name);
        indent!(self.output, {
            genln!(self.output, "fn new() -> Self {{");
            indent!(self.output, {
                genln!(self.output, "Self {{");
                indent!(self.output, {
                    for field in self.input.message_fields() {
                        Generator::<FieldDescriptor, _>::from_other(self, field).generate_new()?;
                    }

                    for oneof in self.input.oneofs() {
                        Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_new()?;
                    }

                    genln!(
                        self.output,
                        "unknown_fields: {}::UnknownFieldSet::new()",
                        self.options.crate_name
                    );
                });
                genln!(self.output, "}}");
            });
            genln!(self.output, "}}");
            genln!(self.output, "fn merge(&mut self, other: &Self) {{");
            indent!(self.output, {
                for field in self.input.fields() {
                    Generator::<FieldDescriptor, _>::from_other(self, field)
                        .generate_field_merge()?;
                }

                genln!(
                    self.output,
                    "self.unknown_fields.merge(&other.unknown_fields);"
                );
            });
            genln!(self.output, "}}");
        });
        genln!(self.output, "}}");

        Ok(())
    }

    pub fn generate_message_impl(&mut self) -> Result {
        genln!(self.output; "impl {crate_name}::Message for {full_type_name} {{" => self.vars, crate_name, full_type_name);
        indent!(self.output, {
            genln!(self.output; "fn descriptor() -> &'static {crate_name}::reflect::MessageDescriptor {{" => self.vars, crate_name);
            indent!(self.output, {
                genln!(self.output, "&self::file()");
                let mut message_access = format!(".messages()[{}]", self.input.scope_index());
                let mut scope = self.input.scope();
                while let CompositeScope::Message(m) = scope {
                    message_access.insert_str(0, &format!(".messages()[{}]", m.scope_index()));
                    scope = m.scope();
                }
                gen!(self.output, "{}", message_access);
            });
            genln!(self.output, "}}");
        });
        genln!(self.output, "}}");
        Ok(())
    }

    pub fn generate_struct_impl(&mut self) -> Result {
        genln!(self.output; "impl {full_type_name} {{" => self.vars, full_type_name);
        indent!(self.output, {
            for field in self.input.message_fields() {
                let mut generator = Generator::<FieldDescriptor, _>::from_other(self, field);

                generator.generate_field_number_constant()?;
                generator.generate_default_value()?;
                generator.generate_accessors()?;
            }

            for oneof in self.input.oneofs() {
                Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_accessor()?;
            }
        });
        genln!(self.output, "}}");

        Ok(())
    }
}

generator_new!(FieldDescriptor, proto, options;
    "proto_name", proto.name().to_string(),
    "proto_type", names::get_proto_type(proto),
    "name", names::get_field_name(proto),
    "field_name", names::get_struct_field_name(proto),
    "base_type", names::get_rust_type(names::TypeResolution::Base, proto, &options.crate_name),
    "indirected_type", names::get_rust_type(names::TypeResolution::Indirection, proto, &options.crate_name),
    "field_type", names::get_rust_type(names::TypeResolution::Full, proto, &options.crate_name),
    "crate_name", options.crate_name.clone(),
    "new_value", default_field_value(proto, &options.crate_name),
    "field_number_const", names::get_field_number_const_name(proto),
    "number", proto.number().get().to_string(),
    "default", names::get_field_default_value_name(proto),
    "default_type", match proto.field_type() {
        FieldType::String => format!("&'static str"),
        FieldType::Bytes => format!("&'static [u8]"),
        _ => names::get_rust_type(names::TypeResolution::Indirection, proto, &options.crate_name),
    },
    "default_value", {
        match proto.default_value() {
            DefaultValue::Invalid | DefaultValue::None => {
                match proto.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => String::new(),
                    FieldType::Enum(e) => {
                        match e.values().iter().find(|f| f.number() == 0) {
                            Some(defined) => {
                                format!("{}::EnumValue::Defined({})", options.crate_name, names::get_full_enum_variant_name(defined, Some(proto.file()), &options.crate_name))
                            },
                            None => {
                                format!("{}::EnumValue::Undefined(0)", options.crate_name)
                            }
                        }
                    },
                    FieldType::String => format!("\"\""),
                    FieldType::Bytes => format!("&[]"),
                    FieldType::Bool => format!("false"),
                    FieldType::Float | FieldType::Double => format!("0.0"),
                    _ => format!("0")
                }
            },
            DefaultValue::String(s) => s.chars().flat_map(char::escape_default).collect(),
            DefaultValue::Bool(b) => b.to_string(),
            DefaultValue::Double(d) => d.to_string(),
            DefaultValue::SignedInt(s) => s.to_string(),
            DefaultValue::UnsignedInt(u) => u.to_string(),
            DefaultValue::Enum(e) => format!("{}::EnumValue::Defined({})", options.crate_name, names::get_full_enum_variant_name(e, Some(proto.file()), &options.crate_name)),
            DefaultValue::Bytes(b) => format!("&{:?}", b)
        }
    },
    "codec", names::get_field_codec_name(proto),
    "oneof", {
        match proto.scope() {
            FieldScope::Oneof(o) => names::get_oneof_name(o),
            _ => String::new()
        }
    },
    "tag_size", protrust::io::sizes::uint32(io::Tag::new(proto.number(), proto.wire_type()).get()).to_string(),
    "tag", io::Tag::new(proto.number(), proto.wire_type()).get().to_string(),
    "tags", {
        if proto.packed() {
            format!("{} | {}", io::Tag::new(proto.number(), proto.wire_type()).get(), io::Tag::new(proto.number(), proto.field_type().wire_type()).get())
        } else {
            io::Tag::new(proto.number(), proto.wire_type()).get().to_string()
        }
    },
    "end_tag", {
        if let FieldType::Group(_) = proto.field_type() {
            io::Tag::new(proto.number(), WireType::EndGroup).get().to_string()
        } else {
            String::new()
        }
    },
    "tag_bytes", {
        let tag = io::Tag::new(proto.number(), proto.wire_type()).get().to_le();

        let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
        let mut output = protrust::io::CodedOutput::new(&mut bytes);
        output.write_raw_tag(tag).expect("Couldn't write tag to vector");

        format!("{:?}", bytes)
    },
    "end_tag_bytes", {
        if let FieldType::Group(_) = proto.field_type() {
            let tag = io::Tag::new(proto.number(), WireType::EndGroup).get().to_le();

            let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
            let mut output = protrust::io::CodedOutput::new(&mut bytes);
            output.write_raw_tag(tag).expect("Couldn't write tag to vector");

            format!("{:?}", bytes)
        } else {
            String::new()
        }
    });

impl<W: Write> Generator<'_, FieldDescriptor, Printer<W>> {
    pub fn generate_rustdoc_comments(&mut self) -> Result {
        if let Some(source_info) = self.input.source_code_info() {
            generate_rustdoc_comments(self.output, source_info)?
        }

        Ok(())
    }

    pub fn generate_struct_field(&mut self) -> Result {
        genln!(self.output; "{field_name}: {field_type}," => self.vars, field_name, field_type);

        Ok(())
    }

    pub fn generate_oneof_field(&mut self) -> Result {
        self.generate_rustdoc_comments()?;
        genln!(self.output; "{name}({indirected_type})," => self.vars, name, indirected_type);
        Ok(())
    }

    pub fn generate_new(&mut self) -> Result {
        genln!(self.output; "{field_name}: {new_value}," => self.vars, field_name, new_value);
        Ok(())
    }

    pub fn generate_field_merge(&mut self) -> Result {
        match self.input.scope() {
            FieldScope::Oneof(_) => {
                if is_copy_type(self.input.field_type()) {
                    genln!(self.output; "if let self::{oneof}::{name}({field_name}) = other.{field_name} {{" => self.vars, oneof, name, field_name);
                } else {
                    genln!(self.output; "if let self::{oneof}::{name}({field_name}) = &other.{field_name} {{" => self.vars, oneof, name, field_name);
                }
                indent!(self.output, {
                    match self.input.field_type() {
                        FieldType::Message(_) | FieldType::Group(_) => {
                            genln!(self.output; "if let self::{oneof}::{name}(existing) = &mut self.{field_name} {{" => self.vars, oneof, name, field_name);
                            indent!(self.output, {
                                genln!(self.output; "existing.merge({field_name});" => self.vars, field_name);
                            });
                            genln!(self.output, "}} else {{");
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = self::{oneof}::{name}({field_name}.clone());" => self.vars, field_name, name, oneof);
                            });
                            genln!(self.output, "}}");
                        }
                        FieldType::Bytes | FieldType::String => {
                            genln!(self.output; "self.{field_name} = self::{oneof}::{name}({field_name}.clone());" => self.vars, field_name, name, oneof);
                        }
                        _ => {
                            genln!(self.output; "self.{field_name} = self::{oneof}::{name}({field_name});" => self.vars, field_name, name, oneof);
                        }
                    }
                });
                genln!(self.output, "}}");
            }
            FieldScope::Message(_) => match self.input.label() {
                FieldLabel::Optional | FieldLabel::Required => match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        genln!(self.output; "if let ::std::option::Option::Some({field_name}) = &other.{field_name} {{" => self.vars, field_name);
                        indent!(self.output, {
                            genln!(self.output; "self.{field_name}.get_or_insert_with(|| ::std::boxed::Box::new({crate_name}::LiteMessage::new())).merge({field_name});" => self.vars, crate_name, field_name);
                        });
                        genln!(self.output, "}}");
                    }
                    FieldType::Bytes | FieldType::String => {
                        genln!(self.output; "self.{field_name} = other.{field_name}.clone();" => self.vars, field_name);
                    }
                    _ => {
                        genln!(self.output;"self.{field_name} = other.{field_name};" => self.vars, field_name);
                    }
                },
                FieldLabel::Repeated => {
                    genln!(self.output; "self.{field_name}.merge(&other.{field_name});" => self.vars, field_name);
                }
            },
            _ => {}
        }

        Ok(())
    }

    pub fn generate_merge_arm(&mut self) -> Result {
        genln!(self.output; "{tags} => " => self.vars, tags);

        match self.input.label() {
            FieldLabel::Repeated => {
                gen!(self.output; "self.{field_name}.add_entries(input, &{codec})?" => self.vars, field_name, codec)
            }
            _ => match self.input.scope() {
                FieldScope::Message(_) => match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        gen!(self.output; "input.read_message(&mut **self.{field_name}.get_or_insert_with(|| ::std::boxed::Box::new({crate_name}::LiteMessage::new())))?" => self.vars, field_name, crate_name)
                    }
                    _ => {
                        gen!(self.output; "self.{field_name} = " => self.vars, field_name);
                        if self.input.file().syntax() == Syntax::Proto2 {
                            gen!(self.output, "::std::option::Option::Some(");
                        }

                        gen!(self.output; "input.read_{proto_type}()?" => self.vars, proto_type);

                        if self.input.file().syntax() == Syntax::Proto2 {
                            gen!(self.output, ")");
                        }
                    }
                },
                FieldScope::Oneof(_) => match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        indent!(self.output, {
                            genln!(self.output; "if let self::{oneof}::{name}({field_name}) = &mut self.{field_name} {{" => self.vars, oneof, name, field_name);
                            indent!(self.output, {
                                genln!(self.output; "{field_name}.merge_from(input)?;" => self.vars, field_name);
                            });
                            genln!(self.output, "}} else {{");
                            indent!(self.output, {
                                genln!(self.output; "let mut {field_name} = ::std::boxed::Box::new(<{base_type} as {crate_name}::LiteMessage>::new());" => self.vars, field_name, base_type, crate_name);
                                genln!(self.output; "{field_name}.merge_from(input)?;" => self.vars, field_name);
                                genln!(self.output; "self.{field_name} = self::{oneof}::{name}({field_name})" => self.vars, field_name, oneof, name);
                            });
                            genln!(self.output, "}}");
                        });
                    }
                    _ => {
                        gen!(self.output; "self.{field_name} = self::{oneof}::{name}(input.read_{proto_type}()?)" => self.vars, field_name, oneof, name, proto_type)
                    }
                },
                _ => unreachable!(),
            },
        }

        gen!(self.output, ",");

        Ok(())
    }

    pub fn generate_size_calculator(&mut self) -> Result {
        if self.input.label() == FieldLabel::Repeated {
            if self.options.size_checks {
                genln!(self.output; "size = size.checked_add(self.{field_name}.calculate_size(&{codec})?)?;" => self.vars, field_name, codec);
            } else {
                genln!(self.output; "size += self.{field_name}.calculate_size(&{codec});" => self.vars, field_name, codec);
            }
        } else {
            if let FieldScope::Oneof(_) = self.input.scope() {
                if is_copy_type(self.input.field_type()) {
                    genln!(self.output; "if let self::{oneof}::{name}({field_name}) = self.{field_name} {{" => self.vars, oneof, name, field_name);
                } else {
                    genln!(self.output; "if let self::{oneof}::{name}({field_name}) = &self.{field_name} {{" => self.vars, oneof, name, field_name);
                }
                self.output.indent();
            } else {
                if is_copy_type(self.input.field_type()) {
                    genln!(self.output; "let {field_name} = self.{field_name};" => self.vars, field_name);
                } else {
                    genln!(self.output; "let {field_name} = &self.{field_name};" => self.vars, field_name);
                }

                match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        genln!(self.output; "if let ::std::option::Option::Some({field_name}) = {field_name} {{" => self.vars, field_name);
                        self.output.indent();
                    }
                    _ => {
                        if self.input.file().syntax() == Syntax::Proto2 {
                            genln!(self.output; "if let ::std::option::Option::Some({field_name}) = {field_name} {{" => self.vars, field_name);
                            self.output.indent();
                        }
                        match self.input.field_type() {
                            FieldType::Bytes => {
                                genln!(self.output; "if {field_name}.as_slice() != Self::{default} {{" => self.vars, field_name, default);
                            }
                            _ => {
                                genln!(self.output; "if {field_name} != Self::{default} {{" => self.vars, field_name, default);
                            }
                        }
                        self.output.indent();
                    }
                }
            }

            if self.options.size_checks {
                genln!(self.output; "size = size.checked_add({tag_size})?;" => self.vars, tag_size);

                match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        genln!(self.output; "size = size.checked_add({crate_name}::io::sizes::{proto_type}(&**{field_name}));" => self.vars, field_name, crate_name, proto_type);
                    }
                    _ => {
                        genln!(self.output; "size = size.checked_add({crate_name}::io::sizes::{proto_type}({field_name}));" => self.vars, field_name, crate_name, proto_type);
                    }
                }
            } else {
                genln!(self.output; "size += {tag_size};" => self.vars, tag_size);

                match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        genln!(self.output; "size += {crate_name}::io::sizes::{proto_type}(&**{field_name});" => self.vars, field_name, crate_name, proto_type);
                    }
                    _ => {
                        genln!(self.output; "size += {crate_name}::io::sizes::{proto_type}({field_name});" => self.vars, field_name, crate_name, proto_type);
                    }
                }
            }

            if let FieldScope::Oneof(_) = self.input.scope() {
                self.output.unindent();
                gen!(self.output, "\n}}");
            } else {
                match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        self.output.unindent();
                        gen!(self.output, "\n}}");
                    }
                    _ => {
                        if self.input.file().syntax() == Syntax::Proto2 {
                            self.output.unindent();
                            gen!(self.output, "\n}}");
                        }
                        self.output.unindent();
                        gen!(self.output, "\n}}");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate_writer(&mut self) -> Result {
        if self.input.label() == FieldLabel::Repeated {
            genln!(self.output; "self.{field_name}.write_to(output, &{codec})?;" => self.vars, field_name, codec);
        } else {
            if let FieldScope::Oneof(_) = self.input.scope() {
                if is_copy_type(self.input.field_type()) {
                    genln!(self.output; "if let self::{oneof}::{name}({field_name}) = self.{field_name} {{" => self.vars, oneof, name, field_name);
                } else {
                    genln!(self.output; "if let self::{oneof}::{name}({field_name}) = &self.{field_name} {{" => self.vars, oneof, name, field_name);
                }
                self.output.indent();
            } else {
                if is_copy_type(self.input.field_type()) {
                    genln!(self.output; "let {field_name} = self.{field_name};" => self.vars, field_name);
                } else {
                    genln!(self.output; "let {field_name} = &self.{field_name};" => self.vars, field_name);
                }

                match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        genln!(self.output; "if let ::std::option::Option::Some({field_name}) = {field_name} {{" => self.vars, field_name);
                        self.output.indent();
                    }
                    _ => {
                        if self.input.file().syntax() == Syntax::Proto2 {
                            genln!(self.output; "if let ::std::option::Option::Some({field_name}) = {field_name} {{" => self.vars, field_name);
                            self.output.indent();
                        }
                        match self.input.field_type() {
                            FieldType::Bytes => {
                                genln!(self.output; "if {field_name}.as_slice() != Self::{default} {{" => self.vars, field_name, default);
                            }
                            _ => {
                                genln!(self.output; "if {field_name} != Self::{default} {{" => self.vars, field_name, default);
                            }
                        }
                        self.output.indent();
                    }
                }
            }

            genln!(self.output; "output.write_raw_tag_bytes(&{tag_bytes})?;" => self.vars, tag_bytes);
            match self.input.field_type() {
                FieldType::Message(_) | FieldType::Group(_) => {
                    genln!(self.output; "output.write_{proto_type}(&**{field_name})?;" => self.vars, proto_type, field_name);
                }
                _ => {
                    genln!(self.output; "output.write_{proto_type}({field_name})?;" => self.vars, proto_type, field_name);
                }
            }

            if let FieldType::Group(_) = self.input.field_type() {
                genln!(self.output; "output.write_raw_tag_bytes(&{end_tag_bytes})?;" => self.vars, end_tag_bytes);
            }

            if let FieldScope::Oneof(_) = self.input.scope() {
                self.output.unindent();
                genln!(self.output, "}}");
            } else {
                match self.input.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        self.output.unindent();
                        genln!(self.output, "}}");
                    }
                    _ => {
                        if self.input.file().syntax() == Syntax::Proto2 {
                            self.output.unindent();
                            genln!(self.output, "}}");
                        }
                        self.output.unindent();
                        genln!(self.output, "}}");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate_field_number_constant(&mut self) -> Result {
        genln!(self.output; "/// Gets the field number of the [`{proto_name}`] field" => self.vars, proto_name);
        genln!(self.output, "///");
        genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
        genln!(self.output; "pub const {field_number_const}: i32 = {number};" => self.vars, field_number_const, number);
        Ok(())
    }

    pub fn generate_default_value(&mut self) -> Result {
        if self.input.label() != FieldLabel::Repeated {
            match self.input.field_type() {
                FieldType::Message(_) | FieldType::Group(_) => {}
                _ => {
                    genln!(self.output; "/// A constant value representing the default value of the [`{proto_name}`] field" => self.vars, proto_name);
                    genln!(self.output, "///");
                    genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                    genln!(self.output; "pub const {default}: {default_type} = {default_value};" => self.vars, default, default_type, default_value);
                }
            }
        }

        Ok(())
    }

    pub fn generate_codec(&mut self) -> Result {
        if self.input.label() == FieldLabel::Repeated {
            match self.input.field_type() {
                FieldType::Message(m) if m.map_entry() => {
                    genln!(self.output; "static {codec}: {crate_name}::collections::MapCodec<" => self.vars, codec, crate_name);
                    let generator =
                        Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[0]);
                    gen!(generator.output; "{base_type}, " => generator.vars, base_type);
                    let generator =
                        Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[1]);
                    gen!(generator.output; "{base_type}" => generator.vars, base_type);
                    gen!(self.output; "> = {crate_name}::collections::MapCodec::new(" => self.vars, crate_name);

                    Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[0])
                        .generate_codec_new()?;
                    gen!(self.output, ", ");
                    Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[1])
                        .generate_codec_new()?;
                    gen!(self.output; ", {tag});" => self.vars, tag);
                }
                _ => {
                    genln!(self.output; "static {codec}: {crate_name}::Codec<{base_type}> = " => self.vars, codec, crate_name, base_type);
                    self.generate_codec_new()?;
                    gen!(self.output, ";");
                }
            }
        }

        Ok(())
    }

    pub fn generate_is_initialized(&mut self) -> Result {
        match self.input.label() {
            FieldLabel::Repeated => {
                genln!(self.output; "if !self.{field_name}.is_initialized() {{" => self.vars, field_name);
                indent!(self.output, {
                    genln!(self.output, "return false;");
                });
                genln!(self.output, "}}");
            }
            _ => {
                if self.input.field_type().is_message() || self.input.field_type().is_group() {
                    genln!(self.output; "if let Some({field_name}) = &self.{field_name} {{" => self.vars, field_name);
                    indent!(self.output, {
                        genln!(self.output; "if !{crate_name}::CodedMessage::is_initialized(&**{field_name}) {{" => self.vars, crate_name, field_name);
                        indent!(self.output, {
                            genln!(self.output, "return false;");
                        });
                        genln!(self.output, "}}");
                    });
                    genln!(self.output, "}}");
                } else {
                    genln!(self.output; "if self.{field_name}.is_none() {{" => self.vars, field_name);
                    indent!(self.output, {
                        genln!(self.output, "return false;");
                    });
                    genln!(self.output, "}}");
                }
            }
        }

        Ok(())
    }

    pub fn generate_codec_new(&mut self) -> Result {
        match self.input.field_type() {
            FieldType::Group(_) => {
                gen!(self.output; "{crate_name}::Codec::group({tag}, {end_tag})" => self.vars, crate_name, tag, end_tag)
            }
            _ => {
                gen!(self.output; "{crate_name}::Codec::{proto_type}({tag})" => self.vars, crate_name, proto_type, tag)
            }
        }
        Ok(())
    }

    pub fn generate_accessors(&mut self) -> Result {
        match self.input.label() {
            FieldLabel::Repeated => {
                self.generate_rustdoc_comments()?;
                genln!(self.output; "pub fn {field_name}(&self) -> &{field_type} {{" => self.vars, field_name, field_type);
                indent!(self.output, {
                    genln!(self.output; "&self.{field_name}" => self.vars, field_name);
                });
                genln!(self.output, "}}");
                genln!(self.output; "/// Returns a unique reference to the [`{proto_name}`] field" => self.vars, proto_name);
                genln!(self.output, "///");
                genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                genln!(self.output; "pub fn {name}_mut(&mut self) -> &mut {field_type} {{" => self.vars, name, field_type);
                indent!(self.output, {
                    genln!(self.output; "&mut self.{field_name}" => self.vars, field_name);
                });
                genln!(self.output, "}}");
            }
            _ => {
                match self.input.file().syntax() {
                    Syntax::Proto2 => match self.input.field_type() {
                        FieldType::Message(_) | FieldType::Group(_) => {
                            self.generate_rustdoc_comments()?;
                            genln!(self.output; "pub fn {name}_option(&self) -> ::std::option::Option<&{base_type}> {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.as_ref().map(|b| &**b)" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns a unique reference to the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn {name}_mut(&mut self) -> &mut {base_type} {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.get_or_insert_with(|| ::std::boxed::Box::new({crate_name}::LiteMessage::new())).as_mut()" => self.vars, crate_name, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns a bool indicating the presence of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn has_{name}(&self) -> bool {{" => self.vars, name);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.is_some()" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Sets the value of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn set_{name}(&mut self, value: {base_type}) {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = ::std::option::Option::Some(::std::boxed::Box::new(value))" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Takes the value of the [`{proto_name}`] field, leaving it empty" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn take_{name}(&mut self) -> ::std::option::Option<{base_type}> {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.take().map(|b| *b)" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Clears the value of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn clear_{name}(&mut self) {{" => self.vars, name);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = ::std::option::Option::None" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                        }
                        FieldType::String | FieldType::Bytes => {
                            self.generate_rustdoc_comments()?;
                            if *self.input.field_type() == FieldType::String {
                                genln!(self.output; "pub fn {field_name}(&self) -> &str {{" => self.vars, field_name);
                            } else {
                                genln!(self.output; "pub fn {field_name}(&self) -> &[u8] {{" => self.vars, field_name);
                            }
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.as_ref().map(|v| &**v).unwrap_or(Self::{default})" => self.vars, default, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns an [`Option`] representing the presence of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output, "/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html");
                            genln!(self.output; "pub fn {name}_option(&self) -> ::std::option::Option<&{base_type}> {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.as_ref()" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns a unique reference to the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn {name}_mut(&mut self) -> &mut {base_type} {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                if *self.input.field_type() == FieldType::String {
                                    genln!(self.output; "self.{field_name}.get_or_insert_with(::std::string::String::new)" => self.vars, field_name);
                                } else {
                                    genln!(self.output; "self.{field_name}.get_or_insert_with(::std::vec::Vec::new)" => self.vars, field_name);
                                }
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns a bool indicating the presence of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn has_{name}(&self) -> bool {{" => self.vars, name);
                            indent!(self.output, {
                                genln!(self.output; "self.{name}.is_some()" => self.vars, name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Sets the value of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn set_{name}(&mut self, value: {indirected_type}) {{" => self.vars, name, indirected_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = ::std::option::Option::Some(value)" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Takes the value of the [`{proto_name}`] field, leaving it empty" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn take_{name}(&mut self) -> {field_type} {{" => self.vars, name, field_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.take()" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Clears the value of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn clear_{name}(&mut self) {{" => self.vars, name);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = ::std::option::Option::None" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                        }
                        _ => {
                            self.generate_rustdoc_comments()?;
                            genln!(self.output; "pub fn {field_name}(&self) -> {base_type} {{" => self.vars, field_name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.unwrap_or(Self::{default})" => self.vars, default, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns an [`Option`] representing the presence of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output, "/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html");
                            genln!(self.output; "pub fn {name}_option(&self) -> ::std::option::Option<{base_type}> {{" => self.vars, name, base_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Returns a bool indicating the presence of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn has_{name}(&self) -> bool {{" => self.vars, name);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}.is_some()" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Sets the value of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn set_{name}(&mut self, value: {indirected_type}) {{" => self.vars, name, indirected_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = ::std::option::Option::Some(value)" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                            genln!(self.output; "/// Clears the value of the [`{proto_name}`] field" => self.vars, proto_name);
                            genln!(self.output, "///");
                            genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                            genln!(self.output; "pub fn clear_{name}(&mut self) {{" => self.vars, name);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name} = ::std::option::Option::None" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                        }
                    },
                    Syntax::Proto3 => {
                        self.generate_rustdoc_comments()?;
                        if is_copy_type(self.input.field_type()) {
                            genln!(self.output; "pub fn {field_name}(&self) -> {field_type} {{" => self.vars, field_name, field_type);
                            indent!(self.output, {
                                genln!(self.output; "self.{field_name}" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                        } else {
                            genln!(self.output; "pub fn {field_name}(&self) -> &{field_type} {{" => self.vars, field_name, field_type);
                            indent!(self.output, {
                                genln!(self.output; "&self.{field_name}" => self.vars, field_name);
                            });
                            genln!(self.output, "}}");
                        }
                        genln!(self.output; "/// Returns a unique reference to the [`{proto_name}`] field" => self.vars, proto_name);
                        genln!(self.output, "///");
                        genln!(self.output; "/// [`{proto_name}`]: #method.{name}" => self.vars, proto_name, name);
                        genln!(self.output; "pub fn {name}_mut(&mut self) -> &mut {field_type} {{" => self.vars, name, field_type);
                        indent!(self.output, {
                            genln!(self.output; "&mut self.{field_name}" => self.vars, field_name);
                        });
                        genln!(self.output, "}}");
                    }
                    _ => panic!("Unknown syntax"),
                }
            }
        }

        Ok(())
    }
}

fn default_field_value(field: &FieldDescriptor, crate_name: &str) -> String {
    match field.label() {
        FieldLabel::Optional | FieldLabel::Required => {
            if field.file().syntax() == Syntax::Proto2 {
                format!("::std::option::Option::None")
            } else {
                match field.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        format!("::std::option::Option::None")
                    }
                    FieldType::String => format!("::std::string::String::new()"),
                    FieldType::Bytes => format!("::std::vec::Vec::new()"),
                    _ => "Self::".to_string() + &names::get_field_default_value_name(field),
                }
            }
        }
        FieldLabel::Repeated => {
            if let FieldType::Message(m) = field.field_type() {
                if m.map_entry() {
                    return format!("{}::collections::MapField::new()", crate_name);
                }
            }

            format!("{}::collections::RepeatedField::new()", crate_name)
        }
    }
}

fn is_copy_type(ft: &FieldType) -> bool {
    match ft {
        FieldType::Message(_) | FieldType::Group(_) | FieldType::Bytes | FieldType::String => false,
        _ => true,
    }
}

generator_new!(EnumDescriptor, proto, options;
    "type_name", names::get_enum_type_name(proto),
    "crate_name", options.crate_name.clone(),
    "full_type_name", names::get_full_enum_type_name(proto, Some(proto.file()), &options.crate_name));

impl<W: Write> Generator<'_, EnumDescriptor, Printer<W>> {
    pub fn generate_rustdoc_comments(&mut self) -> Result {
        if let Some(source_info) = self.input.source_code_info() {
            generate_rustdoc_comments(self.output, source_info)?
        }

        Ok(())
    }

    pub fn generate(&mut self) -> Result {
        let mut values: HashMap<String, &EnumValueDescriptor> = HashMap::new();
        for value in self.input.values() {
            let mut name = names::get_enum_variant_name(value);
            while values.contains_key(&name) {
                eprintln!("{} already exists, adding '_' to differentiate", name);
                name += "_";
            }
            values.insert(name, value);
        }

        let mut values = values.iter().collect::<Vec<_>>();
        values.sort_unstable_by_key(|(_, v)| v.index());

        self.generate_rustdoc_comments()?;
        genln!(
            self.output,
            "#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]"
        );
        genln!(self.output; "pub enum {type_name} {{" => self.vars, type_name);
        indent!(self.output, {
            for (name, value) in values.iter() {
                if let Some(source_info) = value.source_code_info() {
                    generate_rustdoc_comments(self.output, source_info)?;
                }
                genln!(self.output, "{variant},", variant = name);
            }
        });
        genln!(self.output, "}}");
        genln!(self.output; "unsafe impl {crate_name}::Enum for {full_type_name} {{ }}" => self.vars, crate_name, full_type_name);
        genln!(self.output; "impl ::std::convert::TryFrom<i32> for {full_type_name} {{" => self.vars, full_type_name);
        indent!(self.output, {
            genln!(self.output; "type Error = {crate_name}::VariantUndefinedError;" => self.vars, crate_name);
            genln!(self.output; "fn try_from(value: i32) -> ::std::result::Result<Self, {crate_name}::VariantUndefinedError> {{" => self.vars, crate_name);
            indent!(self.output, {
                genln!(self.output, "match value {{");
                indent!(self.output, {
                    for (name, value) in values.iter() {
                        genln!(
                            self.output,
                            "{} => ::std::result::Result::Ok(self::{}::{}),",
                            value.number(),
                            names::get_enum_type_name(value.enum_type()),
                            name
                        );
                    }
                    genln!(self.output; "_ => ::std::result::Result::Err({crate_name}::VariantUndefinedError)" => self.vars, crate_name);
                });
                genln!(self.output, "}}");
            });
            genln!(self.output, "}}");
        });
        genln!(self.output, "}}");
        genln!(self.output; "impl ::std::convert::From<{full_type_name}> for i32 {{" => self.vars, full_type_name);
        indent!(self.output, {
            genln!(self.output; "fn from(value: {full_type_name}) -> i32 {{" => self.vars, full_type_name);
            indent!(self.output, {
                genln!(self.output, "match value {{");
                indent!(self.output, {
                    for (name, value) in values.iter() {
                        genln!(
                            self.output,
                            "{}::{} => {},",
                            names::get_enum_type_name(value.enum_type()),
                            name,
                            value.number()
                        );
                    }
                });
                genln!(self.output, "}}");
            });
            genln!(self.output, "}}");
        });
        genln!(self.output, "}}");

        Ok(())
    }
}

generator_new!(OneofDescriptor, proto, options;
    "name", proto.name().to_string(),
    "field_name", proto.name().to_string(),
    "type_name", names::get_oneof_name(proto));

impl<W: Write> Generator<'_, OneofDescriptor, Printer<W>> {
    pub fn generate_rustdoc_comments(&mut self) -> Result {
        if let Some(source_info) = self.input.source_code_info() {
            generate_rustdoc_comments(self.output, source_info)?
        }

        Ok(())
    }

    pub fn generate_type(&mut self) -> Result {
        self.generate_rustdoc_comments()?;
        genln!(self.output, "#[derive(Clone, Debug, PartialEq)]");
        genln!(self.output; "pub enum {type_name} {{" => self.vars, type_name);
        indent!(self.output, {
            genln!(self.output, "/// No value");
            genln!(self.output, "None,");

            for field in self.input.fields() {
                Generator::<FieldDescriptor, _>::from_other(self, field).generate_oneof_field()?;
            }
        });
        genln!(self.output, "}}");
        Ok(())
    }

    pub fn generate_struct_field(&mut self) -> Result {
        genln!(self.output; "{field_name}: self::{type_name}," => self.vars, field_name, type_name);
        Ok(())
    }

    pub fn generate_new(&mut self) -> Result {
        genln!(self.output; "{field_name}: self::{type_name}::None," => self.vars, field_name, type_name);
        Ok(())
    }

    pub fn generate_accessor(&mut self) -> Result {
        genln!(self.output; "/// Gets a shared reference to the [`{name}`] oneof field" => self.vars, name);
        genln!(self.output, "///");
        genln!(self.output; "/// [`{name}`]: enum.{type_name}.html" => self.vars, type_name, name);
        genln!(self.output; "pub fn {field_name}(&self) -> &self::{type_name} {{" => self.vars, field_name, type_name);
        indent!(self.output, {
            genln!(self.output; "&self.{field_name}" => self.vars, field_name);
        });
        genln!(self.output, "}}");

        genln!(self.output; "/// Gets a unique reference to the [`{name}`] oneof field" => self.vars, name);
        genln!(self.output, "///");
        genln!(self.output; "/// [`{name}`]: enum.{type_name}.html" => self.vars, type_name, name);
        genln!(self.output; "pub fn {field_name}_mut(&mut self) -> &mut self::{type_name} {{" => self.vars, field_name, type_name);
        indent!(self.output, {
            genln!(self.output; "&mut self.{field_name}" => self.vars, field_name);
        });
        genln!(self.output, "}}");

        Ok(())
    }
}

fn generate_rustdoc_comments<W: Write>(
    printer: &mut Printer<W>,
    source_info: &SourceCodeInfo,
) -> Result {
    if let Some(comments) = source_info
        .leading_comments()
        .or(source_info.trailing_comments())
    {
        let mut printer = printer::DocPrinter::new(printer);
        let mut events = Parser::new(comments).peekable();
        while let Some(event) = events.next() {
            let peek = events.peek();
            match event {
                Event::End(Tag::Paragraph) if peek == None => {}
                Event::Start(Tag::Paragraph) | Event::End(Tag::Paragraph) => genln!(printer),
                Event::Start(Tag::Code) | Event::End(Tag::Code) => gen!(printer, "`"),
                Event::Text(val)
                | Event::FootnoteReference(val)
                | Event::Html(val)
                | Event::InlineHtml(val) => gen!(printer, "{}", val),
                Event::SoftBreak | Event::HardBreak => genln!(printer),
                Event::Start(Tag::CodeBlock(std::borrow::Cow::Borrowed(""))) => {
                    gen!(printer, "```text\n")
                }
                Event::Start(Tag::CodeBlock(code)) => gen!(printer, "```{}\n", code),
                Event::End(Tag::CodeBlock(_)) => gen!(printer, "```\n"),
                Event::Start(Tag::Header(i)) => gen!(printer, "{} ", "#".repeat(i as usize)),
                Event::Start(Tag::List(start)) => {
                    printer.start_list(start);
                    genln!(printer);
                }
                Event::End(Tag::List(_)) => {
                    printer.end_list();
                }
                Event::Start(Tag::Item) => {
                    printer.start_item();
                    match printer.current_item_number() {
                        Some(number) => gen!(printer, "{}. ", number),
                        None => gen!(printer, "* "),
                    }
                }
                Event::End(Tag::Item) => {
                    printer.end_item();
                    genln!(printer);
                }
                Event::Start(Tag::Link(_, ref title)) if title.is_empty() => gen!(printer, "["),
                Event::Start(Tag::Link(_, _)) => gen!(printer, "["),
                Event::End(Tag::Link(ref link, ref title)) if title.is_empty() => {
                    gen!(printer, "]({})", link)
                }
                Event::End(Tag::Link(_, ref title)) => gen!(printer, "][{}]", title),
                Event::End(Tag::Header(_)) => genln!(printer),
                Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => gen!(printer, "*"),
                Event::Start(Tag::Strong) | Event::End(Tag::Strong) => gen!(printer, "**"),
                u => panic!("Unknown event / tag: {:?}", u),
            }
        }
    }

    Ok(())
}
