// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/api.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 2]> = ::std::option::Option::None;

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut [
            10, 25, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 97, 112, 
            105, 46, 112, 114, 111, 116, 111, 18, 15, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 
            111, 98, 117, 102, 26, 36, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 
            102, 47, 115, 111, 117, 114, 99, 101, 95, 99, 111, 110, 116, 101, 120, 116, 46, 112, 114, 111, 
            116, 111, 26, 26, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 
            116, 121, 112, 101, 46, 112, 114, 111, 116, 111, 34, 193, 2, 10, 3, 65, 112, 105, 18, 18, 
            10, 4, 110, 97, 109, 101, 24, 1, 32, 1, 40, 9, 82, 4, 110, 97, 109, 101, 18, 49, 
            10, 7, 109, 101, 116, 104, 111, 100, 115, 24, 2, 32, 3, 40, 11, 50, 23, 46, 103, 111, 
            111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 77, 101, 116, 104, 111, 100, 
            82, 7, 109, 101, 116, 104, 111, 100, 115, 18, 49, 10, 7, 111, 112, 116, 105, 111, 110, 115, 
            24, 3, 32, 3, 40, 11, 50, 23, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 
            111, 98, 117, 102, 46, 79, 112, 116, 105, 111, 110, 82, 7, 111, 112, 116, 105, 111, 110, 115, 
            18, 24, 10, 7, 118, 101, 114, 115, 105, 111, 110, 24, 4, 32, 1, 40, 9, 82, 7, 118, 
            101, 114, 115, 105, 111, 110, 18, 69, 10, 14, 115, 111, 117, 114, 99, 101, 95, 99, 111, 110, 
            116, 101, 120, 116, 24, 5, 32, 1, 40, 11, 50, 30, 46, 103, 111, 111, 103, 108, 101, 46, 
            112, 114, 111, 116, 111, 98, 117, 102, 46, 83, 111, 117, 114, 99, 101, 67, 111, 110, 116, 101, 
            120, 116, 82, 13, 115, 111, 117, 114, 99, 101, 67, 111, 110, 116, 101, 120, 116, 18, 46, 10, 
            6, 109, 105, 120, 105, 110, 115, 24, 6, 32, 3, 40, 11, 50, 22, 46, 103, 111, 111, 103, 
            108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 77, 105, 120, 105, 110, 82, 6, 109, 
            105, 120, 105, 110, 115, 18, 47, 10, 6, 115, 121, 110, 116, 97, 120, 24, 7, 32, 1, 40, 
            14, 50, 23, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 
            83, 121, 110, 116, 97, 120, 82, 6, 115, 121, 110, 116, 97, 120, 34, 178, 2, 10, 6, 77, 
            101, 116, 104, 111, 100, 18, 18, 10, 4, 110, 97, 109, 101, 24, 1, 32, 1, 40, 9, 82, 
            4, 110, 97, 109, 101, 18, 40, 10, 16, 114, 101, 113, 117, 101, 115, 116, 95, 116, 121, 112, 
            101, 95, 117, 114, 108, 24, 2, 32, 1, 40, 9, 82, 14, 114, 101, 113, 117, 101, 115, 116, 
            84, 121, 112, 101, 85, 114, 108, 18, 43, 10, 17, 114, 101, 113, 117, 101, 115, 116, 95, 115, 
            116, 114, 101, 97, 109, 105, 110, 103, 24, 3, 32, 1, 40, 8, 82, 16, 114, 101, 113, 117, 
            101, 115, 116, 83, 116, 114, 101, 97, 109, 105, 110, 103, 18, 42, 10, 17, 114, 101, 115, 112, 
            111, 110, 115, 101, 95, 116, 121, 112, 101, 95, 117, 114, 108, 24, 4, 32, 1, 40, 9, 82, 
            15, 114, 101, 115, 112, 111, 110, 115, 101, 84, 121, 112, 101, 85, 114, 108, 18, 45, 10, 18, 
            114, 101, 115, 112, 111, 110, 115, 101, 95, 115, 116, 114, 101, 97, 109, 105, 110, 103, 24, 5, 
            32, 1, 40, 8, 82, 17, 114, 101, 115, 112, 111, 110, 115, 101, 83, 116, 114, 101, 97, 109, 
            105, 110, 103, 18, 49, 10, 7, 111, 112, 116, 105, 111, 110, 115, 24, 6, 32, 3, 40, 11, 
            50, 23, 46, 103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 46, 79, 
            112, 116, 105, 111, 110, 82, 7, 111, 112, 116, 105, 111, 110, 115, 18, 47, 10, 6, 115, 121, 
            110, 116, 97, 120, 24, 7, 32, 1, 40, 14, 50, 23, 46, 103, 111, 111, 103, 108, 101, 46, 
            112, 114, 111, 116, 111, 98, 117, 102, 46, 83, 121, 110, 116, 97, 120, 82, 6, 115, 121, 110, 
            116, 97, 120, 34, 47, 10, 5, 77, 105, 120, 105, 110, 18, 18, 10, 4, 110, 97, 109, 101, 
            24, 1, 32, 1, 40, 9, 82, 4, 110, 97, 109, 101, 18, 18, 10, 4, 114, 111, 111, 116, 
            24, 2, 32, 1, 40, 9, 82, 4, 114, 111, 111, 116, 66, 117, 10, 19, 99, 111, 109, 46, 
            103, 111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 66, 8, 65, 112, 105, 
            80, 114, 111, 116, 111, 80, 1, 90, 43, 103, 111, 111, 103, 108, 101, 46, 103, 111, 108, 97, 
            110, 103, 46, 111, 114, 103, 47, 103, 101, 110, 112, 114, 111, 116, 111, 47, 112, 114, 111, 116, 
            111, 98, 117, 102, 47, 97, 112, 105, 59, 97, 112, 105, 162, 2, 3, 71, 80, 66, 170, 2, 
            30, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 111, 98, 117, 102, 46, 87, 101, 108, 
            108, 75, 110, 111, 119, 110, 84, 121, 112, 101, 115, 98, 6, 112, 114, 111, 116, 111, 51, 
        ].as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([crate::wkt::source_context::pool(), crate::wkt::r#type::pool(), ]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap()
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/api.proto").unwrap());
    }
}

