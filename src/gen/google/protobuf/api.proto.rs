//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/api.proto


#[derive(Clone, PartialEq)]
pub struct Api {
    pub name: std::string::String,
    pub methods: crate::collections::RepeatedField<std::boxed::Box<self::Method>>,
    pub options: crate::collections::RepeatedField<std::boxed::Box<super::google_protobuf_type_proto::Option>>,
    pub version: std::string::String,
    pub source_context: std::option::Option<std::boxed::Box<super::google_protobuf_source_context_proto::SourceContext>>,
    pub mixins: crate::collections::RepeatedField<std::boxed::Box<self::Mixin>>,
    pub syntax: crate::EnumValue<super::google_protobuf_type_proto::Syntax>,
    _unknown_fields: crate::UnknownFieldSet
}
static API_NAME_DEFAULT_VALUE: &'static str = "";
static API_METHODS_CODEC: crate::Codec<std::boxed::Box<self::Method>> = crate::Codec::message(18);
static API_OPTIONS_CODEC: crate::Codec<std::boxed::Box<super::google_protobuf_type_proto::Option>> = crate::Codec::message(26);
static API_VERSION_DEFAULT_VALUE: &'static str = "";
static API_MIXINS_CODEC: crate::Codec<std::boxed::Box<self::Mixin>> = crate::Codec::message(50);
static API_SYNTAX_DEFAULT_VALUE: crate::EnumValue<super::google_protobuf_type_proto::Syntax> = crate::EnumValue::Defined(super::google_protobuf_type_proto::Syntax::Proto2);
impl crate::CodedMessage for self::Api {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.methods.add_entries(tag.get(), input, &API_METHODS_CODEC)?,
                26 => self.options.add_entries(tag.get(), input, &API_OPTIONS_CODEC)?,
                34 => self.version = input.read_string()?,
                42 => input.read_message(self.source_context.get_or_insert_with(crate::LiteMessage::new))?,
                50 => self.mixins.add_entries(tag.get(), input, &API_MIXINS_CODEC)?,
                56 => self.syntax = input.read_enum_value()?,
                _ => { }
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> std::option::Option<i32> {
        let mut size = 0i32;
        let name = &self.name;
        if name != API_NAME_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(name)?)?;
        }
        size = size.checked_add(self.methods.calculate_size(&API_METHODS_CODEC)?)?;
        size = size.checked_add(self.options.calculate_size(&API_OPTIONS_CODEC)?)?;
        let version = &self.version;
        if version != API_VERSION_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(version)?)?;
        }
        let source_context = &self.source_context;
        if let std::option::Option::Some(source_context) = source_context {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::message(source_context)?)?;
        }
        size = size.checked_add(self.mixins.calculate_size(&API_MIXINS_CODEC)?)?;
        let syntax = self.syntax;
        if syntax != API_SYNTAX_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::enum_value(syntax))?;
        }
        std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != API_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        self.methods.write_to(output, &API_METHODS_CODEC)?;
        self.options.write_to(output, &API_OPTIONS_CODEC)?;
        let version = &self.version;
        if version != API_VERSION_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(version)?;
        }
        let source_context = &self.source_context;
        if let std::option::Option::Some(source_context) = source_context {
            output.write_raw_tag_bytes(&[42])?;
            output.write_message(source_context)?;
        }
        self.mixins.write_to(output, &API_MIXINS_CODEC)?;
        let syntax = self.syntax;
        if syntax != API_SYNTAX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[56])?;
            output.write_enum_value(syntax)?;
        }
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Api {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            methods: crate::collections::RepeatedField::new(),
            options: crate::collections::RepeatedField::new(),
            version: std::string::String::new(),
            source_context: std::option::Option::None,
            mixins: crate::collections::RepeatedField::new(),
            syntax: API_SYNTAX_DEFAULT_VALUE,
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.methods.merge(&other.methods);
        self.options.merge(&other.options);
        self.version = other.version.clone();
        if let std::option::Option::Some(source_context) = &other.source_context {
            self.source_context.get_or_insert_with(crate::LiteMessage::new).merge(source_context);
        }
        self.mixins.merge(&other.mixins);
        self.syntax = other.syntax;
    }
}
impl crate::Message for self::Api {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Api {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'methods' field
    pub const METHODS_FIELD_NUMBER: i32 = 2;
    /// Gets the field number of the 'options' field
    pub const OPTIONS_FIELD_NUMBER: i32 = 3;
    /// Gets the field number of the 'version' field
    pub const VERSION_FIELD_NUMBER: i32 = 4;
    /// Gets the field number of the 'source_context' field
    pub const SOURCE_CONTEXT_FIELD_NUMBER: i32 = 5;
    /// Gets the field number of the 'mixins' field
    pub const MIXINS_FIELD_NUMBER: i32 = 6;
    /// Gets the field number of the 'syntax' field
    pub const SYNTAX_FIELD_NUMBER: i32 = 7;
}
#[derive(Clone, PartialEq)]
pub struct Method {
    pub name: std::string::String,
    pub request_type_url: std::string::String,
    pub request_streaming: bool,
    pub response_type_url: std::string::String,
    pub response_streaming: bool,
    pub options: crate::collections::RepeatedField<std::boxed::Box<super::google_protobuf_type_proto::Option>>,
    pub syntax: crate::EnumValue<super::google_protobuf_type_proto::Syntax>,
    _unknown_fields: crate::UnknownFieldSet
}
static METHOD_NAME_DEFAULT_VALUE: &'static str = "";
static METHOD_REQUEST_TYPE_URL_DEFAULT_VALUE: &'static str = "";
static METHOD_REQUEST_STREAMING_DEFAULT_VALUE: bool = false;
static METHOD_RESPONSE_TYPE_URL_DEFAULT_VALUE: &'static str = "";
static METHOD_RESPONSE_STREAMING_DEFAULT_VALUE: bool = false;
static METHOD_OPTIONS_CODEC: crate::Codec<std::boxed::Box<super::google_protobuf_type_proto::Option>> = crate::Codec::message(50);
static METHOD_SYNTAX_DEFAULT_VALUE: crate::EnumValue<super::google_protobuf_type_proto::Syntax> = crate::EnumValue::Defined(super::google_protobuf_type_proto::Syntax::Proto2);
impl crate::CodedMessage for self::Method {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.request_type_url = input.read_string()?,
                24 => self.request_streaming = input.read_bool()?,
                34 => self.response_type_url = input.read_string()?,
                40 => self.response_streaming = input.read_bool()?,
                50 => self.options.add_entries(tag.get(), input, &METHOD_OPTIONS_CODEC)?,
                56 => self.syntax = input.read_enum_value()?,
                _ => { }
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> std::option::Option<i32> {
        let mut size = 0i32;
        let name = &self.name;
        if name != METHOD_NAME_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(name)?)?;
        }
        let request_type_url = &self.request_type_url;
        if request_type_url != METHOD_REQUEST_TYPE_URL_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(request_type_url)?)?;
        }
        let request_streaming = self.request_streaming;
        if request_streaming != METHOD_REQUEST_STREAMING_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bool(request_streaming))?;
        }
        let response_type_url = &self.response_type_url;
        if response_type_url != METHOD_RESPONSE_TYPE_URL_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(response_type_url)?)?;
        }
        let response_streaming = self.response_streaming;
        if response_streaming != METHOD_RESPONSE_STREAMING_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::bool(response_streaming))?;
        }
        size = size.checked_add(self.options.calculate_size(&METHOD_OPTIONS_CODEC)?)?;
        let syntax = self.syntax;
        if syntax != METHOD_SYNTAX_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::enum_value(syntax))?;
        }
        std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != METHOD_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let request_type_url = &self.request_type_url;
        if request_type_url != METHOD_REQUEST_TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[18])?;
            output.write_string(request_type_url)?;
        }
        let request_streaming = self.request_streaming;
        if request_streaming != METHOD_REQUEST_STREAMING_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[24])?;
            output.write_bool(request_streaming)?;
        }
        let response_type_url = &self.response_type_url;
        if response_type_url != METHOD_RESPONSE_TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(response_type_url)?;
        }
        let response_streaming = self.response_streaming;
        if response_streaming != METHOD_RESPONSE_STREAMING_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[40])?;
            output.write_bool(response_streaming)?;
        }
        self.options.write_to(output, &METHOD_OPTIONS_CODEC)?;
        let syntax = self.syntax;
        if syntax != METHOD_SYNTAX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[56])?;
            output.write_enum_value(syntax)?;
        }
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Method {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            request_type_url: std::string::String::new(),
            request_streaming: METHOD_REQUEST_STREAMING_DEFAULT_VALUE,
            response_type_url: std::string::String::new(),
            response_streaming: METHOD_RESPONSE_STREAMING_DEFAULT_VALUE,
            options: crate::collections::RepeatedField::new(),
            syntax: METHOD_SYNTAX_DEFAULT_VALUE,
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.request_type_url = other.request_type_url.clone();
        self.request_streaming = other.request_streaming;
        self.response_type_url = other.response_type_url.clone();
        self.response_streaming = other.response_streaming;
        self.options.merge(&other.options);
        self.syntax = other.syntax;
    }
}
impl crate::Message for self::Method {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Method {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'request_type_url' field
    pub const REQUEST_TYPE_URL_FIELD_NUMBER: i32 = 2;
    /// Gets the field number of the 'request_streaming' field
    pub const REQUEST_STREAMING_FIELD_NUMBER: i32 = 3;
    /// Gets the field number of the 'response_type_url' field
    pub const RESPONSE_TYPE_URL_FIELD_NUMBER: i32 = 4;
    /// Gets the field number of the 'response_streaming' field
    pub const RESPONSE_STREAMING_FIELD_NUMBER: i32 = 5;
    /// Gets the field number of the 'options' field
    pub const OPTIONS_FIELD_NUMBER: i32 = 6;
    /// Gets the field number of the 'syntax' field
    pub const SYNTAX_FIELD_NUMBER: i32 = 7;
}
#[derive(Clone, PartialEq)]
pub struct Mixin {
    pub name: std::string::String,
    pub root: std::string::String,
    _unknown_fields: crate::UnknownFieldSet
}
static MIXIN_NAME_DEFAULT_VALUE: &'static str = "";
static MIXIN_ROOT_DEFAULT_VALUE: &'static str = "";
impl crate::CodedMessage for self::Mixin {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.root = input.read_string()?,
                _ => { }
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> std::option::Option<i32> {
        let mut size = 0i32;
        let name = &self.name;
        if name != MIXIN_NAME_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(name)?)?;
        }
        let root = &self.root;
        if root != MIXIN_ROOT_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::string(root)?)?;
        }
        std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != MIXIN_NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let root = &self.root;
        if root != MIXIN_ROOT_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[18])?;
            output.write_string(root)?;
        }
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Mixin {
    fn new() -> Self {
        Self {
            name: std::string::String::new(),
            root: std::string::String::new(),
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.root = other.root.clone();
    }
}
impl crate::Message for self::Mixin {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Mixin {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'root' field
    pub const ROOT_FIELD_NUMBER: i32 = 2;
}