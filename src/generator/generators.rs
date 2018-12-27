use std::fmt::Write;
use std::collections::HashMap;
use protrust::io::WireType;
use protrust::prelude::*;
use protrust::reflect::*;
use protrust::descriptor::FileOptions_OptimizeMode as OptimizeMode;
use super::printer;
use super::names;

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
}

impl Default for Options {
    fn default() -> Self {
        Options {
            crate_name: "protrust".to_string(),
            no_json: false,
            pub_fields: false,
        }
    }
}

macro_rules! var {
    ($target:expr, $var:expr) => {
        $target.get(stringify!($var)).ok_or_else(|| Error::MissingVariable(stringify!($var)))?
    };
}

macro_rules! gen {
    ($target:expr; $vars:expr => $fmt:expr, $($arg:ident),*) 
        => (write!($target, $fmt, $($arg = var!($vars, $arg)),*)?);
    ($dst:expr, $($arg:tt)*) => (write!($dst, $($arg)*)?);
}

macro_rules! generator_new {
    ($type:ty, $p:ident, $o:ident; $($key:expr, $value:expr),*) => {
        impl<'a, W> Generator<'a, $type, W> {
            pub fn new(printer: &'a mut printer::Printer<W>, $p: &'a $type, $o: &'a Options) -> Generator<'a, $type, W> {
                let mut generator = Generator {
                    printer,
                    proto: $p,
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
                Self::new(&mut other.printer, proto, &other.options)
            }
        }
    };
}

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    FormatError,
    MissingVariable(&'static str)
}

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Error {
        Error::FormatError
    }
}

pub struct Generator<'a, T, W> {
    vars: HashMap<&'static str, String>,
    printer: &'a mut printer::Printer<W>,
    proto: &'a T,
    options: &'a Options
}

generator_new!(FileDescriptor, proto, options; 
    "file", proto.name().to_string());

impl<W: Write> Generator<'_, FileDescriptor, W> {
    pub fn generate(&mut self) -> Result {
        gen!(self.printer; self.vars => 
"//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: {file}

", file);

        // static descriptor code

        // extensions
        //for _extension in self.proto.extensions() {
        //
        //}

        // messages
        for message in self.proto.messages() {
            Generator::<MessageDescriptor, _>::from_other(self, message).generate()?;
        }

        // enums
        for enum_type in self.proto.enums() {
            Generator::<EnumDescriptor, _>::from_other(self, enum_type).generate()?;
        }

        Ok(())
    }
}

generator_new!(MessageDescriptor, proto, options;
    "type_name", names::get_message_type_name(proto),
    "full_type_name", names::get_full_message_type_name(proto, proto.file()),
    "crate_name", options.crate_name.clone());