/// Gets the pool containing all the symbols in this proto file and its dependencies
pub fn pool() -> &'static crate::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
/// Gets the file descriptor representing the proto that created this generated file
pub fn file() -> &'static crate::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
/// Api is a light-weight descriptor for an API Interface.
///
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See https://cloud.google.com/apis/design/glossary for
/// detailed terminology.
#[derive(Clone, Debug, PartialEq)]
pub struct Api {
    name: ::std::string::String,
    methods: crate::collections::RepeatedField<self::Method>,
    options: crate::collections::RepeatedField<crate::wkt::r#type::Option>,
    version: ::std::string::String,
    source_context: ::std::option::Option<::std::boxed::Box<crate::wkt::source_context::SourceContext>>,
    mixins: crate::collections::RepeatedField<self::Mixin>,
    syntax: crate::EnumValue<crate::wkt::r#type::Syntax>,
    unknown_fields: crate::UnknownFieldSet
}
static API_METHODS_CODEC: crate::Codec<self::Method> = crate::Codec::message(18);
static API_OPTIONS_CODEC: crate::Codec<crate::wkt::r#type::Option> = crate::Codec::message(26);
static API_MIXINS_CODEC: crate::Codec<self::Mixin> = crate::Codec::message(50);
impl crate::CodedMessage for self::Api {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.methods.add_entries(tag.get(), input, &API_METHODS_CODEC)?,
                26 => self.options.add_entries(tag.get(), input, &API_OPTIONS_CODEC)?,
                34 => self.version = input.read_string()?,
                42 => input.read_message(self.source_context.get_or_insert_with(crate::LiteMessage::new))?,
                50 => self.mixins.add_entries(tag.get(), input, &API_MIXINS_CODEC)?,
                56 => self.syntax = input.read_enum_value()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        size += self.methods.calculate_size(&API_METHODS_CODEC);
        size += self.options.calculate_size(&API_OPTIONS_CODEC);
        let version = &self.version;
        if version != Self::VERSION_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(version);
        }
        let source_context = &self.source_context;
        if let ::std::option::Option::Some(source_context) = source_context {
            size += 1;
            size += crate::io::sizes::message(source_context);
        }
        size += self.mixins.calculate_size(&API_MIXINS_CODEC);
        let syntax = self.syntax;
        if syntax != Self::SYNTAX_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::enum_value(syntax);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        self.methods.write_to(output, &API_METHODS_CODEC)?;
        self.options.write_to(output, &API_OPTIONS_CODEC)?;
        let version = &self.version;
        if version != Self::VERSION_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(version)?;
        }
        let source_context = &self.source_context;
        if let ::std::option::Option::Some(source_context) = source_context {
            output.write_raw_tag_bytes(&[42])?;
            output.write_message(source_context)?;
        }
        self.mixins.write_to(output, &API_MIXINS_CODEC)?;
        let syntax = self.syntax;
        if syntax != Self::SYNTAX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[56])?;
            output.write_enum_value(syntax)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Api {
    fn new() -> Self {
        Self {
            name: ::std::string::String::new(),
            methods: crate::collections::RepeatedField::new(),
            options: crate::collections::RepeatedField::new(),
            version: ::std::string::String::new(),
            source_context: ::std::option::Option::None,
            mixins: crate::collections::RepeatedField::new(),
            syntax: Self::SYNTAX_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.methods.merge(&other.methods);
        self.options.merge(&other.options);
        self.version = other.version.clone();
        if let ::std::option::Option::Some(source_context) = &other.source_context {
            self.source_context.get_or_insert_with(crate::LiteMessage::new).merge(source_context);
        }
        self.mixins.merge(&other.mixins);
        self.syntax = other.syntax;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Api {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Api {
    /// Gets the field number of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    pub fn name(&self) -> &::std::string::String {
        &self.name
    }
    /// Returns a unique reference to the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
    /// Gets the field number of the [`methods`] field
    ///
    /// [`methods`]: #method.methods
    pub const METHODS_FIELD_NUMBER: i32 = 2;
    /// The methods of this interface, in unspecified order.
    pub fn methods(&self) -> &crate::collections::RepeatedField<self::Method> {
        &self.methods
    }
    /// Returns a unique reference to the [`methods`] field
    ///
    /// [`methods`]: #method.methods
    pub fn methods_mut(&mut self) -> &mut crate::collections::RepeatedField<self::Method> {
        &mut self.methods
    }
    /// Gets the field number of the [`options`] field
    ///
    /// [`options`]: #method.options
    pub const OPTIONS_FIELD_NUMBER: i32 = 3;
    /// Any metadata attached to the interface.
    pub fn options(&self) -> &crate::collections::RepeatedField<crate::wkt::r#type::Option> {
        &self.options
    }
    /// Returns a unique reference to the [`options`] field
    ///
    /// [`options`]: #method.options
    pub fn options_mut(&mut self) -> &mut crate::collections::RepeatedField<crate::wkt::r#type::Option> {
        &mut self.options
    }
    /// Gets the field number of the [`version`] field
    ///
    /// [`version`]: #method.version
    pub const VERSION_FIELD_NUMBER: i32 = 4;
    /// A constant value representing the default value of the [`version`] field
    ///
    /// [`version`]: #method.version
    pub const VERSION_DEFAULT_VALUE: &'static str = "";
    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    ///
    /// The versioning schema uses [semantic
    /// versioning](http://semver.org) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    ///
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    ///
    ///
    pub fn version(&self) -> &::std::string::String {
        &self.version
    }
    /// Returns a unique reference to the [`version`] field
    ///
    /// [`version`]: #method.version
    pub fn version_mut(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }
    /// Gets the field number of the [`source_context`] field
    ///
    /// [`source_context`]: #method.source_context
    pub const SOURCE_CONTEXT_FIELD_NUMBER: i32 = 5;
    /// Source context for the protocol buffer service represented by this
    /// message.
    pub fn source_context(&self) -> &::std::option::Option<::std::boxed::Box<crate::wkt::source_context::SourceContext>> {
        &self.source_context
    }
    /// Returns a unique reference to the [`source_context`] field
    ///
    /// [`source_context`]: #method.source_context
    pub fn source_context_mut(&mut self) -> &mut ::std::option::Option<::std::boxed::Box<crate::wkt::source_context::SourceContext>> {
        &mut self.source_context
    }
    /// Gets the field number of the [`mixins`] field
    ///
    /// [`mixins`]: #method.mixins
    pub const MIXINS_FIELD_NUMBER: i32 = 6;
    /// Included interfaces. See [Mixin][].
    pub fn mixins(&self) -> &crate::collections::RepeatedField<self::Mixin> {
        &self.mixins
    }
    /// Returns a unique reference to the [`mixins`] field
    ///
    /// [`mixins`]: #method.mixins
    pub fn mixins_mut(&mut self) -> &mut crate::collections::RepeatedField<self::Mixin> {
        &mut self.mixins
    }
    /// Gets the field number of the [`syntax`] field
    ///
    /// [`syntax`]: #method.syntax
    pub const SYNTAX_FIELD_NUMBER: i32 = 7;
    /// A constant value representing the default value of the [`syntax`] field
    ///
    /// [`syntax`]: #method.syntax
    pub const SYNTAX_DEFAULT_VALUE: crate::EnumValue<crate::wkt::r#type::Syntax> = crate::EnumValue::Defined(crate::wkt::r#type::Syntax::Proto2);
    /// The source syntax of the service.
    pub fn syntax(&self) -> crate::EnumValue<crate::wkt::r#type::Syntax> {
        self.syntax
    }
    /// Returns a unique reference to the [`syntax`] field
    ///
    /// [`syntax`]: #method.syntax
    pub fn syntax_mut(&mut self) -> &mut crate::EnumValue<crate::wkt::r#type::Syntax> {
        &mut self.syntax
    }
}
/// Method represents a method of an API interface.
#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    name: ::std::string::String,
    request_type_url: ::std::string::String,
    request_streaming: bool,
    response_type_url: ::std::string::String,
    response_streaming: bool,
    options: crate::collections::RepeatedField<crate::wkt::r#type::Option>,
    syntax: crate::EnumValue<crate::wkt::r#type::Syntax>,
    unknown_fields: crate::UnknownFieldSet
}
static METHOD_OPTIONS_CODEC: crate::Codec<crate::wkt::r#type::Option> = crate::Codec::message(50);
impl crate::CodedMessage for self::Method {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.request_type_url = input.read_string()?,
                24 => self.request_streaming = input.read_bool()?,
                34 => self.response_type_url = input.read_string()?,
                40 => self.response_streaming = input.read_bool()?,
                50 => self.options.add_entries(tag.get(), input, &METHOD_OPTIONS_CODEC)?,
                56 => self.syntax = input.read_enum_value()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        let request_type_url = &self.request_type_url;
        if request_type_url != Self::REQUEST_TYPE_URL_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(request_type_url);
        }
        let request_streaming = self.request_streaming;
        if request_streaming != Self::REQUEST_STREAMING_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::bool(request_streaming);
        }
        let response_type_url = &self.response_type_url;
        if response_type_url != Self::RESPONSE_TYPE_URL_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(response_type_url);
        }
        let response_streaming = self.response_streaming;
        if response_streaming != Self::RESPONSE_STREAMING_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::bool(response_streaming);
        }
        size += self.options.calculate_size(&METHOD_OPTIONS_CODEC);
        let syntax = self.syntax;
        if syntax != Self::SYNTAX_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::enum_value(syntax);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let request_type_url = &self.request_type_url;
        if request_type_url != Self::REQUEST_TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[18])?;
            output.write_string(request_type_url)?;
        }
        let request_streaming = self.request_streaming;
        if request_streaming != Self::REQUEST_STREAMING_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[24])?;
            output.write_bool(request_streaming)?;
        }
        let response_type_url = &self.response_type_url;
        if response_type_url != Self::RESPONSE_TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[34])?;
            output.write_string(response_type_url)?;
        }
        let response_streaming = self.response_streaming;
        if response_streaming != Self::RESPONSE_STREAMING_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[40])?;
            output.write_bool(response_streaming)?;
        }
        self.options.write_to(output, &METHOD_OPTIONS_CODEC)?;
        let syntax = self.syntax;
        if syntax != Self::SYNTAX_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[56])?;
            output.write_enum_value(syntax)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Method {
    fn new() -> Self {
        Self {
            name: ::std::string::String::new(),
            request_type_url: ::std::string::String::new(),
            request_streaming: Self::REQUEST_STREAMING_DEFAULT_VALUE,
            response_type_url: ::std::string::String::new(),
            response_streaming: Self::RESPONSE_STREAMING_DEFAULT_VALUE,
            options: crate::collections::RepeatedField::new(),
            syntax: Self::SYNTAX_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
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
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Method {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[1]
    }
}
impl self::Method {
    /// Gets the field number of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    /// The simple name of this method.
    pub fn name(&self) -> &::std::string::String {
        &self.name
    }
    /// Returns a unique reference to the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
    /// Gets the field number of the [`request_type_url`] field
    ///
    /// [`request_type_url`]: #method.request_type_url
    pub const REQUEST_TYPE_URL_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`request_type_url`] field
    ///
    /// [`request_type_url`]: #method.request_type_url
    pub const REQUEST_TYPE_URL_DEFAULT_VALUE: &'static str = "";
    /// A URL of the input message type.
    pub fn request_type_url(&self) -> &::std::string::String {
        &self.request_type_url
    }
    /// Returns a unique reference to the [`request_type_url`] field
    ///
    /// [`request_type_url`]: #method.request_type_url
    pub fn request_type_url_mut(&mut self) -> &mut ::std::string::String {
        &mut self.request_type_url
    }
    /// Gets the field number of the [`request_streaming`] field
    ///
    /// [`request_streaming`]: #method.request_streaming
    pub const REQUEST_STREAMING_FIELD_NUMBER: i32 = 3;
    /// A constant value representing the default value of the [`request_streaming`] field
    ///
    /// [`request_streaming`]: #method.request_streaming
    pub const REQUEST_STREAMING_DEFAULT_VALUE: bool = false;
    /// If true, the request is streamed.
    pub fn request_streaming(&self) -> bool {
        self.request_streaming
    }
    /// Returns a unique reference to the [`request_streaming`] field
    ///
    /// [`request_streaming`]: #method.request_streaming
    pub fn request_streaming_mut(&mut self) -> &mut bool {
        &mut self.request_streaming
    }
    /// Gets the field number of the [`response_type_url`] field
    ///
    /// [`response_type_url`]: #method.response_type_url
    pub const RESPONSE_TYPE_URL_FIELD_NUMBER: i32 = 4;
    /// A constant value representing the default value of the [`response_type_url`] field
    ///
    /// [`response_type_url`]: #method.response_type_url
    pub const RESPONSE_TYPE_URL_DEFAULT_VALUE: &'static str = "";
    /// The URL of the output message type.
    pub fn response_type_url(&self) -> &::std::string::String {
        &self.response_type_url
    }
    /// Returns a unique reference to the [`response_type_url`] field
    ///
    /// [`response_type_url`]: #method.response_type_url
    pub fn response_type_url_mut(&mut self) -> &mut ::std::string::String {
        &mut self.response_type_url
    }
    /// Gets the field number of the [`response_streaming`] field
    ///
    /// [`response_streaming`]: #method.response_streaming
    pub const RESPONSE_STREAMING_FIELD_NUMBER: i32 = 5;
    /// A constant value representing the default value of the [`response_streaming`] field
    ///
    /// [`response_streaming`]: #method.response_streaming
    pub const RESPONSE_STREAMING_DEFAULT_VALUE: bool = false;
    /// If true, the response is streamed.
    pub fn response_streaming(&self) -> bool {
        self.response_streaming
    }
    /// Returns a unique reference to the [`response_streaming`] field
    ///
    /// [`response_streaming`]: #method.response_streaming
    pub fn response_streaming_mut(&mut self) -> &mut bool {
        &mut self.response_streaming
    }
    /// Gets the field number of the [`options`] field
    ///
    /// [`options`]: #method.options
    pub const OPTIONS_FIELD_NUMBER: i32 = 6;
    /// Any metadata attached to the method.
    pub fn options(&self) -> &crate::collections::RepeatedField<crate::wkt::r#type::Option> {
        &self.options
    }
    /// Returns a unique reference to the [`options`] field
    ///
    /// [`options`]: #method.options
    pub fn options_mut(&mut self) -> &mut crate::collections::RepeatedField<crate::wkt::r#type::Option> {
        &mut self.options
    }
    /// Gets the field number of the [`syntax`] field
    ///
    /// [`syntax`]: #method.syntax
    pub const SYNTAX_FIELD_NUMBER: i32 = 7;
    /// A constant value representing the default value of the [`syntax`] field
    ///
    /// [`syntax`]: #method.syntax
    pub const SYNTAX_DEFAULT_VALUE: crate::EnumValue<crate::wkt::r#type::Syntax> = crate::EnumValue::Defined(crate::wkt::r#type::Syntax::Proto2);
    /// The source syntax of this method.
    pub fn syntax(&self) -> crate::EnumValue<crate::wkt::r#type::Syntax> {
        self.syntax
    }
    /// Returns a unique reference to the [`syntax`] field
    ///
    /// [`syntax`]: #method.syntax
    pub fn syntax_mut(&mut self) -> &mut crate::EnumValue<crate::wkt::r#type::Syntax> {
        &mut self.syntax
    }
}
/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
///
/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
///
/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
///
/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the [root][] path if
///   specified.
///
/// Example of a simple mixin:
///
///     package google.acl.v1;
///     service AccessControl {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v1/{resource=**}:getAcl";
///       }
///     }
///
///     package google.storage.v2;
///     service Storage {
///       rpc GetAcl(GetAclRequest) returns (Acl);
///
///       // Get a data record.
///       rpc GetData(GetDataRequest) returns (Data) {
///         option (google.api.http).get = "/v2/{resource=**}";
///       }
///     }
///
/// Example of a mixin configuration:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inherting
/// documentation and annotations as follows:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/{resource=**}:getAcl";
///       }
///       ...
///     }
///
/// Note how the version in the path pattern changed from `v1` to `v2`.
///
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///         root: acls
///
/// This implies the following inherited HTTP annotation:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///       }
///       ...
///     }
#[derive(Clone, Debug, PartialEq)]
pub struct Mixin {
    name: ::std::string::String,
    root: ::std::string::String,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Mixin {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = input.read_string()?,
                18 => self.root = input.read_string()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(name);
        }
        let root = &self.root;
        if root != Self::ROOT_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(root);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if name != Self::NAME_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(name)?;
        }
        let root = &self.root;
        if root != Self::ROOT_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[18])?;
            output.write_string(root)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Mixin {
    fn new() -> Self {
        Self {
            name: ::std::string::String::new(),
            root: ::std::string::String::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.root = other.root.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Mixin {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[2]
    }
}
impl self::Mixin {
    /// Gets the field number of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    /// The fully qualified name of the interface which is included.
    pub fn name(&self) -> &::std::string::String {
        &self.name
    }
    /// Returns a unique reference to the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
    /// Gets the field number of the [`root`] field
    ///
    /// [`root`]: #method.root
    pub const ROOT_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`root`] field
    ///
    /// [`root`]: #method.root
    pub const ROOT_DEFAULT_VALUE: &'static str = "";
    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    pub fn root(&self) -> &::std::string::String {
        &self.root
    }
    /// Returns a unique reference to the [`root`] field
    ///
    /// [`root`]: #method.root
    pub fn root_mut(&mut self) -> &mut ::std::string::String {
        &mut self.root
    }
}