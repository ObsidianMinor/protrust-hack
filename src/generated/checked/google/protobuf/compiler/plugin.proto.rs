// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/compiler/plugin.proto


pub fn file() -> &'static crate::reflect::FileDescriptor {
    super::pool().find_file_by_name("google/protobuf/compiler/plugin.proto").unwrap()
}

/// The version number of protocol compiler.
#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    major: ::std::option::Option<i32>,
    minor: ::std::option::Option<i32>,
    patch: ::std::option::Option<i32>,
    suffix: ::std::option::Option<::std::string::String>,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Version {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.major = ::std::option::Option::Some(input.read_int32()?),
                16 => self.minor = ::std::option::Option::Some(input.read_int32()?),
                24 => self.patch = ::std::option::Option::Some(input.read_int32()?),
                34 => self.suffix = ::std::option::Option::Some(input.read_string()?),
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let major = self.major;
        if let ::std::option::Option::Some(major) = major {
            if major != Self::MAJOR_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::int32(major));
            }
        }
        let minor = self.minor;
        if let ::std::option::Option::Some(minor) = minor {
            if minor != Self::MINOR_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::int32(minor));
            }
        }
        let patch = self.patch;
        if let ::std::option::Option::Some(patch) = patch {
            if patch != Self::PATCH_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::int32(patch));
            }
        }
        let suffix = &self.suffix;
        if let ::std::option::Option::Some(suffix) = suffix {
            if suffix != Self::SUFFIX_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(suffix));
            }
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let major = self.major;
        if let ::std::option::Option::Some(major) = major {
            if major != Self::MAJOR_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[8])?;
                output.write_int32(major)?;
            }
        }
        let minor = self.minor;
        if let ::std::option::Option::Some(minor) = minor {
            if minor != Self::MINOR_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[16])?;
                output.write_int32(minor)?;
            }
        }
        let patch = self.patch;
        if let ::std::option::Option::Some(patch) = patch {
            if patch != Self::PATCH_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[24])?;
                output.write_int32(patch)?;
            }
        }
        let suffix = &self.suffix;
        if let ::std::option::Option::Some(suffix) = suffix {
            if suffix != Self::SUFFIX_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[34])?;
                output.write_string(suffix)?;
            }
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Version {
    fn new() -> Self {
        Self {
            major: ::std::option::Option::None,
            minor: ::std::option::Option::None,
            patch: ::std::option::Option::None,
            suffix: ::std::option::Option::None,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.major = other.major;
        self.minor = other.minor;
        self.patch = other.patch;
        self.suffix = other.suffix.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Version {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Version {
    /// Gets the field number of the [`major`] field
    ///
    /// [`major`]: #method.major
    pub const MAJOR_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`major`] field
    ///
    /// [`major`]: #method.major
    pub const MAJOR_DEFAULT_VALUE: i32 = 0;
    pub fn major(&self) -> i32 {
        self.major.unwrap_or(Self::MAJOR_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`major`] field
    ///
    /// [`major`]: #method.major
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn major_option(&self) -> ::std::option::Option<i32> {
        self.major
    }
    /// Returns a bool indicating the presence of the [`major`] field
    ///
    /// [`major`]: #method.major
    pub fn has_major(&self) -> bool {
        self.major.is_some()
    }
    /// Sets the value of the [`major`] field
    ///
    /// [`major`]: #method.major
    pub fn set_major(&mut self, value: i32) {
        self.major = ::std::option::Option::Some(value)
    }
    /// Clears the value of the [`major`] field
    ///
    /// [`major`]: #method.major
    pub fn clear_major(&mut self) {
        self.major = ::std::option::Option::None
    }
    /// Gets the field number of the [`minor`] field
    ///
    /// [`minor`]: #method.minor
    pub const MINOR_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`minor`] field
    ///
    /// [`minor`]: #method.minor
    pub const MINOR_DEFAULT_VALUE: i32 = 0;
    pub fn minor(&self) -> i32 {
        self.minor.unwrap_or(Self::MINOR_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`minor`] field
    ///
    /// [`minor`]: #method.minor
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn minor_option(&self) -> ::std::option::Option<i32> {
        self.minor
    }
    /// Returns a bool indicating the presence of the [`minor`] field
    ///
    /// [`minor`]: #method.minor
    pub fn has_minor(&self) -> bool {
        self.minor.is_some()
    }
    /// Sets the value of the [`minor`] field
    ///
    /// [`minor`]: #method.minor
    pub fn set_minor(&mut self, value: i32) {
        self.minor = ::std::option::Option::Some(value)
    }
    /// Clears the value of the [`minor`] field
    ///
    /// [`minor`]: #method.minor
    pub fn clear_minor(&mut self) {
        self.minor = ::std::option::Option::None
    }
    /// Gets the field number of the [`patch`] field
    ///
    /// [`patch`]: #method.patch
    pub const PATCH_FIELD_NUMBER: i32 = 3;
    /// A constant value representing the default value of the [`patch`] field
    ///
    /// [`patch`]: #method.patch
    pub const PATCH_DEFAULT_VALUE: i32 = 0;
    pub fn patch(&self) -> i32 {
        self.patch.unwrap_or(Self::PATCH_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`patch`] field
    ///
    /// [`patch`]: #method.patch
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn patch_option(&self) -> ::std::option::Option<i32> {
        self.patch
    }
    /// Returns a bool indicating the presence of the [`patch`] field
    ///
    /// [`patch`]: #method.patch
    pub fn has_patch(&self) -> bool {
        self.patch.is_some()
    }
    /// Sets the value of the [`patch`] field
    ///
    /// [`patch`]: #method.patch
    pub fn set_patch(&mut self, value: i32) {
        self.patch = ::std::option::Option::Some(value)
    }
    /// Clears the value of the [`patch`] field
    ///
    /// [`patch`]: #method.patch
    pub fn clear_patch(&mut self) {
        self.patch = ::std::option::Option::None
    }
    /// Gets the field number of the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    pub const SUFFIX_FIELD_NUMBER: i32 = 4;
    /// A constant value representing the default value of the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    pub const SUFFIX_DEFAULT_VALUE: &'static str = "";
    /// A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
    /// be empty for mainline stable releases.
    pub fn suffix(&self) -> &str {
        self.suffix.as_ref().map(|v| &**v).unwrap_or(Self::SUFFIX_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn suffix_option(&self) -> ::std::option::Option<&::std::string::String> {
        self.suffix.as_ref()
    }
    /// Returns a unique reference to the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    pub fn suffix_mut(&mut self) -> &mut ::std::string::String {
        self.suffix.get_or_insert_with(::std::string::String::new)
    }
    /// Returns a bool indicating the presence of the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    pub fn has_suffix(&self) -> bool {
        self.suffix.is_some()
    }
    /// Sets the value of the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    pub fn set_suffix(&mut self, value: ::std::string::String) {
        self.suffix = ::std::option::Option::Some(value)
    }
    /// Takes the value of the [`suffix`] field, leaving it empty
    ///
    /// [`suffix`]: #method.suffix
    pub fn take_suffix(&mut self) -> ::std::option::Option<::std::string::String> {
        self.suffix.take()
    }
    /// Clears the value of the [`suffix`] field
    ///
    /// [`suffix`]: #method.suffix
    pub fn clear_suffix(&mut self) {
        self.suffix = ::std::option::Option::None
    }
}
/// An encoded CodeGeneratorRequest is written to the plugin's stdin.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeGeneratorRequest {
    file_to_generate: crate::collections::RepeatedField<::std::string::String>,
    parameter: ::std::option::Option<::std::string::String>,
    proto_file: crate::collections::RepeatedField<crate::descriptor::FileDescriptorProto>,
    compiler_version: ::std::option::Option<::std::boxed::Box<self::Version>>,
    unknown_fields: crate::UnknownFieldSet
}
static CODE_GENERATOR_REQUEST_FILE_TO_GENERATE_CODEC: crate::Codec<::std::string::String> = crate::Codec::string(10);
static CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC: crate::Codec<crate::descriptor::FileDescriptorProto> = crate::Codec::message(122);
impl crate::CodedMessage for self::CodeGeneratorRequest {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.file_to_generate.add_entries(input, &CODE_GENERATOR_REQUEST_FILE_TO_GENERATE_CODEC)?,
                18 => self.parameter = ::std::option::Option::Some(input.read_string()?),
                122 => self.proto_file.add_entries(input, &CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC)?,
                26 => input.read_message(&mut **self.compiler_version.get_or_insert_with(|| ::std::boxed::Box::new(crate::LiteMessage::new())))?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        size = size.checked_add(self.file_to_generate.calculate_size(&CODE_GENERATOR_REQUEST_FILE_TO_GENERATE_CODEC)?)?;
        let parameter = &self.parameter;
        if let ::std::option::Option::Some(parameter) = parameter {
            if parameter != Self::PARAMETER_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(parameter));
            }
        }
        size = size.checked_add(self.proto_file.calculate_size(&CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC)?)?;
        let compiler_version = &self.compiler_version;
        if let ::std::option::Option::Some(compiler_version) = compiler_version {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::message(&**compiler_version));
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.file_to_generate.write_to(output, &CODE_GENERATOR_REQUEST_FILE_TO_GENERATE_CODEC)?;
        let parameter = &self.parameter;
        if let ::std::option::Option::Some(parameter) = parameter {
            if parameter != Self::PARAMETER_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[18])?;
                output.write_string(parameter)?;
            }
        }
        self.proto_file.write_to(output, &CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC)?;
        let compiler_version = &self.compiler_version;
        if let ::std::option::Option::Some(compiler_version) = compiler_version {
            output.write_raw_tag_bytes(&[26])?;
            output.write_message(&**compiler_version)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
    fn is_initialized(&self) -> bool {
        if !self.proto_file.is_initialized() {
            return false;
        }
        if let Some(compiler_version) = &self.compiler_version {
            if !crate::CodedMessage::is_initialized(&**compiler_version) {
                return false;
            }
        }
        true
    }
}
impl crate::LiteMessage for self::CodeGeneratorRequest {
    fn new() -> Self {
        Self {
            file_to_generate: crate::collections::RepeatedField::new(),
            parameter: ::std::option::Option::None,
            proto_file: crate::collections::RepeatedField::new(),
            compiler_version: ::std::option::Option::None,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.file_to_generate.merge(&other.file_to_generate);
        self.parameter = other.parameter.clone();
        self.proto_file.merge(&other.proto_file);
        if let ::std::option::Option::Some(compiler_version) = &other.compiler_version {
            self.compiler_version.get_or_insert_with(|| ::std::boxed::Box::new(crate::LiteMessage::new())).merge(compiler_version);
        }
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::CodeGeneratorRequest {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[1]
    }
}
impl self::CodeGeneratorRequest {
    /// Gets the field number of the [`file_to_generate`] field
    ///
    /// [`file_to_generate`]: #method.file_to_generate
    pub const FILE_TO_GENERATE_FIELD_NUMBER: i32 = 1;
    /// The .proto files that were explicitly listed on the command-line.  The
    /// code generator should generate code only for these files.  Each file's
    /// descriptor will be included in proto_file, below.
    pub fn file_to_generate(&self) -> &crate::collections::RepeatedField<::std::string::String> {
        &self.file_to_generate
    }
    /// Returns a unique reference to the [`file_to_generate`] field
    ///
    /// [`file_to_generate`]: #method.file_to_generate
    pub fn file_to_generate_mut(&mut self) -> &mut crate::collections::RepeatedField<::std::string::String> {
        &mut self.file_to_generate
    }
    /// Gets the field number of the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    pub const PARAMETER_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    pub const PARAMETER_DEFAULT_VALUE: &'static str = "";
    /// The generator parameter passed on the command-line.
    pub fn parameter(&self) -> &str {
        self.parameter.as_ref().map(|v| &**v).unwrap_or(Self::PARAMETER_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn parameter_option(&self) -> ::std::option::Option<&::std::string::String> {
        self.parameter.as_ref()
    }
    /// Returns a unique reference to the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    pub fn parameter_mut(&mut self) -> &mut ::std::string::String {
        self.parameter.get_or_insert_with(::std::string::String::new)
    }
    /// Returns a bool indicating the presence of the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    pub fn has_parameter(&self) -> bool {
        self.parameter.is_some()
    }
    /// Sets the value of the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    pub fn set_parameter(&mut self, value: ::std::string::String) {
        self.parameter = ::std::option::Option::Some(value)
    }
    /// Takes the value of the [`parameter`] field, leaving it empty
    ///
    /// [`parameter`]: #method.parameter
    pub fn take_parameter(&mut self) -> ::std::option::Option<::std::string::String> {
        self.parameter.take()
    }
    /// Clears the value of the [`parameter`] field
    ///
    /// [`parameter`]: #method.parameter
    pub fn clear_parameter(&mut self) {
        self.parameter = ::std::option::Option::None
    }
    /// Gets the field number of the [`proto_file`] field
    ///
    /// [`proto_file`]: #method.proto_file
    pub const PROTO_FILE_FIELD_NUMBER: i32 = 15;
    /// FileDescriptorProtos for all files in files_to_generate and everything
    /// they import.  The files will appear in topological order, so each file
    /// appears before any file that imports it.
    /// 
    /// protoc guarantees that all proto_files will be written after
    /// the fields above, even though this is not technically guaranteed by the
    /// protobuf wire format.  This theoretically could allow a plugin to stream
    /// in the FileDescriptorProtos and handle them one by one rather than read
    /// the entire set into memory at once.  However, as of this writing, this
    /// is not similarly optimized on protoc's end -- it will store all fields in
    /// memory at once before sending them to the plugin.
    /// 
    /// Type names of fields and extensions in the FileDescriptorProto are always
    /// fully qualified.
    pub fn proto_file(&self) -> &crate::collections::RepeatedField<crate::descriptor::FileDescriptorProto> {
        &self.proto_file
    }
    /// Returns a unique reference to the [`proto_file`] field
    ///
    /// [`proto_file`]: #method.proto_file
    pub fn proto_file_mut(&mut self) -> &mut crate::collections::RepeatedField<crate::descriptor::FileDescriptorProto> {
        &mut self.proto_file
    }
    /// Gets the field number of the [`compiler_version`] field
    ///
    /// [`compiler_version`]: #method.compiler_version
    pub const COMPILER_VERSION_FIELD_NUMBER: i32 = 3;
    /// The version number of protocol compiler.
    pub fn compiler_version_option(&self) -> ::std::option::Option<&self::Version> {
        self.compiler_version.as_ref().map(|b| &**b)
    }
    /// Returns a unique reference to the [`compiler_version`] field
    ///
    /// [`compiler_version`]: #method.compiler_version
    pub fn compiler_version_mut(&mut self) -> &mut self::Version {
        self.compiler_version.get_or_insert_with(|| ::std::boxed::Box::new(crate::LiteMessage::new())).as_mut()
    }
    /// Returns a bool indicating the presence of the [`compiler_version`] field
    ///
    /// [`compiler_version`]: #method.compiler_version
    pub fn has_compiler_version(&self) -> bool {
        self.compiler_version.is_some()
    }
    /// Sets the value of the [`compiler_version`] field
    ///
    /// [`compiler_version`]: #method.compiler_version
    pub fn set_compiler_version(&mut self, value: self::Version) {
        self.compiler_version = ::std::option::Option::Some(::std::boxed::Box::new(value))
    }
    /// Takes the value of the [`compiler_version`] field, leaving it empty
    ///
    /// [`compiler_version`]: #method.compiler_version
    pub fn take_compiler_version(&mut self) -> ::std::option::Option<self::Version> {
        self.compiler_version.take().map(|b| *b)
    }
    /// Clears the value of the [`compiler_version`] field
    ///
    /// [`compiler_version`]: #method.compiler_version
    pub fn clear_compiler_version(&mut self) {
        self.compiler_version = ::std::option::Option::None
    }
}
/// The plugin writes an encoded CodeGeneratorResponse to stdout.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeGeneratorResponse {
    error: ::std::option::Option<::std::string::String>,
    file: crate::collections::RepeatedField<self::CodeGeneratorResponse_File>,
    unknown_fields: crate::UnknownFieldSet
}
static CODE_GENERATOR_RESPONSE_FILE_CODEC: crate::Codec<self::CodeGeneratorResponse_File> = crate::Codec::message(122);
impl crate::CodedMessage for self::CodeGeneratorResponse {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.error = ::std::option::Option::Some(input.read_string()?),
                122 => self.file.add_entries(input, &CODE_GENERATOR_RESPONSE_FILE_CODEC)?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let error = &self.error;
        if let ::std::option::Option::Some(error) = error {
            if error != Self::ERROR_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(error));
            }
        }
        size = size.checked_add(self.file.calculate_size(&CODE_GENERATOR_RESPONSE_FILE_CODEC)?)?;
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let error = &self.error;
        if let ::std::option::Option::Some(error) = error {
            if error != Self::ERROR_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[10])?;
                output.write_string(error)?;
            }
        }
        self.file.write_to(output, &CODE_GENERATOR_RESPONSE_FILE_CODEC)?;
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
    fn is_initialized(&self) -> bool {
        if !self.file.is_initialized() {
            return false;
        }
        true
    }
}
impl crate::LiteMessage for self::CodeGeneratorResponse {
    fn new() -> Self {
        Self {
            error: ::std::option::Option::None,
            file: crate::collections::RepeatedField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.error = other.error.clone();
        self.file.merge(&other.file);
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::CodeGeneratorResponse {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[2]
    }
}
impl self::CodeGeneratorResponse {
    /// Gets the field number of the [`error`] field
    ///
    /// [`error`]: #method.error
    pub const ERROR_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`error`] field
    ///
    /// [`error`]: #method.error
    pub const ERROR_DEFAULT_VALUE: &'static str = "";
    /// Error message.  If non-empty, code generation failed.  The plugin process
    /// should exit with status code zero even if it reports an error in this way.
    /// 
    /// This should be used to indicate errors in .proto files which prevent the
    /// code generator from generating correct code.  Errors which indicate a
    /// problem in protoc itself -- such as the input CodeGeneratorRequest being
    /// unparseable -- should be reported by writing a message to stderr and
    /// exiting with a non-zero status code.
    pub fn error(&self) -> &str {
        self.error.as_ref().map(|v| &**v).unwrap_or(Self::ERROR_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`error`] field
    ///
    /// [`error`]: #method.error
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn error_option(&self) -> ::std::option::Option<&::std::string::String> {
        self.error.as_ref()
    }
    /// Returns a unique reference to the [`error`] field
    ///
    /// [`error`]: #method.error
    pub fn error_mut(&mut self) -> &mut ::std::string::String {
        self.error.get_or_insert_with(::std::string::String::new)
    }
    /// Returns a bool indicating the presence of the [`error`] field
    ///
    /// [`error`]: #method.error
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    /// Sets the value of the [`error`] field
    ///
    /// [`error`]: #method.error
    pub fn set_error(&mut self, value: ::std::string::String) {
        self.error = ::std::option::Option::Some(value)
    }
    /// Takes the value of the [`error`] field, leaving it empty
    ///
    /// [`error`]: #method.error
    pub fn take_error(&mut self) -> ::std::option::Option<::std::string::String> {
        self.error.take()
    }
    /// Clears the value of the [`error`] field
    ///
    /// [`error`]: #method.error
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    /// Gets the field number of the [`file`] field
    ///
    /// [`file`]: #method.file
    pub const FILE_FIELD_NUMBER: i32 = 15;
    pub fn file(&self) -> &crate::collections::RepeatedField<self::CodeGeneratorResponse_File> {
        &self.file
    }
    /// Returns a unique reference to the [`file`] field
    ///
    /// [`file`]: #method.file
    pub fn file_mut(&mut self) -> &mut crate::collections::RepeatedField<self::CodeGeneratorResponse_File> {
        &mut self.file
    }
}
/// Represents a single generated file.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeGeneratorResponse_File {
    name: ::std::option::Option<::std::string::String>,
    insertion_point: ::std::option::Option<::std::string::String>,
    content: ::std::option::Option<::std::string::String>,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::CodeGeneratorResponse_File {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = ::std::option::Option::Some(input.read_string()?),
                18 => self.insertion_point = ::std::option::Option::Some(input.read_string()?),
                122 => self.content = ::std::option::Option::Some(input.read_string()?),
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let name = &self.name;
        if let ::std::option::Option::Some(name) = name {
            if name != Self::NAME_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(name));
            }
        }
        let insertion_point = &self.insertion_point;
        if let ::std::option::Option::Some(insertion_point) = insertion_point {
            if insertion_point != Self::INSERTION_POINT_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(insertion_point));
            }
        }
        let content = &self.content;
        if let ::std::option::Option::Some(content) = content {
            if content != Self::CONTENT_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(content));
            }
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let name = &self.name;
        if let ::std::option::Option::Some(name) = name {
            if name != Self::NAME_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[10])?;
                output.write_string(name)?;
            }
        }
        let insertion_point = &self.insertion_point;
        if let ::std::option::Option::Some(insertion_point) = insertion_point {
            if insertion_point != Self::INSERTION_POINT_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[18])?;
                output.write_string(insertion_point)?;
            }
        }
        let content = &self.content;
        if let ::std::option::Option::Some(content) = content {
            if content != Self::CONTENT_DEFAULT_VALUE {
                output.write_raw_tag_bytes(&[122])?;
                output.write_string(content)?;
            }
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::CodeGeneratorResponse_File {
    fn new() -> Self {
        Self {
            name: ::std::option::Option::None,
            insertion_point: ::std::option::Option::None,
            content: ::std::option::Option::None,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.insertion_point = other.insertion_point.clone();
        self.content = other.content.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::CodeGeneratorResponse_File {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[2].messages()[0]
    }
}
impl self::CodeGeneratorResponse_File {
    /// Gets the field number of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    /// The file name, relative to the output directory.  The name must not
    /// contain "." or ".." components and must be relative, not be absolute (so,
    /// the file cannot lie outside the output directory).  "/" must be used as
    /// the path separator, not "".
    /// 
    /// If the name is omitted, the content will be appended to the previous
    /// file.  This allows the generator to break large files into small chunks,
    /// and allows the generated text to be streamed back to protoc so that large
    /// files need not reside completely in memory at one time.  Note that as of
    /// this writing protoc does not optimize for this -- it will read the entire
    /// CodeGeneratorResponse before writing files to disk.
    pub fn name(&self) -> &str {
        self.name.as_ref().map(|v| &**v).unwrap_or(Self::NAME_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`name`] field
    ///
    /// [`name`]: #method.name
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn name_option(&self) -> ::std::option::Option<&::std::string::String> {
        self.name.as_ref()
    }
    /// Returns a unique reference to the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        self.name.get_or_insert_with(::std::string::String::new)
    }
    /// Returns a bool indicating the presence of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
    /// Sets the value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn set_name(&mut self, value: ::std::string::String) {
        self.name = ::std::option::Option::Some(value)
    }
    /// Takes the value of the [`name`] field, leaving it empty
    ///
    /// [`name`]: #method.name
    pub fn take_name(&mut self) -> ::std::option::Option<::std::string::String> {
        self.name.take()
    }
    /// Clears the value of the [`name`] field
    ///
    /// [`name`]: #method.name
    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None
    }
    /// Gets the field number of the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub const INSERTION_POINT_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub const INSERTION_POINT_DEFAULT_VALUE: &'static str = "";
    /// If non-empty, indicates that the named file should already exist, and the
    /// content here is to be inserted into that file at a defined insertion
    /// point.  This feature allows a code generator to extend the output
    /// produced by another code generator.  The original generator may provide
    /// insertion points by placing special annotations in the file that look
    /// like:
    /// @@protoc_insertion_point(NAME)
    /// The annotation can have arbitrary text before and after it on the line,
    /// which allows it to be placed in a comment.  NAME should be replaced with
    /// an identifier naming the point -- this is what other generators will use
    /// as the insertion_point.  Code inserted at this point will be placed
    /// immediately above the line containing the insertion point (thus multiple
    /// insertions to the same point will come out in the order they were added).
    /// The double-@ is intended to make it unlikely that the generated code
    /// could contain things that look like insertion points by accident.
    /// 
    /// For example, the C++ code generator places the following line in the
    /// .pb.h files that it generates:
    /// // @@protoc_insertion_point(namespace_scope)
    /// This line appears within the scope of the file's package namespace, but
    /// outside of any particular class.  Another plugin can then specify the
    /// insertion_point "namespace_scope" to generate additional classes or
    /// other declarations that should be placed in this scope.
    /// 
    /// Note that if the line containing the insertion point begins with
    /// whitespace, the same whitespace will be added to every line of the
    /// inserted text.  This is useful for languages like Python, where
    /// indentation matters.  In these languages, the insertion point comment
    /// should be indented the same amount as any inserted code will need to be
    /// in order to work correctly in that context.
    /// 
    /// The code generator that generates the initial file and the one which
    /// inserts into it must both run as part of a single invocation of protoc.
    /// Code generators are executed in the order in which they appear on the
    /// command line.
    /// 
    /// If |insertion_point| is present, |name| must also be present.
    pub fn insertion_point(&self) -> &str {
        self.insertion_point.as_ref().map(|v| &**v).unwrap_or(Self::INSERTION_POINT_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn insertion_point_option(&self) -> ::std::option::Option<&::std::string::String> {
        self.insertion_point.as_ref()
    }
    /// Returns a unique reference to the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub fn insertion_point_mut(&mut self) -> &mut ::std::string::String {
        self.insertion_point.get_or_insert_with(::std::string::String::new)
    }
    /// Returns a bool indicating the presence of the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub fn has_insertion_point(&self) -> bool {
        self.insertion_point.is_some()
    }
    /// Sets the value of the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub fn set_insertion_point(&mut self, value: ::std::string::String) {
        self.insertion_point = ::std::option::Option::Some(value)
    }
    /// Takes the value of the [`insertion_point`] field, leaving it empty
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub fn take_insertion_point(&mut self) -> ::std::option::Option<::std::string::String> {
        self.insertion_point.take()
    }
    /// Clears the value of the [`insertion_point`] field
    ///
    /// [`insertion_point`]: #method.insertion_point
    pub fn clear_insertion_point(&mut self) {
        self.insertion_point = ::std::option::Option::None
    }
    /// Gets the field number of the [`content`] field
    ///
    /// [`content`]: #method.content
    pub const CONTENT_FIELD_NUMBER: i32 = 15;
    /// A constant value representing the default value of the [`content`] field
    ///
    /// [`content`]: #method.content
    pub const CONTENT_DEFAULT_VALUE: &'static str = "";
    /// The file contents.
    pub fn content(&self) -> &str {
        self.content.as_ref().map(|v| &**v).unwrap_or(Self::CONTENT_DEFAULT_VALUE)
    }
    /// Returns an [`Option`] representing the presence of the [`content`] field
    ///
    /// [`content`]: #method.content
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    pub fn content_option(&self) -> ::std::option::Option<&::std::string::String> {
        self.content.as_ref()
    }
    /// Returns a unique reference to the [`content`] field
    ///
    /// [`content`]: #method.content
    pub fn content_mut(&mut self) -> &mut ::std::string::String {
        self.content.get_or_insert_with(::std::string::String::new)
    }
    /// Returns a bool indicating the presence of the [`content`] field
    ///
    /// [`content`]: #method.content
    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }
    /// Sets the value of the [`content`] field
    ///
    /// [`content`]: #method.content
    pub fn set_content(&mut self, value: ::std::string::String) {
        self.content = ::std::option::Option::Some(value)
    }
    /// Takes the value of the [`content`] field, leaving it empty
    ///
    /// [`content`]: #method.content
    pub fn take_content(&mut self) -> ::std::option::Option<::std::string::String> {
        self.content.take()
    }
    /// Clears the value of the [`content`] field
    ///
    /// [`content`]: #method.content
    pub fn clear_content(&mut self) {
        self.content = ::std::option::Option::None
    }
}