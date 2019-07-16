use crate::reflect::{
    EnumDescriptor, EnumValueDescriptor, FieldDescriptor, FieldLabel, FieldType,
    FileDescriptor, MessageDescriptor, OneofDescriptor, CompositeScope
};
use heck::{CamelCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use syn::{Ident, Path, Result, Type};

/// Gets the module name for a file
///
/// The module name is determined by getting the file name and
/// replacing all characters not a-z, A-Z, 0-9, or underscores with
/// underscores. If the result of that is a Rust keyword, this
/// returns an Err.
pub fn get_rust_file_mod_name(file: &FileDescriptor) -> Result<Ident> {
    let mut name = Cow::Owned(
        file.name()
            .chars()
            .map(|s| if !s.is_ascii_alphanumeric() { '_' } else { s })
            .collect::<String>(),
    );
    escape_identifier(&mut name);
    syn::parse_str(&name)
}

/// Gets the type name for a message. It is not permitted to be a Rust keyword
pub fn get_message_type(t: &MessageDescriptor) -> Result<Ident> {
    let mut name = Cow::Borrowed(t.name());
    escape_identifier(&mut name);
    syn::parse_str(&name)
}

/// Gets the mod name for a message. After any transformations, it is not permitted to be a Rust keyword
pub fn get_message_mod(t: &MessageDescriptor) -> Result<Ident> {
    let mut name = Cow::Owned(t.name().to_snake_case());
    escape_identifier(&mut name);
    syn::parse_str(&name)
}

pub fn get_oneof_type(o: &OneofDescriptor) -> Result<Ident> {
    let mut name = Cow::Owned(o.name().to_camel_case());
    escape_identifier(&mut name);
    syn::parse_str(&name)
}

pub fn get_oneof_field(o: &OneofDescriptor) -> Result<Ident> {
    let mut name = Cow::Borrowed(o.name());
    escape_identifier(&mut name);
    syn::parse_str(&name)
}

/// Gets the type name for an enum. It is not permitted to be a Rust keyword
pub fn get_enum_type(t: &EnumDescriptor) -> Result<Ident> {
    let mut name = Cow::Borrowed(t.name());
    escape_identifier(&mut name);
    syn::parse_str(&name)
}

pub fn get_enum_variant(v: &EnumValueDescriptor) -> Result<Ident> {
    let stripped = try_remove_prefix(v.enum_type().name(), v.name());
    let mut result = Cow::Owned(stripped.to_camel_case());
    escape_identifier(&mut result);

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
pub enum Scope<'a, 'b> {
    /// A file scope, used for resolving type paths from the top-level of a file.
    File(&'b FileDescriptor<'a>),
    /// A message scope, used for resolving type paths from the module of a message.
    Message(&'b MessageDescriptor<'a>)
}

impl<'a, 'b> From<&'b CompositeScope<'a>> for Scope<'a, 'b> {
    fn from(scope: &'b CompositeScope<'a>) -> Self {
        match scope {
            CompositeScope::File(f) => Scope::File(&**f),
            CompositeScope::Message(m) => Scope::Message(&**m)
        }
    }
}

/// Creates a full path to a message type starting from `self` (scope)
pub fn get_message_type_path<'a>(
    t: &MessageDescriptor<'a>,
    scope: Option<Scope<'a, '_>>,
) -> Result<Path> {
    get_type_path(get_message_type(t)?, Scope::from(t.scope()), scope)
}

pub fn get_message_mod_path<'a>(
    t: &MessageDescriptor<'a>,
    scope: Option<Scope<'a, '_>>,
) -> Result<Path> {
    get_type_path(get_message_mod(t)?, Scope::from(t.scope()), scope)
}

pub fn get_oneof_type_path<'a>(
    o: &OneofDescriptor<'a>,
    scope: Option<Scope<'a, '_>>,
) -> Result<Path> {
    get_type_path(get_oneof_type(o)?, Scope::Message(o.message()), scope)
}

