//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/compiler/plugin.proto

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: ::std::option::Option<i32>,
    pub minor: ::std::option::Option<i32>,
    pub patch: ::std::option::Option<i32>,
    pub suffix: ::std::option::Option<::std::string::String>,
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
                tag => self.unknown_fields.merge_from(tag, input)?
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
                size = size.checked_add(crate::io::sizes::int32(major))?;
            }
        }
        let minor = self.minor;
        if let ::std::option::Option::Some(minor) = minor {
            if minor != Self::MINOR_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::int32(minor))?;
            }
        }
        let patch = self.patch;
        if let ::std::option::Option::Some(patch) = patch {
            if patch != Self::PATCH_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::int32(patch))?;
            }
        }
        let suffix = &self.suffix;
        if let ::std::option::Option::Some(suffix) = suffix {
            if suffix != Self::SUFFIX_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(suffix)?)?;
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
}
impl ::std::clone::Clone for self::Version {
    fn clone(&self) -> Self {
        Self {
            major: self.major.clone(),
            minor: self.minor.clone(),
            patch: self.patch.clone(),
            suffix: self.suffix.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.major = other.major;
        self.minor = other.minor;
        self.patch = other.patch;
        self.suffix = other.suffix.clone();
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Version {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Version {
    /// Gets the field number of the 'major' field
    pub const MAJOR_FIELD_NUMBER: i32 = 1;
    pub const MAJOR_DEFAULT_VALUE: i32 = 0;
    /// Gets the field number of the 'minor' field
    pub const MINOR_FIELD_NUMBER: i32 = 2;
    pub const MINOR_DEFAULT_VALUE: i32 = 0;
    /// Gets the field number of the 'patch' field
    pub const PATCH_FIELD_NUMBER: i32 = 3;
    pub const PATCH_DEFAULT_VALUE: i32 = 0;
    /// Gets the field number of the 'suffix' field
    pub const SUFFIX_FIELD_NUMBER: i32 = 4;
    pub const SUFFIX_DEFAULT_VALUE: &'static str = "";
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: crate::collections::RepeatedField<::std::string::String>,
    pub parameter: ::std::option::Option<::std::string::String>,
    pub proto_file: crate::collections::RepeatedField<crate::descriptor::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<::std::boxed::Box<self::Version>>,
    unknown_fields: crate::UnknownFieldSet
}
static CODE_GENERATOR_REQUEST_FILE_TO_GENERATE_CODEC: crate::Codec<::std::string::String> = crate::Codec::string(10);
static CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC: crate::Codec<crate::descriptor::FileDescriptorProto> = crate::Codec::message(122);
impl crate::CodedMessage for self::CodeGeneratorRequest {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.file_to_generate.add_entries(tag.get(), input, &CODE_GENERATOR_REQUEST_FILE_TO_GENERATE_CODEC)?,
                18 => self.parameter = ::std::option::Option::Some(input.read_string()?),
                122 => self.proto_file.add_entries(tag.get(), input, &CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC)?,
                26 => input.read_message(self.compiler_version.get_or_insert_with(crate::LiteMessage::new))?,
                tag => self.unknown_fields.merge_from(tag, input)?
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
                size = size.checked_add(crate::io::sizes::string(parameter)?)?;
            }
        }
        size = size.checked_add(self.proto_file.calculate_size(&CODE_GENERATOR_REQUEST_PROTO_FILE_CODEC)?)?;
        let compiler_version = &self.compiler_version;
        if let ::std::option::Option::Some(compiler_version) = compiler_version {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::message(compiler_version)?)?;
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
            output.write_message(compiler_version)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
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
}
impl ::std::clone::Clone for self::CodeGeneratorRequest {
    fn clone(&self) -> Self {
        Self {
            file_to_generate: self.file_to_generate.clone(),
            parameter: self.parameter.clone(),
            proto_file: self.proto_file.clone(),
            compiler_version: self.compiler_version.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.file_to_generate.clone_from(&other.file_to_generate);
        self.parameter = other.parameter.clone();
        self.proto_file.clone_from(&other.proto_file);
        if let ::std::option::Option::Some(compiler_version) = &other.compiler_version {
            self.compiler_version.get_or_insert_with(crate::LiteMessage::new).clone_from(compiler_version);
        }
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::CodeGeneratorRequest {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::CodeGeneratorRequest {
    /// Gets the field number of the 'file_to_generate' field
    pub const FILE_TO_GENERATE_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'parameter' field
    pub const PARAMETER_FIELD_NUMBER: i32 = 2;
    pub const PARAMETER_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'proto_file' field
    pub const PROTO_FILE_FIELD_NUMBER: i32 = 15;
    /// Gets the field number of the 'compiler_version' field
    pub const COMPILER_VERSION_FIELD_NUMBER: i32 = 3;
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option<::std::string::String>,
    pub file: crate::collections::RepeatedField<self::CodeGeneratorResponse_File>,
    unknown_fields: crate::UnknownFieldSet
}
static CODE_GENERATOR_RESPONSE_FILE_CODEC: crate::Codec<self::CodeGeneratorResponse_File> = crate::Codec::message(122);
impl crate::CodedMessage for self::CodeGeneratorResponse {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.error = ::std::option::Option::Some(input.read_string()?),
                122 => self.file.add_entries(tag.get(), input, &CODE_GENERATOR_RESPONSE_FILE_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
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
                size = size.checked_add(crate::io::sizes::string(error)?)?;
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
}
impl crate::LiteMessage for self::CodeGeneratorResponse {
    fn new() -> Self {
        Self {
            error: ::std::option::Option::None,
            file: crate::collections::RepeatedField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::CodeGeneratorResponse {
    fn clone(&self) -> Self {
        Self {
            error: self.error.clone(),
            file: self.file.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.error = other.error.clone();
        self.file.clone_from(&other.file);
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::CodeGeneratorResponse {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::CodeGeneratorResponse {
    /// Gets the field number of the 'error' field
    pub const ERROR_FIELD_NUMBER: i32 = 1;
    pub const ERROR_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'file' field
    pub const FILE_FIELD_NUMBER: i32 = 15;
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorResponse_File {
    pub name: ::std::option::Option<::std::string::String>,
    pub insertion_point: ::std::option::Option<::std::string::String>,
    pub content: ::std::option::Option<::std::string::String>,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::CodeGeneratorResponse_File {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name = ::std::option::Option::Some(input.read_string()?),
                18 => self.insertion_point = ::std::option::Option::Some(input.read_string()?),
                122 => self.content = ::std::option::Option::Some(input.read_string()?),
                tag => self.unknown_fields.merge_from(tag, input)?
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
                size = size.checked_add(crate::io::sizes::string(name)?)?;
            }
        }
        let insertion_point = &self.insertion_point;
        if let ::std::option::Option::Some(insertion_point) = insertion_point {
            if insertion_point != Self::INSERTION_POINT_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(insertion_point)?)?;
            }
        }
        let content = &self.content;
        if let ::std::option::Option::Some(content) = content {
            if content != Self::CONTENT_DEFAULT_VALUE {
                size = size.checked_add(1)?;
                size = size.checked_add(crate::io::sizes::string(content)?)?;
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
}
impl ::std::clone::Clone for self::CodeGeneratorResponse_File {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            insertion_point: self.insertion_point.clone(),
            content: self.content.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.insertion_point = other.insertion_point.clone();
        self.content = other.content.clone();
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::CodeGeneratorResponse_File {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::CodeGeneratorResponse_File {
    /// Gets the field number of the 'name' field
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const NAME_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'insertion_point' field
    pub const INSERTION_POINT_FIELD_NUMBER: i32 = 2;
    pub const INSERTION_POINT_DEFAULT_VALUE: &'static str = "";
    /// Gets the field number of the 'content' field
    pub const CONTENT_FIELD_NUMBER: i32 = 15;
    pub const CONTENT_DEFAULT_VALUE: &'static str = "";
}