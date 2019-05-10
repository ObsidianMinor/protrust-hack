//! This is the main file that contains the code generators
//! for the included protoc-gen-rust plugin in protrust.
//! 
//! Included is the Options struct, specifying code-gen options,
//! the Generator struct, the main generator that holds Options and the generate method for generating code,
//! and the SourceGenerator struct, a generic generator for generating code using the information from a descriptor.
//! 
//! The plugin works by creating a Generator with a provided mutable Context. The plugin then calls
//! Generator::generate which will start the process of generating code. Generating code works by creating a SourceGenerator
//! for a file and calling a method on it. From there, the SourceGenerator can make new SourceGenerators to generate code
//! that uses other descriptors for reference.
//! 
//! Source is generated through syn and quote. This adds constraints (for example, no stray comments) 
//! however it makes it easier to check syntax ahead of time. It also makes it easier to read and write code templates.

use itertools::Itertools;
use protrust::io::{Tag, WireType};
use protrust::plugin::{names::{self, Scope, TypeKind, FieldName}, Context};
use protrust::reflect::{Descriptor, Syntax, FileDescriptor, MessageDescriptor, CompositeScope, EnumDescriptor, OneofDescriptor, FieldDescriptor, DefaultValue, FieldType, FieldLabel, FieldScope, SourceCodeInfo};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::error::Error;
use syn::{Ident, Expr, ExprLit, Path};

pub struct Options {
    /// Allows users to change the name of the crate for referencing the codegen modules.
    ///
    /// The default is 'protrust'
    pub crate_name: Path,
    /// Allows users to make the compiler not generate JSON trait implementations, even for proto3 files
    pub no_json: bool,
    /// Uses checked addition in CodedMessage::calculate_size. Must be used with the checked_size feature
    pub size_checks: bool,
    /// Removes all reflection usage in generated code
    pub no_reflection: bool,
    /// Includes the specified modules in a generated code module
    pub external_modules: Vec<Path>,
}

#[derive(Debug)]
pub enum OptionError {
    UnknownOption(String, Option<String>),
    ParseError(String, Box<dyn Error>)
}

impl std::fmt::Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for OptionError { }

impl Options {
    pub fn parse_from<'a>(params: impl Iterator<Item = (&'a str, Option<&'a str>)>) -> std::result::Result<Options, OptionError> {
        let mut options = Options {
            crate_name: syn::parse_str("::protrust").unwrap(),
            no_json: false,
            size_checks: false,
            no_reflection: false,
            external_modules: Vec::new()
        };
        for param in params {
            match param {
                ("crate_name", Some(c)) => options.crate_name = syn::parse_str(c).map_err(|e| OptionError::ParseError(c.to_string(), Box::new(e)))?,
                ("no_json", None) => options.no_json = true,
                ("size_checks", None) => options.size_checks = true,
                ("no_reflection", None) => options.no_reflection = true,
                ("external_modules", Some(e)) => 
                    options.external_modules = 
                        e.split('+')
                            .map(syn::parse_str)
                            .collect::<syn::Result<Vec<Path>>>()
                            .map_err(|e| OptionError::ParseError(e.to_string(), Box::new(e)))?,
                (key, value) => return Err(OptionError::UnknownOption(key.to_string(), value.map(|v| v.to_string())))
            }
        }
        Ok(options)
    }
}

type Result<T = TokenStream> = std::result::Result<T, Box<dyn Error>>;

pub struct Generator<'a, 'b> {
    options: Options,
    context: &'a mut Context<'b>
}

impl<'a, 'b> Generator<'a, 'b> {
    pub fn new(context: &'a mut Context<'b>, options: Options) -> Generator<'a, 'b> {
        Generator {
            context,
            options
        }
    }

    pub fn generate(&mut self) -> Result<()> {
        let externals = self.generate_externals_mod()?;
        let mods = self.generate_mods()?;

        self.context.add_output_file("mod.rs", quote! {
            #externals
            #mods
        });

        Ok(())
    }

    fn generate_externals_mod(&self) -> Result {
        let collected_input: Box<[&FileDescriptor]> = self.context.input_files().collect();
        let input_files_set: HashSet<_> = collected_input.iter().map(|r| *r).collect();
        let all_deps: HashSet<_> = flatten_deps(collected_input.iter().map(|r| *r)).collect();
        let external_deps = input_files_set.difference(&all_deps).map(|r| *r).collect::<Vec<&FileDescriptor>>();
        let mut external_files_to_rust_names: Vec<Ident> = Vec::new();
        for file in &external_deps {
            external_files_to_rust_names.push(names::get_rust_file_mod_name(file)?);
        }
        let crate_file = 
            if external_deps.iter().any(|f| match f.name() {
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
            }) {
                let c = &self.options.crate_name;
                Some(syn::parse2::<Path>(quote!(#c::generated))?)
            } else {
                None
            };
        let external_deps = 
            self.options.external_modules
                .iter()
                .cloned()
                .map(|p| {
                    if p.leading_colon.is_none() {
                        syn::parse2(quote!(::#p)).expect("couldn't append a leading colon2 to the path")
                    } else {
                        p
                    }
                })
                .chain(crate_file)
                .collect::<Vec<Path>>();

        let stream = quote! {
            mod externals {
                #(pub(super) use #external_deps::*;
                )*
            }
            #(use self::externals::#external_files_to_rust_names;
            )*
        };

        Ok(stream)
    }

    fn generate_mods(&self) -> Result {
        let mods = self.context.input_files().map(|file| {
            let mod_name = names::get_rust_file_mod_name(file)?;
            let file_stream = SourceGenerator::new(file, &self.options).generate()?;
            Ok(quote! {
                pub mod #mod_name {
                    #file_stream
                }
            })
        }).collect::<Result<Vec<_>>>()?;

        let stream = quote! {
            #(#mods
            )*
        };
        Ok(stream)
    }
}

fn flatten_deps<'a, T>(files: T) -> Box<(dyn Iterator<Item = &'a FileDescriptor> + 'a)> 
    where 
        T: 'a + Iterator<Item = &'a FileDescriptor> {
    Box::new(files
        .flat_map(|f| f.dependencies()
            .iter()
            .map(|r| &**r)
            .chain(flatten_deps(f.public_dependencies().iter().map(|r| &**r))))
        .unique())
}

pub struct SourceGenerator<'a, T> {
    options: &'a Options,
    descriptor: &'a T,
}

