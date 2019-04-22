use protrust::reflect::*;

/// Gets the file name for a generated file
pub fn get_rust_file_name(file: &FileDescriptor) -> String {
    file.name().to_string() + ".rs"
}

/// Gets the module name for a file
pub fn get_rust_file_mod_name(file: &FileDescriptor) -> String {
    file.name()
        .replace(|s| s == '/' || s == '-' || s == '.', "_")
}

/// Gets the name of a struct name for a message type
pub fn get_message_type_name(message: &MessageDescriptor) -> String {
    get_type_name(message.name())
}

/// Gets the name of a module name for a message type
pub fn get_message_type_module_name(message: &MessageDescriptor) -> String {
    let mut name = String::new();
    let mut last_was_cap = false;

    for c in message.name().chars() {
        if name.len() != 0 && c.is_uppercase() {
            if !last_was_cap {
                name.push('_');
            }
            last_was_cap = true;
        } else {
            last_was_cap = false;
        }
        for c in c.to_lowercase() {
            name.push(c);
        }
    }

    name
}

/// Resolves a fully qualified struct name for a message type from a relative scope.
/// If no scope is given, it's resolved from the generated module scope.
pub fn get_full_message_type_name(message: &MessageDescriptor, scope: Option<Scope>) -> String {
    get_full_type_name(message.name(), message.scope(), scope)
}

/// Resolves a fully qualified module name for a message type from a relative scope.
/// If no scope is given, it's resolved from the generated module scope.
pub fn get_full_message_type_module_name(
    message: &MessageDescriptor,
    scope: Option<Scope>,
) -> String {
    get_full_type_name(
        &get_message_type_module_name(message),
        message.scope(),
        scope,
    )
}

pub fn get_enum_type_name(enum_type: &EnumDescriptor) -> String {
    get_type_name(enum_type.name())
}

pub fn get_full_enum_type_name(enum_type: &EnumDescriptor, scope: Option<Scope>) -> String {
    get_full_type_name(enum_type.name(), enum_type.scope(), scope)
}

// ported from https://github.com/protocolbuffers/protobuf/blob/704037f23a9ede00ec4fcf40d568712ce6200934/src/google/protobuf/compiler/csharp/csharp_helpers.cc
pub fn get_enum_variant_name(value: &EnumValueDescriptor) -> String {
    let stripped = try_remove_prefix(value.enum_type().name(), value.name());
    let result = shouty_to_pascal_case(stripped);

    result
}

pub fn get_full_enum_variant_name(value: &EnumValueDescriptor, scope: Option<Scope>) -> String {
    format!(
        "{}::{}",
        get_full_enum_type_name(value.enum_type(), scope),
        get_enum_variant_name(value)
    )
}

pub fn get_field_name(field: &FieldDescriptor) -> String {
    match field.scope() {
        FieldScope::Message(_) if field.proto().has_extendee() => field.name().to_uppercase(),
        FieldScope::File(_) => field.name().to_uppercase(),
        _ => field.name().to_string(),
    }
}

pub fn get_oneof_case_name(field: &FieldDescriptor) -> String {
    underscores_to_pascal_case(field.name(), false)
}

pub fn get_struct_field_name(field: &FieldDescriptor) -> String {
    let mut name = match field.scope() {
        FieldScope::Message(_) | FieldScope::File(_) => field.name().clone(),
        FieldScope::Oneof(o) => o.name().clone(),
    }
    .to_string();
    escape_name(&mut name);
    name
}

pub fn get_field_default_value_name(field: &FieldDescriptor) -> String {
    field.name().to_ascii_uppercase() + "_DEFAULT_VALUE"
}

pub fn get_field_codec_name(field: &FieldDescriptor) -> String {
    pascal_to_shouty_case(&get_message_type_name(field.message()))
        + "_"
        + &field.name().to_ascii_uppercase()
        + "_CODEC"
}

pub fn get_field_number_const_name(field: &FieldDescriptor) -> String {
    field.name().to_ascii_uppercase() + "_FIELD_NUMBER"
}

fn pascal_to_shouty_case(value: &str) -> String {
    let mut result = String::new();
    let mut chars = value.chars().peekable();
    while let Some(c) = chars.next() {
        result.push(c.to_ascii_uppercase());

        if let Some(peek) = chars.peek() {
            if peek.is_ascii_uppercase() {
                result.push('_');
            }
        }
    }

    result
}

/// Specifies the type resolution mode of get_rust_type
pub enum TypeResolution {
    /// The base type. For enums this resolves as EnumValue
    Base,
    /// The type including indirection types (boxes for messages and groups)
    Indirection,
    /// The full type including option and repeated field wrappers
    Full,
}