/// Creates a full path to an enum type starting from `self` (scope)
pub fn get_enum_type_path<'a>(
    t: &EnumDescriptor<'a>,
    scope: Option<Scope<'a, '_>>,
) -> Result<Path> {
    get_type_path(get_enum_type(t)?, Scope::from(t.scope()), scope)
}

pub fn get_enum_variant_path<'a>(
    v: &EnumValueDescriptor<'a>,
    scope: Option<Scope<'a, '_>>,
) -> Result<Path> {
    let e_path = get_enum_type_path(v.enum_type(), scope)?;
    let variant = get_enum_variant(v)?;
    syn::parse2(quote!(#e_path::#variant))
}

fn get_type_path<'a>(
    name: Ident,
    type_scope: Scope<'a, '_>,
    from_scope: Option<Scope<'a, '_>>,
) -> Result<Path> {
    fn push_scope(stream: &mut TokenStream, scope: Scope) -> Result<()> {
        match scope {
            Scope::File(f) => {
                let mod_ident = get_rust_file_mod_name(f)?;
                *stream = quote!(#stream::#mod_ident)
            }
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
            if scope != type_scope {
                loop {
                    if let Some(index) = scopes.iter().position(|s| *s == scope) {
                        for scope in &scopes[..index] {
                            push_scope(&mut stream, *scope)?;
                        }
                        break;
                    } else {
                        stream = quote!(#stream::super);
                        match scope {
                            Scope::File(_) => {
                                for scope in scopes.iter().rev() {
                                    push_scope(&mut stream, *scope)?;
                                }
                                break;
                            }
                            Scope::Message(m) => {
                                scope = Scope::from(m.scope());
                            }
                        }
                    }
                }
            }
        }
        None => {
            for scope in scopes.iter().rev() {
                push_scope(&mut stream, *scope)?;
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
    let mut name = Cow::Owned(match name {
        FieldName::Field => f.name().to_string(),
        FieldName::Get => f.name().to_string(),
        FieldName::GetOption => f.name().to_string() + "_option",
        FieldName::GetMut => f.name().to_string() + "_mut",
        FieldName::Set => "set_".to_string() + f.name(),
        FieldName::Take => "take_".to_string() + f.name(),
        FieldName::HasValue => "has_".to_string() + f.name(),
        FieldName::Clear => "clear_".to_string() + f.name(),
        FieldName::OneofCase => f.name().to_camel_case(),
        FieldName::Reflector => f.name().to_uppercase() + "_REFLECTOR",
        FieldName::Extension => f.name().to_uppercase(),
        FieldName::DefaultValue => f.name().to_uppercase() + "_DEFAULT_VALUE",
        FieldName::Codec => f.name().to_uppercase() + "_CODEC",
        FieldName::FieldNumber => f.name().to_uppercase() + "_FIELD_NUMBER",
    });
    escape_identifier(&mut name);
    syn::parse_str(&name)
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

pub fn get_rust_type<'a>(
    f: &FieldDescriptor<'a>,
    kind: TypeKind,
    scope: Option<Scope<'a, '_>>,
    crate_path: &Path,
    no_std: bool,
) -> Result<Type> {
    match f.field_type() {
        FieldType::Double => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<f64>))
            }
            _ => syn::parse_str("f64"),
        },
        FieldType::Float => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<f32>))
            }
            _ => syn::parse_str("f32"),
        },
        FieldType::Int64 | FieldType::Sint64 | FieldType::Sfixed64 => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<i64>))
            }
            _ => syn::parse_str("i64"),
        },
        FieldType::Uint64 | FieldType::Fixed64 => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<u64>))
            }
            _ => syn::parse_str("u64"),
        },
        FieldType::Int32 | FieldType::Sint32 | FieldType::Sfixed32 => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<i32>))
            }
            _ => syn::parse_str("i32"),
        },
        FieldType::Uint32 | FieldType::Fixed32 => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<u32>))
            }
            _ => syn::parse_str("u32"),
        },
        FieldType::Bool => match (kind, f.label()) {
            (TypeKind::Indirected, FieldLabel::Repeated)
            | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => {
                syn::parse2(quote!(#crate_path::collections::RepeatedField<bool>))
            }
            _ => syn::parse_str("bool"),
        },
        FieldType::String => match kind {
            TypeKind::DefaultNoRef => syn::parse_str("str"),
            TypeKind::Default => syn::parse_str("&'static str"),
            TypeKind::Base => {
                if no_std {
                    syn::parse_str("::alloc::string::String")
                } else {
                    syn::parse_str("::std::string::String")
                }
            }
            _ => {
                if f.label() == FieldLabel::Repeated {
                    if no_std {
                        syn::parse2(
                            quote!(#crate_path::collections::RepeatedField<::alloc::string::String>),
                        )
                    } else {
                        syn::parse2(
                            quote!(#crate_path::collections::RepeatedField<::std::string::String>),
                        )
                    }
                } else {
                    if no_std {
                        syn::parse_str("::alloc::string::String")
                    } else {
                        syn::parse_str("::std::string::String")
                    }
                }
            }
        },
        FieldType::Bytes => match kind {
            TypeKind::DefaultNoRef => syn::parse_str("[u8]"),
            TypeKind::Default => syn::parse_str("&'static [u8]"),
            TypeKind::Base => {
                if no_std {
                    syn::parse_str("::alloc::vec::Vec<u8>")
                } else {
                    syn::parse_str("::std::vec::Vec<u8>")
                }
            }
            _ => {
                if f.label() == FieldLabel::Repeated {
                    if no_std {
                        syn::parse2(
                            quote!(#crate_path::collections::RepeatedField<::alloc::vec::Vec<u8>>),
                        )
                    } else {
                        syn::parse2(
                            quote!(#crate_path::collections::RepeatedField<::std::vec::Vec<u8>>),
                        )
                    }
                } else {
                    if no_std {
                        syn::parse_str("::alloc::vec::Vec<u8>")
                    } else {
                        syn::parse_str("::std::vec::Vec<u8>")
                    }
                }
            }
        },
        FieldType::Enum(e) => {
            let tp = get_enum_type_path(&e, scope)?;
            match (kind, f.label()) {
                (TypeKind::Indirected, FieldLabel::Repeated)
                | (TypeKind::CollectionWrapped, FieldLabel::Repeated) => syn::parse2(
                    quote!(#crate_path::collections::RepeatedField<#crate_path::EnumValue<#tp>>),
                ),
                _ => syn::parse2(quote!(#crate_path::EnumValue<#tp>)),
            }
        }
        FieldType::Message(m) | FieldType::Group(m) => {
            let tp = get_message_type_path(&m, scope)?;
            match kind {
                TypeKind::Indirected | TypeKind::CollectionWrapped => {
                    if f.label() == FieldLabel::Repeated {
                        if m.is_map_entry() {
                            let key = get_rust_type(
                                &m.fields()[0],
                                TypeKind::Base,
                                scope,
                                crate_path,
                                no_std,
                            )?;
                            let value = get_rust_type(
                                &m.fields()[1],
                                TypeKind::Base,
                                scope,
                                crate_path,
                                no_std,
                            )?;
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
                }
                _ => syn::parse2(quote!(#tp)),
            }
        }
    }
}

fn escape_identifier(id: &mut Cow<str>) {
    match &**id {
        "as" | "break" | "const" | "continue" | "else" | "enum" | "false" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref"
        | "return" | "static" | "struct" | "trait" | "true" | "type" | "unsafe" | "use"
        | "where" | "while" | "dyn" | "abstract" | "become" | "box" | "do" | "final" | "macro"
        | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield" | "async" | "await"
        | "try" => {
            id.to_mut().insert_str(0, "r#");
        }
        _ => {}
    }
}