impl<'a, T> SourceGenerator<'a, T> {
    pub fn new(descriptor: &'a T, options: &'a Options) -> Self {
        SourceGenerator { descriptor, options }
    }

    pub fn with<'b: 'a, U>(&'b self, other: &'b U) -> SourceGenerator<'b, U> {
        SourceGenerator {
            options: self.options,
            descriptor: other
        }
    }
}

impl SourceGenerator<'_, FileDescriptor> {
    pub fn generate(&self) -> Result {
        let crt = &self.options.crate_name;
        let file_str: syn::ExprLit = syn::parse_str(&format!("\"{}\"", self.descriptor.name()))?;

        let substreams: Vec<TokenStream> = {
            self.descriptor
                .extensions()
                .iter()
                .map(|e| {
                    self.with::<FieldDescriptor>(e).generate_extension()
                })
                .chain(
            self.descriptor
                .messages()
                .iter()
                .map(|m| {
                    self.with::<MessageDescriptor>(m).generate()
                })
                .chain(
            self.descriptor
                .enums()
                .iter()
                .map(|e| {
                    self.with::<EnumDescriptor>(e).generate()
                })))
                .collect::<std::result::Result<Vec<TokenStream>, _>>()
        }?;

        let stream = quote! {
            /// Gets the descriptor for the file this module was generated for
            pub fn file() -> &'static #crt::reflect::FieldDescriptor {
                super::pool().find_file_by_name(#file_str).unwrap()
            }

            #(#substreams
            )*
        };
        Ok(stream)
    }
}

