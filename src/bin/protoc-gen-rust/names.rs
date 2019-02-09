use protrust::reflect::*;

pub fn get_rust_file_name(file: &FileDescriptor) -> String {
    file.name().to_string() + ".rs"
}

pub fn get_rust_file_mod_name(file: &FileDescriptor) -> String {
    file.name()
        .replace(|s| s == '/' || s == '-' || s == '.', "_")
}

pub fn get_message_type_name(message: &MessageDescriptor) -> String {
    get_type_name(message.name(), message.scope())
}

pub fn get_full_message_type_name(
    message: &MessageDescriptor,
    file: Option<&FileDescriptor>,
    crate_name: &str,
) -> String {
    get_full_type_name(message.name(), message.scope(), file, crate_name)
}

pub fn get_enum_type_name(enum_type: &EnumDescriptor) -> String {
    get_type_name(enum_type.name(), enum_type.scope())
}

pub fn get_full_enum_type_name(
    enum_type: &EnumDescriptor,
    file: Option<&FileDescriptor>,
    crate_name: &str,
) -> String {
    get_full_type_name(enum_type.name(), enum_type.scope(), file, crate_name)
}

// ported from https://github.com/protocolbuffers/protobuf/blob/704037f23a9ede00ec4fcf40d568712ce6200934/src/google/protobuf/compiler/csharp/csharp_helpers.cc
pub fn get_enum_variant_name(value: &EnumValueDescriptor) -> String {
    let stripped = try_remove_prefix(value.enum_type().name(), value.name());
    let result = shouty_to_pascal_case(stripped);

    result
}

pub fn get_full_enum_variant_name(
    value: &EnumValueDescriptor,
    file: Option<&FileDescriptor>,
    crate_name: &str,
) -> String {
    format!(
        "{}::{}",
        get_full_enum_type_name(value.enum_type(), file, crate_name),
        get_enum_variant_name(value)
    )
}

pub fn get_field_name(field: &FieldDescriptor) -> String {
    match field.scope() {
        FieldScope::Message(_) | FieldScope::File(_) => field.name().to_string(),
        FieldScope::Oneof(_) => underscores_to_pascal_case(field.name(), false),
    }
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

/// Gets the rust type for a field
pub fn get_rust_type(res: TypeResolution, field: &FieldDescriptor, crate_name: &str) -> String {
    use protrust::reflect::FieldType::*;
    match res {
        TypeResolution::Base => match field.field_type() {
            Message(m) | Group(m) => get_full_message_type_name(m, Some(field.file()), crate_name),
            Enum(e) => format!(
                "{}::EnumValue<{}>",
                crate_name,
                get_full_enum_type_name(e, Some(field.file()), crate_name)
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
            let base = get_rust_type(TypeResolution::Base, field, crate_name);
            match field.field_type() {
                Message(_) | Group(_) => format!("::std::boxed::Box<{}>", base),
                _ => base,
            }
        }
        TypeResolution::Full => match field.label() {
            FieldLabel::Optional | FieldLabel::Required => {
                let base = get_rust_type(TypeResolution::Indirection, field, crate_name);
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
                            get_rust_type(TypeResolution::Base, &m.fields()[0], crate_name),
                            get_rust_type(TypeResolution::Base, &m.fields()[1], crate_name)
                        );
                    }
                }

                format!(
                    "{}::collections::RepeatedField<{}>",
                    crate_name,
                    get_rust_type(TypeResolution::Base, field, crate_name)
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
        FieldType::Message(_) => "message",
        FieldType::Group(_) => "group",
    }
    .to_string()
}

pub fn get_oneof_name(oneof: &OneofDescriptor) -> String {
    get_message_type_name(oneof.message()) + "_" + &underscores_to_pascal_case(oneof.name(), false)
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

fn get_type_name(name: &str, mut scope: &CompositeScope) -> String {
    let mut type_name = name.to_string();

    while let CompositeScope::Message(parent) = scope {
        type_name.insert(0, '_');
        type_name.insert_str(0, parent.name());
        scope = parent.scope();
    }

    type_name
}

fn get_full_type_name(
    name: &str,
    scope: &CompositeScope,
    file: Option<&FileDescriptor>,
    crate_name: &str,
) -> String {
    let mut full = get_type_name(name, scope);

    if let Some(file) = file {
        if scope.file() == file {
            full.insert_str(0, "self::");
        } else {
            let file = scope.file();
            if let Some(file) = well_known_file(file) {
                full.insert_str(0, "::");
                full.insert_str(0, file);
                full.insert_str(0, "::");
                full.insert_str(0, crate_name);
            } else {
                full.insert_str(0, "::");
                full.insert_str(0, &get_rust_file_mod_name(file));
                full.insert_str(0, "super::");
            }
        }
    } else {
        full.insert_str(0, "::");
        full.insert_str(0, &get_rust_file_mod_name(scope.file()))
    }

    full
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

fn well_known_file(file: &FileDescriptor) -> Option<&'static str> {
    match file.name() {
        "google/protobuf/descriptor.proto" => Some("descriptor"),
        "google/protobuf/compiler/plugin.proto" => Some("plugin"),
        "google/protobuf/any.proto" => Some("wkt::any"),
        "google/protobuf/api.proto" => Some("wkt::api"),
        "google/protobuf/duration.proto" => Some("wkt::duration"),
        "google/protobuf/empty.proto" => Some("wkt::empty"),
        "google/protobuf/field_mask.proto" => Some("wkt::field_mask"),
        "google/protobuf/source_context.proto" => Some("wkt::source_context"),
        "google/protobuf/struct.proto" => Some("wkt::r#struct"),
        "google/protobuf/timestamp.proto" => Some("wkt::timestamp"),
        "google/protobuf/type.proto" => Some("wkt::r#type"),
        "google/protobuf/wrappers.proto" => Some("wkt::wrappers"),
        _ => None,
    }
}
