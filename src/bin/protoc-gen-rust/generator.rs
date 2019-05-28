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
use protrust::CodedMessage;
use protrust::io::{Tag, WireType};
use protrust::plugin::{names::{self, Scope, TypeKind, FieldName}, Context};
use protrust::reflect::{Descriptor, Syntax, FileDescriptor, MessageDescriptor, CompositeScope, EnumDescriptor, OneofDescriptor, FieldDescriptor, DefaultValue, FieldType, FieldLabel, FieldScope, SourceCodeInfo};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};
use std::process::{Stdio, Command};
use syn::{Token, Ident, Lit, Expr, ExprLit, Path};

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
        match self {
            OptionError::UnknownOption(k, v) => write!(f, "unknown option: ({}, {:#?})", k, v),
            OptionError::ParseError(v, err) => write!(f, "error parsing value \"{}\": {}", v, err)
        }
    }
}

impl Error for OptionError { }

impl Options {
    pub fn parse_from<'a>(params: Option<impl Iterator<Item = (&'a str, Option<&'a str>)>>) -> std::result::Result<Options, OptionError> {
        let mut options = Options {
            crate_name: syn::parse_str("::protrust").unwrap(),
            no_json: false,
            size_checks: false,
            no_reflection: false,
            external_modules: Vec::new()
        };
        if let Some(params) = params {
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
        let registry = self.generate_extension_registry()?;
        let pool = self.generate_descriptor_pool()?;
        let mods = self.generate_mods()?;

        let stream = quote! {
            #externals
            #registry
            #pool
            #mods
        };

        self.context.add_output_file("mod.rs".to_string(), self.format_stream(&stream)?);

        Ok(())
    }

    fn generate_externals_mod(&self) -> Result {
        let collected_input: Box<[&FileDescriptor]> = self.context.input_files().collect();
        let input_files_set: HashSet<_> = collected_input.iter().map(|r| *r).collect();
        let all_deps: HashSet<_> = flatten_deps(collected_input.iter().map(|r| *r)).collect();
        let external_deps = all_deps.difference(&input_files_set).map(|r| *r).collect::<Vec<&FileDescriptor>>();
        let external_files_to_rust_names = external_deps.iter().map(|f| names::get_rust_file_mod_name(f)).collect::<syn::Result<Vec<_>>>()?;
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

    fn generate_extension_registry(&self) -> Result {
        let c = &self.options.crate_name;
        let external_mods = &self.options.external_modules;
        let external_count = syn::parse_str::<Lit>(&self.options.external_modules.len().to_string())?;
        let extensions = {
            let mut extensions: fnv::FnvHashMap<_, _> = Default::default();
            for (msg, field) in self.context
                .input_files()
                .flat_map(|f| f.extensions())
                .chain(self.context.input_files().flat_map(|f| f.flatten_messages().flat_map(|m| m.extensions())))
                .map(|e| (e.message(), &**e)) {
                    extensions.entry(msg).or_insert_with(Vec::new).push(field);
            }
            extensions
        };
        let extension_pairs = 
            extensions
                .iter()
                .map(|(k, v)| {
                    let t = names::get_message_type_path(k, None)?;
                    let extension_paths = v.iter().map(|e| {
                        let n = names::get_field_name(e, FieldName::Extension)?;
                        match e.scope() {
                            FieldScope::Message(m) => {
                                let m = names::get_message_mod_path(m, None)?;
                                syn::parse2(quote!(#m::#n))
                            },
                            FieldScope::File(f) => {
                                let m = names::get_rust_file_mod_name(f)?;
                                syn::parse2(quote!(self::#m::#n))
                            },
                            _ => unreachable!()
                        }
                    }).collect::<syn::Result<Vec<Path>>>()?;
                    Ok(quote! {
                        (::std::any::TypeId::of::<#t>(), &[ #(&#extension_paths, )*])
                    })
                })
                .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            static mut EXTERNAL_REGISTRIES: ::std::option::Option<[&'static #c::ExtensionRegistry; #external_count]> = ::std::option::Option::None;
            static mut EXTENSIONS_REGISTRY: ::std::option::Option<#c::ExtensionRegistry> = ::std::option::Option::None;
            static EXTENSIONS_INIT: ::std::sync::Once = ::std::sync::Once::new();
            fn extensions_init() {
                unsafe {
                    self::EXTERNAL_REGISTRIES = ::std::option::Option::Some([
                        #(#external_mods::extensions(), )*
                    ]);
                    self::EXTENSIONS_REGISTRY = ::std::option::Option::Some(#c::ExtensionRegistry::new(self::EXTERNAL_REGISTRIES.as_ref().unwrap(), &[
                        #(#extension_pairs,
                        )*
                    ]));
                }
            }
            /// Gets the extension registry containing all the extensions contained in this generated code module
            pub fn extensions() -> &'static #c::ExtensionRegistry {
                unsafe {
                    self::EXTENSIONS_INIT.call_once(extensions_init);
                    self::EXTENSIONS_REGISTRY.as_ref().unwrap()
                }
            }
        })
    }

    fn generate_descriptor_pool(&self) -> Result {
        let c = &self.options.crate_name;
        let rc = std::iter::repeat(c); // c iterators for use in generating FILES
        let rcc = std::iter::repeat(c); // another c iterator
        let external_mods = &self.options.external_modules;
        let ec = syn::parse_str::<Lit>(&external_mods.len().to_string())?;
        let files = self.context.input_files().collect::<Vec<_>>();
        let fc = syn::parse_str::<Lit>(&files.len().to_string())?;
        let blobs = files
            .iter()
            .map(|f| get_blob_name(f))
            .collect::<syn::Result<Vec<_>>>()?;
        let info = files.iter().map(|f| SourceGenerator::new(*f, &self.options).generate_code_info()).collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            static mut EXTERNAL_DEPS: ::std::option::Option<[&'static #c::reflect::DescriptorPool<'static>; #ec]> = ::std::option::Option::None;
            static mut FILES: ::std::option::Option<[#c::descriptor::FileDescriptorProto; #fc]> = ::std::option::Option::None;
            static mut POOL: ::std::option::Option<#c::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
            static POOL_INIT: ::std::sync::Once = ::std::sync::Once::new();
            fn pool_init() {
                unsafe {
                    self::EXTERNAL_DEPS = ::std::option::Option::Some([
                        #(#external_mods::pool(), 
                        )*
                    ]);
                    self::FILES = ::std::option::Option::Some([
                        #(#rc::LiteMessage::read_new_from_input(&mut #rcc::io::CodedInput::new(&mut #blobs.as_ref()).with_registry(::std::option::Option::Some(self::extensions()))).expect("couldn't read file descriptor"),
                        )*
                    ]);
                    self::POOL = ::std::option::Option::Some(#c::reflect::DescriptorPool::build_from_generated_code(self::FILES.as_ref().unwrap().as_ref(), self::EXTERNAL_DEPS.as_ref().unwrap(), ::std::boxed::Box::new([
                        #(#info,
                        )*
                    ])));
                }
            }
            /// Gets the descriptor pool containing all the reflection information contained in this generated code module
            pub fn pool() -> &'static #c::reflect::DescriptorPool<'static> {
                unsafe {
                    self::POOL_INIT.call_once(pool_init);
                    self::POOL.as_ref().unwrap()
                }
            }
        })
    }

    fn generate_mods(&self) -> Result {
        let mods = self.context.input_files().map(|file| {
            let mod_name = names::get_rust_file_mod_name(file)?;
            let generator = SourceGenerator::new(file, &self.options);
            let blob = generator.generate_file_blob()?;
            let file_stream = generator.generate()?;
            Ok(quote! {
                #blob
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

    fn format_stream(&self, code: &TokenStream) -> Result<String> {
        let mut result = "/* generated by protoc-gen-rust */\n\n".to_string();

        let stream = code.to_string();

        match self.rustfmt_generated_string(&stream) {
            Ok(fmted) => {
                result += &fmted;
            },
            Err(err) => {
                eprintln!("Failed to run rustfmt: {} (non-fatal, continuing)", err);
                result += &stream;
            }
        }

        Ok(result)
    }

    fn rustfmt_generated_string(&self, source: &str) -> io::Result<String> {
        let rustfmt = 
            which::which("rustfmt")
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        let mut cmd = Command::new(rustfmt);
        cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());

        let mut child = cmd.spawn()?;
        let mut cstdin = child.stdin.take().unwrap();
        let mut cstdout = child.stdout.take().unwrap();

        let source = source.to_owned();

        let write_thread = std::thread::spawn(move || {
            let _ = cstdin.write_all(source.as_bytes());
            source
        });

        let mut output = Vec::new();
        std::io::copy(&mut cstdout, &mut output)?;

        let status = child.wait()?;
        let source = write_thread.join().unwrap();

        match String::from_utf8(output) {
            Ok(bindings) => {
                match status.code() {
                    Some(0) => Ok(bindings),
                    Some(2) => Err(io::Error::new(io::ErrorKind::Other, "rustfmt parsing errors".to_owned())),
                    Some(3) => {
                        eprintln!("Rustfmt could not format some lines.");
                        Ok(bindings)
                    },
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Internal rustfmt error".to_owned()))
                }
            },
            _ => Ok(source)
        }
    }
}

fn flatten_deps<'a, T>(files: T) -> Box<(dyn Iterator<Item = &'a FileDescriptor> + 'a)> 
    where 
        T: 'a + Iterator<Item = &'a FileDescriptor> {
    Box::new(files
        .flat_map(|f| f.dependencies()
            .iter()
            .map(|r| &**r)
            .chain(flatten_public_deps(f)))
        .unique())
}

fn flatten_public_deps<'a>(f: &'a FileDescriptor) -> Box<(dyn Iterator<Item = &'a FileDescriptor> + 'a)> {
    flatten_deps(f.public_dependencies().iter().map(|r| &**r))
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
        let file_str = self.descriptor.name();

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
            pub fn file() -> &'static #crt::reflect::FileDescriptor {
                super::pool().find_file_by_name(#file_str).unwrap()
            }

            #(#substreams
            )*
        };
        Ok(stream)
    }

    pub fn generate_file_blob(&self) -> Result {
        let name = get_blob_name(self.descriptor)?;
        let mut proto = self.descriptor.proto().clone();
        proto.clear_source_code_info();
        let vec = proto.write_to_vec().unwrap();
        let bytes = vec.iter().map(|b| syn::parse_str::<syn::Lit>(&b.to_string()).unwrap());
        Ok(quote! {
            static #name: &'static [u8] = &[ #(#bytes,)* ];
        })
    }

    pub fn generate_code_info(&self) -> Result {
        let c = &self.options.crate_name;
        let structs = 
            if self.descriptor.messages().len() != 0 {
                let info = self.descriptor
                    .messages()
                    .iter()
                    .filter(|m| !m.map_entry())
                    .map(|m| self.with::<MessageDescriptor>(m).generate_struct_info())
                    .collect::<Result<Vec<_>>>()?;
                quote! {
                    ::std::option::Option::Some(::std::boxed::Box::new([
                        #(#info,
                        )*
                    ]))
                }
            } else {
                quote!(::std::option::Option::None)
            };
        let fields =
            if self.descriptor.extensions().len() != 0 {
                let info = self.descriptor
                    .extensions()
                    .iter()
                    .map(|f| self.with::<FieldDescriptor>(f).generate_accessor_ref())
                    .collect::<Result<Vec<_>>>()?;
                quote! {
                    ::std::option::Option::Some(::std::boxed::Box::new([
                        #(#info,
                        )*
                    ]))
                }
            } else {
                quote!(::std::option::Option::None)
            };
        
        Ok(quote! {
            #c::reflect::GeneratedCodeInfo {
                structs: #structs,
                fields: #fields,
            }
        })
    }
}