impl SourceGenerator<'_, MessageDescriptor> {
    pub fn generate(&self) -> Result {
        let docs = self.generate_rustdoc_comments()?;
        let name = names::get_message_type(self.descriptor)?;
        let fields: Vec<TokenStream> =
            self.descriptor
                .message_fields()
                .iter()
                .map(|f| 
                    self.with::<FieldDescriptor>(f).generate_struct_field())
                .chain(self.descriptor
                    .oneofs()
                    .iter()
                    .map(|o| 
                        self.with::<OneofDescriptor>(o).generate_struct_field()))
                .collect::<std::result::Result<Vec<TokenStream>, _>>()?;
        let c = &self.options.crate_name;
        let extensions = 
            if self.descriptor.proto().extension_range().len() != 0 {
                Some(quote!(extensions: #c::ExtensionSet<Self>))
            } else {
                None
            };

        let coded = self.generate_coded_message_impl()?;
        let lite = self.generate_lite_message_impl()?;
        let extendable = self.generate_extendable_message_impl()?;
        let message = self.generate_message_impl()?;
        let struct_impl = self.generate_struct_impl()?;
        let message_mod = self.generate_message_mod()?;

        let stream = quote! {
            #docs
            #[derive(Clone, Debug, PartialEq, Default)]
            pub struct #name {
                #(#fields,
                )*
                unknown_fields: #c::UnknownFieldSet,
                #extensions,
            }

            #coded
            #lite
            #extendable
            #message
            #struct_impl
            #message_mod
        };

        Ok(stream)
    }

    pub fn generate_coded_message_impl(&self) -> Result {
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor, Some(Scope::from(self.descriptor.scope())))?;
        let merger = self.generate_merge_from()?;
        let calculator = self.generate_calculate_size()?;
        let writer = self.generate_write_to()?;
        let initialized = self.generate_is_initialized()?;
        Ok(quote! {
            impl #c::CodedMessage for #t {
                #merger
                #calculator
                #writer
                #initialized
            }
        })
    }

    fn get_end_group_tag(&self) -> Option<Tag> {
        match self.descriptor.scope() {
            CompositeScope::Message(m) => {
                for field in m.fields() {
                    match field.field_type() {
                        FieldType::Group(m) if **m == *self.descriptor => {
                            return Some(Tag::new(field.number(), WireType::EndGroup))
                        },
                        _ => { }
                    }
                }
                for extension in m.extensions() {
                    match extension.field_type() {
                        FieldType::Group(m) if **m == *self.descriptor => {
                            return Some(Tag::new(extension.number(), WireType::EndGroup))
                        },
                        _ => { }
                    }
                }
                None
            },
            CompositeScope::File(f) => {
                for extension in f.extensions() {
                    match extension.field_type() {
                        FieldType::Group(m) if **m == *self.descriptor => {
                            return Some(Tag::new(extension.number(), WireType::EndGroup))
                        },
                        _ => { }
                    }
                }
                None
            }
        }
    }

    fn generate_merge_from(&self) -> Result {
        let c = &self.options.crate_name;
        let field_mergers = 
            self.descriptor
                .fields()
                .iter()
                .map(|f| 
                    self.with::<FieldDescriptor>(f).generate_merge_arm())
                .collect::<Result<Vec<_>>>()?;
        let end_group_break: Option<TokenStream> = {
            self.get_end_group_tag()
                .map(|t| syn::parse_str::<Ident>(&t.to_string()))
                .transpose()?
                .map(|id| quote!(#id => break,))
        };
        let unknown_field_merge = 
            if self.descriptor.proto().extension_range().len() != 0 {
                quote! {
                    if !self.extensions.merge_from(tag, input)? {
                        self.unknown_fields.merge_from(tag, input)?
                    }
                }
            } else {
                quote! {
                    self.unknown_fields.merge_from(tag, input)?
                }
            };
        Ok(quote! {
            fn merge_from(&mut self, input: &mut #c::io::CodedInput) -> #c::io::InputResult<()> {
                while let ::std::option::Option::Some(tag) = input.read_tag()? {
                    match tag.get() {
                        #(#field_mergers,
                        )*
                        #end_group_break
                        _ => {
                            #unknown_field_merge
                        }
                    }
                }
                ::std::result::Result::Ok(())
            }
        })
    }
    fn generate_calculate_size(&self) -> Result {
        let size_calculators = 
            self.descriptor
                .fields()
                .iter()
                .map(|f| 
                    self.with::<FieldDescriptor>(f).generate_size_calculator())
                .collect::<Result<Vec<_>>>()?;
        let extensions = 
            if self.descriptor.proto().extension_range().len() != 0 {
                if self.options.size_checks {
                    Some(quote!(size = size.checked_add(self.extensions.calculate_size()?)?;))
                } else {
                    Some(quote!(size += self.extensions.calculate_size();))
                }
            } else {
                None
            };
        if self.options.size_checks {
            Ok(quote! {
                fn calculate_size(&self) -> ::std::option::Option<i32> {
                    let mut size = 0i32;
                    #(#size_calculators
                    )*
                    #extensions
                    size = size.checked_add(self.unknown_fields.calculate_size()?)?;
                    ::std::option::Option::Some(size)
                }
            })
        } else {
            Ok(quote! {
                fn calculate_size(&self) -> i32 {
                    let mut size = 0i32;
                    #(#size_calculators
                    )*
                    #extensions
                    size += self.unknown_fields.calculate_size();
                    size
                }
            })
        }
    }

    fn generate_write_to(&self) -> Result {
        let c = &self.options.crate_name;
        let field_writers = 
            self.descriptor
                .fields()
                .iter()
                .map(|f| 
                    self.with::<FieldDescriptor>(f).generate_value_writer())
                .collect::<Result<Vec<_>>>()?;
        let extensions = 
            if self.descriptor.proto().extension_range().len() != 0 {
                Some(quote!(self.extensions.write_to(output)?;))
            } else {
                None
            };
        Ok(quote! {
            fn write_to(&self, output: &mut #c::io::CodedOutput) -> #c::io::OutputResult {
                #(#field_writers
                )*
                #extensions
                self.unknown_fields.write_to(output)?;
                ::std::result::Result::Ok(())
            }
        })
    }

    fn generate_is_initialized(&self) -> Result {
        let checks = 
            self.descriptor
                .fields()
                .iter()
                .map(|f| 
                    self.with::<FieldDescriptor>(f).generate_initialized_check())
                .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            fn is_initialized(&self) -> bool {
                #(#checks
                )*
                true
            }
        })
    }

    pub fn generate_lite_message_impl(&self) -> Result {
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor, Some(Scope::from(self.descriptor.scope())))?;
        let merge = self.generate_merge_fn()?;
        Ok(quote! {
            impl #c::LiteMessage for #t {
                #merge
            }
        })
    }

    fn generate_merge_fn(&self) -> Result {
        let field_merge = 
            self.descriptor
                .fields()
                .iter()
                .map(|f| self.with::<FieldDescriptor>(f).generate_merge())
                .collect::<Result<Vec<_>>>()?;
        let extensions_merge = 
            if self.descriptor.proto().extension_range().len() != 0 {
                Some(quote!(self.extensions.merge(other.extensions)))
            } else {
                None
            };
        Ok(quote! {
            fn merge(&mut self, other: &Self) {
                #(#field_merge
                )*
                #extensions_merge
                self.unknown_fields.merge(other.unknown_fields);
            }
        })
    }

    pub fn generate_extendable_message_impl(&self) -> Result<Option<TokenStream>> {
        if self.descriptor.proto().extension_range().len() != 0 {
            let c = &self.options.crate_name;
            let t = names::get_message_type_path(self.descriptor, Some(Scope::from(self.descriptor.scope())))?;
            Ok(Some(quote! {
                impl #c::ExtensionMessage for #t {
                    fn registry(&self) -> ::std::option::Option<&'static #c::ExtensionRegistry> { self.extensions.registry() }
                    fn replace_registry(&mut self, extensions: ::std::option::Option<&'static #c::ExtensionRegistry>) -> ::std::option::Option<&'static #c::ExtensionRegistry> { self.extensions.replace_registry(extensions) }
                    fn has_extension<T: #c::ExtensionIdentifier>(&self, extension: &'static T) -> bool { self.extensions.has_extension(extension) }
                    fn has_extension_unchecked<T: #c::ExtensionIdentifier>(&self, extension: &'static T) -> bool { self.extensions.has_extension_unchecked(extension) }
                    fn get_value<V: ::std::clone::Clone + std::cmp::PartialEq + ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync, D: ::std::marker::Sync>(&self, extension: &'static #c::Extension<Self, V, D>) -> Option<&V> { self.extensions.get_value(extension) }
                    fn get_value_or_default<V: ::std::clone::Clone + std::cmp::PartialEq + ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + ::std::ops::Deref<Target = L>, D: ::std::marker::Sync + ::std::ops::Deref<Target = L>, L>(&self, extension: &'static #c::Extension<Self, V, D>) -> Option<&L> { self.extensions.get_value_or_default(extension) }
                    fn get_repeated_value<V: ::std::cmp::PartialEq + ::std::clone::Clone + ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync>(&self, extension: &'static #c::RepeatedExtension<Self, V>) -> Option<&#c::collections::RepeatedField<V>> { self.extensions.get_repeated_value(extension) }
                    fn field<V: ::std::default::Default + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync, D: ::std::marker::Sync>(&mut self, extension: &'static #c::Extension<Self, V, D>) -> ::std::option::Option<#c::ExtensionField<Self, V, D>> { self.extensions.field(extension) }
                    fn repeated_field<V: ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync>(&mut self, extension: &'static #c::RepeatedExtension<Self, V>) -> ::std::option::Option<#c::RepeatedExtensionField<Self, V>> { self.extensions.repeated_field(extension) }
                }
            }))
        } else {
            Ok(None)
        }
    }

    pub fn generate_message_impl(&self) -> Result<Option<TokenStream>> {
        if !self.options.no_reflection {
            let c = &self.options.crate_name;
            let t = names::get_message_type(self.descriptor)?;
            let descriptor = {
                let index: ExprLit = syn::parse_str(&self.descriptor.scope_index().to_string())?;
                match self.descriptor.scope() {
                    CompositeScope::File(_) => {
                        quote!(&self::file().messages()[#index])
                    },
                    CompositeScope::Message(m) => {
                        let t = names::get_message_type(m)?;
                        quote!(&<self::super::#t as #c::Message>::descriptor().messages()[#index])
                    }
                }
            };
            Ok(Some(quote! {
                impl #c::Message for self::#t {
                    fn descriptor() -> &'static #c::reflect::MessageDescriptor {
                        #descriptor
                    }
                }
            }))
        } else {
            Ok(None)
        }
    }

    pub fn generate_struct_impl(&self) -> Result {
        let t = names::get_message_type(self.descriptor)?;
        let impls = 
            self.descriptor
                .fields()
                .iter()
                .map(|f|
                    self.with::<FieldDescriptor>(f).generate_impl())
                .chain(
            self.descriptor
                .oneofs()
                .iter()
                .map(|o|
                    self.with::<OneofDescriptor>(o).generate_accessor()))
                .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            impl self::#t {
                #(#impls
                )*
            }
        })
    }

    pub fn generate_message_mod(&self) -> Result<Option<TokenStream>> {
        let streams = 
            self.descriptor
                .fields()
                .iter()
                .map(|f|
                    self.with::<FieldDescriptor>(f).generate_codec().transpose())
                .flatten()
                .chain(
            self.descriptor
                .fields()
                .iter()
                .map(|f|
                    self.with::<FieldDescriptor>(f).generate_reflector().transpose())
                .flatten()
                .chain(
            self.descriptor
                .enums()
                .iter()
                .map(|e| 
                    self.with::<EnumDescriptor>(e).generate())
                .chain(
            self.descriptor
                .oneofs()
                .iter()
                .map(|o|
                    self.with::<OneofDescriptor>(o).generate_enum())
                .chain(
            self.descriptor
                .extensions()
                .iter()
                .map(|e|
                    self.with::<FieldDescriptor>(e).generate_extension())
                .chain(
            self.descriptor
                .messages()
                .iter()
                .filter(|m| !m.map_entry())
                .map(|m|
                    self.with::<MessageDescriptor>(m).generate()))))))
                .collect::<std::result::Result<Vec<TokenStream>, _>>()?;

        if streams.len() != 0 {
            let t = names::get_message_mod(self.descriptor)?;
            Ok(Some(quote! {
                /// A module containing the types, extensions, and oneof enums contained in its corresponding message type
                pub mod #t {
                    #(#streams
                    )*
                }
            }))
        } else {
            Ok(None)
        }
    }

    pub fn generate_rustdoc_comments(&self) -> Result<Option<TokenStream>> {
        generate_rustdoc_comments(self.descriptor.source_code_info())
    }
}