#[derive(Clone, Copy)]
pub enum TypeScope {
    /// For oneof fields, the message that contains the oneof. For map fields, the message that contains the map
    Message,
    /// The full scope
    Full,
}

/// Gets the rust type for a field
pub fn get_rust_type(
    res: TypeResolution,
    field: &FieldDescriptor,
    scope: TypeScope,
    crate_name: &str,
) -> String {
    use protrust::reflect::FieldType::*;
    let field_scope = match (scope, field.scope()) {
        (TypeScope::Message, FieldScope::Oneof(o)) => Scope::Composite(o.message().scope()),
        (TypeScope::Message, FieldScope::Message(m)) if m.map_entry() => match m.scope() {
            CompositeScope::Message(m) => Scope::Composite(m.scope()),
            _ => unreachable!(),
        },
        (_, _) => Scope::Field(field),
    };
    match res {
        TypeResolution::Base => match field.field_type() {
            Message(m) | Group(m) => get_full_message_type_name(m, Some(field_scope)),
            Enum(e) => format!(
                "{}::EnumValue<{}>",
                crate_name,
                get_full_enum_type_name(e, Some(field_scope))
            ),
            Bytes => format!("::std::vec::Vec<u8>"),
            String => format!("::std::string::String"),
            Bool => format!("bool"),
            Sfixed32 | Sint32 | Int32 => format!("i32"),
            Fixed32 | Uint32 => format!("u32"),
            Sfixed64 | Sint64 | Int64 => format!("i64"),
            Fixed64 | Uint64 => format!("u64"),
            Double => format!("f64"),
            Float => format!("f32"),
        },
        TypeResolution::Indirection => {
            let base = get_rust_type(TypeResolution::Base, field, scope, crate_name);
            match field.field_type() {
                Message(_) | Group(_) => format!("::std::boxed::Box<{}>", base),
                _ => base,
            }
        }
        TypeResolution::Full => match field.label() {
            FieldLabel::Optional | FieldLabel::Required => {
                let base = get_rust_type(TypeResolution::Indirection, field, scope, crate_name);
                if field.file().syntax() == Syntax::Proto2 {
                    format!("::std::option::Option<{}>", base)
                } else {
                    if let FieldScope::Oneof(_) = field.scope() {
                        base
                    } else {
                        match field.field_type() {
                            FieldType::Message(_) | FieldType::Group(_) => {
                                format!("::std::option::Option<{}>", base)
                            }
                            _ => base,
                        }
                    }
                }
            }
            FieldLabel::Repeated => {
                if let FieldType::Message(m) = field.field_type() {
                    if m.map_entry() {
                        return format!(
                            "{}::collections::MapField<{}, {}>",
                            crate_name,
                            get_rust_type(
                                TypeResolution::Base,
                                &m.fields()[0],
                                TypeScope::Message,
                                crate_name
                            ),
                            get_rust_type(
                                TypeResolution::Base,
                                &m.fields()[1],
                                TypeScope::Message,
                                crate_name
                            )
                        );
                    }
                }

                format!(
                    "{}::collections::RepeatedField<{}>",
                    crate_name,
                    get_rust_type(TypeResolution::Base, field, scope, crate_name)
                )
            }
        },
    }
}

/// Get the protobuf type name of a field. This resolves to the original type name for most field types,
/// however for messages and groups are resolved to "message" and "group" respectively. Enums are returned as "enum_value"
///
/// This method is used for easy method naming as read, write, size, codec, and extension creation methods all use these names
pub fn get_proto_type(field: &FieldDescriptor) -> String {
    match field.field_type() {
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
    }
    .to_string()
}

pub fn get_oneof_name(oneof: &OneofDescriptor) -> String {
    underscores_to_pascal_case(oneof.name(), false)
}

fn escape_name(name: &mut String) {
    match &**name {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
        | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "dyn" | "abstract"
        | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yeild" | "async" | "await" | "try" => name.insert_str(0, "r#"),
        _ => {}
    }
}

fn get_type_name(name: &str) -> String {
    name.to_string()
}