impl<W: Write> Generator<'_, MessageDescriptor, W> {
    pub fn generate(&mut self) -> Result {
        gen!(self.printer; self.vars => 
"
#[derive(Clone, PartialEq)]
pub struct {type_name} {{", type_name);
        self.printer.indent();

        for field in self.proto.fields().iter().filter(|f| message_scope(f.scope())) {
            writeln!(self.printer)?;
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_struct_field()?;
        }

        for oneof in self.proto.oneofs() {
            writeln!(self.printer)?;
            Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_struct_field()?;
        }

        gen!(self.printer; self.vars => "\n_unknown_fields: {crate_name}::UnknownFieldSet", crate_name);

        self.printer.unindent();
        gen!(self.printer, "\n}}");

        for field in self.proto.fields() {
            let mut generator = Generator::<FieldDescriptor, _>::from_other(self, field);
            generator.generate_default_value()?;
            generator.generate_codec()?;
        }

        for oneof in self.proto.oneofs() {
            Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_type()?;
        }

        self.generate_coded_message_impl()?;
        self.generate_lite_message_impl()?;

        if let Some(EnumValue::Defined(o)) = self.proto.file().options().map(|o| o.optimize_for.unwrap_or(EnumValue::Defined(OptimizeMode::Speed))) {
            if o != OptimizeMode::LiteRuntime {
                self.generate_message_impl()?;
            }
        }

        self.generate_struct_impl()?;

        for nested in self.proto.messages().iter().filter(|m| !m.map_entry()) {
            Generator::<MessageDescriptor, _>::from_other(self, nested).generate()?;
        }

        for nested in self.proto.enums() {
            Generator::<EnumDescriptor, _>::from_other(self, nested).generate()?;
        }

        Ok(())
    }

    pub fn generate_coded_message_impl(&mut self) -> Result {
        gen!(self.printer; self.vars => "\nimpl {crate_name}::CodedMessage for {full_type_name} {{", crate_name, full_type_name);
        self.printer.indent();

        gen!(self.printer; self.vars => "\nfn merge_from(&mut self, input: &mut {crate_name}::io::CodedInput) -> {crate_name}::io::InputResult<()> {{", crate_name);
        self.printer.indent();

        gen!(self.printer, "\nwhile let std::option::Option::Some(tag) = input.read_tag()? {{");
        self.printer.indent();
        gen!(self.printer, "\nmatch tag.get() {{");
        self.printer.indent();

        for field in self.proto.fields() {
            writeln!(self.printer)?;
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_merge_arm()?;
        }

        gen!(self.printer, "\n_ => {{ }}");// todo, read unknown fields
        self.printer.unindent();
        gen!(self.printer, "\n}}");
        self.printer.unindent();
        gen!(self.printer, "\n}}");
        gen!(self.printer, "\nstd::result::Result::Ok(())");
        self.printer.unindent();
        gen!(self.printer, "\n}}");

        gen!(self.printer, "\nfn calculate_size(&self) -> std::option::Option<i32> {{");
        self.printer.indent();
        gen!(self.printer, "\nlet mut size = 0i32;");

        for field in self.proto.fields() {
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_size_calculator()?;
        }

        gen!(self.printer, "\nstd::option::Option::Some(size)");
        self.printer.unindent();
        gen!(self.printer, "\n}}");

        gen!(self.printer; self.vars => "\nfn write_to(&self, output: &mut {crate_name}::io::CodedOutput) -> {crate_name}::io::OutputResult {{", crate_name);
        self.printer.indent();

        for field in self.proto.fields() {
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_writer()?;
        }

        gen!(self.printer, "\nstd::result::Result::Ok(())");

        self.printer.unindent();
        gen!(self.printer, "\n}}");

        self.printer.unindent();
        gen!(self.printer, "\n}}");

        Ok(())
    }

    pub fn generate_lite_message_impl(&mut self) -> Result {
        gen!(self.printer; self.vars => "\nimpl {crate_name}::LiteMessage for {full_type_name} {{", crate_name, full_type_name);
        self.printer.indent();

        write!(self.printer, "\nfn new() -> Self {{")?;
        self.printer.indent();
        write!(self.printer, "\nSelf {{")?;
        self.printer.indent();

        for field in self.proto.fields().iter().filter(|p| message_scope(p.scope())) {
            writeln!(self.printer)?;
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_new()?;
        }

        for oneof in self.proto.oneofs() {
            writeln!(self.printer)?;
            Generator::<OneofDescriptor, _>::from_other(self, oneof).generate_new()?;
        }

        write!(self.printer, "\n_unknown_fields: {}::UnknownFieldSet::new()", self.options.crate_name)?;

        self.printer.unindent();
        write!(self.printer, "\n}}")?;
        self.printer.unindent();
        write!(self.printer, "\n}}")?;
        write!(self.printer, "\nfn merge(&mut self, other: &Self) {{")?;
        self.printer.indent();

        for field in self.proto.fields() {
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_field_merge()?;
        }

        self.printer.unindent();
        write!(self.printer, "\n}}")?;
        self.printer.unindent();
        write!(self.printer, "\n}}\n")?;

        Ok(())
    }

    pub fn generate_message_impl(&mut self) -> Result {
        gen!(self.printer; self.vars => "impl {crate_name}::Message for {full_type_name} {{", crate_name, full_type_name);
        self.printer.indent();
        gen!(self.printer; self.vars => "\nfn descriptor() -> &'static {crate_name}::reflect::MessageDescriptor {{", crate_name);
        self.printer.indent();

        gen!(self.printer, "\nunimplemented!()");

        self.printer.unindent();
        write!(self.printer, "\n}}")?;
        self.printer.unindent();
        write!(self.printer, "\n}}")?;
        Ok(())
    }

    pub fn generate_struct_impl(&mut self) -> Result {
        gen!(self.printer; self.vars => "\nimpl {full_type_name} {{", full_type_name);
        self.printer.indent();

        for field in self.proto.fields() {
            let mut generator = Generator::<FieldDescriptor, _>::from_other(self, field);

            generator.generate_field_number_constant()?;
            generator.generate_accessors()?;
        }

        self.printer.unindent();
        gen!(self.printer, "\n}}");

        Ok(())
    }
}

fn message_scope(s: &FieldScope) -> bool {
    match s {
        FieldScope::Message(_) => true,
        _ => false
    }
}

generator_new!(FieldDescriptor, proto, options;
    "proto_name", proto.name().clone(),
    "proto_type", names::get_proto_type(proto),
    "name", names::get_field_name(proto),
    "field_name", names::get_struct_field_name(proto),
    "base_type", names::get_rust_type(names::TypeResolution::Base, proto, &options.crate_name),
    "indirected_type", names::get_rust_type(names::TypeResolution::Indirection, proto, &options.crate_name),
    "field_type", names::get_rust_type(names::TypeResolution::Full, proto, &options.crate_name),
    "crate_name", options.crate_name.clone(),
    "new_value", default_field_value(proto, &options.crate_name),
    "field_number_const", names::get_field_number_const_name(proto),
    "number", proto.number().to_string(),
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
                                format!("{}::EnumValue::Defined({})", options.crate_name, names::get_full_enum_variant_name(defined, proto.file()))
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
            DefaultValue::Enum(e) => format!("{}::EnumValue::Defined({})", options.crate_name, names::get_full_enum_variant_name(e, proto.file())),
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
    "tag_size", {
        let wt = proto.wire_type();
        let tag = WireType::make_tag(proto.number(), wt);

        protrust::io::sizes::uint32(tag).to_string()
    },
    "tag", {
        if proto.packed() {
            WireType::make_tag(proto.number(), WireType::LengthDelimited).to_string()
        } else {
            WireType::make_tag(proto.number(), proto.wire_type()).to_string()
        }
    },
    "tags", {
        if proto.packed() {
            format!("{} | {}", WireType::make_tag(proto.number(), proto.wire_type()), WireType::make_tag(proto.number(), WireType::LengthDelimited))
        } else {
            WireType::make_tag(proto.number(), proto.wire_type()).to_string()
        }
    },
    "end_tag", {
        if let FieldType::Group(_) = proto.field_type() {
            WireType::make_tag(proto.number(), WireType::EndGroup).to_string()
        } else {
            String::new()
        }
    },
    "tag_bytes", {
        let tag;
        if proto.packed() {
            tag = WireType::make_tag(proto.number(), WireType::LengthDelimited);
        } else {
            tag = WireType::make_tag(proto.number(), proto.wire_type()).to_le();
        }

        let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
        let mut output = protrust::io::CodedOutput::new(&mut bytes);
        output.write_raw_tag(tag).expect("Couldn't write tag to vector");

        format!("{:?}", bytes)
    },
    "end_tag_bytes", {
        if let FieldType::Group(_) = proto.field_type() {
            let tag = WireType::make_tag(proto.number(), WireType::EndGroup);

            let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
            let mut output = protrust::io::CodedOutput::new(&mut bytes);
            output.write_raw_tag(tag).expect("Couldn't write tag to vector");

            format!("{:?}", bytes)
        } else {
            String::new()
        }
    });

impl<W: Write> Generator<'_, FieldDescriptor, W> {
    pub fn generate_struct_field(&mut self) -> Result {
        if let FieldScope::Message(_) = self.proto.scope() {
            if self.options.pub_fields {
                write!(self.printer, "pub ")?;
            }

            gen!(self.printer; self.vars => "{field_name}: {field_type},", field_name, field_type);
        }

        Ok(())
    }

    pub fn generate_oneof_field(&mut self) -> Result {
        if let FieldScope::Oneof(_) = self.proto.scope() {
            gen!(self.printer; self.vars => "{name}({field_type}),", name, field_type);
        }

        Ok(())
    }

    pub fn generate_new(&mut self) -> Result {
        if let FieldScope::Message(_) = self.proto.scope() {
            gen!(self.printer; self.vars => "{field_name}: {new_value},", field_name, new_value);
        }

        Ok(())
    }

    pub fn generate_field_merge(&mut self) -> Result {
        match self.proto.scope() {
            FieldScope::Oneof(_) => {
                if is_copy_type(self.proto.field_type()) {
                    gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = other.{field_name} {{", oneof, name, field_name);
                } else {
                    gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = &other.{field_name} {{", oneof, name, field_name);
                }
                self.printer.indent();

                match self.proto.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}(existing) = &mut self.{field_name} {{", oneof, name, field_name);
                        self.printer.indent();

                        gen!(self.printer; self.vars => "\nexisting.merge({field_name});", field_name);

                        self.printer.unindent();
                        gen!(self.printer, "\n}} else {{");
                        self.printer.indent();

                        gen!(self.printer; self.vars => "\nself.{field_name} = self::{oneof}::{name}({field_name}.clone());", field_name, name, oneof);

                        self.printer.unindent();
                        gen!(self.printer, "\n}}");
                    },
                    FieldType::Bytes | FieldType::String => {
                        gen!(self.printer; self.vars => "\nself.{field_name} = self::{oneof}::{name}({field_name}.clone());", field_name, name, oneof);
                    },
                    _ => {
                        gen!(self.printer; self.vars => "\nself.{field_name} = self::{oneof}::{name}({field_name});", field_name, name, oneof);
                    }
                }

                if let FieldScope::Oneof(_) = self.proto.scope() {
                    self.printer.unindent();
                    gen!(self.printer, "\n}}");
                }
            },
            FieldScope::Message(_) => {
                match self.proto.label() {
                    FieldLabel::Optional | FieldLabel::Required => {
                        match self.proto.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                gen!(self.printer; self.vars => "\nif let std::option::Option::Some({field_name}) = &other.{field_name} {{", field_name);
                                self.printer.indent();
                                gen!(self.printer; self.vars => "\nself.{field_name}.get_or_insert_with({crate_name}::LiteMessage::new).merge({field_name});", crate_name, field_name);
                                self.printer.unindent();
                                gen!(self.printer, "\n}}");
                            },
                            FieldType::Bytes | FieldType::String => {
                                gen!(self.printer; self.vars => "\nself.{field_name} = other.{field_name}.clone();", field_name);
                            },
                            _ => {
                                gen!(self.printer; self.vars => "\nself.{field_name} = other.{field_name};", field_name);
                            }
                        }
                    },
                    FieldLabel::Repeated => {
                        gen!(self.printer; self.vars => "\nself.{field_name}.merge(&other.{field_name});", field_name);
                    }
                }
            },
            _ => { }
        }

        Ok(())
    }

    pub fn generate_merge_arm(&mut self) -> Result {
        gen!(self.printer; self.vars => "{tags} => ", tags);

        match self.proto.label() {
            FieldLabel::Repeated => gen!(self.printer; self.vars => "self.{field_name}.add_entries(tag.get(), input, &{codec})?", field_name, codec),
            _ => {
                match self.proto.scope() {
                    FieldScope::Message(_) => {
                        match self.proto.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => gen!(self.printer; self.vars => "input.read_message(self.{field_name}.get_or_insert_with({crate_name}::LiteMessage::new))?", field_name, crate_name),
                            _ => {
                                gen!(self.printer; self.vars => "self.{field_name} = ", field_name);
                                if self.proto.file().syntax() == Syntax::Proto2 {
                                    gen!(self.printer, "std::option::Option::Some(");
                                }

                                gen!(self.printer; self.vars => "input.read_{proto_type}()?", proto_type);

                                if self.proto.file().syntax() == Syntax::Proto2 {
                                    gen!(self.printer, ")");
                                }
                            }
                        }
                    },
                    FieldScope::Oneof(_) => {
                        match self.proto.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                self.printer.indent();
                                gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = &mut self.{field_name} {{", oneof, name, field_name);
                                self.printer.indent();
                                gen!(self.printer; self.vars => 
"
{field_name}.merge_from(input)?;", field_name);
                                self.printer.unindent();
                                gen!(self.printer, "\n}} else {{");
                                self.printer.indent();
                                gen!(self.printer; self.vars => 
"
let mut {field_name} = std::boxed::Box::new(<{base_type} as {crate_name}::LiteMessage>::new());
{field_name}.merge_from(input)?;
self.{field_name} = self::{oneof}::{name}({field_name})", base_type, field_name, crate_name, oneof, name);
                                self.printer.unindent();
                                gen!(self.printer, "\n}}");
                                self.printer.unindent();
                            }
                            _ => gen!(self.printer; self.vars => "self.{field_name} = self::{oneof}::{name}(input.read_{proto_type}()?)", field_name, oneof, name, proto_type)
                        }
                    },
                    _ => unreachable!()
                }
            }
        }

        gen!(self.printer, ",");

        Ok(())
    }

    pub fn generate_size_calculator(&mut self) -> Result {
        if self.proto.label() == FieldLabel::Repeated {
            gen!(self.printer; self.vars => "\nsize = size.checked_add(self.{field_name}.calculate_size(&{codec})?)?;", field_name, codec);
        } else {
            if let FieldScope::Oneof(_) = self.proto.scope() {
                if is_copy_type(self.proto.field_type()) {
                    gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = self.{field_name} {{", oneof, name, field_name);
                } else {
                    gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = &self.{field_name} {{", oneof, name, field_name);
                }
                self.printer.indent();
            } else {
                if is_copy_type(self.proto.field_type()) {
                    gen!(self.printer; self.vars => "\nlet {field_name} = self.{field_name};", field_name);
                } else {
                    gen!(self.printer; self.vars => "\nlet {field_name} = &self.{field_name};", field_name);
                }

                match self.proto.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        gen!(self.printer; self.vars => "\nif let std::option::Option::Some({field_name}) = {field_name} {{", field_name);
                        self.printer.indent();
                    },
                    _ => {
                        if self.proto.file().syntax() == Syntax::Proto2 {
                            gen!(self.printer; self.vars => "\nif let std::option::Option::Some({field_name}) = {field_name} {{", field_name);
                            self.printer.indent();
                        }
                        match self.proto.field_type() {
                            FieldType::Bytes => 
                                gen!(self.printer; self.vars => "\nif {field_name}.as_slice() != {default} {{", field_name, default),
                            _ => 
                                gen!(self.printer; self.vars => "\nif {field_name} != {default} {{", field_name, default)
                        }
                        self.printer.indent();
                    }
                }
            }

            gen!(self.printer; self.vars => "\nsize = size.checked_add({tag_size})?;", tag_size);
            gen!(self.printer; self.vars => "\nsize = size.checked_add({crate_name}::io::sizes::{proto_type}({field_name})", field_name, crate_name, proto_type);

            if is_copy_type(self.proto.field_type()) {
                gen!(self.printer, ")?;");
            } else {
                gen!(self.printer, "?)?;");
            }

            if let FieldScope::Oneof(_) = self.proto.scope() {
                self.printer.unindent();
                gen!(self.printer, "\n}}");
            } else {
                match self.proto.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        self.printer.unindent();
                        gen!(self.printer, "\n}}");
                    },
                    _ => {
                        if self.proto.file().syntax() == Syntax::Proto2 {
                            self.printer.unindent();
                            gen!(self.printer, "\n}}");
                        }
                        self.printer.unindent();
                        gen!(self.printer, "\n}}");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate_writer(&mut self) -> Result {
        if self.proto.label() == FieldLabel::Repeated {
            gen!(self.printer; self.vars => "\nself.{field_name}.write_to(output, &{codec})?;", field_name, codec);
        } else {
            if let FieldScope::Oneof(_) = self.proto.scope() {
                if is_copy_type(self.proto.field_type()) {
                    gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = self.{field_name} {{", oneof, name, field_name);
                } else {
                    gen!(self.printer; self.vars => "\nif let self::{oneof}::{name}({field_name}) = &self.{field_name} {{", oneof, name, field_name);
                }
                self.printer.indent();
            } else {
                if is_copy_type(self.proto.field_type()) {
                    gen!(self.printer; self.vars => "\nlet {field_name} = self.{field_name};", field_name);
                } else {
                    gen!(self.printer; self.vars => "\nlet {field_name} = &self.{field_name};", field_name);
                }

                match self.proto.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        gen!(self.printer; self.vars => "\nif let std::option::Option::Some({field_name}) = {field_name} {{", field_name);
                        self.printer.indent();
                    },
                    _ => {
                        if self.proto.file().syntax() == Syntax::Proto2 {
                            gen!(self.printer; self.vars => "\nif let std::option::Option::Some({field_name}) = {field_name} {{", field_name);
                            self.printer.indent();
                        }
                        match self.proto.field_type() {
                            FieldType::Bytes => 
                                gen!(self.printer; self.vars => "\nif {field_name}.as_slice() != {default} {{", field_name, default),
                            _ => 
                                gen!(self.printer; self.vars => "\nif {field_name} != {default} {{", field_name, default)
                        }
                        self.printer.indent();
                    }
                }
            }

            gen!(self.printer; self.vars => "\noutput.write_raw_tag_bytes(&{tag_bytes})?;", tag_bytes);
            gen!(self.printer; self.vars => "\noutput.write_{proto_type}({field_name})?;", proto_type, field_name);

            if let FieldType::Group(_) = self.proto.field_type() {
                gen!(self.printer; self.vars => "\noutput.write_raw_tag_bytes(&{end_tag_bytes})?;", end_tag_bytes);
            }

            if let FieldScope::Oneof(_) = self.proto.scope() {
                self.printer.unindent();
                gen!(self.printer, "\n}}");
            } else {
                match self.proto.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        self.printer.unindent();
                        gen!(self.printer, "\n}}");
                    },
                    _ => {
                        if self.proto.file().syntax() == Syntax::Proto2 {
                            self.printer.unindent();
                            gen!(self.printer, "\n}}");
                        }
                        self.printer.unindent();
                        gen!(self.printer, "\n}}");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate_field_number_constant(&mut self) -> Result {
        gen!(self.printer; self.vars => 
"
/// Gets the field number of the '{proto_name}' field
pub const {field_number_const}: i32 = {number};", proto_name, field_number_const, number);
        Ok(())
    }

    pub fn generate_default_value(&mut self) -> Result {
        if self.proto.label() != FieldLabel::Repeated {
            match self.proto.field_type() {
                FieldType::Message(_) | FieldType::Group(_) => { },
                _ => {
                    gen!(self.printer; self.vars => "\nstatic {default}: {default_type} = {default_value};", default, default_type, default_value);
                }
            }
        }

        Ok(())
    }

    pub fn generate_codec(&mut self) -> Result {
        if self.proto.label() == FieldLabel::Repeated {
            match self.proto.field_type() {
                FieldType::Message(m) if m.map_entry() => {
                    gen!(self.printer; self.vars => "\nstatic {codec}: ({crate_name}::Codec<", codec, crate_name);
                    let generator = Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[0]);
                    gen!(generator.printer; generator.vars => "{indirected_type}", indirected_type);

                    gen!(self.printer; self.vars => ">, {crate_name}::Codec<", crate_name);
                    let generator = Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[1]);
                    gen!(generator.printer; generator.vars => "{indirected_type}", indirected_type);
                    gen!(self.printer, ">) = (");

                    Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[0]).generate_codec_new()?;
                    gen!(self.printer, ", ");
                    Generator::<FieldDescriptor, _>::from_other(self, &m.fields()[1]).generate_codec_new()?;
                    gen!(self.printer, ");");
                },
                _ => {
                    gen!(self.printer; self.vars => "\nstatic {codec}: {crate_name}::Codec<{indirected_type}> = ", codec, crate_name, indirected_type);
                    self.generate_codec_new()?;
                    gen!(self.printer, ";");
                }
            }
        }

        Ok(())
    }

    pub fn generate_codec_new(&mut self) -> Result {
        match self.proto.field_type() {
            FieldType::Group(_) => gen!(self.printer; self.vars => "{crate_name}::Codec::group({tag}, {end_tag})", crate_name, tag, end_tag),
            _ => gen!(self.printer; self.vars => "{crate_name}::Codec::{proto_type}({tag})", crate_name, proto_type, tag)
        }
        Ok(())
    }

    pub fn generate_accessors(&mut self) -> Result {
        Ok(())
    }
}

fn default_field_value(field: &FieldDescriptor, crate_name: &str) -> String {
    match field.label() {
        FieldLabel::Optional | FieldLabel::Required => {
            if field.file().syntax() == Syntax::Proto2 {
                format!("std::option::Option::None")
            } else {
                match field.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => format!("std::option::Option::None"),
                    FieldType::String => format!("std::string::String::new()"),
                    FieldType::Bytes => format!("std::vec::Vec::new()"),
                    _ => names::get_field_default_value_name(field)
                }
            }
        },
        FieldLabel::Repeated => {
            if let FieldType::Message(m) = field.field_type() {
                if m.map_entry() {
                    return format!("{}::collections::MapField::new()", crate_name)
                }
            }

            format!("{}::collections::RepeatedField::new()", crate_name)
        }
    }
}

fn is_copy_type(ft: &FieldType) -> bool {
    match ft {
        FieldType::Message(_) |
        FieldType::Group(_)   |
        FieldType::Bytes      |
        FieldType::String => false,
        _ => true
    }
}

generator_new!(EnumDescriptor, proto, options;
    "type_name", names::get_enum_type_name(proto),
    "crate_name", options.crate_name.clone(),
    "full_type_name", names::get_full_enum_type_name(proto, proto.file()));

impl<W: Write> Generator<'_, EnumDescriptor, W> {
    pub fn generate(&mut self) -> Result {
        gen!(self.printer; self.vars => 
"
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum {type_name} {{", type_name);
        self.printer.indent();

        for value in self.proto.values() {
            writeln!(self.printer)?;
            Generator::<EnumValueDescriptor, _>::from_other(self, value).generate_variant()?;
        }

        self.printer.unindent();
        gen!(self.printer; self.vars => "\n}}\nimpl std::convert::TryFrom<i32> for {full_type_name} {{", full_type_name);
        self.printer.indent();
        gen!(self.printer; self.vars => concat!(
            "\ntype Error = {crate_name}::VariantUndefinedError;",
            "\n",
            "\nfn try_from(value: i32) -> std::result::Result<Self, {crate_name}::VariantUndefinedError> {{"), crate_name);
        self.printer.indent();
        gen!(self.printer, "\nmatch value {{");
        self.printer.indent();

        for value in self.proto.values() {
            writeln!(self.printer)?;
            Generator::<EnumValueDescriptor, _>::from_other(self, value).generate_int_match()?;
        }

        gen!(self.printer; self.vars => "\n_ => std::result::Result::Err({crate_name}::VariantUndefinedError)", crate_name);
        self.printer.unindent();
        gen!(self.printer, "\n}}");
        self.printer.unindent();
        gen!(self.printer, "\n}}");
        self.printer.unindent();
        gen!(self.printer, "\n}}");

        gen!(self.printer; self.vars => "\nimpl std::convert::From<{full_type_name}> for i32 {{", full_type_name);
        self.printer.indent();
        gen!(self.printer; self.vars => "\nfn from(value: {full_type_name}) -> i32 {{", full_type_name);
        self.printer.indent();
        gen!(self.printer, "\nvalue as i32");
        self.printer.unindent();
        gen!(self.printer, "\n}}");
        self.printer.unindent();
        gen!(self.printer, "\n}}");

        Ok(())
    }
}

generator_new!(EnumValueDescriptor, proto, options;
    "full_variant", names::get_full_enum_variant_name(proto, proto.file()),
    "variant", names::get_enum_variant_name(proto),
    "number", proto.number().to_string());

impl<W: Write> Generator<'_, EnumValueDescriptor, W> {
    pub fn generate_variant(&mut self) -> Result {
        gen!(self.printer; self.vars => "{variant} = {number},", variant, number);
        Ok(())
    }

    pub fn generate_int_match(&mut self) -> Result {
        gen!(self.printer; self.vars => "{number} => std::result::Result::Ok({full_variant}),", number, full_variant);
        Ok(())
    }
}

generator_new!(OneofDescriptor, proto, options;
    "name", proto.name().clone(),
    "field_name", {
        let mut base = proto.name().clone();
        if !options.pub_fields {
            base.push('_')
        }

        base
    },
    "type_name", names::get_oneof_name(proto));

impl<W: Write> Generator<'_, OneofDescriptor, W> {
    pub fn generate_type(&mut self) -> Result {
        gen!(self.printer; self.vars => 
"
#[derive(Clone, PartialEq)]
pub enum {type_name} {{", type_name);
        self.printer.indent();

        gen!(self.printer, "\nNone,");

        for field in self.proto.fields() {
            writeln!(self.printer)?;
            Generator::<FieldDescriptor, _>::from_other(self, field).generate_oneof_field()?;
        }

        self.printer.unindent();
        write!(self.printer, "\n}}\n")?;

        Ok(())
    }

    pub fn generate_struct_field(&mut self) -> Result {
        if self.options.pub_fields {
            write!(self.printer, "pub ")?;
        }

        gen!(self.printer; self.vars => "{field_name}: {type_name},", field_name, type_name);

        Ok(())
    }

    pub fn generate_new(&mut self) -> Result {
        gen!(self.printer; self.vars => "{field_name}: self::{type_name}::None,", field_name, type_name);

        Ok(())
    }
}