impl SourceGenerator<'_, EnumDescriptor> {
    pub fn generate(&self) -> Result {
        let docs = self.generate_rustdoc_comments()?;
        let t = names::get_enum_type(self.descriptor)?;
        let c = &self.options.crate_name;
        let variants: Vec<Ident> = {
            self.descriptor
                .values()
                .iter()
                .map(|v| names::get_enum_variant(v))
                .collect::<syn::Result<Vec<Ident>>>()
        }?;
        let descriptor = {
            let index: ExprLit = syn::parse_str(&self.descriptor.scope_index().to_string())?;
            match self.descriptor.scope() {
                CompositeScope::File(_) => {
                    quote!(&self::file().enums()[#index])
                },
                CompositeScope::Message(m) => {
                    let t = names::get_message_type(m)?;
                    quote!(&<self::super::#t as #c::Message>::descriptor().enums()[#index])
                }
            }
        };
        let try_from = self.generate_try_from_i32_impl()?;
        let from = self.generate_i32_from_impl()?;

        Ok(quote! {
            #docs
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            pub enum #t {
                #(#variants,
                )*
            }
            impl #c::Enum for self::#t {
                fn descriptor() -> &'static #c::reflect::EnumDescriptor {
                    #descriptor
                }
            }
            #try_from
            #from
        })
    }

    fn generate_try_from_i32_impl(&self) -> Result {
        let c = &self.options.crate_name;
        let t = names::get_enum_type_path(self.descriptor, Some(Scope::from(self.descriptor.scope())))?;
        let matches: Vec<TokenStream> = {
            self.descriptor
                .values()
                .iter()
                .map(|f| {
                    let num = syn::parse_str::<ExprLit>(&f.number().to_string())?;
                    let variant = names::get_enum_variant_path(f, Some(Scope::from(self.descriptor.scope())))?;
                    Ok(quote!(#num => ::std::result::Result::Ok(#variant)))
                })
                .collect::<syn::Result<Vec<TokenStream>>>()
        }?;
        Ok(quote! {
            impl ::std::convert::TryFrom<i32> for #t {
                type Error = #c::VariantUndefinedError;
                fn try_from(value: i32) -> ::std::result::Result<Self, #c::VariantUndefinedError> {
                    #[allow(unreachable_patterns)]
                    match value {
                        #(#matches,
                        )*
                        _ => ::std::result::Result::Err(#c::VariantUndefinedError)
                    }
                }
            }
        })
    }

    fn generate_i32_from_impl(&self) -> Result {
        let t = names::get_enum_type_path(self.descriptor, Some(Scope::from(self.descriptor.scope())))?;
        let matches: Vec<TokenStream> = {
            self.descriptor
                .values()
                .iter()
                .map(|f| {
                    let num = syn::parse_str::<ExprLit>(&f.number().to_string())?;
                    let variant = names::get_enum_variant_path(f, Some(Scope::from(self.descriptor.scope())))?;
                    Ok(quote!(#variant => #num))
                })
                .collect::<syn::Result<Vec<TokenStream>>>()
        }?;
        Ok(quote! {
            impl ::std::convert::From<#t> for i32 {
                fn from(value: #t) -> i32 {
                    match value {
                        #(#matches,
                        )*
                    }
                }
            }
        })
    }

    pub fn generate_rustdoc_comments(&self) -> Result<Option<TokenStream>> {
        generate_rustdoc_comments(self.descriptor.source_code_info())
    }
}

