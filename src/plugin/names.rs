use crate::reflect::{FileDescriptor, CompositeScope, EnumDescriptor, EnumValueDescriptor, MessageDescriptor, OneofDescriptor, FieldDescriptor, FieldLabel, FieldType};
use heck::{CamelCase, SnakeCase};
use proc_macro2::TokenStream;
use syn::{Ident, Type, Path, Result};
use quote::quote;

/// Gets the module name for a file
/// 
/// The module name is determined by getting the file name and 
/// replacing all characters not a-z, A-Z, 0-9, or underscores with 
/// underscores. If the result of that is a Rust keyword, this 
/// returns an Err.
pub fn get_rust_file_mod_name(file: &FileDescriptor) -> Result<Ident> {
    syn::parse_str(
        &file.name()
            .chars()
            .map(|s| if !s.is_ascii_alphanumeric() || s != '_' { '_' } else { s })
            .collect::<String>())
}

/// Gets the type name for a message. It is not permitted to be a Rust keyword
pub fn get_message_type(t: &MessageDescriptor) -> Result<Ident> {
    syn::parse_str(t.name())
}

/// Gets the mod name for a message. After any transformations, it is not permitted to be a Rust keyword
pub fn get_message_mod(t: &MessageDescriptor) -> Result<Ident> {
    syn::parse_str(&t.name().to_snake_case())
}

pub fn get_oneof_type(o: &OneofDescriptor) -> Result<Ident> {
    syn::parse_str(&o.name().to_camel_case())
}

/// Gets the type name for an enum. It is not permitted to be a Rust keyword
pub fn get_enum_type(t: &EnumDescriptor) -> Result<Ident> {
    syn::parse_str(t.name())
}

pub fn get_enum_variant(v: &EnumValueDescriptor) -> Result<Ident> {
    let stripped = try_remove_prefix(v.enum_type().name(), v.name());
    let result = stripped.to_camel_case();

    syn::parse_str(&result)
}

pub fn get_enum_type_variant(v: &EnumValueDescriptor) -> Result<Path> {
    let t = get_enum_type(v.enum_type())?;
    let v = get_enum_variant(v)?;
    syn::parse2(quote!(#t::#v))
}

fn try_remove_prefix<'a>(type_name: &str, value_name: &'a str) -> &'a str {
    let mut prefix = type_name
        .char_indices()
        .filter(|(_, c)| *c != '_')
        .map(|(i, c)| (i, c.to_ascii_lowercase()));
    let mut value = value_name
        .char_indices()
        .filter(|(_, c)| *c != '_')
        .map(|(i, c)| (i, c.to_ascii_lowercase()));

    loop {
        match (prefix.next(), value.next()) {
            (_, None) => return value_name,
            (Some((_, a)), Some((_, b))) if a != b => return value_name,
            (a, Some((ib, b))) if a.map(|t| t.1) != Some(b) => {
                return &value_name[ib..].trim_start_matches('_');
            }
            _ => (),
        }
    }
}

/// A type resolution scope
#[derive(PartialEq, Clone, Copy)]
pub enum Scope<'a> {
    /// A file scope, used for resolving type paths from the top-level of a file.
    File(&'a FileDescriptor),
    /// A message scope, used for resolving type paths from the module of a message.
    Message(&'a MessageDescriptor)
}

impl<'a> From<&'a CompositeScope> for Scope<'a> {
    fn from(scope: &'a CompositeScope) -> Self {
        match scope {
            CompositeScope::File(f) => Scope::File(&**f),
            CompositeScope::Message(m) => Scope::Message(&**m)
        }
    }
}

/// Creates a full path to a message type starting from `self` (scope)
pub fn get_message_type_path(t: &MessageDescriptor, scope: Option<Scope>) -> Result<Path> {
    get_type_path(get_message_type(t)?, Scope::from(t.scope()), scope)
}

pub fn get_message_mod_path(t: &MessageDescriptor, scope: Option<Scope>) -> Result<Path> {
    get_type_path(get_message_mod(t)?, Scope::from(t.scope()), scope)
}

pub fn get_oneof_type_path(o: &OneofDescriptor, scope: Option<Scope>) -> Result<Path> {
    get_type_path(get_oneof_type(o)?, Scope::Message(o.message()), scope)
}

