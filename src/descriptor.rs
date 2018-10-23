use ::codegen;

#[derive(Clone, PartialEq)]
pub struct FileDescriptorSet {
    file_: Vec<FileDescriptorProto>,
    _unknown: codegen::UnknownFieldSet
}

static FILE_DESCRIPTOR_SET_FILE_CODEC: codegen::Codec<FileDescriptorProto> = codegen::Codec::<FileDescriptorProto>::message(10);

impl codegen::LiteMessage for FileDescriptorSet {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => codegen::collections::repeated::add_entries(&mut self.file_, tag.get(), input, &FILE_DESCRIPTOR_SET_FILE_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for FileDescriptorSet {
    fn new() -> Self {
        FileDescriptorSet {
            file_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for FileDescriptorSet { }

impl codegen::GeneratedMessage for FileDescriptorSet { }

impl FileDescriptorSet {
    pub fn ref_file(&self) -> &Vec<FileDescriptorProto> {
        &self.file_
    }
    pub fn mut_file(&mut self) -> &mut Vec<FileDescriptorProto> {
        &mut self.file_
    }
}

#[derive(Clone, PartialEq)]
pub struct FileDescriptorProto {
    name_: Option<String>,
    package_: Option<String>,
    dependency_: Vec<String>,
    public_dependency_: Vec<i32>,
    weak_dependency_: Vec<i32>,
    message_type_: Vec<DescriptorProto>,
    enum_type_: Vec<EnumDescriptorProto>,
    service_: Vec<ServiceDescriptorProto>,
    extension_: Vec<FieldDescriptorProto>,
    options_: Option<FileOptions>,
    source_code_info_: Option<SourceCodeInfo>,
    syntax_: Option<String>,
    _unknown: codegen::UnknownFieldSet
}

static FILE_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static FILE_DESCRIPTOR_PROTO_PACKAGE_DEFAULT: String = String::new();
static FILE_DESCRIPTOR_PROTO_DEPENDENCY_CODEC: codegen::Codec<String> = codegen::Codec::<String>::string(String::new(), 26);
static FILE_DESCRIPTOR_PROTO_PUBLIC_DEPENDENCY_CODEC: codegen::Codec<i32> = codegen::Codec::<i32>::int32(0, 80);
static FILE_DESCRIPTOR_PROTO_WEAK_DEPENDENCY_CODEC: codegen::Codec<i32> = codegen::Codec::<i32>::int32(0, 88);
static FILE_DESCRIPTOR_PROTO_MESSAGE_TYPE_CODEC: codegen::Codec<DescriptorProto> = codegen::Codec::<DescriptorProto>::message(34);
static FILE_DESCRIPTOR_PROTO_ENUM_TYPE_CODEC: codegen::Codec<EnumDescriptorProto> = codegen::Codec::<EnumDescriptorProto>::message(42);
static FILE_DESCRIPTOR_PROTO_SERVICE_CODEC: codegen::Codec<ServiceDescriptorProto> = codegen::Codec::<ServiceDescriptorProto>::message(50);
static FILE_DESCRIPTOR_PROTO_EXTENSION_CODEC: codegen::Codec<FieldDescriptorProto> = codegen::Codec::<FieldDescriptorProto>::message(58);
static FILE_DESCRIPTOR_PROTO_SYNTAX_DEFAULT: String = String::new();

impl codegen::LiteMessage for FileDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => self.package_ = Some(input.read_string()?),
                26 => codegen::collections::repeated::add_entries(&mut self.dependency_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_DEPENDENCY_CODEC)?,
                34 => codegen::collections::repeated::add_entries(&mut self.message_type_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_MESSAGE_TYPE_CODEC)?,
                42 => codegen::collections::repeated::add_entries(&mut self.enum_type_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_ENUM_TYPE_CODEC)?,
                50 => codegen::collections::repeated::add_entries(&mut self.service_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_SERVICE_CODEC)?,
                58 => codegen::collections::repeated::add_entries(&mut self.extension_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_EXTENSION_CODEC)?,
                66 => input.read_message(self.options_.get_or_insert_with(<FileOptions as codegen::GeneratedLiteMessage>::new))?,
                74 => input.read_message(self.source_code_info_.get_or_insert_with(<SourceCodeInfo as codegen::GeneratedLiteMessage>::new))?,
                80 | 
                82 => codegen::collections::repeated::add_entries(&mut self.public_dependency_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_PUBLIC_DEPENDENCY_CODEC)?,
                88 |
                90 => codegen::collections::repeated::add_entries(&mut self.weak_dependency_, tag.get(), input, &FILE_DESCRIPTOR_PROTO_WEAK_DEPENDENCY_CODEC)?,
                98 => self.syntax_ = Some(input.read_string()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for FileDescriptorProto {
    fn new() -> Self {
        FileDescriptorProto {
            name_: None,
            package_: None,
            dependency_: Vec::new(),
            public_dependency_: Vec::new(),
            weak_dependency_: Vec::new(),
            message_type_: Vec::new(),
            enum_type_: Vec::new(),
            service_: Vec::new(),
            extension_: Vec::new(),
            options_: None,
            source_code_info_: None,
            syntax_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for FileDescriptorProto { }

impl codegen::GeneratedMessage for FileDescriptorProto { }

impl FileDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &FILE_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_package(&self) -> Option<&String> {
        self.package_.as_ref()
    }
    pub fn ref_package_or_default(&self) -> &String {
        if let Some(ref package_) = self.package_ {
            package_
        } else {
            &FILE_DESCRIPTOR_PROTO_PACKAGE_DEFAULT
        }
    }
    pub fn mut_package(&mut self) -> Option<&mut String> {
        self.package_.as_mut()
    }
    pub fn set_package(&mut self, value: String) {
        self.package_ = Some(value)
    }
    pub fn has_package(&self) -> bool {
        self.package_.is_some()
    }
    pub fn clear_package(&mut self) {
        self.package_ = None
    }

    pub fn ref_dependency(&self) -> &Vec<String> {
        &self.dependency_
    }
    pub fn mut_dependency(&mut self) -> &mut Vec<String> {
        &mut self.dependency_
    }

    pub fn ref_public_dependency(&self) -> &Vec<i32> {
        &self.public_dependency_
    }
    pub fn mut_public_dependency(&mut self) -> &mut Vec<i32> {
        &mut self.public_dependency_
    }

    pub fn ref_weak_dependency(&self) -> &Vec<i32> {
        &self.weak_dependency_
    }
    pub fn mut_weak_dependency(&mut self) -> &mut Vec<i32> {
        &mut self.weak_dependency_
    }

    pub fn ref_message_type(&self) -> &Vec<DescriptorProto> {
        &self.message_type_
    }
    pub fn mut_message_type(&mut self) -> &mut Vec<DescriptorProto> {
        &mut self.message_type_
    }

    pub fn ref_enum_type(&self) -> &Vec<EnumDescriptorProto> {
        &self.enum_type_
    }
    pub fn mut_enum_type(&mut self) -> &mut Vec<EnumDescriptorProto> {
        &mut self.enum_type_
    }

    pub fn ref_service(&self) -> &Vec<ServiceDescriptorProto> {
        &self.service_
    }
    pub fn mut_service(&mut self) -> &mut Vec<ServiceDescriptorProto> {
        &mut self.service_
    }

    pub fn ref_extension(&self) -> &Vec<FieldDescriptorProto> {
        &self.extension_
    }
    pub fn mut_extension(&mut self) -> &mut Vec<FieldDescriptorProto> {
        &mut self.extension_
    }

    pub fn ref_options(&self) -> Option<&FileOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut FileOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: FileOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }

    pub fn ref_source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.source_code_info_.as_ref()
    }
    pub fn mut_source_code_info(&mut self) -> Option<&mut SourceCodeInfo> {
        self.source_code_info_.as_mut()
    }
    pub fn set_source_code_info(&mut self, value: SourceCodeInfo) {
        self.source_code_info_ = Some(value)
    }
    pub fn has_source_code_info(&self) -> bool {
        self.source_code_info_.is_some()
    }
    pub fn clear_source_code_info(&mut self) {
        self.source_code_info_ = None
    }

    pub fn ref_syntax(&self) -> Option<&String> {
        self.syntax_.as_ref()
    }
    pub fn ref_syntax_or_default(&self) -> &String {
        if let Some(ref syntax_) = self.syntax_ {
            syntax_
        } else {
            &FILE_DESCRIPTOR_PROTO_SYNTAX_DEFAULT
        }
    }
    pub fn mut_syntax(&mut self) -> Option<&mut String> {
        self.syntax_.as_mut()
    }
    pub fn set_syntax(&mut self, value: String) {
        self.syntax_ = Some(value)
    }
    pub fn has_syntax(&self) -> bool {
        self.syntax_.is_some()
    }
    pub fn clear_syntax(&mut self) {
        self.syntax_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct DescriptorProto {
    name_: Option<String>,
    field_: Vec<FieldDescriptorProto>,
    extension_: Vec<FieldDescriptorProto>,
    nested_type_: Vec<DescriptorProto>,
    enum_type_: Vec<EnumDescriptorProto>,
    extension_range_: Vec<DescriptorProto_ExtensionRange>,
    oneof_decl_: Vec<OneofDescriptorProto>,
    options_: Option<MessageOptions>,
    reserved_range_: Vec<DescriptorProto_ReservedRange>,
    reserved_name_: Vec<String>,
    _unknown: codegen::UnknownFieldSet
}

static DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static DESCRIPTOR_PROTO_FIELD_CODEC: codegen::Codec<FieldDescriptorProto> = codegen::Codec::<FieldDescriptorProto>::message(18);
static DESCRIPTOR_PROTO_EXTENSION_CODEC: codegen::Codec<FieldDescriptorProto> = codegen::Codec::<FieldDescriptorProto>::message(50);
static DESCRIPTOR_PROTO_NESTED_TYPE_CODEC: codegen::Codec<DescriptorProto> = codegen::Codec::<DescriptorProto>::message(26);
static DESCRIPTOR_PROTO_ENUM_TYPE_CODEC: codegen::Codec<EnumDescriptorProto> = codegen::Codec::<EnumDescriptorProto>::message(34);
static DESCRIPTOR_PROTO_EXTENSION_RANGE_CODEC: codegen::Codec<DescriptorProto_ExtensionRange> = codegen::Codec::<DescriptorProto_ExtensionRange>::message(42);
static DESCRIPTOR_PROTO_ONEOF_DECL_CODEC: codegen::Codec<OneofDescriptorProto> = codegen::Codec::<OneofDescriptorProto>::message(66);
static DESCRIPTOR_PROTO_RESERVED_RANGE_CODEC: codegen::Codec<DescriptorProto_ReservedRange> = codegen::Codec::<DescriptorProto_ReservedRange>::message(74);
static DESCRIPTOR_PROTO_RESERVED_NAME_CODEC: codegen::Codec<String> = codegen::Codec::<String>::string(String::new(), 82);

impl codegen::LiteMessage for DescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => codegen::collections::repeated::add_entries(&mut self.field_, tag.get(), input, &DESCRIPTOR_PROTO_FIELD_CODEC)?,
                26 => codegen::collections::repeated::add_entries(&mut self.nested_type_, tag.get(), input, &DESCRIPTOR_PROTO_NESTED_TYPE_CODEC)?,
                34 => codegen::collections::repeated::add_entries(&mut self.enum_type_, tag.get(), input, &DESCRIPTOR_PROTO_ENUM_TYPE_CODEC)?,
                42 => codegen::collections::repeated::add_entries(&mut self.extension_range_, tag.get(), input, &DESCRIPTOR_PROTO_EXTENSION_RANGE_CODEC)?,
                50 => codegen::collections::repeated::add_entries(&mut self.extension_, tag.get(), input, &DESCRIPTOR_PROTO_EXTENSION_CODEC)?,
                58 => input.read_message(self.options_.get_or_insert_with(<MessageOptions as codegen::GeneratedLiteMessage>::new))?,
                66 => codegen::collections::repeated::add_entries(&mut self.oneof_decl_, tag.get(), input, &DESCRIPTOR_PROTO_ONEOF_DECL_CODEC)?,
                74 => codegen::collections::repeated::add_entries(&mut self.reserved_range_, tag.get(), input, &DESCRIPTOR_PROTO_RESERVED_RANGE_CODEC)?,
                82 => codegen::collections::repeated::add_entries(&mut self.reserved_name_, tag.get(), input, &DESCRIPTOR_PROTO_RESERVED_NAME_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for DescriptorProto {
    fn new() -> Self {
        DescriptorProto {
            name_: None,
            field_: Vec::new(),
            extension_: Vec::new(),
            nested_type_: Vec::new(),
            enum_type_: Vec::new(),
            extension_range_: Vec::new(),
            oneof_decl_: Vec::new(),
            options_: None,
            reserved_range_: Vec::new(),
            reserved_name_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for DescriptorProto { }

impl codegen::GeneratedMessage for DescriptorProto { }

impl DescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_field(&self) -> &Vec<FieldDescriptorProto> {
        &self.field_
    }
    pub fn mut_field(&mut self) -> &mut Vec<FieldDescriptorProto> {
        &mut self.field_
    }

    pub fn ref_extension(&self) -> &Vec<FieldDescriptorProto> {
        &self.extension_
    }
    pub fn mut_extension(&mut self) -> &mut Vec<FieldDescriptorProto> {
        &mut self.extension_
    }

    pub fn ref_nested_type(&self) -> &Vec<DescriptorProto> {
        &self.nested_type_
    }
    pub fn mut_nested_type(&mut self) -> &mut Vec<DescriptorProto> {
        &mut self.nested_type_
    }

    pub fn ref_enum_type(&self) -> &Vec<EnumDescriptorProto> {
        &self.enum_type_
    }
    pub fn mut_enum_type(&mut self) -> &mut Vec<EnumDescriptorProto> {
        &mut self.enum_type_
    }

    pub fn ref_extension_range(&self) -> &Vec<DescriptorProto_ExtensionRange> {
        &self.extension_range_
    }
    pub fn mut_extension_range(&mut self) -> &mut Vec<DescriptorProto_ExtensionRange> {
        &mut self.extension_range_
    }

    pub fn ref_oneof_decl(&self) -> &Vec<OneofDescriptorProto> {
        &self.oneof_decl_
    }
    pub fn mut_oneof_decl(&mut self) -> &mut Vec<OneofDescriptorProto> {
        &mut self.oneof_decl_
    }

    pub fn ref_options(&self) -> Option<&MessageOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut MessageOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: MessageOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }

    pub fn ref_reserved_range(&self) -> &Vec<DescriptorProto_ReservedRange> {
        &self.reserved_range_
    }
    pub fn mut_reserved_range(&mut self) -> &mut Vec<DescriptorProto_ReservedRange> {
        &mut self.reserved_range_
    }

    pub fn ref_reserved_name(&self) -> &Vec<String> {
        &self.reserved_name_
    }
    pub fn mut_reserved_name(&mut self) -> &mut Vec<String> {
        &mut self.reserved_name_
    }
}

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct DescriptorProto_ExtensionRange {
    start_: Option<i32>,
    end_: Option<i32>,
    options_: Option<ExtensionRangeOptions>,
    _unknown: codegen::UnknownFieldSet
}

static DESCRIPTOR_PROTO_EXTENSION_RANGE_START_DEFAULT: i32 = 0;
static DESCRIPTOR_PROTO_EXTENSION_RANGE_END_DEFAULT: i32 = 0;

impl codegen::LiteMessage for DescriptorProto_ExtensionRange {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.start_ = Some(input.read_int32()?),
                16 => self.end_ = Some(input.read_int32()?),
                26 => input.read_message(self.options_.get_or_insert_with(<ExtensionRangeOptions as codegen::GeneratedLiteMessage>::new))?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for DescriptorProto_ExtensionRange {
    fn new() -> Self {
        DescriptorProto_ExtensionRange {
            start_: None,
            end_: None,
            options_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for DescriptorProto_ExtensionRange { }

impl codegen::GeneratedMessage for DescriptorProto_ExtensionRange { }

impl DescriptorProto_ExtensionRange {
    pub fn ref_start(&self) -> Option<&i32> {
        self.start_.as_ref()
    }
    pub fn ref_start_or_default(&self) -> &i32 {
        if let Some(ref start_) = self.start_ {
            start_
        } else {
            &DESCRIPTOR_PROTO_EXTENSION_RANGE_START_DEFAULT
        }
    }
    pub fn mut_start(&mut self) -> Option<&mut i32> {
        self.start_.as_mut()
    }
    pub fn set_start(&mut self, value: i32) {
        self.start_ = Some(value)
    }
    pub fn has_start(&self) -> bool {
        self.start_.is_some()
    }
    pub fn clear_start(&mut self) {
        self.start_ = None
    }

    pub fn ref_end(&self) -> Option<&i32> {
        self.end_.as_ref()
    }
    pub fn ref_end_or_default(&self) -> &i32 {
        if let Some(ref end_) = self.end_ {
            end_
        } else {
            &DESCRIPTOR_PROTO_EXTENSION_RANGE_END_DEFAULT
        }
    }
    pub fn mut_end(&mut self) -> Option<&mut i32> {
        self.end_.as_mut()
    }
    pub fn set_end(&mut self, value: i32) {
        self.end_ = Some(value)
    }
    pub fn has_end(&self) -> bool {
        self.end_.is_some()
    }
    pub fn clear_end(&mut self) {
        self.end_ = None
    }

    pub fn ref_options(&self) -> Option<&ExtensionRangeOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut ExtensionRangeOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: ExtensionRangeOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }
}

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct DescriptorProto_ReservedRange {
    start_: Option<i32>,
    end_: Option<i32>,
    _unknown: codegen::UnknownFieldSet
}

static DESCRIPTOR_PROTO_RESERVED_RANGE_START_DEFAULT: i32 = 0;
static DESCRIPTOR_PROTO_RESERVED_RANGE_END_DEFAULT: i32 = 0;

impl codegen::LiteMessage for DescriptorProto_ReservedRange {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.start_ = Some(input.read_int32()?),
                16 => self.end_ = Some(input.read_int32()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for DescriptorProto_ReservedRange {
    fn new() -> Self {
        DescriptorProto_ReservedRange {
            start_: None,
            end_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for DescriptorProto_ReservedRange { }

impl codegen::GeneratedMessage for DescriptorProto_ReservedRange { }

impl DescriptorProto_ReservedRange {
        pub fn ref_start(&self) -> Option<&i32> {
        self.start_.as_ref()
    }
    pub fn ref_start_or_default(&self) -> &i32 {
        if let Some(ref start_) = self.start_ {
            start_
        } else {
            &DESCRIPTOR_PROTO_RESERVED_RANGE_START_DEFAULT
        }
    }
    pub fn mut_start(&mut self) -> Option<&mut i32> {
        self.start_.as_mut()
    }
    pub fn set_start(&mut self, value: i32) {
        self.start_ = Some(value)
    }
    pub fn has_start(&self) -> bool {
        self.start_.is_some()
    }
    pub fn clear_start(&mut self) {
        self.start_ = None
    }

    pub fn ref_end(&self) -> Option<&i32> {
        self.end_.as_ref()
    }
    pub fn ref_end_or_default(&self) -> &i32 {
        if let Some(ref end_) = self.end_ {
            end_
        } else {
            &DESCRIPTOR_PROTO_RESERVED_RANGE_END_DEFAULT
        }
    }
    pub fn mut_end(&mut self) -> Option<&mut i32> {
        self.end_.as_mut()
    }
    pub fn set_end(&mut self, value: i32) {
        self.end_ = Some(value)
    }
    pub fn has_end(&self) -> bool {
        self.end_.is_some()
    }
    pub fn clear_end(&mut self) {
        self.end_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct ExtensionRangeOptions {
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static EXTENSION_RANGE_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for ExtensionRangeOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &EXTENSION_RANGE_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for ExtensionRangeOptions {
    fn new() -> Self {
        ExtensionRangeOptions {
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for ExtensionRangeOptions { }

impl codegen::GeneratedMessage for ExtensionRangeOptions { }

impl ExtensionRangeOptions {
    pub fn ref_uninterpted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq)]
pub struct FieldDescriptorProto {
    name_: Option<String>,
    number_: Option<i32>,
    label_: Option<codegen::EnumValue<FieldDescriptorProto_Label>>,
    type_: Option<codegen::EnumValue<FieldDescriptorProto_Type>>,
    type_name_: Option<String>,
    extendee_: Option<String>,
    default_value_: Option<String>,
    oneof_index_: Option<i32>,
    json_name_: Option<String>,
    options_: Option<FieldOptions>,
    _unknown: codegen::UnknownFieldSet
}

static FIELD_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static FIELD_DESCRIPTOR_PROTO_NUMBER_DEFAULT: i32 = 0;
static FIELD_DESCRIPTOR_PROTO_LABEL_DEFAULT: codegen::EnumValue<FieldDescriptorProto_Label> = codegen::EnumValue::Undefined::<FieldDescriptorProto_Label>(0);
static FIELD_DESCRIPTOR_PROTO_TYPE_DEFAULT: codegen::EnumValue<FieldDescriptorProto_Type> = codegen::EnumValue::Undefined::<FieldDescriptorProto_Type>(0);
static FIELD_DESCRIPTOR_PROTO_TYPE_NAME_DEFAULT: String = String::new();
static FIELD_DESCRIPTOR_PROTO_EXTENDEE_DEFAULT: String = String::new();
static FIELD_DESCRIPTOR_PROTO_DEFAULT_VALUE_DEFAULT: String = String::new();
static FIELD_DESCRIPTOR_PROTO_ONEOF_INDEX_DEFAULT: i32 = 0;
static FIELD_DESCRIPTOR_PROTO_JSON_NAME_DEFAULT: String = String::new();

impl codegen::LiteMessage for FieldDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => self.extendee_ = Some(input.read_string()?),
                24 => self.number_ = Some(input.read_int32()?),
                32 => self.label_ = Some(codegen::EnumValue::from(input.read_int32()?)),
                40 => self.type_ = Some(codegen::EnumValue::from(input.read_int32()?)),
                50 => self.type_name_ = Some(input.read_string()?),
                58 => self.default_value_ = Some(input.read_string()?),
                66 => input.read_message(self.options_.get_or_insert_with(<FieldOptions as codegen::GeneratedLiteMessage>::new))?,
                72 => self.oneof_index_ = Some(input.read_int32()?),
                82 => self.json_name_ = Some(input.read_string()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for FieldDescriptorProto {
    fn new() -> Self {
        FieldDescriptorProto {
            name_: None,
            number_: None,
            label_: None,
            type_: None,
            type_name_: None,
            extendee_: None,
            default_value_: None,
            oneof_index_: None,
            json_name_: None,
            options_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for FieldDescriptorProto { }

impl codegen::GeneratedMessage for FieldDescriptorProto { }

impl FieldDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &FIELD_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_number(&self) -> Option<&i32> {
        self.number_.as_ref()
    }
    pub fn ref_number_or_default(&self) -> &i32 {
        if let Some(ref number_) = self.number_ {
            number_
        } else {
            &FIELD_DESCRIPTOR_PROTO_NUMBER_DEFAULT
        }
    }
    pub fn mut_number(&mut self) -> Option<&mut i32> {
        self.number_.as_mut()
    }
    pub fn set_number(&mut self, value: i32) {
        self.number_ = Some(value)
    }
    pub fn has_number(&self) -> bool {
        self.number_.is_some()
    }
    pub fn clear_number(&mut self) {
        self.number_ = None
    }

    pub fn ref_label(&self) -> Option<&codegen::EnumValue<FieldDescriptorProto_Label>> {
        self.label_.as_ref()
    }
    pub fn ref_label_or_default(&self) -> &codegen::EnumValue<FieldDescriptorProto_Label> {
        if let Some(ref label_) = self.label_ {
            label_
        } else {
            &FIELD_DESCRIPTOR_PROTO_LABEL_DEFAULT
        }
    }
    pub fn mut_label(&mut self) -> Option<&mut codegen::EnumValue<FieldDescriptorProto_Label>> {
        self.label_.as_mut()
    }
    pub fn set_label(&mut self, value: codegen::EnumValue<FieldDescriptorProto_Label>) {
        self.label_ = Some(value)
    }
    pub fn has_label(&self) -> bool {
        self.label_.is_some()
    }
    pub fn clear_label(&mut self) {
        self.label_ = None
    }

    pub fn ref_type(&self) -> Option<&codegen::EnumValue<FieldDescriptorProto_Type>> {
        self.type_.as_ref()
    }
    pub fn ref_type_or_default(&self) -> &codegen::EnumValue<FieldDescriptorProto_Type> {
        if let Some(ref type_) = self.type_ {
            type_
        } else {
            &FIELD_DESCRIPTOR_PROTO_TYPE_DEFAULT
        }
    }
    pub fn mut_type(&mut self) -> Option<&mut codegen::EnumValue<FieldDescriptorProto_Type>> {
        self.type_.as_mut()
    }
    pub fn set_type(&mut self, value: codegen::EnumValue<FieldDescriptorProto_Type>) {
        self.type_ = Some(value)
    }
    pub fn has_type(&self) -> bool {
        self.type_.is_some()
    }
    pub fn clear_type(&mut self) {
        self.type_ = None
    }

    pub fn ref_type_name(&self) -> Option<&String> {
        self.type_name_.as_ref()
    }
    pub fn ref_type_name_or_default(&self) -> &String {
        if let Some(ref type_name_) = self.type_name_ {
            type_name_
        } else {
            &FIELD_DESCRIPTOR_PROTO_TYPE_NAME_DEFAULT
        }
    }
    pub fn mut_type_name(&mut self) -> Option<&mut String> {
        self.type_name_.as_mut()
    }
    pub fn set_type_name(&mut self, value: String) {
        self.type_name_ = Some(value)
    }
    pub fn has_type_name(&self) -> bool {
        self.type_name_.is_some()
    }
    pub fn clear_type_name(&mut self) {
        self.type_name_ = None
    }

    pub fn ref_extendee(&self) -> Option<&String> {
        self.extendee_.as_ref()
    }
    pub fn ref_extendee_or_default(&self) -> &String {
        if let Some(ref extendee_) = self.extendee_ {
            extendee_
        } else {
            &FIELD_DESCRIPTOR_PROTO_EXTENDEE_DEFAULT
        }
    }
    pub fn mut_extendee(&mut self) -> Option<&mut String> {
        self.extendee_.as_mut()
    }
    pub fn set_extendee(&mut self, value: String) {
        self.extendee_ = Some(value)
    }
    pub fn has_extendee(&self) -> bool {
        self.extendee_.is_some()
    }
    pub fn clear_extendee(&mut self) {
        self.extendee_ = None
    }

    pub fn ref_default_value(&self) -> Option<&String> {
        self.default_value_.as_ref()
    }
    pub fn ref_default_value_or_default(&self) -> &String {
        if let Some(ref default_value_) = self.default_value_ {
            default_value_
        } else {
            &FIELD_DESCRIPTOR_PROTO_DEFAULT_VALUE_DEFAULT
        }
    }
    pub fn mut_default_value(&mut self) -> Option<&mut String> {
        self.default_value_.as_mut()
    }
    pub fn set_default_value(&mut self, value: String) {
        self.default_value_ = Some(value)
    }
    pub fn has_default_value(&self) -> bool {
        self.default_value_.is_some()
    }
    pub fn clear_default_value(&mut self) {
        self.default_value_ = None
    }

    pub fn ref_oneof_index(&self) -> Option<&i32> {
        self.oneof_index_.as_ref()
    }
    pub fn ref_oneof_index_or_default(&self) -> &i32 {
        if let Some(ref oneof_index_) = self.oneof_index_ {
            oneof_index_
        } else {
            &FIELD_DESCRIPTOR_PROTO_ONEOF_INDEX_DEFAULT
        }
    }
    pub fn mut_oneof_index(&mut self) -> Option<&mut i32> {
        self.oneof_index_.as_mut()
    }
    pub fn set_oneof_index(&mut self, value: i32) {
        self.oneof_index_ = Some(value)
    }
    pub fn has_oneof_index(&self) -> bool {
        self.oneof_index_.is_some()
    }
    pub fn clear_oneof_index(&mut self) {
        self.oneof_index_ = None
    }

    pub fn ref_json_name(&self) -> Option<&String> {
        self.json_name_.as_ref()
    }
    pub fn ref_json_name_or_default(&self) -> &String {
        if let Some(ref json_name_) = self.json_name_ {
            json_name_
        } else {
            &FIELD_DESCRIPTOR_PROTO_JSON_NAME_DEFAULT
        }
    }
    pub fn mut_json_name(&mut self) -> Option<&mut String> {
        self.json_name_.as_mut()
    }
    pub fn set_json_name(&mut self, value: String) {
        self.json_name_ = Some(value)
    }
    pub fn has_json_name(&self) -> bool {
        self.json_name_.is_some()
    }
    pub fn clear_json_name(&mut self) {
        self.json_name_ = None
    }
}

#[derive(Clone, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum FieldDescriptorProto_Type {
    Double = 1,
    Float = 2,
    Int64 = 3,
    UInt64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    Group = 10,
    Message = 11,
    Bytes = 12,
    UInt32 = 13,
    Enum = 14,
    SFixed32 = 15,
    SFixed64 = 16,
    SInt32 = 17,
    SInt64 = 18
}

impl codegen::TryFrom<i32> for FieldDescriptorProto_Type {
    type Error = codegen::VariantUndefinedError;

    fn try_from(value: i32) -> Result<FieldDescriptorProto_Type, codegen::VariantUndefinedError> {
        match value {
            1 => Ok(FieldDescriptorProto_Type::Double),
            2 => Ok(FieldDescriptorProto_Type::Float),
            3 => Ok(FieldDescriptorProto_Type::Int64),
            4 => Ok(FieldDescriptorProto_Type::UInt64),
            5 => Ok(FieldDescriptorProto_Type::Int32),
            6 => Ok(FieldDescriptorProto_Type::Fixed64),
            7 => Ok(FieldDescriptorProto_Type::Fixed32),
            8 => Ok(FieldDescriptorProto_Type::Bool),
            9 => Ok(FieldDescriptorProto_Type::String),
            10 => Ok(FieldDescriptorProto_Type::Group),
            11 => Ok(FieldDescriptorProto_Type::Message),
            12 => Ok(FieldDescriptorProto_Type::Bytes),
            13 => Ok(FieldDescriptorProto_Type::UInt32),
            14 => Ok(FieldDescriptorProto_Type::Enum),
            15 => Ok(FieldDescriptorProto_Type::SFixed32),
            16 => Ok(FieldDescriptorProto_Type::SFixed64),
            17 => Ok(FieldDescriptorProto_Type::SInt32),
            18 => Ok(FieldDescriptorProto_Type::SInt64),
            _ => Err(codegen::VariantUndefinedError)
        }
    }
}

impl codegen::From<FieldDescriptorProto_Type> for i32 {
    fn from(value: FieldDescriptorProto_Type) -> i32 {
        value as i32
    }
}

#[derive(Clone, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum FieldDescriptorProto_Label {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

impl codegen::TryFrom<i32> for FieldDescriptorProto_Label {
    type Error = codegen::VariantUndefinedError;

    fn try_from(value: i32) -> Result<FieldDescriptorProto_Label, codegen::VariantUndefinedError> {
        match value {
            1 => Ok(FieldDescriptorProto_Label::Optional),
            2 => Ok(FieldDescriptorProto_Label::Required),
            3 => Ok(FieldDescriptorProto_Label::Repeated),
            _ => Err(codegen::VariantUndefinedError)
        }
    }
}

impl codegen::From<FieldDescriptorProto_Label> for i32 {
    fn from(value: FieldDescriptorProto_Label) -> i32 {
        value as i32
    }
}

#[derive(Clone, PartialEq)]
pub struct OneofDescriptorProto {
    name_: Option<String>,
    options_: Option<OneofOptions>,
    _unknown: codegen::UnknownFieldSet,
}

static ONEOF_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();

impl codegen::LiteMessage for OneofDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => input.read_message(self.options_.get_or_insert_with(<OneofOptions as codegen::GeneratedLiteMessage>::new))?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for OneofDescriptorProto {
    fn new() -> Self {
        OneofDescriptorProto {
            name_: None,
            options_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for OneofDescriptorProto { }

impl codegen::GeneratedMessage for OneofDescriptorProto { }

impl OneofDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &ONEOF_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_options(&self) -> Option<&OneofOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut OneofOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: OneofOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct EnumDescriptorProto {
    name_: Option<String>,
    value_: Vec<EnumValueDescriptorProto>,
    options_: Option<EnumOptions>,
    reserved_range_: Vec<EnumDescriptorProto_EnumReservedRange>,
    reserved_name_: Vec<String>,
    _unknown: codegen::UnknownFieldSet
}

static ENUM_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static ENUM_DESCRIPTOR_PROTO_VALUE_CODEC: codegen::Codec<EnumValueDescriptorProto> = codegen::Codec::<EnumValueDescriptorProto>::message(18);
static ENUM_DESCRIPTOR_PROTO_RESERVED_RANGE_CODEC: codegen::Codec<EnumDescriptorProto_EnumReservedRange> = codegen::Codec::<EnumDescriptorProto_EnumReservedRange>::message(34);
static ENUM_DESCRIPTOR_PROTO_RESERVED_NAME_CODEC: codegen::Codec<String> = codegen::Codec::<String>::string(String::new(), 42);

impl codegen::LiteMessage for EnumDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => codegen::collections::repeated::add_entries(&mut self.value_, tag.get(), input, &ENUM_DESCRIPTOR_PROTO_VALUE_CODEC)?,
                26 => input.read_message(self.options_.get_or_insert_with(<EnumOptions as codegen::GeneratedLiteMessage>::new))?,
                34 => codegen::collections::repeated::add_entries(&mut self.reserved_range_, tag.get(), input, &ENUM_DESCRIPTOR_PROTO_RESERVED_RANGE_CODEC)?,
                42 => codegen::collections::repeated::add_entries(&mut self.reserved_name_, tag.get(), input, &ENUM_DESCRIPTOR_PROTO_RESERVED_NAME_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for EnumDescriptorProto {
    fn new() -> Self {
        EnumDescriptorProto {
            name_: None,
            value_: Vec::new(),
            options_: None,
            reserved_range_: Vec::new(),
            reserved_name_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for EnumDescriptorProto { }

impl codegen::GeneratedMessage for EnumDescriptorProto { }

impl EnumDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &ENUM_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_value(&self) -> &Vec<EnumValueDescriptorProto> {
        &self.value_
    }
    pub fn mut_value(&mut self) -> &mut Vec<EnumValueDescriptorProto> {
        &mut self.value_
    }

    pub fn ref_options(&self) -> Option<&EnumOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut EnumOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: EnumOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }

    pub fn ref_reserved_range(&self) -> &Vec<EnumDescriptorProto_EnumReservedRange> {
        &self.reserved_range_
    }
    pub fn mut_reserved_range(&mut self) -> &mut Vec<EnumDescriptorProto_EnumReservedRange> {
        &mut self.reserved_range_
    }

    pub fn ref_reserved_name(&self) -> &Vec<String> {
        &self.reserved_name_
    }
    pub fn mut_reserved_name(&mut self) -> &mut Vec<String> {
        &mut self.reserved_name_
    }
}

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct EnumDescriptorProto_EnumReservedRange {
    start_: Option<i32>,
    end_: Option<i32>,
    _unknown: codegen::UnknownFieldSet
}

static ENUM_DESCRIPTOR_PROTO_ENUM_RESERVED_RANGE_START_DEFAULT: i32 = 0;
static ENUM_DESCRIPTOR_PROTO_ENUM_RESERVED_RANGE_END_DEFAULT: i32 = 0;

impl codegen::LiteMessage for EnumDescriptorProto_EnumReservedRange {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.start_ = Some(input.read_int32()?),
                16 => self.end_ = Some(input.read_int32()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for EnumDescriptorProto_EnumReservedRange {
    fn new() -> Self {
        EnumDescriptorProto_EnumReservedRange {
            start_: None,
            end_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for EnumDescriptorProto_EnumReservedRange { }

impl codegen::GeneratedMessage for EnumDescriptorProto_EnumReservedRange { }

impl EnumDescriptorProto_EnumReservedRange {
    pub fn ref_start(&self) -> Option<&i32> {
        self.start_.as_ref()
    }
    pub fn ref_start_or_default(&self) -> &i32 {
        if let Some(ref start_) = self.start_ {
            start_
        } else {
            &ENUM_DESCRIPTOR_PROTO_ENUM_RESERVED_RANGE_START_DEFAULT
        }
    }
    pub fn mut_start(&mut self) -> Option<&mut i32> {
        self.start_.as_mut()
    }
    pub fn set_start(&mut self, value: i32) {
        self.start_ = Some(value)
    }
    pub fn has_start(&self) -> bool {
        self.start_.is_some()
    }
    pub fn clear_start(&mut self) {
        self.start_ = None
    }

    pub fn ref_end(&self) -> Option<&i32> {
        self.end_.as_ref()
    }
    pub fn ref_end_or_default(&self) -> &i32 {
        if let Some(ref end_) = self.end_ {
            end_
        } else {
            &ENUM_DESCRIPTOR_PROTO_ENUM_RESERVED_RANGE_END_DEFAULT
        }
    }
    pub fn mut_end(&mut self) -> Option<&mut i32> {
        self.end_.as_mut()
    }
    pub fn set_end(&mut self, value: i32) {
        self.end_ = Some(value)
    }
    pub fn has_end(&self) -> bool {
        self.end_.is_some()
    }
    pub fn clear_end(&mut self) {
        self.end_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct EnumValueDescriptorProto {
    name_: Option<String>,
    number_: Option<i32>,
    options_: Option<EnumValueOptions>,
    _unknown: codegen::UnknownFieldSet
}

static ENUM_VALUE_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static ENUM_VALUE_DESCRIPTOR_PROTO_NUMBER_DEFAULT: i32 = 0;

impl codegen::LiteMessage for EnumValueDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                16 => self.number_ = Some(input.read_int32()?),
                26 => input.read_message(self.options_.get_or_insert_with(<EnumValueOptions as codegen::GeneratedLiteMessage>::new))?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for EnumValueDescriptorProto {
    fn new() -> Self {
        EnumValueDescriptorProto {
            name_: None,
            number_: None,
            options_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for EnumValueDescriptorProto { }

impl codegen::GeneratedMessage for EnumValueDescriptorProto { }

impl EnumValueDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &ENUM_VALUE_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_number(&self) -> Option<&i32> {
        self.number_.as_ref()
    }
    pub fn ref_number_or_default(&self) -> &i32 {
        if let Some(ref number_) = self.number_ {
            number_
        } else {
            &ENUM_VALUE_DESCRIPTOR_PROTO_NUMBER_DEFAULT
        }
    }
    pub fn mut_number(&mut self) -> Option<&mut i32> {
        self.number_.as_mut()
    }
    pub fn set_number(&mut self, value: i32) {
        self.number_ = Some(value)
    }
    pub fn has_number(&self) -> bool {
        self.number_.is_some()
    }
    pub fn clear_number(&mut self) {
        self.number_ = None
    }

    pub fn ref_options(&self) -> Option<&EnumValueOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut EnumValueOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: EnumValueOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct ServiceDescriptorProto {
    name_: Option<String>,
    method_: Vec<MethodDescriptorProto>,
    options_: Option<ServiceOptions>,
    _unknown: codegen::UnknownFieldSet
}

static SERVICE_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static SERVICE_DESCRIPTOR_PROTO_METHOD_CODEC: codegen::Codec<MethodDescriptorProto> = codegen::Codec::<MethodDescriptorProto>::message(18);

impl codegen::LiteMessage for ServiceDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => codegen::collections::repeated::add_entries(&mut self.method_, tag.get(), input, &SERVICE_DESCRIPTOR_PROTO_METHOD_CODEC)?,
                26 => input.read_message(self.options_.get_or_insert_with(<ServiceOptions as codegen::GeneratedLiteMessage>::new))?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for ServiceDescriptorProto {
    fn new() -> Self {
        ServiceDescriptorProto {
            name_: None,
            method_: Vec::new(),
            options_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for ServiceDescriptorProto { }

impl codegen::GeneratedMessage for ServiceDescriptorProto { }

impl ServiceDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &SERVICE_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_method(&self) -> &Vec<MethodDescriptorProto> {
        &self.method_
    }
    pub fn mut_method(&mut self) -> &mut Vec<MethodDescriptorProto> {
        &mut self.method_
    }

    pub fn ref_options(&self) -> Option<&ServiceOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut ServiceOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: ServiceOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct MethodDescriptorProto {
    name_: Option<String>,
    input_type_: Option<String>,
    output_type_: Option<String>,
    options_: Option<MethodOptions>,
    client_streaming_: Option<bool>,
    server_streaming_: Option<bool>,
    _unknown: codegen::UnknownFieldSet
}

static METHOD_DESCRIPTOR_PROTO_NAME_DEFAULT: String = String::new();
static METHOD_DESCRIPTOR_PROTO_INPUT_TYPE_DEFAULT: String = String::new();
static METHOD_DESCRIPTOR_PROTO_OUTPUT_TYPE_DEFAULT: String = String::new();
static METHOD_DESCRIPTOR_PROTO_CLIENT_STREAMING_DEFAULT: bool = false;
static METHOD_DESCRIPTOR_PROTO_SERVER_STREAMING_DEFAULT: bool = false;

impl codegen::LiteMessage for MethodDescriptorProto {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_ = Some(input.read_string()?),
                18 => self.input_type_ = Some(input.read_string()?),
                26 => self.output_type_ = Some(input.read_string()?),
                34 => input.read_message(self.options_.get_or_insert_with(<MethodOptions as codegen::GeneratedLiteMessage>::new))?,
                40 => self.client_streaming_ = Some(input.read_bool()?),
                48 => self.server_streaming_ = Some(input.read_bool()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for MethodDescriptorProto {
    fn new() -> Self {
        MethodDescriptorProto {
            name_: None,
            input_type_: None,
            output_type_: None,
            options_: None,
            client_streaming_: None,
            server_streaming_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for MethodDescriptorProto { }

impl codegen::GeneratedMessage for MethodDescriptorProto { }

impl MethodDescriptorProto {
    pub fn ref_name(&self) -> Option<&String> {
        self.name_.as_ref()
    }
    pub fn ref_name_or_default(&self) -> &String {
        if let Some(ref name_) = self.name_ {
            name_
        } else {
            &METHOD_DESCRIPTOR_PROTO_NAME_DEFAULT
        }
    }
    pub fn mut_name(&mut self) -> Option<&mut String> {
        self.name_.as_mut()
    }
    pub fn set_name(&mut self, value: String) {
        self.name_ = Some(value)
    }
    pub fn has_name(&self) -> bool {
        self.name_.is_some()
    }
    pub fn clear_name(&mut self) {
        self.name_ = None
    }

    pub fn ref_input_type(&self) -> Option<&String> {
        self.input_type_.as_ref()
    }
    pub fn ref_input_type_or_default(&self) -> &String {
        if let Some(ref input_type_) = self.input_type_ {
            input_type_
        } else {
            &METHOD_DESCRIPTOR_PROTO_INPUT_TYPE_DEFAULT
        }
    }
    pub fn mut_input_type(&mut self) -> Option<&mut String> {
        self.input_type_.as_mut()
    }
    pub fn set_input_type(&mut self, value: String) {
        self.input_type_ = Some(value)
    }
    pub fn has_input_type(&self) -> bool {
        self.input_type_.is_some()
    }
    pub fn clear_input_type(&mut self) {
        self.input_type_ = None
    }

    pub fn ref_output_type(&self) -> Option<&String> {
        self.output_type_.as_ref()
    }
    pub fn ref_output_type_or_default(&self) -> &String {
        if let Some(ref output_type_) = self.output_type_ {
            output_type_
        } else {
            &METHOD_DESCRIPTOR_PROTO_OUTPUT_TYPE_DEFAULT
        }
    }
    pub fn mut_output_type(&mut self) -> Option<&mut String> {
        self.output_type_.as_mut()
    }
    pub fn set_output_type(&mut self, value: String) {
        self.output_type_ = Some(value)
    }
    pub fn has_output_type(&self) -> bool {
        self.output_type_.is_some()
    }
    pub fn clear_output_type(&mut self) {
        self.output_type_ = None
    }

    pub fn ref_options(&self) -> Option<&MethodOptions> {
        self.options_.as_ref()
    }
    pub fn mut_options(&mut self) -> Option<&mut MethodOptions> {
        self.options_.as_mut()
    }
    pub fn set_options(&mut self, value: MethodOptions) {
        self.options_ = Some(value)
    }
    pub fn has_options(&self) -> bool {
        self.options_.is_some()
    }
    pub fn clear_options(&mut self) {
        self.options_ = None
    }

    pub fn ref_client_streaming(&self) -> Option<&bool> {
        self.client_streaming_.as_ref()
    }
    pub fn ref_client_streaming_or_default(&self) -> &bool {
        if let Some(ref client_streaming_) = self.client_streaming_ {
            client_streaming_
        } else {
            &METHOD_DESCRIPTOR_PROTO_CLIENT_STREAMING_DEFAULT
        }
    }
    pub fn mut_client_streaming(&mut self) -> Option<&mut bool> {
        self.client_streaming_.as_mut()
    }
    pub fn set_client_streaming(&mut self, value: bool) {
        self.client_streaming_ = Some(value)
    }
    pub fn has_client_streaming(&self) -> bool {
        self.client_streaming_.is_some()
    }
    pub fn clear_client_streaming(&mut self) {
        self.client_streaming_ = None
    }

    pub fn ref_server_streaming(&self) -> Option<&bool> {
        self.server_streaming_.as_ref()
    }
    pub fn ref_server_streaming_or_default(&self) -> &bool {
        if let Some(ref server_streaming_) = self.server_streaming_ {
            server_streaming_
        } else {
            &METHOD_DESCRIPTOR_PROTO_SERVER_STREAMING_DEFAULT
        }
    }
    pub fn mut_server_streaming(&mut self) -> Option<&mut bool> {
        self.server_streaming_.as_mut()
    }
    pub fn set_server_streaming(&mut self, value: bool) {
        self.server_streaming_ = Some(value)
    }
    pub fn has_server_streaming(&self) -> bool {
        self.server_streaming_.is_some()
    }
    pub fn clear_server_streaming(&mut self) {
        self.server_streaming_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct FileOptions { 
    java_package_: Option<String>,
    java_outer_classname_: Option<String>,
    java_multiple_files_: Option<bool>,
    java_generate_equals_and_hash_: Option<bool>,
    java_string_check_utf8_: Option<bool>,
    optimize_for_: Option<codegen::EnumValue<FileOptions_OptimizeMode>>,
    go_package_: Option<String>,
    cc_generic_services_: Option<bool>,
    java_generic_services_: Option<bool>,
    py_generic_services_: Option<bool>,
    php_generic_services_: Option<bool>,
    deprecated_: Option<bool>,
    cc_enable_arenas_: Option<bool>,
    objc_class_prefix_: Option<String>,
    csharp_namespace_: Option<String>,
    swift_prefix_: Option<String>,
    php_class_prefix_: Option<String>,
    php_namespace_: Option<String>,
    php_metadata_namespace_: Option<String>,
    ruby_package_: Option<String>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static FILE_OPTIONS_JAVA_PACKAGE_DEFAULT: String = String::new();
static FILE_OPTIONS_JAVA_OUTER_CLASSNAME_DEFAULT: String = String::new();
static FILE_OPTIONS_JAVA_MULTIPLE_FILES_DEFAULT: bool = false;
static FILE_OPTIONS_JAVA_GENERATE_EQUALS_AND_HASH_DEFAULT: bool = false;
static FILE_OPTIONS_JAVA_STRING_CHECK_UTF8_DEFAULT: bool = false;
static FILE_OPTIONS_OPTIMIZE_FOR_DEFAULT: codegen::EnumValue<FileOptions_OptimizeMode> = codegen::EnumValue::Defined(FileOptions_OptimizeMode::Speed);
static FILE_OPTIONS_GO_PACKAGE_DEFAULT: String = String::new();
static FILE_OPTIONS_CC_GENERIC_SERVICES_DEFAULT: bool = false;
static FILE_OPTIONS_JAVA_GENERIC_SERVICES_DEFAULT: bool = false;
static FILE_OPTIONS_PY_GENERIC_SERVICES_DEFAULT: bool = false;
static FILE_OPTIONS_PHP_GENERIC_SERVICES_DEFAULT: bool = false;
static FILE_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static FILE_OPTIONS_CC_ENABLE_ARENAS_DEFAULT: bool = false;
static FILE_OPTIONS_OBJC_CLASS_PREFIX_DEFAULT: String = String::new();
static FILE_OPTIONS_CSHARP_NAMESPACE_DEFAULT: String = String::new();
static FILE_OPTIONS_SWIFT_PREFIX_DEFAULT: String = String::new();
static FILE_OPTIONS_PHP_CLASS_PREFIX_DEFAULT: String = String::new();
static FILE_OPTIONS_PHP_NAMESPACE_DEFAULT: String = String::new();
static FILE_OPTIONS_PHP_METADATA_NAMESPACE_DEFAULT: String = String::new();
static FILE_OPTIONS_RUBY_PACKAGE_DEFAULT: String = String::new();
static FILE_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for FileOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.java_package_ = Some(input.read_string()?),
                66 => self.java_outer_classname_ = Some(input.read_string()?),
                72 => self.optimize_for_ = Some(codegen::EnumValue::from(input.read_int32()?)),
                80 => self.java_multiple_files_ = Some(input.read_bool()?),
                90 => self.go_package_ = Some(input.read_string()?),
                128 => self.cc_generic_services_ = Some(input.read_bool()?),
                136 => self.java_generic_services_ = Some(input.read_bool()?),
                144 => self.py_generic_services_ = Some(input.read_bool()?),
                160 => self.java_generate_equals_and_hash_ = Some(input.read_bool()?),
                184 => self.deprecated_ = Some(input.read_bool()?),
                216 => self.java_string_check_utf8_ = Some(input.read_bool()?),
                248 => self.cc_enable_arenas_ = Some(input.read_bool()?),
                290 => self.objc_class_prefix_ = Some(input.read_string()?),
                298 => self.csharp_namespace_ = Some(input.read_string()?),
                314 => self.swift_prefix_ = Some(input.read_string()?),
                322 => self.php_class_prefix_ = Some(input.read_string()?),
                330 => self.php_namespace_ = Some(input.read_string()?),
                336 => self.php_generic_services_ = Some(input.read_bool()?),
                354 => self.php_metadata_namespace_ = Some(input.read_string()?),
                362 => self.ruby_package_ = Some(input.read_string()?),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &FILE_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for FileOptions {
    fn new() -> Self {
        FileOptions {
            java_package_: None,
            java_outer_classname_: None,
            java_multiple_files_: None,
            java_generate_equals_and_hash_: None,
            java_string_check_utf8_: None,
            optimize_for_: None,
            go_package_: None,
            cc_generic_services_: None,
            java_generic_services_: None,
            py_generic_services_: None,
            php_generic_services_: None,
            deprecated_: None,
            cc_enable_arenas_: None,
            objc_class_prefix_: None,
            csharp_namespace_: None,
            swift_prefix_: None,
            php_class_prefix_: None,
            php_namespace_: None,
            php_metadata_namespace_: None,
            ruby_package_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for FileOptions { }

impl codegen::GeneratedMessage for FileOptions { }

impl FileOptions {
    pub fn ref_java_package(&self) -> Option<&String> {
        self.java_package_.as_ref()
    }
    pub fn ref_java_package_or_default(&self) -> &String {
        if let Some(ref java_package_) = self.java_package_ {
            java_package_
        } else {
            &FILE_OPTIONS_JAVA_PACKAGE_DEFAULT
        }
    }
    pub fn mut_java_package(&mut self) -> Option<&mut String> {
        self.java_package_.as_mut()
    }
    pub fn set_java_package(&mut self, value: String) {
        self.java_package_ = Some(value)
    }
    pub fn has_java_package(&self) -> bool {
        self.java_package_.is_some()
    }
    pub fn clear_java_package(&mut self) {
        self.java_package_ = None
    }

    pub fn ref_java_outer_classname(&self) -> Option<&String> {
        self.java_outer_classname_.as_ref()
    }
    pub fn ref_java_outer_classname_or_default(&self) -> &String {
        if let Some(ref java_outer_classname_) = self.java_outer_classname_ {
            java_outer_classname_
        } else {
            &FILE_OPTIONS_JAVA_OUTER_CLASSNAME_DEFAULT
        }
    }
    pub fn mut_java_outer_classname(&mut self) -> Option<&mut String> {
        self.java_outer_classname_.as_mut()
    }
    pub fn set_java_outer_classname(&mut self, value: String) {
        self.java_outer_classname_ = Some(value)
    }
    pub fn has_java_outer_classname(&self) -> bool {
        self.java_outer_classname_.is_some()
    }
    pub fn clear_java_outer_classname(&mut self) {
        self.java_outer_classname_ = None
    }

    pub fn ref_java_multiple_files(&self) -> Option<&bool> {
        self.java_multiple_files_.as_ref()
    }
    pub fn ref_java_multiple_files_or_default(&self) -> &bool {
        if let Some(ref java_multiple_files_) = self.java_multiple_files_ {
            java_multiple_files_
        } else {
            &FILE_OPTIONS_JAVA_MULTIPLE_FILES_DEFAULT
        }
    }
    pub fn mut_java_multiple_files(&mut self) -> Option<&mut bool> {
        self.java_multiple_files_.as_mut()
    }
    pub fn set_java_multiple_files(&mut self, value: bool) {
        self.java_multiple_files_ = Some(value)
    }
    pub fn has_java_multiple_files(&self) -> bool {
        self.java_multiple_files_.is_some()
    }
    pub fn clear_java_multiple_files(&mut self) {
        self.java_multiple_files_ = None
    }

    pub fn ref_java_generate_equals_and_hash(&self) -> Option<&bool> {
        self.java_generate_equals_and_hash_.as_ref()
    }
    pub fn ref_java_generate_equals_and_hash_or_default(&self) -> &bool {
        if let Some(ref java_generate_equals_and_hash_) = self.java_generate_equals_and_hash_ {
            java_generate_equals_and_hash_
        } else {
            &FILE_OPTIONS_JAVA_GENERATE_EQUALS_AND_HASH_DEFAULT
        }
    }
    pub fn mut_java_generate_equals_and_hash(&mut self) -> Option<&mut bool> {
        self.java_generate_equals_and_hash_.as_mut()
    }
    pub fn set_java_generate_equals_and_hash(&mut self, value: bool) {
        self.java_generate_equals_and_hash_ = Some(value)
    }
    pub fn has_java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash_.is_some()
    }
    pub fn clear_java_generate_equals_and_hash(&mut self) {
        self.java_generate_equals_and_hash_ = None
    }
    pub fn ref_java_string_check_utf8(&self) -> Option<&bool> {
        self.java_string_check_utf8_.as_ref()
    }
    pub fn ref_java_string_check_utf8_or_default(&self) -> &bool {
        if let Some(ref java_string_check_utf8_) = self.java_string_check_utf8_ {
            java_string_check_utf8_
        } else {
            &FILE_OPTIONS_JAVA_STRING_CHECK_UTF8_DEFAULT
        }
    }
    pub fn mut_java_string_check_utf8(&mut self) -> Option<&mut bool> {
        self.java_string_check_utf8_.as_mut()
    }
    pub fn set_java_string_check_utf8(&mut self, value: bool) {
        self.java_string_check_utf8_ = Some(value)
    }
    pub fn has_java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8_.is_some()
    }
    pub fn clear_java_string_check_utf8(&mut self) {
        self.java_string_check_utf8_ = None
    }

    pub fn ref_optimize_for(&self) -> Option<&codegen::EnumValue<FileOptions_OptimizeMode>> {
        self.optimize_for_.as_ref()
    }
    pub fn ref_optimize_for_or_default(&self) -> &codegen::EnumValue<FileOptions_OptimizeMode> {
        if let Some(ref optimize_for_) = self.optimize_for_ {
            optimize_for_
        } else {
            &FILE_OPTIONS_OPTIMIZE_FOR_DEFAULT
        }
    }
    pub fn mut_optimize_for(&mut self) -> Option<&mut codegen::EnumValue<FileOptions_OptimizeMode>> {
        self.optimize_for_.as_mut()
    }
    pub fn set_optimize_for(&mut self, value: codegen::EnumValue<FileOptions_OptimizeMode>) {
        self.optimize_for_ = Some(value)
    }
    pub fn has_optimize_for(&self) -> bool {
        self.optimize_for_.is_some()
    }
    pub fn clear_optimize_for(&mut self) {
        self.optimize_for_ = None
    }

    pub fn ref_go_package(&self) -> Option<&String> {
        self.go_package_.as_ref()
    }
    pub fn ref_go_package_or_default(&self) -> &String {
        if let Some(ref go_package_) = self.go_package_ {
            go_package_
        } else {
            &FILE_OPTIONS_GO_PACKAGE_DEFAULT
        }
    }
    pub fn mut_go_package(&mut self) -> Option<&mut String> {
        self.go_package_.as_mut()
    }
    pub fn set_go_package(&mut self, value: String) {
        self.go_package_ = Some(value)
    }
    pub fn has_go_package(&self) -> bool {
        self.go_package_.is_some()
    }
    pub fn clear_go_package(&mut self) {
        self.go_package_ = None
    }

    pub fn ref_cc_generic_services(&self) -> Option<&bool> {
        self.cc_generic_services_.as_ref()
    }
    pub fn ref_cc_generic_services_or_default(&self) -> &bool {
        if let Some(ref cc_generic_services_) = self.cc_generic_services_ {
            cc_generic_services_
        } else {
            &FILE_OPTIONS_CC_GENERIC_SERVICES_DEFAULT
        }
    }
    pub fn mut_cc_generic_services(&mut self) -> Option<&mut bool> {
        self.cc_generic_services_.as_mut()
    }
    pub fn set_cc_generic_services(&mut self, value: bool) {
        self.cc_generic_services_ = Some(value)
    }
    pub fn has_cc_generic_services(&self) -> bool {
        self.cc_generic_services_.is_some()
    }
    pub fn clear_cc_generic_services(&mut self) {
        self.cc_generic_services_ = None
    }

    pub fn ref_java_generic_services(&self) -> Option<&bool> {
        self.java_generic_services_.as_ref()
    }
    pub fn ref_java_generic_services_or_default(&self) -> &bool {
        if let Some(ref java_generic_services_) = self.java_generic_services_ {
            java_generic_services_
        } else {
            &FILE_OPTIONS_JAVA_GENERIC_SERVICES_DEFAULT
        }
    }
    pub fn mut_java_generic_services(&mut self) -> Option<&mut bool> {
        self.java_generic_services_.as_mut()
    }
    pub fn set_java_generic_services(&mut self, value: bool) {
        self.java_generic_services_ = Some(value)
    }
    pub fn has_java_generic_services(&self) -> bool {
        self.java_generic_services_.is_some()
    }
    pub fn clear_java_generic_services(&mut self) {
        self.java_generic_services_ = None
    }

    pub fn ref_py_generic_services(&self) -> Option<&bool> {
        self.py_generic_services_.as_ref()
    }
    pub fn ref_py_generic_services_or_default(&self) -> &bool {
        if let Some(ref py_generic_services_) = self.py_generic_services_ {
            py_generic_services_
        } else {
            &FILE_OPTIONS_PY_GENERIC_SERVICES_DEFAULT
        }
    }
    pub fn mut_py_generic_services(&mut self) -> Option<&mut bool> {
        self.py_generic_services_.as_mut()
    }
    pub fn set_py_generic_services(&mut self, value: bool) {
        self.py_generic_services_ = Some(value)
    }
    pub fn has_py_generic_services(&self) -> bool {
        self.py_generic_services_.is_some()
    }
    pub fn clear_py_generic_services(&mut self) {
        self.py_generic_services_ = None
    }

    pub fn ref_php_generic_services(&self) -> Option<&bool> {
        self.php_generic_services_.as_ref()
    }
    pub fn ref_php_generic_services_or_default(&self) -> &bool {
        if let Some(ref php_generic_services_) = self.php_generic_services_ {
            php_generic_services_
        } else {
            &FILE_OPTIONS_PHP_GENERIC_SERVICES_DEFAULT
        }
    }
    pub fn mut_php_generic_services(&mut self) -> Option<&mut bool> {
        self.php_generic_services_.as_mut()
    }
    pub fn set_php_generic_services(&mut self, value: bool) {
        self.php_generic_services_ = Some(value)
    }
    pub fn has_php_generic_services(&self) -> bool {
        self.php_generic_services_.is_some()
    }
    pub fn clear_php_generic_services(&mut self) {
        self.php_generic_services_ = None
    }

    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &FILE_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }

    pub fn ref_cc_enable_arenas(&self) -> Option<&bool> {
        self.cc_enable_arenas_.as_ref()
    }
    pub fn ref_cc_enable_arenas_or_default(&self) -> &bool {
        if let Some(ref cc_enable_arenas_) = self.cc_enable_arenas_ {
            cc_enable_arenas_
        } else {
            &FILE_OPTIONS_CC_ENABLE_ARENAS_DEFAULT
        }
    }
    pub fn mut_cc_enable_arenas(&mut self) -> Option<&mut bool> {
        self.cc_enable_arenas_.as_mut()
    }
    pub fn set_cc_enable_arenas(&mut self, value: bool) {
        self.cc_enable_arenas_ = Some(value)
    }
    pub fn has_cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas_.is_some()
    }
    pub fn clear_cc_enable_arenas(&mut self) {
        self.cc_enable_arenas_ = None
    }

    pub fn ref_objc_class_prefix(&self) -> Option<&String> {
        self.objc_class_prefix_.as_ref()
    }
    pub fn ref_objc_class_prefix_or_default(&self) -> &String {
        if let Some(ref objc_class_prefix_) = self.objc_class_prefix_ {
            objc_class_prefix_
        } else {
            &FILE_OPTIONS_OBJC_CLASS_PREFIX_DEFAULT
        }
    }
    pub fn mut_objc_class_prefix(&mut self) -> Option<&mut String> {
        self.objc_class_prefix_.as_mut()
    }
    pub fn set_objc_class_prefix(&mut self, value: String) {
        self.objc_class_prefix_ = Some(value)
    }
    pub fn has_objc_class_prefix(&self) -> bool {
        self.objc_class_prefix_.is_some()
    }
    pub fn clear_objc_class_prefix(&mut self) {
        self.objc_class_prefix_ = None
    }

    pub fn ref_csharp_namespace(&self) -> Option<&String> {
        self.csharp_namespace_.as_ref()
    }
    pub fn ref_csharp_namespace_or_default(&self) -> &String {
        if let Some(ref csharp_namespace_) = self.csharp_namespace_ {
            csharp_namespace_
        } else {
            &FILE_OPTIONS_CSHARP_NAMESPACE_DEFAULT
        }
    }
    pub fn mut_csharp_namespace(&mut self) -> Option<&mut String> {
        self.csharp_namespace_.as_mut()
    }
    pub fn set_csharp_namespace(&mut self, value: String) {
        self.csharp_namespace_ = Some(value)
    }
    pub fn has_csharp_namespace(&self) -> bool {
        self.csharp_namespace_.is_some()
    }
    pub fn clear_csharp_namespace(&mut self) {
        self.csharp_namespace_ = None
    }

    pub fn ref_swift_prefix(&self) -> Option<&String> {
        self.swift_prefix_.as_ref()
    }
    pub fn ref_swift_prefix_or_default(&self) -> &String {
        if let Some(ref swift_prefix_) = self.swift_prefix_ {
            swift_prefix_
        } else {
            &FILE_OPTIONS_SWIFT_PREFIX_DEFAULT
        }
    }
    pub fn mut_swift_prefix(&mut self) -> Option<&mut String> {
        self.swift_prefix_.as_mut()
    }
    pub fn set_swift_prefix(&mut self, value: String) {
        self.swift_prefix_ = Some(value)
    }
    pub fn has_swift_prefix(&self) -> bool {
        self.swift_prefix_.is_some()
    }
    pub fn clear_swift_prefix(&mut self) {
        self.swift_prefix_ = None
    }

    pub fn ref_php_class_prefix(&self) -> Option<&String> {
        self.php_class_prefix_.as_ref()
    }
    pub fn ref_php_class_prefix_or_default(&self) -> &String {
        if let Some(ref php_class_prefix_) = self.php_class_prefix_ {
            php_class_prefix_
        } else {
            &FILE_OPTIONS_PHP_CLASS_PREFIX_DEFAULT
        }
    }
    pub fn mut_php_class_prefix(&mut self) -> Option<&mut String> {
        self.php_class_prefix_.as_mut()
    }
    pub fn set_php_class_prefix(&mut self, value: String) {
        self.php_class_prefix_ = Some(value)
    }
    pub fn has_php_class_prefix(&self) -> bool {
        self.php_class_prefix_.is_some()
    }
    pub fn clear_php_class_prefix(&mut self) {
        self.php_class_prefix_ = None
    }

    pub fn ref_php_namespace(&self) -> Option<&String> {
        self.php_namespace_.as_ref()
    }
    pub fn ref_php_namespace_or_default(&self) -> &String {
        if let Some(ref php_namespace_) = self.php_namespace_ {
            php_namespace_
        } else {
            &FILE_OPTIONS_PHP_NAMESPACE_DEFAULT
        }
    }
    pub fn mut_php_namespace(&mut self) -> Option<&mut String> {
        self.php_namespace_.as_mut()
    }
    pub fn set_php_namespace(&mut self, value: String) {
        self.php_namespace_ = Some(value)
    }
    pub fn has_php_namespace(&self) -> bool {
        self.php_namespace_.is_some()
    }
    pub fn clear_php_namespace(&mut self) {
        self.php_namespace_ = None
    }

    pub fn ref_php_metadata_namespace(&self) -> Option<&String> {
        self.php_metadata_namespace_.as_ref()
    }
    pub fn ref_php_metadata_namespace_or_default(&self) -> &String {
        if let Some(ref php_metadata_namespace_) = self.php_metadata_namespace_ {
            php_metadata_namespace_
        } else {
            &FILE_OPTIONS_PHP_METADATA_NAMESPACE_DEFAULT
        }
    }
    pub fn mut_php_metadata_namespace(&mut self) -> Option<&mut String> {
        self.php_metadata_namespace_.as_mut()
    }
    pub fn set_php_metadata_namespace(&mut self, value: String) {
        self.php_metadata_namespace_ = Some(value)
    }
    pub fn has_php_metadata_namespace(&self) -> bool {
        self.php_metadata_namespace_.is_some()
    }
    pub fn clear_php_metadata_namespace(&mut self) {
        self.php_metadata_namespace_ = None
    }

    pub fn ref_ruby_package(&self) -> Option<&String> {
        self.ruby_package_.as_ref()
    }
    pub fn ref_ruby_package_or_default(&self) -> &String {
        if let Some(ref ruby_package_) = self.ruby_package_ {
            ruby_package_
        } else {
            &FILE_OPTIONS_RUBY_PACKAGE_DEFAULT
        }
    }
    pub fn mut_ruby_package(&mut self) -> Option<&mut String> {
        self.ruby_package_.as_mut()
    }
    pub fn set_ruby_package(&mut self, value: String) {
        self.ruby_package_ = Some(value)
    }
    pub fn has_ruby_package(&self) -> bool {
        self.ruby_package_.is_some()
    }
    pub fn clear_ruby_package(&mut self) {
        self.ruby_package_ = None
    }

    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum FileOptions_OptimizeMode {
    Speed = 1,
    CodeSize = 2,
    LiteRuntime = 3
}

impl codegen::TryFrom<i32> for FileOptions_OptimizeMode {
    type Error = codegen::VariantUndefinedError;

    fn try_from(value: i32) -> Result<FileOptions_OptimizeMode, codegen::VariantUndefinedError> {
        match value {
            1 => Ok(FileOptions_OptimizeMode::Speed),
            2 => Ok(FileOptions_OptimizeMode::CodeSize),
            3 => Ok(FileOptions_OptimizeMode::LiteRuntime),
            _ => Err(codegen::VariantUndefinedError)
        }
    }
}

impl codegen::From<FileOptions_OptimizeMode> for i32 {
    fn from(value: FileOptions_OptimizeMode) -> i32 {
        value as i32
    }
}

#[derive(Clone, PartialEq)]
pub struct MessageOptions {
    message_set_wire_format_: Option<bool>,
    no_standard_descriptor_accessor_: Option<bool>,
    deprecated_: Option<bool>,
    map_entry_: Option<bool>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static MESSAGE_OPTIONS_MESSAGE_SET_WIRE_FORMAT_DEFAULT: bool = false;
static MESSAGE_OPTIONS_NO_STANDARD_DESCRIPTOR_ACCESSOR_DEFAULT: bool = false;
static MESSAGE_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static MESSAGE_OPTIONS_MAP_ENTRY_DEFAULT: bool = false;
static MESSAGE_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for MessageOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.message_set_wire_format_ = Some(input.read_bool()?),
                16 => self.no_standard_descriptor_accessor_ = Some(input.read_bool()?),
                24 => self.deprecated_ = Some(input.read_bool()?),
                56 => self.map_entry_ = Some(input.read_bool()?),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &MESSAGE_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for MessageOptions {
    fn new() -> Self {
        MessageOptions {
            message_set_wire_format_: None,
            no_standard_descriptor_accessor_: None,
            deprecated_: None,
            map_entry_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for MessageOptions { }

impl codegen::GeneratedMessage for MessageOptions { }

impl MessageOptions {
    pub fn ref_message_set_wire_format(&self) -> Option<&bool> {
        self.message_set_wire_format_.as_ref()
    }
    pub fn ref_message_set_wire_format_or_default(&self) -> &bool {
        if let Some(ref message_set_wire_format_) = self.message_set_wire_format_ {
            message_set_wire_format_
        } else {
            &MESSAGE_OPTIONS_MESSAGE_SET_WIRE_FORMAT_DEFAULT
        }
    }
    pub fn mut_message_set_wire_format(&mut self) -> Option<&mut bool> {
        self.message_set_wire_format_.as_mut()
    }
    pub fn set_message_set_wire_format(&mut self, value: bool) {
        self.message_set_wire_format_ = Some(value)
    }
    pub fn has_message_set_wire_format(&self) -> bool {
        self.message_set_wire_format_.is_some()
    }
    pub fn clear_message_set_wire_format(&mut self) {
        self.message_set_wire_format_ = None
    }

    pub fn ref_no_standard_descriptor_accessor(&self) -> Option<&bool> {
        self.no_standard_descriptor_accessor_.as_ref()
    }
    pub fn ref_no_standard_descriptor_accessor_or_default(&self) -> &bool {
        if let Some(ref no_standard_descriptor_accessor_) = self.no_standard_descriptor_accessor_ {
            no_standard_descriptor_accessor_
        } else {
            &MESSAGE_OPTIONS_NO_STANDARD_DESCRIPTOR_ACCESSOR_DEFAULT
        }
    }
    pub fn mut_no_standard_descriptor_accessor(&mut self) -> Option<&mut bool> {
        self.no_standard_descriptor_accessor_.as_mut()
    }
    pub fn set_no_standard_descriptor_accessor(&mut self, value: bool) {
        self.no_standard_descriptor_accessor_ = Some(value)
    }
    pub fn has_no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor_.is_some()
    }
    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        self.no_standard_descriptor_accessor_ = None
    }

    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &MESSAGE_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }

    pub fn ref_map_entry(&self) -> Option<&bool> {
        self.map_entry_.as_ref()
    }
    pub fn ref_map_entry_or_default(&self) -> &bool {
        if let Some(ref map_entry_) = self.map_entry_ {
            map_entry_
        } else {
            &MESSAGE_OPTIONS_MAP_ENTRY_DEFAULT
        }
    }
    pub fn mut_map_entry(&mut self) -> Option<&mut bool> {
        self.map_entry_.as_mut()
    }
    pub fn set_map_entry(&mut self, value: bool) {
        self.map_entry_ = Some(value)
    }
    pub fn has_map_entry(&self) -> bool {
        self.map_entry_.is_some()
    }
    pub fn clear_map_entry(&mut self) {
        self.map_entry_ = None
    }

    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq)]
pub struct FieldOptions {
    ctype_: Option<codegen::EnumValue<FieldOptions_CType>>,
    packed_: Option<bool>,
    jstype_: Option<codegen::EnumValue<FieldOptions_JSType>>,
    lazy_: Option<bool>,
    deprecated_: Option<bool>,
    weak_: Option<bool>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static FIELD_OPTIONS_CTYPE_DEFAULT: codegen::EnumValue<FieldOptions_CType> = codegen::EnumValue::Defined(FieldOptions_CType::String);
static FIELD_OPTIONS_PACKED_DEFAULT: bool = false;
static FIELD_OPTIONS_JSTYPE_DEFAULT: codegen::EnumValue<FieldOptions_JSType> = codegen::EnumValue::Defined(FieldOptions_JSType::Normal);
static FIELD_OPTIONS_LAZY_DEFAULT: bool = false;
static FIELD_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static FIELD_OPTIONS_WEAK_DEFAULT: bool = false;
static FIELD_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for FieldOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.ctype_ = Some(codegen::EnumValue::from(input.read_int32()?)),
                16 => self.packed_ = Some(input.read_bool()?),
                24 => self.deprecated_ = Some(input.read_bool()?),
                40 => self.lazy_ = Some(input.read_bool()?),
                48 => self.jstype_ = Some(codegen::EnumValue::from(input.read_int32()?)),
                80 => self.weak_ = Some(input.read_bool()?),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &FIELD_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for FieldOptions {
    fn new() -> Self {
        FieldOptions {
            ctype_: None,
            packed_: None,
            jstype_: None,
            lazy_: None,
            deprecated_: None,
            weak_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for FieldOptions { }

impl codegen::GeneratedMessage for FieldOptions { }

impl FieldOptions {
    pub fn ref_ctype(&self) -> Option<&codegen::EnumValue<FieldOptions_CType>> {
        self.ctype_.as_ref()
    }
    pub fn ref_ctype_or_default(&self) -> &codegen::EnumValue<FieldOptions_CType> {
        if let Some(ref ctype_) = self.ctype_ {
            ctype_
        } else {
            &FIELD_OPTIONS_CTYPE_DEFAULT
        }
    }
    pub fn mut_ctype(&mut self) -> Option<&mut codegen::EnumValue<FieldOptions_CType>> {
        self.ctype_.as_mut()
    }
    pub fn set_ctype(&mut self, value: codegen::EnumValue<FieldOptions_CType>) {
        self.ctype_ = Some(value)
    }
    pub fn has_ctype(&self) -> bool {
        self.ctype_.is_some()
    }
    pub fn clear_ctype(&mut self) {
        self.ctype_ = None
    }

    pub fn ref_packed(&self) -> Option<&bool> {
        self.packed_.as_ref()
    }
    pub fn ref_packed_or_default(&self) -> &bool {
        if let Some(ref packed_) = self.packed_ {
            packed_
        } else {
            &FIELD_OPTIONS_PACKED_DEFAULT
        }
    }
    pub fn mut_packed(&mut self) -> Option<&mut bool> {
        self.packed_.as_mut()
    }
    pub fn set_packed(&mut self, value: bool) {
        self.packed_ = Some(value)
    }
    pub fn has_packed(&self) -> bool {
        self.packed_.is_some()
    }
    pub fn clear_packed(&mut self) {
        self.packed_ = None
    }

    pub fn ref_jstype(&self) -> Option<&codegen::EnumValue<FieldOptions_JSType>> {
        self.jstype_.as_ref()
    }
    pub fn ref_jstype_or_default(&self) -> &codegen::EnumValue<FieldOptions_JSType> {
        if let Some(ref jstype_) = self.jstype_ {
            jstype_
        } else {
            &FIELD_OPTIONS_JSTYPE_DEFAULT
        }
    }
    pub fn mut_jstype(&mut self) -> Option<&mut codegen::EnumValue<FieldOptions_JSType>> {
        self.jstype_.as_mut()
    }
    pub fn set_jstype(&mut self, value: codegen::EnumValue<FieldOptions_JSType>) {
        self.jstype_ = Some(value)
    }
    pub fn has_jstype(&self) -> bool {
        self.jstype_.is_some()
    }
    pub fn clear_jstype(&mut self) {
        self.jstype_ = None
    }

    pub fn ref_lazy(&self) -> Option<&bool> {
        self.lazy_.as_ref()
    }
    pub fn ref_lazy_or_default(&self) -> &bool {
        if let Some(ref lazy_) = self.lazy_ {
            lazy_
        } else {
            &FIELD_OPTIONS_LAZY_DEFAULT
        }
    }
    pub fn mut_lazy(&mut self) -> Option<&mut bool> {
        self.lazy_.as_mut()
    }
    pub fn set_lazy(&mut self, value: bool) {
        self.lazy_ = Some(value)
    }
    pub fn has_lazy(&self) -> bool {
        self.lazy_.is_some()
    }
    pub fn clear_lazy(&mut self) {
        self.lazy_ = None
    }

    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &FIELD_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }
    
    pub fn ref_weak(&self) -> Option<&bool> {
        self.weak_.as_ref()
    }
    pub fn ref_weak_or_default(&self) -> &bool {
        if let Some(ref weak_) = self.weak_ {
            weak_
        } else {
            &FIELD_OPTIONS_WEAK_DEFAULT
        }
    }
    pub fn mut_weak(&mut self) -> Option<&mut bool> {
        self.weak_.as_mut()
    }
    pub fn set_weak(&mut self, value: bool) {
        self.weak_ = Some(value)
    }
    pub fn has_weak(&self) -> bool {
        self.weak_.is_some()
    }
    pub fn clear_weak(&mut self) {
        self.weak_ = None
    }

    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum FieldOptions_CType {
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

impl codegen::TryFrom<i32> for FieldOptions_CType {
    type Error = codegen::VariantUndefinedError;

    fn try_from(value: i32) -> Result<FieldOptions_CType, codegen::VariantUndefinedError> {
        match value {
            0 => Ok(FieldOptions_CType::String),
            1 => Ok(FieldOptions_CType::Cord),
            2 => Ok(FieldOptions_CType::StringPiece),
            _ => Err(codegen::VariantUndefinedError)
        }
    }
}

impl codegen::From<FieldOptions_CType> for i32 {
    fn from(value: FieldOptions_CType) -> i32 {
        value as i32
    }
}

#[derive(Clone, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum FieldOptions_JSType {
    Normal = 0,
    String = 1,
    Number = 2,
}

impl codegen::TryFrom<i32> for FieldOptions_JSType {
    type Error = codegen::VariantUndefinedError;

    fn try_from(value: i32) -> Result<FieldOptions_JSType, codegen::VariantUndefinedError> {
        match value {
            0 => Ok(FieldOptions_JSType::Normal),
            1 => Ok(FieldOptions_JSType::String),
            2 => Ok(FieldOptions_JSType::Number),
            _ => Err(codegen::VariantUndefinedError)
        }
    }
}

impl codegen::From<FieldOptions_JSType> for i32 {
    fn from(value: FieldOptions_JSType) -> i32 {
        value as i32
    }
}

#[derive(Clone, PartialEq)]
pub struct OneofOptions {
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static ONEOF_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for OneofOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &ONEOF_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for OneofOptions {
    fn new() -> Self {
        OneofOptions {
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for OneofOptions { }

impl codegen::GeneratedMessage for OneofOptions { }

impl OneofOptions {
    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq)]
pub struct EnumOptions {
    allow_alias_: Option<bool>,
    deprecated_: Option<bool>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static ENUM_OPTIONS_ALLOW_ALIAS_DEFAULT: bool = false;
static ENUM_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static ENUM_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for EnumOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                16 => self.allow_alias_ = Some(input.read_bool()?),
                24 => self.deprecated_ = Some(input.read_bool()?),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &ENUM_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for EnumOptions {
    fn new() -> Self {
        EnumOptions {
            allow_alias_: None,
            deprecated_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for EnumOptions { }

impl codegen::GeneratedMessage for EnumOptions { }

impl EnumOptions {
    pub fn ref_allow_alias(&self) -> Option<&bool> {
        self.allow_alias_.as_ref()
    }
    pub fn ref_allow_alias_or_default(&self) -> &bool {
        if let Some(ref allow_alias_) = self.allow_alias_ {
            allow_alias_
        } else {
            &ENUM_OPTIONS_ALLOW_ALIAS_DEFAULT
        }
    }
    pub fn mut_allow_alias(&mut self) -> Option<&mut bool> {
        self.allow_alias_.as_mut()
    }
    pub fn set_allow_alias(&mut self, value: bool) {
        self.allow_alias_ = Some(value)
    }
    pub fn has_allow_alias(&self) -> bool {
        self.allow_alias_.is_some()
    }
    pub fn clear_allow_alias(&mut self) {
        self.allow_alias_ = None
    }

    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &ENUM_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }
    
    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq)]
pub struct EnumValueOptions {
    deprecated_: Option<bool>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static ENUM_VALUE_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static ENUM_VALUE_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for EnumValueOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.deprecated_ = Some(input.read_bool()?),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &ENUM_VALUE_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for EnumValueOptions {
    fn new() -> Self {
        EnumValueOptions {
            deprecated_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for EnumValueOptions { }

impl codegen::GeneratedMessage for EnumValueOptions { }

impl EnumValueOptions {
    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &ENUM_VALUE_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }

    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq)]
pub struct ServiceOptions {
    deprecated_: Option<bool>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static SERVICE_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static SERVICE_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for ServiceOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                264 => self.deprecated_ = Some(input.read_bool()?),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &SERVICE_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for ServiceOptions {
    fn new() -> Self {
        ServiceOptions {
            deprecated_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for ServiceOptions { }

impl codegen::GeneratedMessage for ServiceOptions { }

impl ServiceOptions {
    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &SERVICE_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }

    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq)]
pub struct MethodOptions {
    deprecated_: Option<bool>,
    idempotency_level_: Option<codegen::EnumValue<MethodOptions_IdempotencyLevel>>,
    uninterpreted_option_: Vec<UninterpretedOption>,
    _unknown: codegen::UnknownFieldSet
}

static METHOD_OPTIONS_DEPRECATED_DEFAULT: bool = false;
static METHOD_OPTIONS_IDEMPOTENCY_LEVEL_DEFAULT: codegen::EnumValue<MethodOptions_IdempotencyLevel> = codegen::EnumValue::Defined(MethodOptions_IdempotencyLevel::Unknown);
static METHOD_OPTIONS_UNINTERPRETED_OPTION_CODEC: codegen::Codec<UninterpretedOption> = codegen::Codec::<UninterpretedOption>::message(7994);

impl codegen::LiteMessage for MethodOptions {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.deprecated_ = Some(input.read_bool()?),
                272 => self.idempotency_level_ = Some(codegen::EnumValue::from(input.read_int32()?)),
                7994 => codegen::collections::repeated::add_entries(&mut self.uninterpreted_option_, tag.get(), input, &METHOD_OPTIONS_UNINTERPRETED_OPTION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for MethodOptions {
    fn new() -> Self {
        MethodOptions {
            deprecated_: None,
            idempotency_level_: None,
            uninterpreted_option_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for MethodOptions { }

impl codegen::GeneratedMessage for MethodOptions { }

impl MethodOptions {
    pub fn ref_deprecated(&self) -> Option<&bool> {
        self.deprecated_.as_ref()
    }
    pub fn ref_deprecated_or_default(&self) -> &bool {
        if let Some(ref deprecated_) = self.deprecated_ {
            deprecated_
        } else {
            &METHOD_OPTIONS_DEPRECATED_DEFAULT
        }
    }
    pub fn mut_deprecated(&mut self) -> Option<&mut bool> {
        self.deprecated_.as_mut()
    }
    pub fn set_deprecated(&mut self, value: bool) {
        self.deprecated_ = Some(value)
    }
    pub fn has_deprecated(&self) -> bool {
        self.deprecated_.is_some()
    }
    pub fn clear_deprecated(&mut self) {
        self.deprecated_ = None
    }

    pub fn ref_idempotency_level(&self) -> Option<&codegen::EnumValue<MethodOptions_IdempotencyLevel>> {
        self.idempotency_level_.as_ref()
    }
    pub fn ref_idempotency_level_or_default(&self) -> &codegen::EnumValue<MethodOptions_IdempotencyLevel> {
        if let Some(ref idempotency_level_) = self.idempotency_level_ {
            idempotency_level_
        } else {
            &METHOD_OPTIONS_IDEMPOTENCY_LEVEL_DEFAULT
        }
    }
    pub fn mut_idempotency_level(&mut self) -> Option<&mut codegen::EnumValue<MethodOptions_IdempotencyLevel>> {
        self.idempotency_level_.as_mut()
    }
    pub fn set_idempotency_level(&mut self, value: codegen::EnumValue<MethodOptions_IdempotencyLevel>) {
        self.idempotency_level_ = Some(value)
    }
    pub fn has_idempotency_level(&self) -> bool {
        self.idempotency_level_.is_some()
    }
    pub fn clear_idempotency_level(&mut self) {
        self.idempotency_level_ = None
    }

    pub fn ref_uninterpreted_option(&self) -> &Vec<UninterpretedOption> {
        &self.uninterpreted_option_
    }
    pub fn mut_uninterpreted_option(&mut self) -> &mut Vec<UninterpretedOption> {
        &mut self.uninterpreted_option_
    }
}

#[derive(Clone, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum MethodOptions_IdempotencyLevel {
    Unknown = 0,
    NoSideEffects = 1,
    Idempotent = 2,
}

impl codegen::TryFrom<i32> for MethodOptions_IdempotencyLevel {
    type Error = codegen::VariantUndefinedError;

    fn try_from(value: i32) -> Result<MethodOptions_IdempotencyLevel, codegen::VariantUndefinedError> {
        match value {
            0 => Ok(MethodOptions_IdempotencyLevel::Unknown),
            1 => Ok(MethodOptions_IdempotencyLevel::NoSideEffects),
            2 => Ok(MethodOptions_IdempotencyLevel::Idempotent),
            _ => Err(codegen::VariantUndefinedError)
        }
    }
}

impl codegen::From<MethodOptions_IdempotencyLevel> for i32 {
    fn from(value: MethodOptions_IdempotencyLevel) -> i32 {
        value as i32
    }
}

#[derive(Clone, PartialEq)]
pub struct UninterpretedOption {
    name_: Vec<UninterpretedOption_NamePart>,
    identifier_value_: Option<String>,
    positive_int_value_: Option<u64>,
    negative_int_value_: Option<i64>,
    double_value_: Option<f64>,
    string_value_: Option<Vec<u8>>,
    aggregate_value_: Option<String>,
    _unknown: codegen::UnknownFieldSet
}

static UNINTERPRETED_OPTION_NAME_CODEC: codegen::Codec<UninterpretedOption_NamePart> = codegen::Codec::<UninterpretedOption_NamePart>::message(18);
static UNINTERPRETED_OPTION_IDENTIFIER_VALUE_DEFAULT: String = String::new();
static UNINTERPRETED_OPTION_POSITIVE_INT_VALUE_DEFAULT: u64 = 0;
static UNINTERPRETED_OPTION_NEGATIVE_INT_VALUE_DEFAULT: i64 = 0;
static UNINTERPRETED_OPTION_DOUBLE_VALUE_DEFAULT: f64 = 0.0;
static UNINTERPRETED_OPTION_STRING_VALUE_DEFAULT: Vec<u8> = Vec::new();
static UNINTERPRETED_OPTION_AGGREGATE_VALUE_DEFAULT: String = String::new();

impl codegen::LiteMessage for UninterpretedOption {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                18 => codegen::collections::repeated::add_entries(&mut self.name_, tag.get(), input, &UNINTERPRETED_OPTION_NAME_CODEC)?,
                26 => self.identifier_value_ = Some(input.read_string()?),
                32 => self.positive_int_value_ = Some(input.read_uint64()?),
                40 => self.negative_int_value_ = Some(input.read_int64()?),
                49 => self.double_value_ = Some(input.read_double()?),
                58 => self.string_value_ = Some(input.read_bytes()?),
                66 => self.aggregate_value_ = Some(input.read_string()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for UninterpretedOption {
    fn new() -> Self {
        UninterpretedOption {
            name_: Vec::new(),
            identifier_value_: None,
            positive_int_value_: None,
            negative_int_value_: None,
            double_value_: None,
            string_value_: None,
            aggregate_value_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for UninterpretedOption { }

impl codegen::GeneratedMessage for UninterpretedOption { }

impl UninterpretedOption {
    pub fn ref_name(&self) -> &Vec<UninterpretedOption_NamePart> {
        &self.name_
    }
    pub fn mut_name(&mut self) -> &mut Vec<UninterpretedOption_NamePart> {
        &mut self.name_
    }

    pub fn ref_identifier_value(&self) -> Option<&String> {
        self.identifier_value_.as_ref()
    }
    pub fn ref_identifier_value_or_default(&self) -> &String {
        if let Some(ref identifier_value_) = self.identifier_value_ {
            identifier_value_
        } else {
            &UNINTERPRETED_OPTION_IDENTIFIER_VALUE_DEFAULT
        }
    }
    pub fn mut_identifier_value(&mut self) -> Option<&mut String> {
        self.identifier_value_.as_mut()
    }
    pub fn set_identifier_value(&mut self, value: String) {
        self.identifier_value_ = Some(value)
    }
    pub fn has_identifier_value(&self) -> bool {
        self.identifier_value_.is_some()
    }
    pub fn clear_identifier_value(&mut self) {
        self.identifier_value_ = None
    }

    pub fn ref_positive_int_value(&self) -> Option<&u64> {
        self.positive_int_value_.as_ref()
    }
    pub fn ref_positive_int_value_or_default(&self) -> &u64 {
        if let Some(ref positive_int_value_) = self.positive_int_value_ {
            positive_int_value_
        } else {
            &UNINTERPRETED_OPTION_POSITIVE_INT_VALUE_DEFAULT
        }
    }
    pub fn mut_positive_int_value(&mut self) -> Option<&mut u64> {
        self.positive_int_value_.as_mut()
    }
    pub fn set_positive_int_value(&mut self, value: u64) {
        self.positive_int_value_ = Some(value)
    }
    pub fn has_positive_int_value(&self) -> bool {
        self.positive_int_value_.is_some()
    }
    pub fn clear_positive_int_value(&mut self) {
        self.positive_int_value_ = None
    }

    pub fn ref_negative_int_value(&self) -> Option<&i64> {
        self.negative_int_value_.as_ref()
    }
    pub fn ref_negative_int_value_or_default(&self) -> &i64 {
        if let Some(ref negative_int_value_) = self.negative_int_value_ {
            negative_int_value_
        } else {
            &UNINTERPRETED_OPTION_NEGATIVE_INT_VALUE_DEFAULT
        }
    }
    pub fn mut_negative_int_value(&mut self) -> Option<&mut i64> {
        self.negative_int_value_.as_mut()
    }
    pub fn set_negative_int_value(&mut self, value: i64) {
        self.negative_int_value_ = Some(value)
    }
    pub fn has_negative_int_value(&self) -> bool {
        self.negative_int_value_.is_some()
    }
    pub fn clear_negative_int_value(&mut self) {
        self.negative_int_value_ = None
    }

    pub fn ref_double_value(&self) -> Option<&f64> {
        self.double_value_.as_ref()
    }
    pub fn ref_double_value_or_default(&self) -> &f64 {
        if let Some(ref double_value_) = self.double_value_ {
            double_value_
        } else {
            &UNINTERPRETED_OPTION_DOUBLE_VALUE_DEFAULT
        }
    }
    pub fn mut_double_value(&mut self) -> Option<&mut f64> {
        self.double_value_.as_mut()
    }
    pub fn set_double_value(&mut self, value: f64) {
        self.double_value_ = Some(value)
    }
    pub fn has_double_value(&self) -> bool {
        self.double_value_.is_some()
    }
    pub fn clear_double_value(&mut self) {
        self.double_value_ = None
    }

    pub fn ref_string_value(&self) -> Option<&Vec<u8>> {
        self.string_value_.as_ref()
    }
    pub fn ref_string_value_or_default(&self) -> &Vec<u8> {
        if let Some(ref string_value_) = self.string_value_ {
            string_value_
        } else {
            &UNINTERPRETED_OPTION_STRING_VALUE_DEFAULT
        }
    }
    pub fn mut_string_value(&mut self) -> Option<&mut Vec<u8>> {
        self.string_value_.as_mut()
    }
    pub fn set_string_value(&mut self, value: Vec<u8>) {
        self.string_value_ = Some(value)
    }
    pub fn has_string_value(&self) -> bool {
        self.string_value_.is_some()
    }
    pub fn clear_string_value(&mut self) {
        self.string_value_ = None
    }

    pub fn ref_aggregate_value(&self) -> Option<&String> {
        self.aggregate_value_.as_ref()
    }
    pub fn ref_aggregate_value_or_default(&self) -> &String {
        if let Some(ref aggregate_value_) = self.aggregate_value_ {
            aggregate_value_
        } else {
            &UNINTERPRETED_OPTION_AGGREGATE_VALUE_DEFAULT
        }
    }
    pub fn mut_aggregate_value(&mut self) -> Option<&mut String> {
        self.aggregate_value_.as_mut()
    }
    pub fn set_aggregate_value(&mut self, value: String) {
        self.aggregate_value_ = Some(value)
    }
    pub fn has_aggregate_value(&self) -> bool {
        self.aggregate_value_.is_some()
    }
    pub fn clear_aggregate_value(&mut self) {
        self.aggregate_value_ = None
    }
}

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct UninterpretedOption_NamePart {
    name_part_: Option<String>,
    is_extension_: Option<bool>,
    _unknown: codegen::UnknownFieldSet
}

static UNINTERPRETED_OPTION_NAME_PART_NAME_PART_DEFAULT: String = String::new();
static UNINTERPRETED_OPTION_NAME_PART_IS_EXTENSION_DEFAULT: bool = false;

impl codegen::LiteMessage for UninterpretedOption_NamePart {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.name_part_ = Some(input.read_string()?),
                16 => self.is_extension_ = Some(input.read_bool()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for UninterpretedOption_NamePart {
    fn new() -> Self {
        UninterpretedOption_NamePart {
            name_part_: None,
            is_extension_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for UninterpretedOption_NamePart { }

impl codegen::GeneratedMessage for UninterpretedOption_NamePart { }

impl UninterpretedOption_NamePart {
    pub fn ref_name_part(&self) -> Option<&String> {
        self.name_part_.as_ref()
    }
    pub fn ref_name_part_or_default(&self) -> &String {
        if let Some(ref name_part_) = self.name_part_ {
            name_part_
        } else {
            &UNINTERPRETED_OPTION_NAME_PART_NAME_PART_DEFAULT
        }
    }
    pub fn mut_name_part(&mut self) -> Option<&mut String> {
        self.name_part_.as_mut()
    }
    pub fn set_name_part(&mut self, value: String) {
        self.name_part_ = Some(value)
    }
    pub fn has_name_part(&self) -> bool {
        self.name_part_.is_some()
    }
    pub fn clear_name_part(&mut self) {
        self.name_part_ = None
    }

    pub fn ref_is_extension(&self) -> Option<&bool> {
        self.is_extension_.as_ref()
    }
    pub fn ref_is_extension_or_default(&self) -> &bool {
        if let Some(ref is_extension_) = self.is_extension_ {
            is_extension_
        } else {
            &UNINTERPRETED_OPTION_NAME_PART_IS_EXTENSION_DEFAULT
        }
    }
    pub fn mut_is_extension(&mut self) -> Option<&mut bool> {
        self.is_extension_.as_mut()
    }
    pub fn set_is_extension(&mut self, value: bool) {
        self.is_extension_ = Some(value)
    }
    pub fn has_is_extension(&self) -> bool {
        self.is_extension_.is_some()
    }
    pub fn clear_is_extension(&mut self) {
        self.is_extension_ = None
    }
}

#[derive(Clone, PartialEq)]
pub struct SourceCodeInfo {
    location_: Vec<SourceCodeInfo_Location>,
    _unknown: codegen::UnknownFieldSet
}

static SOURCE_CODE_INFO_LOCATION_CODEC: codegen::Codec<SourceCodeInfo_Location> = codegen::Codec::<SourceCodeInfo_Location>::message(10);

impl codegen::LiteMessage for SourceCodeInfo {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => codegen::collections::repeated::add_entries(&mut self.location_, tag.get(), input, &SOURCE_CODE_INFO_LOCATION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for SourceCodeInfo {
    fn new() -> Self {
        SourceCodeInfo {
            location_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for SourceCodeInfo { }

impl codegen::GeneratedMessage for SourceCodeInfo { }

impl SourceCodeInfo {
    pub fn ref_location(&self) -> &Vec<SourceCodeInfo_Location> {
        &self.location_
    }
    pub fn mut_location(&mut self) -> &mut Vec<SourceCodeInfo_Location> {
        &mut self.location_
    }
}

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct SourceCodeInfo_Location {
    path_: Vec<i32>,
    span_: Vec<i32>,
    leading_comments_: Option<String>,
    trailing_comments_: Option<String>,
    leading_detached_comments_: Vec<String>,
    _unknown: codegen::UnknownFieldSet
}

static SOURCE_CODE_INFO_LOCATION_PATH_CODEC: codegen::Codec<i32> = codegen::Codec::<i32>::int32(0, 10);
static SOURCE_CODE_INFO_LOCATION_SPAN_CODEC: codegen::Codec<i32> = codegen::Codec::<i32>::int32(0, 18);
static SOURCE_CODE_INFO_LOCATION_LEADING_COMMENTS_DEFAULT: String = String::new();
static SOURCE_CODE_INFO_LOCATION_TRAILING_COMMENTS_DEFAULT: String = String::new();
static SOURCE_CODE_INFO_LOCATION_LEADING_DETACHED_COMMENTS_CODEC: codegen::Codec<String> = codegen::Codec::<String>::string(String::new(), 50);

impl codegen::LiteMessage for SourceCodeInfo_Location {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 |
                8 => codegen::collections::repeated::add_entries(&mut self.path_, tag.get(), input, &SOURCE_CODE_INFO_LOCATION_PATH_CODEC)?,
                18 |
                16 => codegen::collections::repeated::add_entries(&mut self.span_, tag.get(), input, &SOURCE_CODE_INFO_LOCATION_SPAN_CODEC)?,
                26 => self.leading_comments_ = Some(input.read_string()?),
                34 => self.trailing_comments_ = Some(input.read_string()?),
                50 => codegen::collections::repeated::add_entries(&mut self.leading_detached_comments_, tag.get(), input, &SOURCE_CODE_INFO_LOCATION_LEADING_DETACHED_COMMENTS_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for SourceCodeInfo_Location {
    fn new() -> Self {
        SourceCodeInfo_Location {
            path_: Vec::new(),
            span_: Vec::new(),
            leading_comments_: None,
            trailing_comments_: None,
            leading_detached_comments_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for SourceCodeInfo_Location { }

impl codegen::GeneratedMessage for SourceCodeInfo_Location { }

impl SourceCodeInfo_Location {
    pub fn ref_path(&self) -> &Vec<i32> {
        &self.path_
    }
    pub fn mut_path(&mut self) -> &mut Vec<i32> {
        &mut self.path_
    }

    pub fn ref_span(&self) -> &Vec<i32> {
        &self.span_
    }
    pub fn mut_span(&mut self) -> &mut Vec<i32> {
        &mut self.span_
    }

    pub fn ref_leading_comments(&self) -> Option<&String> {
        self.leading_comments_.as_ref()
    }
    pub fn ref_leading_comments_or_default(&self) -> &String {
        if let Some(ref leading_comments_) = self.leading_comments_ {
            leading_comments_
        } else {
            &SOURCE_CODE_INFO_LOCATION_LEADING_COMMENTS_DEFAULT
        }
    }
    pub fn mut_leading_comments(&mut self) -> Option<&mut String> {
        self.leading_comments_.as_mut()
    }
    pub fn set_leading_comments(&mut self, value: String) {
        self.leading_comments_ = Some(value)
    }
    pub fn has_leading_comments(&self) -> bool {
        self.leading_comments_.is_some()
    }
    pub fn clear_leading_comments(&mut self) {
        self.leading_comments_ = None
    }

    pub fn ref_trailing_comments(&self) -> Option<&String> {
        self.trailing_comments_.as_ref()
    }
    pub fn ref_trailing_comments_or_default(&self) -> &String {
        if let Some(ref trailing_comments_) = self.trailing_comments_ {
            trailing_comments_
        } else {
            &SOURCE_CODE_INFO_LOCATION_TRAILING_COMMENTS_DEFAULT
        }
    }
    pub fn mut_trailing_comments(&mut self) -> Option<&mut String> {
        self.trailing_comments_.as_mut()
    }
    pub fn set_trailing_comments(&mut self, value: String) {
        self.trailing_comments_ = Some(value)
    }
    pub fn has_trailing_comments(&self) -> bool {
        self.trailing_comments_.is_some()
    }
    pub fn clear_trailing_comments(&mut self) {
        self.trailing_comments_ = None
    }

    pub fn ref_leading_detached_comments(&self) -> &Vec<String> {
        &self.leading_detached_comments_
    }
    pub fn mut_leading_detached_comments(&mut self) -> &mut Vec<String> {
        &mut self.leading_detached_comments_
    }
}

#[derive(Clone, PartialEq)]
pub struct GeneratedCodeInfo {
    annotation_: Vec<GeneratedCodeInfo_Annotation>,
    _unknown: codegen::UnknownFieldSet
}

static GENERATED_CODE_INFO_ANNOTATION_CODEC: codegen::Codec<GeneratedCodeInfo_Annotation> = codegen::Codec::<GeneratedCodeInfo_Annotation>::message(10);

impl codegen::LiteMessage for GeneratedCodeInfo {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => codegen::collections::repeated::add_entries(&mut self.annotation_, tag.get(), input, &GENERATED_CODE_INFO_ANNOTATION_CODEC)?,
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for GeneratedCodeInfo {
    fn new() -> Self {
        GeneratedCodeInfo {
            annotation_: Vec::new(),
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for GeneratedCodeInfo { }

impl codegen::GeneratedMessage for GeneratedCodeInfo { }

impl GeneratedCodeInfo {
    pub fn ref_annotation(&self) -> &Vec<GeneratedCodeInfo_Annotation> {
        &self.annotation_
    }
    pub fn mut_annotation(&mut self) -> &mut Vec<GeneratedCodeInfo_Annotation> {
        &mut self.annotation_
    }
}

#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct GeneratedCodeInfo_Annotation {
    path_: Vec<i32>,
    source_file_: Option<String>,
    begin_: Option<i32>,
    end_: Option<i32>,
    _unknown: codegen::UnknownFieldSet
}

static GENERATED_CODE_INFO_ANNOTATION_PATH_CODEC: codegen::Codec<i32> = codegen::Codec::<i32>::int32(0, 10);
static GENERATED_CODE_INFO_ANNOTATION_SOURCE_FILE_DEFAULT: String = String::new();
static GENERATED_CODE_INFO_ANNOTATION_BEGIN_DEFAULT: i32 = 0;
static GENERATED_CODE_INFO_ANNOTATION_END_DEFAULT: i32 = 0;

impl codegen::LiteMessage for GeneratedCodeInfo_Annotation {
    fn merge_from(&mut self, input: &mut codegen::CodedInput) -> codegen::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                10 |
                8 => codegen::collections::repeated::add_entries(&mut self.path_, tag.get(), input, &GENERATED_CODE_INFO_ANNOTATION_PATH_CODEC)?,
                18 => self.source_file_ = Some(input.read_string()?),
                24 => self.begin_ = Some(input.read_int32()?),
                32 => self.end_ = Some(input.read_int32()?),
                _ => ()
            }
        }
        Ok(())
    }
    fn calculate_size(&self) -> Option<i32> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_to(&self, output: &mut codegen::CodedOutput) -> codegen::OutputResult {
        unimplemented!()
    }
    fn is_initialized(&self) -> bool {
        unimplemented!()
    }
}

impl codegen::GeneratedLiteMessage for GeneratedCodeInfo_Annotation {
    fn new() -> Self {
        GeneratedCodeInfo_Annotation {
            path_: Vec::new(),
            source_file_: None,
            begin_: None,
            end_: None,
            _unknown: codegen::UnknownFieldSet::new()
        }
    }
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl codegen::Message for GeneratedCodeInfo_Annotation { }

impl codegen::GeneratedMessage for GeneratedCodeInfo_Annotation { }

impl GeneratedCodeInfo_Annotation {
    pub fn ref_path(&self) -> &Vec<i32> {
        &self.path_
    }
    pub fn mut_path(&mut self) -> &mut Vec<i32> {
        &mut self.path_
    }

    pub fn ref_source_file(&self) -> Option<&String> {
        self.source_file_.as_ref()
    }
    pub fn ref_source_file_or_default(&self) -> &String {
        if let Some(ref source_file_) = self.source_file_ {
            source_file_
        } else {
            &GENERATED_CODE_INFO_ANNOTATION_SOURCE_FILE_DEFAULT
        }
    }
    pub fn mut_source_file(&mut self) -> Option<&mut String> {
        self.source_file_.as_mut()
    }
    pub fn set_source_file(&mut self, value: String) {
        self.source_file_ = Some(value)
    }
    pub fn has_source_file(&self) -> bool {
        self.source_file_.is_some()
    }
    pub fn clear_source_file(&mut self) {
        self.source_file_ = None
    }

    pub fn ref_begin(&self) -> Option<&i32> {
        self.begin_.as_ref()
    }
    pub fn ref_begin_or_default(&self) -> &i32 {
        if let Some(ref begin_) = self.begin_ {
            begin_
        } else {
            &GENERATED_CODE_INFO_ANNOTATION_BEGIN_DEFAULT
        }
    }
    pub fn mut_begin(&mut self) -> Option<&mut i32> {
        self.begin_.as_mut()
    }
    pub fn set_begin(&mut self, value: i32) {
        self.begin_ = Some(value)
    }
    pub fn has_begin(&self) -> bool {
        self.begin_.is_some()
    }
    pub fn clear_begin(&mut self) {
        self.begin_ = None
    }

    pub fn ref_end(&self) -> Option<&i32> {
        self.end_.as_ref()
    }
    pub fn ref_end_or_default(&self) -> &i32 {
        if let Some(ref end_) = self.end_ {
            end_
        } else {
            &GENERATED_CODE_INFO_ANNOTATION_END_DEFAULT
        }
    }
    pub fn mut_end(&mut self) -> Option<&mut i32> {
        self.end_.as_mut()
    }
    pub fn set_end(&mut self, value: i32) {
        self.end_ = Some(value)
    }
    pub fn has_end(&self) -> bool {
        self.end_.is_some()
    }
    pub fn clear_end(&mut self) {
        self.end_ = None
    }
}