impl SourceGenerator<'_, OneofDescriptor> {
    pub fn generate_struct_field(&self) -> Result {
        let name = syn::parse_str::<Ident>(self.descriptor.name())?;
        let t = names::get_oneof_type_path(self.descriptor, Some(Scope::from(self.descriptor.message().scope())))?;

        Ok(quote!(#name: #t))
    }

    pub fn generate_enum(&self) -> Result {
        let docs = self.generate_rustdoc_comments()?;
        let t = names::get_oneof_type(self.descriptor)?;
        let fields = 
            self.descriptor
                .fields()
                .iter()
                .map(|f| self.with::<FieldDescriptor>(f).generate_enum_field())
                .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            #docs
            #[derive(Clone, Debug, PartialEq)]
            pub enum #t {
                /// No value
                None,
                #(#fields
                )*
            }
            impl ::std::default::Default for self::#t {
                fn default() -> Self {
                    self::#t::None
                }
            }
        })
    }

    pub fn generate_accessor(&self) -> Result {
        let t = names::get_message_type(self.descriptor.message())?;
        let ref_doc_comment = format!(concat!(
            "/// Gets a shared reference to the [`{0}`] oneof field\n",
            "/// \n",
            "/// [`{0}`]: enum.{1}.html"
            ), self.descriptor.name(), t);
        let mut_doc_comment = format!(concat!(
            "/// Gets a unique reference to the [`{0}`] oneof field\n",
            "/// \n",
            "/// [`{0}`]: enum.{1}.html"
            ), self.descriptor.name(), t);
        let f: Ident = syn::parse_str(self.descriptor.name())?;
        let nm: Ident = syn::parse_str(&format!("{}_mut", self.descriptor.name()))?;
        let m = names::get_message_mod(self.descriptor.message())?;

        Ok(quote! {
            #ref_doc_comment
            pub fn #f(&self) -> &self::#m::#t {
                &self.#f
            }
            #mut_doc_comment
            pub fn #nm(&mut self) -> &mut self::#m::&t {
                &mut self.#f
            }
        })
    }

    pub fn generate_rustdoc_comments(&self) -> Result<Option<TokenStream>> {
        generate_rustdoc_comments(self.descriptor.source_code_info())
    }
}