/// Creates a full path to an enum type starting from `self` (scope)
pub fn get_enum_type_path(t: &EnumDescriptor, scope: Option<Scope>) -> Result<Path> {
    get_type_path(get_enum_type(t)?, Scope::from(t.scope()), scope)
}

pub fn get_enum_variant_path(v: &EnumValueDescriptor, scope: Option<Scope>) -> Result<Path> {
    let e_path = get_enum_type_path(v.enum_type(), scope)?;
    let variant = get_enum_variant(v)?;
    syn::parse2(quote!(#e_path::#variant))
}

fn get_type_path(name: Ident, type_scope: Scope, from_scope: Option<Scope>) -> Result<Path> {
    fn push_scope(stream: &mut TokenStream, scope: Scope) -> Result<()> {
        match scope {
            Scope::File(f) => {
                let mod_ident = get_rust_file_mod_name(f)?;
                *stream = quote!(#stream::#mod_ident)
            },
            Scope::Message(m) => {
                let message_mod_ident = get_message_mod(m)?;
                *stream = quote!(#stream::#message_mod_ident)
            }
        }

        Ok(())
    }
    let scopes = {
        let mut traversed = type_scope;
        let mut scopes = Vec::new();
        while let Scope::Message(m) = traversed {
            scopes.push(traversed);
            traversed = Scope::from(m.scope());
        }
        scopes.push(traversed);
        scopes
    };
    let mut stream = quote!(self);

    match from_scope {
        Some(mut scope) => {
            loop {
                if let Some(index) = scopes.iter().position(|s| *s == scope) {
                    for scope in &scopes[..index] {
                        push_scope(&mut stream, *scope)?;
                    }
                } else {
                    stream = quote!(#stream::super);
                    match scope {
                        Scope::File(_) => {
                            for scope in scopes.iter().rev() {
                                push_scope(&mut stream, *scope)?;
                            }
                        },
                        Scope::Message(m) => {
                            scope = Scope::from(m.scope());
                        }
                    }
                }
            }
        },
        None => {
            for scope in scopes {
                push_scope(&mut stream, scope)?;
            }
        }
    }

    syn::parse2(quote!(#stream::#name))
}

pub enum FieldName {
    Field,
    Get,
    GetOption,
    GetMut,
    Set,
    Take,
    HasValue,
    Clear,
    OneofCase,
    Reflector,
    Extension,
    DefaultValue,
    Codec,
    FieldNumber,
}

/// Gets the field name for a field, transforming it for specific names in areas such as oneof cases, 
/// reflector fields, extension fields, default values, accessors, etc.
pub fn get_field_name(f: &FieldDescriptor, name: FieldName) -> Result<Ident> {
    match name {
        FieldName::Field => syn::parse_str(f.name()),
        FieldName::Get => syn::parse_str(f.name()),
        FieldName::GetOption => syn::parse_str(&(f.name().to_string() + "_option")),
        FieldName::GetMut => syn::parse_str(&(f.name().to_string() + "_mut")),
        FieldName::Set => syn::parse_str(&("set_".to_string() + f.name())),
        FieldName::Take => syn::parse_str(&("take_".to_string() + f.name())),
        FieldName::HasValue => syn::parse_str(&("has_".to_string() + f.name())),
        FieldName::Clear => syn::parse_str(&("clear_".to_string() + f.name())),
        FieldName::OneofCase => syn::parse_str(&f.name().to_camel_case()),
        FieldName::Reflector => syn::parse_str(&(f.name().to_uppercase() + "_REFLECTOR")),
        FieldName::Extension => syn::parse_str(&(f.name().to_uppercase())),
        FieldName::DefaultValue => syn::parse_str(&(f.name().to_uppercase() + "_DEFAULT_VALUE")),
        FieldName::Codec => syn::parse_str(&(f.name().to_uppercase() + "_CODEC")),
        FieldName::FieldNumber => syn::parse_str(&(f.name().to_uppercase() + "_FIELD_NUMBER"))
    }
}

pub enum TypeKind {
    /// The type of default values (may be unsized)
    DefaultNoRef,
    /// The type of default values with a static reference attached for compatible types
    Default,
    /// The base type without type indirection or collection wrappers(except for enum types which are wrapped in EnumValue)
    Base,
    /// Wraps types in collection wrappers
    CollectionWrapped,
    /// Wraps types in collection wrappers or Box for single value messages
    Indirected,
}

pub fn get_rust_type(f: &FieldDescriptor, kind: TypeKind, scope: Option<Scope>, crate_path: &Path, no_std: bool) -> Result<Type> {
    match f.field_type() {
        FieldType::Double => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<f64>)),
                _ => syn::parse_str("f64")
            }
        },
        FieldType::Float => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<f32>)),
                _ => syn::parse_str("f32")
            }
        },
        FieldType::Int64 | FieldType::Sint64 | FieldType::Sfixed64 => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<i64>)),
                _ => syn::parse_str("i64")
            }
        },
        FieldType::Uint64 | FieldType::Fixed64 => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<u64>)),
                _ => syn::parse_str("u64")
            }
        },
        FieldType::Int32 | FieldType::Sint32 | FieldType::Sfixed32 => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<i32>)),
                _ => syn::parse_str("i32")
            }
        },
        FieldType::Uint32 | FieldType::Fixed32 => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<u32>)),
                _ => syn::parse_str("u32")
            }
        },
        FieldType::Bool => {
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<bool>)),
                _ => syn::parse_str("bool")
            }
        },
        FieldType::String => {
            match kind {
                TypeKind::DefaultNoRef => syn::parse_str("str"),
                TypeKind::Default => syn::parse_str("&'static str"),
                TypeKind::Base => {
                    if no_std {
                        syn::parse_str("::alloc::string::String")
                    } else {
                        syn::parse_str("::std::string::String")
                    }
                },
                _ => {
                    if f.label() == FieldLabel::Repeated {
                        if no_std {
                            syn::parse2(quote!(crate_path::collections::RepeatedField<::alloc::string::String>))
                        } else {
                            syn::parse2(quote!(crate_path::collections::RepeatedField<::std::string::String>))
                        }
                    } else {
                        if no_std {
                            syn::parse_str("::alloc::string::String")
                        } else {
                            syn::parse_str("::std::string::String")
                        }
                    }
                }
            }
        },
        FieldType::Bytes => {
            match kind {
                TypeKind::DefaultNoRef => syn::parse_str("[u8]"),
                TypeKind::Default => syn::parse_str("&'static [u8]"),
                TypeKind::Base => {
                    if no_std {
                        syn::parse_str("::alloc::vec::Vec<u8>")
                    } else {
                        syn::parse_str("::std::vec::Vec<u8>")
                    }
                },
                _ => {
                    if f.label() == FieldLabel::Repeated {
                        if no_std {
                            syn::parse2(quote!(crate_path::collections::RepeatedField<::alloc::vec::Vec<u8>>))
                        } else {
                            syn::parse2(quote!(crate_path::collections::RepeatedField<::std::vec::Vec<u8>>))
                        }
                    } else {
                        if no_std {
                            syn::parse_str("::alloc::vec::Vec<u8>")
                        } else {
                            syn::parse_str("::std::vec::Vec<u8>")
                        }
                    }
                }
            }
        },
        FieldType::Enum(e) => {
            let tp = get_enum_type_path(e, scope)?;
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated) => syn::parse2(quote!(crate_path::collections::RepeatedField<#crate_path::EnumValue<#tp>>)),
                _ => syn::parse2(quote!(#crate_path::EnumValue<#tp>))
            }
        },
        FieldType::Message(m) | FieldType::Group(m) => {
            let tp = get_message_type_path(m, scope)?;
            match kind {
                TypeKind::Indirected => {
                    if f.label() == FieldLabel::Repeated {
                        if m.map_entry() {
                            let key = get_rust_type(&m.fields()[0], TypeKind::Base, scope, crate_path, no_std)?;
                            let value = get_rust_type(&m.fields()[1], TypeKind::Base, scope, crate_path, no_std)?;
                            syn::parse2(quote!(#crate_path::collections::MapField<#key, #value>))
                        } else {
                            syn::parse2(quote!(#crate_path::collections::RepeatedField<#tp>))
                        }
                    } else {
                        if no_std {
                            syn::parse2(quote!(::alloc::boxed::Box<#tp>))
                        } else {
                            syn::parse2(quote!(::std::boxed::Box<#tp>))
                        }
                    }
                },
                _ => syn::parse2(quote!(#tp))
            }
        },
    }
}