pub enum Scope<'a> {
    Field(&'a FieldDescriptor),
    Composite(&'a CompositeScope),
}

fn get_full_type_name(name: &str, scope: &CompositeScope, current_scope: Option<Scope>) -> String {
    fn push_scope(path: &mut String, scope: &CompositeScope) {
        match scope {
            CompositeScope::File(f) => {
                path.push_str(&get_rust_file_mod_name(f));
            }
            CompositeScope::Message(m) => {
                path.push_str(&get_message_type_module_name(m));
            }
        }
        path.push_str("::");
    }

    fn build_from_composite_scope(
        path: &mut String,
        name: &str,
        scopes: &[&CompositeScope],
        mut current_scope: &CompositeScope,
    ) {
        loop {
            // if any of the scopes in our vector match the current scope, we can build a path from that point
            if let Some(index) = scopes.iter().position(|s| *s == current_scope) {
                scopes[..index].iter().for_each(|s| push_scope(path, s));
                path.push_str(name);
                return;
            // otherwise we continue to move up
            } else {
                path.push_str("super::");
                match current_scope {
                    // if we've hit a the file scope, then our current scope is the mod file
                    // so the only path now is down
                    CompositeScope::File(_) => {
                        scopes.iter().rev().for_each(|s| push_scope(path, s));
                        path.push_str(name);
                        return;
                    }
                    // if we're in a message scope, just reassign the current scope and loop
                    CompositeScope::Message(m) => {
                        current_scope = m.scope();
                    }
                }
            }
        }
    }

    let scopes = {
        // build a vector of every level of our type's scope
        let mut traversed_scope = scope;
        let mut scopes = Vec::new();
        while let CompositeScope::Message(m) = traversed_scope {
            scopes.push(traversed_scope);
            traversed_scope = m.scope();
        }
        scopes.push(traversed_scope);
        scopes
    };
    let mut full = "self::".to_string(); // start with self
    match current_scope {
        // with an existing scope, we have to move up until we've found a scope we can decend into to get the target type
        Some(current_scope) => match current_scope {
            Scope::Field(f) => {
                match f.scope() {
                    FieldScope::File(f) => {
                        if &**f != scope.file() {
                            full.push_str("super::");
                            full.push_str(&get_rust_file_mod_name(scope.file()));
                            full.push_str("::");
                        }

                        if let Some((_, levels)) = scopes.split_last() {
                            levels.iter().rev().for_each(|s| push_scope(&mut full, s));
                        }

                        full.push_str(name);
                    }
                    FieldScope::Oneof(_) => {
                        full.push_str("super::");
                        build_from_composite_scope(&mut full, name, &scopes, f.message().scope());
                    }
                    FieldScope::Message(m) if f.proto().has_extendee() => {
                        full.push_str("super::");
                        build_from_composite_scope(&mut full, name, &scopes, m.scope());
                    }
                    FieldScope::Message(m) => {
                        build_from_composite_scope(&mut full, name, &scopes, m.scope());
                    }
                }
                full
            }
            Scope::Composite(c) => {
                build_from_composite_scope(&mut full, name, &scopes, c);
                full
            }
        },
        // with no scope, we can decend from the mod file
        None => {
            scopes.iter().rev().for_each(|s| push_scope(&mut full, s));
            full.push_str(name);
            full
        }
    }
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

// ported from https://github.com/protocolbuffers/protobuf/blob/704037f23a9ede00ec4fcf40d568712ce6200934/src/google/protobuf/compiler/csharp/csharp_helpers.cc
fn shouty_to_pascal_case(input: &str) -> String {
    let mut result = String::new();
    let mut previous = '_';
    for current in input.chars() {
        if !current.is_ascii_alphanumeric() {
            // do nothing, but skip all other blocks
        } else if !previous.is_ascii_alphanumeric() || previous.is_ascii_digit() {
            result.push(current.to_ascii_uppercase());
        } else if previous.is_ascii_lowercase() {
            result.push(current);
        } else {
            result.push(current.to_ascii_lowercase());
        }

        previous = current;
    }
    result
}

fn underscores_to_camel_case(input: &str, mut cap_next: bool, preserve_dot: bool) -> String {
    let mut result = String::new();

    for (index, character) in input.char_indices() {
        match character {
            c @ 'a'...'z' => {
                if cap_next {
                    result.push(c.to_ascii_uppercase());
                } else {
                    result.push(c);
                }
                cap_next = false;
            }
            c @ 'A'...'Z' => {
                if index == 0 && !cap_next {
                    result.push(c.to_ascii_lowercase());
                } else {
                    result.push(c);
                }
                cap_next = false;
            }
            c @ '0'...'9' => {
                result.push(c);
                cap_next = true;
            }
            c @ _ => {
                cap_next = true;
                if c == '.' && preserve_dot {
                    result.push('.');
                }
            }
        }
    }

    if input.len() > 0 && input.chars().last() == Some('#') {
        result.push('_');
    }

    result
}

fn underscores_to_pascal_case(input: &str, preserve_dot: bool) -> String {
    underscores_to_camel_case(input, true, preserve_dot)
}