impl SourceGenerator<'_, FieldDescriptor> {
    pub fn generate_struct_field(&self) -> Result {
        let name = names::get_field_name(self.descriptor, FieldName::Field)?;
        let t = names::get_rust_type(self.descriptor, TypeKind::Indirected, Some(Scope::from(self.descriptor.message().scope())), &self.options.crate_name, false)?;
        if self.descriptor.file().syntax() == Syntax::Proto2 && self.descriptor.label() != FieldLabel::Repeated {
            Ok(quote!(#name: ::std::option::Option<#t>))
        } else {
            Ok(quote!(#name: #t))
        }
    }

    pub fn generate_enum_field(&self) -> Result {
        let n = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
        let t = names::get_rust_type(self.descriptor, TypeKind::Indirected, Some(Scope::Message(self.descriptor.message())), &self.options.crate_name, false)?;
        Ok(quote!(#n(#t),))
    }

    pub fn generate_merge(&self) -> Result {
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                Ok(quote!(self.#n.merge(&other.#n);))
            },
            _ => {
                match self.descriptor.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        let c = &self.options.crate_name;
                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                        let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                        Ok(quote! {
                            if let ::std::option::Option::Some(#n) = &other.#get {
                                #c::LiteMessage::merge(self.#get_mut, #n);
                            }
                        })
                    },
                    FieldType::Bytes | FieldType::String => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto3 => {
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                Ok(quote! {
                                    if other.#get.len() != 0 {
                                        *self.#get_mut() = ::std::clone::Clone::clone(other.#get);
                                    }
                                })
                            },
                            Syntax::Proto2 => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = other.#get_option {
                                        self.#set(::std::clone::Clone::clone(#n));
                                    }
                                })
                            },
                            _ => unreachable!()
                        }
                    },
                    _ => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto3 => {
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                Ok(quote! {
                                    if other.#get.len() != 0 {
                                        *self.#get_mut() = other.#get;
                                    }
                                })
                            },
                            Syntax::Proto2 => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = other.#get_option {
                                        self.#set(#n);
                                    }
                                })
                            },
                            _ => unreachable!()
                        }
                    }
                }
            }
        }
    }

    pub fn generate_merge_arm(&self) -> Result {
        let tags: TokenStream = {
            let real_tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type()).to_string();
            if self.descriptor.field_type().wire_type().is_packable() {
                let length_tag = Tag::new(self.descriptor.number(), WireType::LengthDelimited).to_string();
                quote!(#real_tag | #length_tag)
            } else {
                quote!(#real_tag)
            }
        };
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                let codec = names::get_field_name(self.descriptor, FieldName::Codec)?;
                Ok(quote!(#tags => self.#n.add_entries(input, &#codec)?))
            },
            _ => {
                unimplemented!()
            }

        }
    }

    pub fn generate_size_calculator(&self) -> Result {
        unimplemented!()
    }

    pub fn generate_value_writer(&self) -> Result {
        unimplemented!()
    }

    pub fn generate_initialized_check(&self) -> Result {
        unimplemented!()
    }

    pub fn generate_extension(&self) -> Result {
        let scope = 
            Some(
                match self.descriptor.scope() {
                    FieldScope::Message(m) => Scope::Message(&**m),
                    FieldScope::File(f) => Scope::File(&**f),
                    _ => unreachable!()
            });

        let docs = self.generate_rustdoc_comments()?;
        let crt = &self.options.crate_name;
        let msg_type = names::get_message_type_path(self.descriptor.message(), scope)?;
        let params = 
            match self.descriptor.field_type() {
                FieldType::Group(_) => {
                    let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type());
                    let lit: ExprLit = syn::parse_str(&tag.to_string())?;

                    let end_tag = Tag::new(self.descriptor.number(), WireType::EndGroup);
                    let end_lit: ExprLit = syn::parse_str(&end_tag.to_string())?;
                    quote!(#lit, #end_lit)
                },
                FieldType::Message(_) => {
                    let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type());
                    let lit: ExprLit = syn::parse_str(&tag.to_string())?;
                    quote!(#lit)
                },
                _ => {
                    let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type());
                    let lit: ExprLit = syn::parse_str(&tag.to_string())?;
                    let dflt = self.get_default_value(scope)?;
                    quote!(#lit, #dflt)
                }
            };
        let proto_type = self.get_proto_type()?;
        let name = names::get_field_name(self.descriptor, FieldName::Extension)?;
        let base = names::get_rust_type(self.descriptor, TypeKind::Base, scope, crt, false)?;
        let dflt = names::get_rust_type(self.descriptor, TypeKind::Default, scope, crt, false)?;
        let stream = match self.descriptor.label() {
            FieldLabel::Repeated => 
                quote! {
                    #docs
                    pub static #name: #crt::RepeatedExtension<#msg_type, #base> = #crt::RepeatedExtension::#proto_type(#params);
                },
            _ => 
                quote! {
                    #docs
                    pub static #name: #crt::Extension<#msg_type, #base, #dflt> = #crt::Extension::#proto_type(#params);
                }
        };

        Ok(stream)
    }

    pub fn generate_impl(&self) -> Result {
        let number = self.generate_field_number()?;
        let default = self.generate_default_value()?;
        let accessors = self.generate_accessors()?;

        Ok(quote! {
            #number
            #default
            #accessors
        })
    }

    fn generate_field_number(&self) -> Result {
        let name = self.descriptor.name();
        let const_name = names::get_field_name(self.descriptor, FieldName::FieldNumber)?;
        let num: ExprLit = syn::parse_str(&self.descriptor.number().to_string())?;
        let doc_comment = format!(concat!(
            "/// The field number for the [`{0}`] field\n",
            "/// \n",
            "/// [`{0}`]: #method.{1}"
            ), name, names::get_field_name(self.descriptor, FieldName::Get)?);

        Ok(quote! {
            #doc_comment
            pub const #const_name: i32 = #num;
        })
    }

    fn generate_default_value(&self) -> Result<Option<TokenStream>> {
        if self.descriptor.label() != FieldLabel::Repeated 
        && !self.descriptor.field_type().is_message()
        && !self.descriptor.field_type().is_group() {
            let name = self.descriptor.name();
            let doc_comment = format!(concat!(
                "/// The default value for the [`{0}`] field\n",
                "/// \n",
                "/// [`{0}`]: #method.{1}"
                ), name, names::get_field_name(self.descriptor, FieldName::Get)?);
            let dn = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
            let scope = Some(Scope::from(self.descriptor.message().scope()));
            let dt = names::get_rust_type(self.descriptor, TypeKind::Default, scope, &self.options.crate_name, false)?;
            let dv = self.get_default_value(scope)?;
            Ok(Some(quote! {
                #doc_comment
                pub const #dn: #dt = #dv;
            }))
        } else {
            Ok(None)
        }
    }

    fn generate_accessors(&self) -> Result {
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let get = self.generate_get_accessor()?;
                let get_mut = self.generate_get_mut_accessor()?;

                Ok(quote! {
                    #get
                    #get_mut
                })
            },
            _ => {
                match self.descriptor.scope() {
                    FieldScope::Oneof(_) => {
                        let get = self.generate_get_accessor()?;
                        let get_mut = self.generate_get_mut_accessor()?;
                        let has_value = self.generate_has_value_accessor()?;
                        let set = self.generate_set_accessor()?;
                        let take = self.generate_take_accessor()?;
                        let clear = self.generate_clear_accessor()?;

                        Ok(quote! {
                            #get
                            #get_mut
                            #has_value
                            #set
                            #take
                            #clear
                        })
                    },
                    FieldScope::Message(_) => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto3 => {
                                let get = self.generate_get_accessor()?;
                                let get_mut = self.generate_get_mut_accessor()?;

                                Ok(quote! {
                                    #get
                                    #get_mut
                                })
                            },
                            Syntax::Proto2 => {
                                let get = self.generate_get_accessor()?;
                                let get_mut = self.generate_get_mut_accessor()?;
                                let has_value = self.generate_has_value_accessor()?;
                                let set = self.generate_set_accessor()?;
                                let take = self.generate_take_accessor()?;
                                let clear = self.generate_clear_accessor()?;

                                Ok(quote! {
                                    #get
                                    #get_mut
                                    #has_value
                                    #set
                                    #take
                                    #clear
                                })
                            },
                            _ => panic!("unknown syntax")
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    fn generate_get_accessor(&self) -> Result {
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let ref_doc_comment = self.generate_rustdoc_comments()?;
                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                let ng = names::get_field_name(self.descriptor, FieldName::Get)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::CollectionWrapped, Some(Scope::from(self.descriptor.message().scope())), &self.options.crate_name, false)?;
                Ok(quote! {
                    #ref_doc_comment
                    pub fn #ng(&self) -> &#t {
                        &self.#n
                    }
                })
            },
            _ => {
                match self.descriptor.scope() {
                    FieldScope::Oneof(_) => {
                        unimplemented!()
                    },
                    FieldScope::Message(_) => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto2 => {
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) | FieldType::Group(_) => {
                                        unimplemented!()
                                    },
                                    FieldType::String | FieldType::Bytes => {
                                        unimplemented!()
                                    },
                                    _ => {
                                        unimplemented!()
                                    }
                                }
                            },
                            Syntax::Proto3 => {
                                unimplemented!()
                            },
                            _ => panic!("unknown syntax")
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    fn generate_get_mut_accessor(&self) -> Result {
        unimplemented!()
    }

    fn generate_has_value_accessor(&self) -> Result {
        unimplemented!()
    }

    fn generate_set_accessor(&self) -> Result {
        unimplemented!()
    }

    fn generate_take_accessor(&self) -> Result {
        unimplemented!()
    }

    fn generate_clear_accessor(&self) -> Result {
        unimplemented!()
    }

    pub fn generate_codec(&self) -> Result<Option<TokenStream>> {
        if self.descriptor.label() == FieldLabel::Repeated {
            match self.descriptor.field_type() {
                FieldType::Message(m) if m.map_entry() => {
                    let scope = Scope::Message(self.descriptor.message());
                    let n = names::get_field_name(self.descriptor, FieldName::Codec)?;
                    let c = &self.options.crate_name;

                    let key_field = &m.fields()[0];
                    let value_field = &m.fields()[1];

                    let kt = names::get_rust_type(key_field, TypeKind::Base, Some(scope), c, false)?;
                    let vt = names::get_rust_type(value_field, TypeKind::Base, Some(scope), c, false)?;
                    let ke = self.with::<FieldDescriptor>(key_field).get_codec_new_expr()?;
                    let ve = self.with::<FieldDescriptor>(value_field).get_codec_new_expr()?;
                    let tg = syn::parse_str::<ExprLit>(&Tag::new(self.descriptor.number(), self.descriptor.wire_type()).to_string())?;

                    Ok(Some(quote!(static #n: #c::collections::MapCodec<#kt, #vt> = #c::collections::MapCodec::new(#ke, #ve, #tg))))
                }
                _ => {
                    let n = names::get_field_name(self.descriptor, FieldName::Codec)?;
                    let c = &self.options.crate_name;
                    let t = names::get_rust_type(self.descriptor, TypeKind::Base, Some(Scope::from(self.descriptor.message().scope())), c, false)?;
                    let e = self.get_codec_new_expr()?;
                    Ok(Some(quote!(static #n: #c::Codec<#t> = #e;)))
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn generate_reflector(&self) -> Result<Option<TokenStream>> {
        if let FieldScope::Oneof(_) = self.descriptor.scope() {
            Ok(Some(self.generate_verbose_field_reflector()?))
        } else
        if self.descriptor.label() == FieldLabel::Repeated {
            Ok(Some(self.generate_simple_field_reflector()?))
        } else
        if self.descriptor.file().syntax() == Syntax::Proto2 {
            Ok(Some(self.generate_verbose_field_reflector()?))
        } else
        if self.descriptor.proto().has_extendee() {
            Ok(None)
        } else {
            Ok(Some(self.generate_simple_field_reflector()?))
        }
    }

    pub fn generate_rustdoc_comments(&self) -> Result<Option<TokenStream>> {
        generate_rustdoc_comments(self.descriptor.source_code_info())
    }

    fn get_proto_type(&self) -> syn::Result<Ident> {
        syn::parse_str(
            match self.descriptor.field_type() {
                FieldType::Double => "double",
                FieldType::Float => "float",
                FieldType::Int64 => "int64",
                FieldType::Uint64 => "uint64",
                FieldType::Sint64 => "sint64",
                FieldType::Fixed64 => "fixed64",
                FieldType::Sfixed64 => "sfixed64",
                FieldType::Int32 => "int32",
                FieldType::Uint32 => "uint32",
                FieldType::Sint32 => "sint32",
                FieldType::Fixed32 => "fixed32",
                FieldType::Sfixed32 => "sfixed32",
                FieldType::Bool => "bool",
                FieldType::String => "string",
                FieldType::Bytes => "bytes",
                FieldType::Enum(_) => "enum_value",
                FieldType::Message(m) if m.proto().extension_range().len() != 0 => "extension_message",
                FieldType::Message(_) => "message",
                FieldType::Group(_) => "group",
            })
    }

    fn get_default_value(&self, scope: Option<Scope>) -> syn::Result<Expr> {
        let c_name = &self.options.crate_name;
        match self.descriptor.default_value() {
            DefaultValue::Invalid | DefaultValue::None => {
                match self.descriptor.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => unreachable!(),
                    FieldType::Enum(e) => {
                        match e.values().iter().find(|f| f.number() == 0) {
                            Some(defined) => {
                                let full_variant = names::get_enum_variant_path(defined, scope)?;
                                syn::parse2(quote!(#c_name::EnumValue::Defined(#full_variant)))
                            },
                            None => {
                                syn::parse2(quote!(#c_name::EnumValue::Undefined(0)))
                            }
                        }
                    },
                    FieldType::String => syn::parse_str(""),
                    FieldType::Bytes => syn::parse_str("&[]"),
                    FieldType::Bool => syn::parse_str("false"),
                    FieldType::Float | FieldType::Double => syn::parse_str("0.0"),
                    _ => syn::parse_str("0")
                }
            },
            DefaultValue::String(s) => syn::parse_str(&format!("\"{}\"", s.chars().flat_map(char::escape_default).collect::<String>())),
            DefaultValue::Bool(b) => syn::parse_str(&b.to_string()),
            DefaultValue::Double(d) => {
                if d.is_finite() {
                    syn::parse_str(&format!("{:?}", d))
                } else {
                    syn::parse_str(&format!("::std::{}::{}", {
                        if *self.descriptor.field_type() == FieldType::Float {
                            "f32"
                        } else {
                            "f64"
                        }
                    }, {
                        if d.is_nan() {
                            "NAN"
                        } else if d.is_infinite() {
                            "INFINITY"
                        } else {
                            "NEG_INFINITY"
                        }
                    }))
                }
            },
            DefaultValue::SignedInt(s) => syn::parse_str(&s.to_string()),
            DefaultValue::UnsignedInt(u) => syn::parse_str(&u.to_string()),
            DefaultValue::Enum(e) => {
                let full_variant = names::get_enum_variant_path(e, scope)?;
                syn::parse2(quote!(#c_name::EnumValue::Defined(#full_variant)))
            },
            DefaultValue::Bytes(b) => syn::parse_str(&format!("&{:?}", b))
        }
    }

    fn get_codec_new_expr(&self) -> syn::Result<Expr> {
        unimplemented!()
    }

    fn generate_verbose_field_reflector(&self) -> Result {
        let scope = 
            match self.descriptor.scope() {
                FieldScope::Message(m) => Scope::Message(m),
                FieldScope::File(f) => Scope::File(f),
                _ => unreachable!(),
            };
        let rn = names::get_field_name(self.descriptor, FieldName::Reflector)?;
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor.message(), Some(scope))?;
        let ft = names::get_rust_type(self.descriptor, TypeKind::Base, Some(scope), c, false)?;
        let n = 
            if self.descriptor.field_type().is_message() || self.descriptor.field_type().is_group() {
                names::get_field_name(self.descriptor, FieldName::Get)?
            } else {
                names::get_field_name(self.descriptor, FieldName::GetOption)?
            };
        let n_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
        let set_n = names::get_field_name(self.descriptor, FieldName::Set)?;
        let take_n = names::get_field_name(self.descriptor, FieldName::Take)?;
        let clear_n = names::get_field_name(self.descriptor, FieldName::Clear)?;

        Ok(quote! {
            pub(super) static #rn: #c::reflect::access::VerboseFieldAccessor<#t, #ft> =
                #c::reflect::access::VerboseFieldAccessor {
                    get_option: #t::#n,
                    get_mut: #t::#n_mut,
                    set: #t::#set_n,
                    take: #t::#take_n,
                    clear: #t::#clear_n,
                };
        })
    }

    fn generate_simple_field_reflector(&self) -> Result {
        let scope = 
            match self.descriptor.scope() {
                FieldScope::Message(m) => Scope::Message(m),
                FieldScope::File(f) => Scope::File(f),
                _ => unreachable!(),
            };
        let rn = names::get_field_name(self.descriptor, FieldName::Reflector)?;
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor.message(), Some(scope))?;
        let ft = names::get_rust_type(self.descriptor, TypeKind::Base, Some(scope), c, false)?;
        let n = names::get_field_name(self.descriptor, FieldName::Get)?;
        let n_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;

        Ok(quote! {
            pub(super) static #rn: #c::reflect::access::SimpleFieldAccessor<#t, #ft> = 
                #c::reflect::access::SimpleFieldAccessor {
                    get: #t::#n,
                    get_mut: #t::#n_mut,
                };
        })
    }
}

fn generate_rustdoc_comments(info: Option<&SourceCodeInfo>) -> Result<Option<TokenStream>> {
    if let Some(info) = info {
        if let Some(comments) = info.leading_comments().or(info.trailing_comments()) {
            let lines = comments.lines().map(|l| format!("/// {}", l));
            return Ok(Some(quote! {
                #(#lines
                )*
            }))
        }
    }
    Ok(None)
}