fn get_blob_name(f: &FileDescriptor) -> syn::Result<Ident> {
    syn::parse_str::<Ident>(&(names::get_rust_file_mod_name(f)?.to_string().to_uppercase() + "_BINARY"))
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
                Some(quote!(extensions: #c::ExtensionSet<Self>,))
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
                #extensions
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

    pub fn generate_struct_info(&self) -> Result {
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor, None)?;
        let structs = 
            if self.descriptor.messages().len() != 0 {
                let info = self.descriptor
                    .messages()
                    .iter()
                    .filter(|m| !m.map_entry())
                    .map(|m| self.with::<MessageDescriptor>(m).generate_struct_info())
                    .collect::<Result<Vec<_>>>()?;
                quote! {
                    ::std::option::Option::Some(::std::boxed::Box::new([
                        #(#info,
                        )*
                    ]))
                }
            } else {
                quote!(::std::option::Option::None)
            };
        let fields =
            if self.descriptor.fields().len() != 0 || self.descriptor.extensions().len() != 0 {
                let info = self.descriptor
                    .fields()
                    .iter()
                    .chain(self.descriptor.extensions().iter())
                    .map(|f| self.with::<FieldDescriptor>(f).generate_accessor_ref())
                    .collect::<Result<Vec<_>>>()?;
                quote! {
                    ::std::option::Option::Some(::std::boxed::Box::new([
                        #(#info,
                        )*
                    ]))
                }
            } else {
                quote!(::std::option::Option::None)
            };
        Ok(quote! {
            #c::reflect::GeneratedStructInfo {
                new: || ::std::boxed::Box::new(<#t as #c::LiteMessage>::new()),
                structs: #structs,
                fields: #fields,
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
                .map(|t| syn::parse_str::<Lit>(&t.to_string()))
                .transpose()?
                .map(|id| quote!(#id => break,))
        };
        let unknown_field_merge = 
            if self.descriptor.proto().extension_range().len() != 0 {
                quote! {
                    if !self.extensions.merge_from(input)? {
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
                Some(quote!(self.extensions.merge(&other.extensions);))
            } else {
                None
            };
        Ok(quote! {
            fn merge(&mut self, other: &Self) {
                #(#field_merge
                )*
                #extensions_merge
                self.unknown_fields.merge(&other.unknown_fields);
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
                    let num = syn::parse_str::<Expr>(&f.number().to_string())?;
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
                    let num = syn::parse_str::<Expr>(&f.number().to_string())?;
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
        let t = names::get_oneof_type(self.descriptor)?;
        let ref_doc_comment = create_rustdoc_comment(&format!(concat!(
            " Gets a shared reference to the [`{0}`] oneof field\n",
            " \n",
            " [`{0}`]: enum.{1}.html"
            ), self.descriptor.name(), t))?;
        let mut_doc_comment = create_rustdoc_comment(&format!(concat!(
            " Gets a unique reference to the [`{0}`] oneof field\n",
            " \n",
            " [`{0}`]: enum.{1}.html"
            ), self.descriptor.name(), t))?;
        let f: Ident = syn::parse_str(self.descriptor.name())?;
        let nm: Ident = syn::parse_str(&format!("{}_mut", self.descriptor.name()))?;
        let m = names::get_message_mod(self.descriptor.message())?;

        Ok(quote! {
            #ref_doc_comment
            pub fn #f(&self) -> &self::#m::#t {
                &self.#f
            }
            #mut_doc_comment
            pub fn #nm(&mut self) -> &mut self::#m::#t {
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
        if (self.descriptor.file().syntax() == Syntax::Proto2 || self.descriptor.field_type().is_message()) && self.descriptor.label() != FieldLabel::Repeated {
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
                match self.descriptor.scope() {
                    FieldScope::Oneof(_) => {
                        match self.descriptor.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                let c = &self.options.crate_name;
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = &other.#get() {
                                        #c::LiteMessage::merge(self.#get_mut(), #n);
                                    }
                                })
                            }
                            FieldType::Bytes | FieldType::String => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = other.#get() {
                                        self.#set(::std::clone::Clone::clone(#n));
                                    }
                                })
                            }
                            _ => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = other.#get() {
                                        self.#set(*#n);
                                    }
                                })
                            }
                        }
                    }
                    FieldScope::Message(_) => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto3 => {
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) => {
                                        let c = &self.options.crate_name;
                                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                        let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = &other.#get() {
                                                #c::LiteMessage::merge(self.#get_mut(), #n);
                                            }
                                        })
                                    }
                                    FieldType::Bytes | FieldType::String => {
                                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                        let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                        Ok(quote! {
                                            if other.#get.len() != 0 {
                                                *self.#get_mut() = ::std::clone::Clone::clone(other.#get());
                                            }
                                        })
                                    }
                                    _ => {
                                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                        let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                        let default_value = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                        Ok(quote! {
                                            if *other.#get() != Self::#default_value {
                                                *self.#get_mut() = *other.#get();
                                            }
                                        })
                                    }
                                }
                            }
                            Syntax::Proto2 => {
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) | FieldType::Group(_) => {
                                        let c = &self.options.crate_name;
                                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                        let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = &other.#get() {
                                                #c::LiteMessage::merge(self.#get_mut(), #n);
                                            }
                                        })
                                    }
                                    FieldType::Bytes | FieldType::String => {
                                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                        let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                        let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = other.#get_option() {
                                                self.#set(::std::clone::Clone::clone(#n));
                                            }
                                        })
                                    }
                                    _ => {
                                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                        let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                        let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = other.#get_option() {
                                                self.#set(*#n);
                                            }
                                        })
                                    }
                                }
                            }
                            _ => unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn generate_merge_arm(&self) -> Result {
        let tags: TokenStream = {
            let real_tag = syn::parse_str::<ExprLit>(&Tag::new(self.descriptor.number(), self.descriptor.wire_type()).to_string())?;
            if self.descriptor.field_type().wire_type().is_packable() && self.descriptor.wire_type() != WireType::LengthDelimited {
                let length_tag = syn::parse_str::<ExprLit>(&Tag::new(self.descriptor.number(), WireType::LengthDelimited).to_string())?;
                quote!(#real_tag | #length_tag)
            } else {
                quote!(#real_tag)
            }
        };
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                let codec = names::get_field_name(self.descriptor, FieldName::Codec)?;
                let m = names::get_message_mod(self.descriptor.message())?;
                Ok(quote!(#tags => self.#n.add_entries(input, &self::#m::#codec)?))
            },
            _ => {
                match self.descriptor.file().syntax() {
                    Syntax::Proto3 => {
                        match self.descriptor.field_type() {
                            FieldType::Message(_) => {
                                let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                let t = self.get_proto_type(Some("read"))?;
                                Ok(quote!(#tags => input.#t(self.#get_mut())?))
                            },
                            _ => {
                                let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                let t = self.get_proto_type(Some("read"))?;
                                Ok(quote!(#tags => *self.#get_mut() = input.#t()?))
                            }
                        }
                    },
                    Syntax::Proto2 => {
                        match self.descriptor.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                                let t = self.get_proto_type(Some("read"))?;
                                Ok(quote!(#tags => input.#t(self.#get_mut())?))
                            },
                            _ => {
                                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                                let t = self.get_proto_type(Some("read"))?;
                                Ok(quote!(#tags => self.#set(input.#t()?)))
                            }
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn generate_size_calculator(&self) -> Result {
        let tag_size: ExprLit = {
            let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type()).get();
            syn::parse_str(&protrust::io::sizes::uint32(tag).to_string())?
        };
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                let codec = names::get_field_name(self.descriptor, FieldName::Codec)?;
                let m = names::get_message_mod(self.descriptor.message())?;
                if self.options.size_checks {
                    Ok(quote!(size = size.checked_add(self.#get().calculate_size(&self::#m::#codec)?)?;))
                } else {
                    Ok(quote!(size += self.#get().calculate_size(&self::#m::#codec);))
                }
            },
            _ => {
                match self.descriptor.scope() {
                    FieldScope::Oneof(_) => {
                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let t = self.get_proto_type(None)?;
                        let c = &self.options.crate_name;
                        if self.options.size_checks {
                            match self.descriptor.field_type() {
                                FieldType::Message(_) => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size = size.checked_add(#tag_size)?;
                                            size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                        }
                                    })
                                },
                                FieldType::Group(_) => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    let end_tag_size: ExprLit = {
                                        let tag = Tag::new(self.descriptor.number(), WireType::EndGroup).get();
                                        syn::parse_str(&protrust::io::sizes::uint32(tag).to_string())?
                                    };
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size = size.checked_add(#tag_size)?;
                                            size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                            size = size.checked_add(#end_tag_size)?;
                                        }
                                    })
                                },
                                FieldType::String | FieldType::Bytes => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size = size.checked_add(#tag_size)?;
                                            size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                        }
                                    })
                                },
                                _ => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size = size.checked_add(#tag_size)?;
                                            size = size.checked_add(#c::io::sizes::#t(*#n))?;
                                        }
                                    })
                                }
                            }
                        } else {
                            match self.descriptor.field_type() {
                                FieldType::Message(_) => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size += #tag_size;
                                            size += #c::io::sizes::#t(#n);
                                        }
                                    })
                                },
                                FieldType::Group(_) => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    let end_tag_size: ExprLit = {
                                        let tag = Tag::new(self.descriptor.number(), WireType::EndGroup).get();
                                        syn::parse_str(&protrust::io::sizes::uint32(tag).to_string())?
                                    };
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size += #tag_size;
                                            size += #c::io::sizes::#t(#n);
                                            size += #end_tag_size;
                                        }
                                    })
                                },
                                FieldType::String | FieldType::Bytes => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size += #tag_size;
                                            size += #c::io::sizes::#t(#n);
                                        }
                                    })
                                },
                                _ => {
                                    let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                    Ok(quote! {
                                        if let ::std::option::Option::Some(#n) = self.#get() {
                                            size += #tag_size;
                                            size += #c::io::sizes::#t(*#n);
                                        }
                                    })
                                }
                            }
                        }
                    }
                    FieldScope::Message(_) => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto3 => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let t = self.get_proto_type(None)?;
                                let c = &self.options.crate_name;
                                if self.options.size_checks {
                                    match self.descriptor.field_type() {
                                        FieldType::Message(_) => {
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get() {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                                }
                                            })
                                        },
                                        FieldType::String | FieldType::Bytes => {
                                            let default_field = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                            Ok(quote! {
                                                if **self.#get() != *Self::#default_field {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(self.#get())?)?;
                                                }
                                            })
                                        },
                                        _ => {
                                            let default_field = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                            Ok(quote! {
                                                if *self.#get() != Self::#default_field {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(*self.#get()))?;
                                                }
                                            })
                                        }
                                    }
                                } else {
                                    match self.descriptor.field_type() {
                                        FieldType::Message(_) => {
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get() {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(#n);
                                                }
                                            })
                                        },
                                        FieldType::String | FieldType::Bytes => {
                                            let default_field = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                            Ok(quote! {
                                                if **self.#get() != *Self::#default_field {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(self.#get());
                                                }
                                            })
                                        }
                                        _ => {
                                            let default_field = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                            Ok(quote! {
                                                if *self.#get() != Self::#default_field {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(*self.#get());
                                                }
                                            })
                                        }
                                    }
                                }
                            },
                            Syntax::Proto2 => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let t = self.get_proto_type(None)?;
                                let c = &self.options.crate_name;
                                if self.options.size_checks {
                                    match self.descriptor.field_type() {
                                        FieldType::Message(_) => {
                                            let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get() {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                                }
                                            })
                                        },
                                        FieldType::Group(_) => {
                                            let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                            let end_tag_size: ExprLit = {
                                                let tag = Tag::new(self.descriptor.number(), WireType::EndGroup).get();
                                                syn::parse_str(&protrust::io::sizes::uint32(tag).to_string())?
                                            };
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get() {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                                    size = size.checked_add(#end_tag_size)?;
                                                }
                                            })
                                        },
                                        FieldType::String | FieldType::Bytes => {
                                            let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get_option() {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(#n)?)?;
                                                }
                                            })
                                        },
                                        _ => {
                                            let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get_option() {
                                                    size = size.checked_add(#tag_size)?;
                                                    size = size.checked_add(#c::io::sizes::#t(*#n))?;
                                                }
                                            })
                                        }
                                    }
                                } else {
                                    match self.descriptor.field_type() {
                                        FieldType::Message(_) => {
                                            let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get() {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(#n);
                                                }
                                            })
                                        },
                                        FieldType::Group(_) => {
                                            let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                            let end_tag_size: ExprLit = {
                                                let tag = Tag::new(self.descriptor.number(), WireType::EndGroup).get();
                                                syn::parse_str(&protrust::io::sizes::uint32(tag).to_string())?
                                            };
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get() {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(#n);
                                                    size += #end_tag_size;
                                                }
                                            })
                                        },
                                        FieldType::String | FieldType::Bytes => {
                                            let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get_option() {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(#n);
                                                }
                                            })
                                        },
                                        _ => {
                                            let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                            Ok(quote! {
                                                if let ::std::option::Option::Some(#n) = self.#get_option() {
                                                    size += #tag_size;
                                                    size += #c::io::sizes::#t(*#n);
                                                }
                                            })
                                        }
                                    }
                                }
                            },
                            _ => unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
                
            }
        }
    }

    pub fn generate_value_writer(&self) -> Result {
        let tag_bytes: syn::Expr = {
            let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type()).get().to_le();
            let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
            let mut output = protrust::io::CodedOutput::new(&mut bytes);
            output.write_raw_tag(tag).unwrap();
            syn::parse_str(&format!("{:?}", bytes))?
        };
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                let codec = names::get_field_name(self.descriptor, FieldName::Codec)?;
                let m = names::get_message_mod(self.descriptor.message())?;
                Ok(quote!(self.#get().write_to(output, &self::#m::#codec)?;))
            },
            _ => {
                match self.descriptor.scope() {
                    FieldScope::Oneof(_) => {
                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let t = self.get_proto_type(Some("write"))?;
                        match self.descriptor.field_type() {
                            FieldType::Message(_) => {
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = self.#get() {
                                        output.write_raw_tag_bytes(&#tag_bytes)?;
                                        output.#t(#n)?;
                                    }
                                })
                            }
                            FieldType::Group(_) => {
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let end_tag_size: syn::Expr = {
                                    let tag = Tag::new(self.descriptor.number(), WireType::EndGroup).get().to_le();
                                    let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
                                    let mut output = protrust::io::CodedOutput::new(&mut bytes);
                                    output.write_raw_tag(tag).unwrap();
                                    syn::parse_str(&format!("{:?}", bytes))?
                                };
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = self.#get() {
                                        output.write_raw_tag_bytes(&#tag_bytes)?;
                                        output.#t(#n)?;
                                        output.write_raw_tag_bytes(&#end_tag_size)?;
                                    }
                                })
                            }
                            FieldType::String | FieldType::Bytes => {
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = self.#get() {
                                        output.write_raw_tag_bytes(&#tag_bytes)?;
                                        output.#t(#n)?;
                                    }
                                })
                            }
                            _ => {
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                Ok(quote! {
                                    if let ::std::option::Option::Some(#n) = self.#get() {
                                        output.write_raw_tag_bytes(&#tag_bytes)?;
                                        output.#t(*#n)?;
                                    }
                                })
                            }
                        }
                    }
                    FieldScope::Message(_) => {
                        match self.descriptor.file().syntax() {
                            Syntax::Proto3 => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                let t = self.get_proto_type(Some("write"))?;
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) => {
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = self.#get() {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(#n)?;
                                            }
                                        })
                                    },
                                    FieldType::String | FieldType::Bytes => {
                                        let default_field = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                        Ok(quote! {
                                            if **self.#get() != *Self::#default_field {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(self.#get())?;
                                            }
                                        })
                                    }
                                    _ => {
                                        let default_field = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                        Ok(quote! {
                                            if *self.#get() != Self::#default_field {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(*self.#get())?;
                                            }
                                        })
                                    }
                                }
                            },
                            Syntax::Proto2 => {
                                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                                let t = self.get_proto_type(Some("write"))?;
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) => {
                                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = self.#get() {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(#n)?;
                                            }
                                        })
                                    }
                                    FieldType::Group(_) => {
                                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                                        let end_tag_size: syn::Expr = {
                                            let tag = Tag::new(self.descriptor.number(), WireType::EndGroup).get().to_le();
                                            let mut bytes = Vec::with_capacity(protrust::io::sizes::uint32(tag) as usize);
                                            let mut output = protrust::io::CodedOutput::new(&mut bytes);
                                            output.write_raw_tag(tag).unwrap();
                                            syn::parse_str(&format!("{:?}", bytes))?
                                        };
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = self.#get() {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(#n)?;
                                                output.write_raw_tag_bytes(&#end_tag_size)?;
                                            }
                                        })
                                    }
                                    FieldType::String | FieldType::Bytes => {
                                        let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = self.#get_option() {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(#n)?;
                                            }
                                        })
                                    }
                                    _ => {
                                        let get_option = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                        Ok(quote! {
                                            if let ::std::option::Option::Some(#n) = self.#get_option() {
                                                output.write_raw_tag_bytes(&#tag_bytes)?;
                                                output.#t(*#n)?;
                                            }
                                        })
                                    }
                                }
                            },
                            _ => unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
                
            }
        }
    }

    pub fn generate_initialized_check(&self) -> Result<Option<TokenStream>> {
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                if let FieldType::Message(m) = self.descriptor.field_type() {
                    if !m.map_entry() || m.fields()[1].field_type().is_message() {
                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                        Ok(Some(quote! {
                            if !self.#get.is_initialized() {
                                return false;
                            }
                        }))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            },
            FieldLabel::Required => {
                match self.descriptor.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        let c = &self.options.crate_name;
                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                        Ok(Some(quote! {
                            if let ::std::option::Option::Some(#n) = self.#get() {
                                if !#c::CodedMessage::is_initialized(#n) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }))
                    },
                    _ => {
                        let has = names::get_field_name(self.descriptor, FieldName::HasValue)?;
                        Ok(Some(quote! {
                            if !self.#has() {
                                return false;
                            }
                        }))
                    }
                }
            },
            FieldLabel::Optional => {
                match self.descriptor.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        let c = &self.options.crate_name;
                        let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                        Ok(Some(quote! {
                            if let ::std::option::Option::Some(#n) = self.#get() {
                                if !#c::CodedMessage::is_initialized(#n) {
                                    return false;
                                }
                            }
                        }))
                    },
                    _ => Ok(None)
                }
            }
        }
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
            if self.descriptor.label() == FieldLabel::Repeated {
                if self.descriptor.field_type().is_group() {
                    let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type());
                    let lit: ExprLit = syn::parse_str(&tag.to_string())?;

                    let end_tag = Tag::new(self.descriptor.number(), WireType::EndGroup);
                    let end_lit: ExprLit = syn::parse_str(&end_tag.to_string())?;
                    quote!(#lit, #end_lit)
                } else {
                    let tag = Tag::new(self.descriptor.number(), self.descriptor.wire_type());
                    let lit: ExprLit = syn::parse_str(&tag.to_string())?;
                    quote!(#lit)
                }
            } else {
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
                }
            };
        let proto_type = self.get_proto_type(None)?;
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
        let doc_comment = create_rustdoc_comment(&format!(concat!(
            "The field number for the [`{0}`] field\n",
            "\n",
            "[`{0}`]: #method.{1}"
            ), name, names::get_field_name(self.descriptor, FieldName::Get)?))?;

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
            let doc_comment = create_rustdoc_comment(&format!(concat!(
                "The default value for the [`{0}`] field\n",
                "\n",
                "[`{0}`]: #method.{1}"
                ), name, names::get_field_name(self.descriptor, FieldName::Get)?))?;
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

    fn generate_accessor_ref(&self) -> Result {
        let c = &self.options.crate_name;
        let f = 
            if self.descriptor.proto().has_extendee() {
                names::get_field_name(self.descriptor, FieldName::Extension)?
            } else {
                names::get_field_name(self.descriptor, FieldName::Reflector)?
            };
        let p =
            match self.descriptor.scope() {
                FieldScope::Message(m) => names::get_message_mod_path(m, None)?,
                FieldScope::Oneof(o) => names::get_message_mod_path(o.message(), None)?,
                FieldScope::File(f) => {
                    let f = names::get_rust_file_mod_name(f)?;
                    syn::parse2(quote!(self::#f))?
                }
            };
        if self.descriptor.label() == FieldLabel::Repeated {
            match self.descriptor.field_type() {
                FieldType::Message(m) if m.map_entry() => {
                    Ok(quote!(#c::reflect::access::FieldAccessor::Map(&#p::#f)))
                }
                _ => {
                    Ok(quote!(#c::reflect::access::FieldAccessor::Repeated(&#p::#f)))
                }
            }
        } else {
            Ok(quote!(#c::reflect::access::FieldAccessor::Single(&#p::#f)))
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
                                if self.descriptor.field_type().is_message() {
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
                                } else {
                                    let get = self.generate_get_accessor()?;
                                    let get_mut = self.generate_get_mut_accessor()?;

                                    Ok(quote! {
                                        #get
                                        #get_mut
                                    })
                                }
                            },
                            Syntax::Proto2 => {
                                let get = self.generate_get_accessor()?;
                                let get_option = self.generate_get_option_accessor()?;
                                let get_mut = self.generate_get_mut_accessor()?;
                                let has_value = self.generate_has_value_accessor()?;
                                let set = self.generate_set_accessor()?;
                                let take = self.generate_take_accessor()?;
                                let clear = self.generate_clear_accessor()?;

                                Ok(quote! {
                                    #get
                                    #get_option
                                    #get_mut
                                    #has_value
                                    #set
                                    #take
                                    #clear
                                })
                            },
                            _ => unreachable!()
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    fn generate_get_accessor(&self) -> Result {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        let docs = self.generate_rustdoc_comments()?;
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                let ng = names::get_field_name(self.descriptor, FieldName::Get)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::CollectionWrapped, scope, &self.options.crate_name, false)?;
                Ok(quote! {
                    #docs
                    pub fn #ng(&self) -> &#t {
                        &self.#n
                    }
                })
            },
            _ => {
                match self.descriptor.scope() {
                    FieldScope::Oneof(o) => {
                        let ot = names::get_oneof_type_path(o, scope)?;
                        let c = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
                        let f = names::get_oneof_field(o)?;
                        let n = names::get_field_name(self.descriptor, FieldName::Get)?;
                        let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                        if self.descriptor.field_type().is_message() {
                            Ok(quote! {
                                #docs
                                pub fn #n(&self) -> ::std::option::Option<&#t> {
                                    match &self.#f {
                                        #ot::#c(#f) => ::std::option::Option::Some(&**#f),
                                        _ => ::std::option::Option::None
                                    }
                                }
                            })
                        } else {
                            Ok(quote! {
                                #docs
                                pub fn #n(&self) -> ::std::option::Option<&#t> {
                                    match &self.#f {
                                        #ot::#c(#f) => ::std::option::Option::Some(#f),
                                        _ => ::std::option::Option::None
                                    }
                                }
                            })
                        }
                    },
                    FieldScope::Message(_) => {
                        let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let get = names::get_field_name(self.descriptor, FieldName::Get)?;
                        match self.descriptor.file().syntax() {
                            Syntax::Proto2 => {
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) | FieldType::Group(_) => {
                                        let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                                        Ok(quote! {
                                            #docs
                                            pub fn #get(&self) -> ::std::option::Option<&#t> {
                                                self.#f.as_ref().map(|v| &**v)
                                            }
                                        })
                                    },
                                    FieldType::String | FieldType::Bytes => {
                                        let default = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                        let t = names::get_rust_type(self.descriptor, TypeKind::DefaultNoRef, scope, &self.options.crate_name, false)?;
                                        Ok(quote! {
                                            #docs
                                            pub fn #get(&self) -> &#t {
                                                self.#f.as_ref().map(|v| &**v).unwrap_or(Self::#default)
                                            }
                                        })
                                    },
                                    _ => {
                                        let default = names::get_field_name(self.descriptor, FieldName::DefaultValue)?;
                                        let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                                        Ok(quote! {
                                            #docs
                                            pub fn #get(&self) -> #t {
                                                self.#f.unwrap_or(Self::#default)
                                            }
                                        })
                                    }
                                }
                            },
                            Syntax::Proto3 => {
                                let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) => {
                                        Ok(quote! {
                                            #docs
                                            pub fn #get(&self) -> ::std::option::Option<&#t> {
                                                self.#f.as_ref().map(|v| &**v)
                                            }
                                        })
                                    },
                                    _ => {
                                        Ok(quote! {
                                            #docs
                                            pub fn #get(&self) -> &#t {
                                                &self.#f
                                            }
                                        })
                                    },
                                }
                            },
                            _ => unreachable!()
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    fn generate_get_mut_accessor(&self) -> Result {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        let docs = create_rustdoc_comment(&format!(concat!(
            "Gets a unique reference to the [`{0}`] field\n",
            "\n",
            "[`{0}`]: #method.{1}"
            ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
        match self.descriptor.label() {
            FieldLabel::Repeated => {
                let n = names::get_field_name(self.descriptor, FieldName::Field)?;
                let ng = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::CollectionWrapped, scope, &self.options.crate_name, false)?;
                Ok(quote! {
                    #docs
                    pub fn #ng(&mut self) -> &mut #t {
                        &mut self.#n
                    }
                })
            },
            _ => {
                match self.descriptor.scope() {
                    FieldScope::Oneof(o) => {
                        let ot = names::get_oneof_type_path(o, scope)?;
                        let c = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
                        let f = names::get_oneof_field(o)?;
                        let n = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                        let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                        if self.descriptor.field_type().is_message() {
                            Ok(quote! {
                                #docs
                                pub fn #n(&mut self) -> &mut #t {
                                    match self.#f {
                                        #ot::#c(_) => (),
                                        _ => self.#f = #ot::#c(::std::default::Default::default()),
                                    }

                                    match self.#f {
                                        #ot::#c(ref mut #f) => &mut **#f,
                                        _ => unsafe { std::hint::unreachable_unchecked() },
                                    }
                                }
                            })
                        } else {
                            Ok(quote! {
                                #docs
                                pub fn #n(&mut self) -> &mut #t {
                                    match self.#f {
                                        #ot::#c(_) => (),
                                        _ => self.#f = #ot::#c(::std::default::Default::default()),
                                    }

                                    match self.#f {
                                        #ot::#c(ref mut #f) => #f,
                                        _ => unsafe { std::hint::unreachable_unchecked() },
                                    }
                                }
                            })
                        }
                    },
                    FieldScope::Message(_) => {
                        let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                        let get_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
                        let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                        match self.descriptor.file().syntax() {
                            Syntax::Proto2 => {
                                Ok(quote! {
                                    #docs
                                    pub fn #get_mut(&mut self) -> &mut #t {
                                        self.#f.get_or_insert_with(::std::default::Default::default)
                                    }
                                })
                            },
                            Syntax::Proto3 => {
                                match self.descriptor.field_type() {
                                    FieldType::Message(_) => {
                                        Ok(quote! {
                                            #docs
                                            pub fn #get_mut(&mut self) -> &mut #t {
                                                self.#f.get_or_insert_with(::std::default::Default::default)
                                            }
                                        })
                                    },
                                    _ => {
                                        Ok(quote! {
                                            #docs
                                            pub fn #get_mut(&mut self) -> &mut #t {
                                                &mut self.#f
                                            }
                                        })
                                    }
                                }
                            },
                            _ => unreachable!()
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    fn generate_get_option_accessor(&self) -> Result<Option<TokenStream>> {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        match self.descriptor.scope() {
            FieldScope::Message(_) => {
                match self.descriptor.file().syntax() {
                    Syntax::Proto2 => {
                        match self.descriptor.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => { Ok(None) },
                            _ => {
                                let docs = create_rustdoc_comment(&format!(concat!(
                                    "Gets an option indicating the presence of the [`{0}`] field\n",
                                    "\n",
                                    "[`{0}`]: #method.{1}"
                                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                                let op = names::get_field_name(self.descriptor, FieldName::GetOption)?;
                                let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                                let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                                Ok(Some(quote! {
                                    #docs
                                    pub fn #op(&self) -> ::std::option::Option<&#t> {
                                        self.#f.as_ref()
                                    }
                                }))
                            }
                        }
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    fn generate_has_value_accessor(&self) -> Result {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        match self.descriptor.scope() {
            FieldScope::Oneof(o) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Gets a bool indicating the presence of the [`{0}`] field\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let h = names::get_field_name(self.descriptor, FieldName::HasValue)?;
                let f = names::get_oneof_field(o)?;
                let o = names::get_oneof_type_path(o, scope)?;
                let c = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
                Ok(quote! {
                    #docs
                    pub fn #h(&self) -> bool {
                        match self.#f {
                            #o::#c(_) => true,
                            _ => false
                        }
                    }
                })
            },
            FieldScope::Message(_) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Gets a bool indicating the presence of the [`{0}`] field\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let h = names::get_field_name(self.descriptor, FieldName::HasValue)?;
                let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                match self.descriptor.file().syntax() {
                    Syntax::Proto2 => {
                        Ok(quote! {
                            #docs
                            pub fn #h(&self) -> bool {
                                self.#f.is_some()
                            }
                        })
                    },
                    Syntax::Proto3 => {
                        Ok(quote! {
                            #docs
                            pub fn #h(&self) -> bool {
                                self.#f.is_some()
                            }
                        })
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    fn generate_set_accessor(&self) -> Result {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        match self.descriptor.scope() {
            FieldScope::Oneof(o) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Sets the value of the [`{0}`] field\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                let f = names::get_oneof_field(o)?;
                let o = names::get_oneof_type_path(o, scope)?;
                let c = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
                Ok(quote! {
                    #docs
                    pub fn #set(&mut self, value: #t) {
                        self.#f = #o::#c(::std::convert::From::from(value))
                    }
                })
            },
            FieldScope::Message(_) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Sets the value of the [`{0}`] field\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let set = names::get_field_name(self.descriptor, FieldName::Set)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                match self.descriptor.file().syntax() {
                    Syntax::Proto2 => {
                        Ok(quote! {
                            #docs
                            pub fn #set(&mut self, value: #t) {
                                self.#f = ::std::option::Option::Some(::std::convert::From::from(value))
                            }
                        })
                    },
                    Syntax::Proto3 if self.descriptor.field_type().is_message() => {
                        Ok(quote! {
                            #docs
                            pub fn #set(&mut self, value: #t) {
                                self.#f = ::std::option::Option::Some(::std::convert::From::from(value))
                            }
                        })
                    }
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    fn generate_take_accessor(&self) -> Result {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        match self.descriptor.scope() {
            FieldScope::Oneof(o) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Takes the value out of the [`{0}`] field, leaving an empty field in its place\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let take = names::get_field_name(self.descriptor, FieldName::Take)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                let f = names::get_oneof_field(o)?;
                let o = names::get_oneof_type_path(o, scope)?;
                let c = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
                match self.descriptor.field_type() {
                    FieldType::Message(_) | FieldType::Group(_) => {
                        Ok(quote! {
                            #docs
                            pub fn #take(&mut self) -> ::std::option::Option<#t> {
                                let #f = ::std::mem::replace(&mut self.#f, #o::None);
                                match #f {
                                    #o::#c(#f) => {
                                        ::std::option::Option::Some(*#f)
                                    },
                                    mut #f => {
                                        ::std::mem::swap(&mut self.#f, &mut #f);
                                        ::std::option::Option::None
                                    }
                                }
                            }
                        })
                    },
                    _ => {
                        Ok(quote! {
                            #docs
                            pub fn #take(&mut self) -> ::std::option::Option<#t> {
                                let #f = ::std::mem::replace(&mut self.#f, #o::None);
                                match #f {
                                    #o::#c(#f) => {
                                        ::std::option::Option::Some(#f)
                                    },
                                    mut #f => {
                                        ::std::mem::swap(&mut self.#f, &mut #f);
                                        ::std::option::Option::None
                                    }
                                }
                            }
                        })
                    }
                }
            },
            FieldScope::Message(_) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Takes the value out of the [`{0}`] field, leaving an empty field in its place\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let take = names::get_field_name(self.descriptor, FieldName::Take)?;
                let t = names::get_rust_type(self.descriptor, TypeKind::Base, scope, &self.options.crate_name, false)?;
                let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                match self.descriptor.file().syntax() {
                    Syntax::Proto2 => {
                        match self.descriptor.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                Ok(quote! {
                                    #docs
                                    pub fn #take(&mut self) -> ::std::option::Option<#t> {
                                        self.#f.take().map(|v| *v)
                                    }
                                })
                            },
                            _ => {
                                Ok(quote! {
                                    #docs
                                    pub fn #take(&mut self) -> ::std::option::Option<#t> {
                                        self.#f.take()
                                    }
                                })
                            }
                        }
                    },
                    Syntax::Proto3 if self.descriptor.field_type().is_message() => {
                        Ok(quote! {
                            #docs
                            pub fn #take(&mut self) -> ::std::option::Option<#t> {
                                self.#f.take().map(|v| *v)
                            }
                        })
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    fn generate_clear_accessor(&self) -> Result {
        let scope = Some(Scope::from(self.descriptor.message().scope()));
        match self.descriptor.scope() {
            FieldScope::Oneof(o) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Clears the value of the [`{0}`] field\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let clear = names::get_field_name(self.descriptor, FieldName::Clear)?;
                let f = names::get_oneof_field(o)?;
                let o = names::get_oneof_type_path(o, scope)?;
                let c = names::get_field_name(self.descriptor, FieldName::OneofCase)?;
                Ok(quote! {
                    #docs
                    pub fn #clear(&mut self) {
                        let #f = ::std::mem::replace(&mut self.#f, #o::None);
                        match #f {
                            #o::#c(_) => { },
                            mut #f => ::std::mem::swap(&mut self.#f, &mut #f),
                        }
                    }
                })
            },
            FieldScope::Message(_) => {
                let docs = create_rustdoc_comment(&format!(concat!(
                    "Clears the value of the [`{0}`] field\n",
                    "\n",
                    "[`{0}`]: #method.{1}"
                    ), self.descriptor.name(), names::get_field_name(self.descriptor, FieldName::Get)?))?;
                let clear = names::get_field_name(self.descriptor, FieldName::Clear)?;
                let f = names::get_field_name(self.descriptor, FieldName::Field)?;
                match self.descriptor.file().syntax() {
                    Syntax::Proto2 => {
                        Ok(quote! {
                            #docs
                            pub fn #clear(&mut self) {
                                self.#f = ::std::option::Option::None;
                            }
                        })
                    },
                    Syntax::Proto3 if self.descriptor.field_type().is_message() => {
                        Ok(quote! {
                            #docs
                            pub fn #clear(&mut self) {
                                self.#f = ::std::option::Option::None;
                            }
                        })
                    }
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
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

                    Ok(Some(quote!(pub(super) static #n: #c::collections::MapCodec<#kt, #vt> = #c::collections::MapCodec::new(#ke, #ve, #tg);)))
                }
                _ => {
                    let n = names::get_field_name(self.descriptor, FieldName::Codec)?;
                    let c = &self.options.crate_name;
                    let t = names::get_rust_type(self.descriptor, TypeKind::Base, Some(Scope::Message(self.descriptor.message())), c, false)?;
                    let e = self.get_codec_new_expr()?;
                    Ok(Some(quote!(pub(super) static #n: #c::Codec<#t> = #e;)))
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn generate_reflector(&self) -> Result<Option<TokenStream>> {
        if let FieldScope::Oneof(_) = self.descriptor.scope() {
            Ok(Some(self.generate_verbose_field_reflector()?))
        } else if self.descriptor.label() == FieldLabel::Repeated {
            Ok(Some(self.generate_simple_field_reflector()?))
        } else if self.descriptor.file().syntax() == Syntax::Proto2 {
            Ok(Some(self.generate_verbose_field_reflector()?))
        } else if self.descriptor.proto().has_extendee() {
            Ok(None)
        } else if self.descriptor.field_type().is_message() {
            Ok(Some(self.generate_simple_option_field_reflector()?))
        } else {
            Ok(Some(self.generate_simple_field_reflector()?))
        }
    }

    pub fn generate_rustdoc_comments(&self) -> Result<Option<TokenStream>> {
        generate_rustdoc_comments(self.descriptor.source_code_info())
    }

    fn get_proto_type(&self, op: Option<&str>) -> syn::Result<Ident> {
        let result =
            match op {
                Some(op) => op.to_string() + "_",
                None => String::new()
            };
        syn::parse_str(
            &(result + 
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
                }))
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
                    FieldType::String => syn::parse_str("\"\""),
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

    fn get_codec_new_expr(&self) -> Result {
        let c = &self.options.crate_name;
        let tag = syn::parse_str::<ExprLit>(&Tag::new(self.descriptor.number(), self.descriptor.wire_type()).to_string())?;
        match self.descriptor.field_type() {
            FieldType::Group(_) => {
                let end_tag = syn::parse_str::<ExprLit>(&Tag::new(self.descriptor.number(), WireType::EndGroup).to_string())?;
                Ok(quote!(#c::Codec::group(#tag, #end_tag)))
            },
            _ => {
                let t = self.get_proto_type(None)?;
                Ok(quote!(#c::Codec::#t(#tag)))
            }
        }
    }

    fn get_reflector_depth(&self) -> usize {
        match self.descriptor.scope() {
            FieldScope::File(_) => 1,
            s => {
                let s = s.message().unwrap_or_else(|| unsafe { std::hint::unreachable_unchecked() }).scope();
                std::iter::successors(Some(s), |s|
                    match s {
                        CompositeScope::Message(m) => Some(m.scope()),
                        CompositeScope::File(_) => None,
                    }).count()
            }
        }
    }

    fn generate_verbose_field_reflector(&self) -> Result {
        let scope = 
            match self.descriptor.scope() {
                FieldScope::Message(m) => Scope::Message(m),
                FieldScope::File(f) => Scope::File(f),
                FieldScope::Oneof(o) => Scope::Message(o.message()),
            };
        let rn = names::get_field_name(self.descriptor, FieldName::Reflector)?;
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor.message(), Some(scope))?;
        let kind = 
            if self.descriptor.label() == FieldLabel::Repeated { 
                TypeKind::CollectionWrapped 
            } else { 
                TypeKind::Base 
            };
        let ft = names::get_rust_type(self.descriptor, kind, Some(scope), c, false)?;
        let is_oneof = 
            match self.descriptor.scope() {
                FieldScope::Oneof(_) => true,
                _ => false
            };
        let n = 
            if is_oneof || self.descriptor.field_type().is_message() || self.descriptor.field_type().is_group() {
                names::get_field_name(self.descriptor, FieldName::Get)?
            } else {
                names::get_field_name(self.descriptor, FieldName::GetOption)?
            };
        let n_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
        let set_n = names::get_field_name(self.descriptor, FieldName::Set)?;
        let take_n = names::get_field_name(self.descriptor, FieldName::Take)?;
        let clear_n = names::get_field_name(self.descriptor, FieldName::Clear)?;
        let pub_scope = std::iter::repeat(Token!(super)(Span::call_site())).take(self.get_reflector_depth());

        Ok(quote! {
            pub(in #(#pub_scope::)*super) static #rn: #c::reflect::access::VerboseFieldAccessor<#t, #ft> =
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
        let kind = 
            if self.descriptor.label() == FieldLabel::Repeated { 
                TypeKind::CollectionWrapped 
            } else { 
                TypeKind::Base 
            };
        let ft = names::get_rust_type(self.descriptor, kind, Some(scope), c, false)?;
        let n = names::get_field_name(self.descriptor, FieldName::Get)?;
        let n_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
        let pub_scope = std::iter::repeat(Token!(super)(Span::call_site())).take(self.get_reflector_depth());

        Ok(quote! {
            pub(in #(#pub_scope::)*super) static #rn: #c::reflect::access::SimpleFieldAccessor<#t, #ft> = 
                #c::reflect::access::SimpleFieldAccessor {
                    get: #t::#n,
                    get_mut: #t::#n_mut,
                };
        })
    }

    fn generate_simple_option_field_reflector(&self) -> Result {
        let scope = 
            match self.descriptor.scope() {
                FieldScope::Message(m) => Scope::Message(m),
                FieldScope::File(f) => Scope::File(f),
                _ => unreachable!(),
            };
        let rn = names::get_field_name(self.descriptor, FieldName::Reflector)?;
        let c = &self.options.crate_name;
        let t = names::get_message_type_path(self.descriptor.message(), Some(scope))?;
        let kind = 
            if self.descriptor.label() == FieldLabel::Repeated { 
                TypeKind::CollectionWrapped 
            } else { 
                TypeKind::Base 
            };
        let ft = names::get_rust_type(self.descriptor, kind, Some(scope), c, false)?;
        let n = names::get_field_name(self.descriptor, FieldName::Get)?;
        let n_mut = names::get_field_name(self.descriptor, FieldName::GetMut)?;
        let pub_scope = std::iter::repeat(Token!(super)(Span::call_site())).take(self.get_reflector_depth());

        Ok(quote! {
            pub(in #(#pub_scope::)*super) static #rn: #c::reflect::access::SimpleOptionFieldAccessor<#t, #ft> = 
                #c::reflect::access::SimpleOptionFieldAccessor {
                    get: #t::#n,
                    get_mut: #t::#n_mut,
                };
        })
    }
}

fn generate_rustdoc_comments(info: Option<&SourceCodeInfo>) -> Result<Option<TokenStream>> {
    if let Some(info) = info {
        if let Some(comments) = info.leading_comments().or(info.trailing_comments()) {
            return Ok(Some(create_rustdoc_comment(comments)?))
        }
    }
    Ok(None)
}

fn create_rustdoc_comment(value: &str) -> Result<TokenStream> {
    let lines = 
        value
            .lines()
            .map(|l| syn::parse_str(&format!("///{}", l)))
            .collect::<syn::Result<Vec<TokenStream>>>()?;
    Ok(quote! {
        #(#lines
        )*
    })
}