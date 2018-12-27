pub use crate::descriptor::FieldDescriptorProto_Label as FieldLabel;

use crate::descriptor::*;
use crate::io::WireType;
use std::collections::HashMap;
use std::convert::AsRef;
use std::mem::zeroed; // zeroed, not uninitialized, since it makes it easier for us to assign values
use std::ops::Deref;

macro_rules! validate {
    ($e:expr, $n:expr) => {
        $e.expect("invalid descriptor: missing field '$n'")
    };
}

enum Symbol {
    File(*mut FileDescriptor),
    Message(*mut MessageDescriptor),
    Field(*mut FieldDescriptor),
    Oneof(*mut OneofDescriptor),
    Enum(*mut EnumDescriptor),
    EnumValue(*mut EnumValueDescriptor),
    Service(*mut ServiceDescriptor),
    Method(*mut MethodDescriptor),
}

/// Represents an immutable reference to a descriptor value
#[derive(PartialEq, Eq)]
pub struct Ref<T>(*mut T);

impl<T> Ref<T> {
    fn new(val: *mut T) -> Ref<T> {
        Ref(val)
    }

    fn clone(this: &Self) -> Ref<T> {
        Ref(this.0)
    }

    unsafe fn get_mut(this: &mut Self) -> &mut T {
        &mut *this.0
    }
}

unsafe impl<T> Sync for Ref<T> {}

unsafe impl<T> Send for Ref<T> {}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

/// Creates a boxed value and turns it into a raw pointer
fn raw_box<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

pub struct DescriptorPool<'a> {
    protos: &'a [Box<FileDescriptorProto>],
    symbols: HashMap<String, Symbol>,
}

impl DescriptorPool<'_> {
    /// Builds a descriptor pool from the slice of file descriptors
    pub fn build_from_files(files: &[Box<FileDescriptorProto>]) -> DescriptorPool {
        let mut pool = DescriptorPool {
            protos: files,
            symbols: HashMap::new(),
        };

        // insert the symbol for each file
        for file in pool.protos.iter() {
            let file = FileDescriptor::new(&**file as *const FileDescriptorProto, &mut pool);
            unsafe {
                (*file).cross_ref(&mut pool);
            }
        }

        pool
    }

    pub fn find_file_by_name(&self, name: &str) -> Option<&FileDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::File(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_message_by_name(&self, name: &str) -> Option<&MessageDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::Message(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_field_by_name(&self, name: &str) -> Option<&FieldDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::Field(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_oneof_by_name(&self, name: &str) -> Option<&OneofDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::Oneof(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&EnumDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::Enum(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_enum_value_by_name(&self, name: &str) -> Option<&EnumValueDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::EnumValue(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_service_by_name(&self, name: &str) -> Option<&ServiceDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::Service(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_method_by_name(&self, name: &str) -> Option<&MethodDescriptor> {
        match self.symbols.get(name) {
            Some(Symbol::Method(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    fn get_file_ref(&mut self, name: &str) -> Ref<FileDescriptor> {
        match self.symbols.get_mut(name) {
            Some(Symbol::File(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }

    fn get_message_ref(&mut self, name: &str) -> Ref<MessageDescriptor> {
        match self.symbols.get_mut(name) {
            Some(Symbol::Message(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }

    fn get_enum_ref(&mut self, name: &str) -> Ref<EnumDescriptor> {
        match self.symbols.get_mut(name) {
            Some(Symbol::Enum(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }

    fn get_enum_value_ref(&mut self, name: &str) -> Ref<EnumValueDescriptor> {
        match self.symbols.get_mut(name) {
            Some(Symbol::EnumValue(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }
}

impl Drop for DescriptorPool<'_> {
    fn drop(&mut self) {
        for (_, value) in self.symbols.drain() {
            unsafe {
                match value {
                    Symbol::File(x) => drop(Box::from_raw(x)),
                    Symbol::Message(x) => drop(Box::from_raw(x)),
                    Symbol::Field(x) => drop(Box::from_raw(x)),
                    Symbol::Oneof(x) => drop(Box::from_raw(x)),
                    Symbol::Enum(x) => drop(Box::from_raw(x)),
                    Symbol::EnumValue(x) => drop(Box::from_raw(x)),
                    Symbol::Service(x) => drop(Box::from_raw(x)),
                    Symbol::Method(x) => drop(Box::from_raw(x)),
                }
            }
        }
    }
}

unsafe impl Send for DescriptorPool<'_> {}

unsafe impl Sync for DescriptorPool<'_> {}

pub trait Descriptor {
    fn name(&self) -> &String;
    fn full_name(&self) -> &String;
    fn file(&self) -> &FileDescriptor;
}

/// Specifies the syntax of a proto file
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Syntax {
    Proto2,
    Proto3,
    Unknown,
}

impl std::fmt::Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Syntax::Proto2 => write!(f, "proto2"),
            Syntax::Proto3 => write!(f, "proto3"),
            Syntax::Unknown => write!(f, "unknown"),
        }
    }
}

fn get_full_type_name(name: &String, scope: &CompositeScope) -> String {
    let mut name = name.clone();
    name.insert(0, '.');
    match scope {
        CompositeScope::Message(m) => name.insert_str(0, m.full_name()),
        CompositeScope::File(f) => {
            name.insert_str(0, f.package());
            name.insert(0, '.');
        }
    }
    name
}

/// Describes a complete .proto file
pub struct FileDescriptor {
    proto: *const FileDescriptorProto,
    dependencies: Box<[Ref<FileDescriptor>]>,
    public_dependencies: Box<[Ref<FileDescriptor>]>,
    messages: Box<[Ref<MessageDescriptor>]>,
    enums: Box<[Ref<EnumDescriptor>]>,
    services: Box<[Ref<ServiceDescriptor>]>,
    extensions: Box<[Ref<FieldDescriptor>]>,
    syntax: Syntax,
}

impl FileDescriptor {
    /// Gets the underlying FileDescriptorProto that created this descriptor
    pub fn proto(&self) -> &FileDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn dependencies(&self) -> &[Ref<FileDescriptor>] {
        &self.dependencies
    }

    pub fn public_dependencies(&self) -> &[Ref<FileDescriptor>] {
        &self.public_dependencies
    }

    /// Gets the name of this file
    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn package(&self) -> &String {
        validate!(self.proto().package.as_ref(), "package")
    }

    /// Gets the messages defined in this file
    pub fn messages(&self) -> &[Ref<MessageDescriptor>] {
        &self.messages
    }

    pub fn enums(&self) -> &[Ref<EnumDescriptor>] {
        &self.enums
    }

    pub fn services(&self) -> &[Ref<ServiceDescriptor>] {
        &self.services
    }

    pub fn extensions(&self) -> &[Ref<FieldDescriptor>] {
        &self.extensions
    }

    pub fn options(&self) -> Option<&FileOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    pub fn syntax(&self) -> Syntax {
        self.syntax
    }

    fn new(proto: *const FileDescriptorProto, pool: &mut DescriptorPool) -> *mut FileDescriptor {
        let descriptor_raw: *mut FileDescriptor;
        let mut descriptor: &mut FileDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw; // make an uninitialized descriptor, box it, return a raw reference
        }

        descriptor.proto = proto;
        descriptor.dependencies = descriptor
            .proto()
            .dependency
            .iter()
            .map(|f| pool.get_file_ref(f))
            .collect::<Vec<_>>()
            .into();

        descriptor.public_dependencies = descriptor
            .proto()
            .public_dependency
            .iter()
            .map(|f| Ref::clone(&descriptor.dependencies[*f as usize]))
            .collect::<Vec<_>>()
            .into();

        descriptor.messages = descriptor
            .proto()
            .message_type
            .iter()
            .map(|m| {
                MessageDescriptor::new(
                    &**m as *const DescriptorProto,
                    CompositeScope::File(Ref::new(descriptor_raw)),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.enums = descriptor
            .proto()
            .enum_type
            .iter()
            .map(|e| {
                EnumDescriptor::new(
                    &**e as *const EnumDescriptorProto,
                    CompositeScope::File(Ref::new(descriptor_raw)),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.services = descriptor
            .proto()
            .service
            .iter()
            .map(|s| {
                ServiceDescriptor::new(
                    &**s as *const ServiceDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.extensions = descriptor
            .proto()
            .extension
            .iter()
            .map(|e| {
                FieldDescriptor::new(
                    &**e as *const FieldDescriptorProto,
                    FieldScope::File(Ref::new(descriptor_raw)),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.syntax = match descriptor.proto().syntax.as_ref().map(AsRef::as_ref) {
            Some("proto3") => Syntax::Proto3,
            Some("proto2") | None => Syntax::Proto2,
            Some(_) => Syntax::Unknown,
        };

        if let Some(_) = pool
            .symbols
            .insert(descriptor.name().clone(), Symbol::File(descriptor_raw))
        {
            panic!()
        }

        descriptor_raw
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        for mut message in self.messages.iter_mut() {
            unsafe { Ref::get_mut(&mut message).cross_ref(pool) }
        }

        for mut service in self.services.iter_mut() {
            unsafe { Ref::get_mut(&mut service).cross_ref(pool) }
        }

        for mut extension in self.extensions.iter_mut() {
            unsafe { Ref::get_mut(&mut extension).cross_ref(pool) }
        }
    }
}

impl PartialEq for FileDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for FileDescriptor {}

unsafe impl Send for FileDescriptor {}

unsafe impl Sync for FileDescriptor {}

impl Descriptor for FileDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.name()
    }
    fn file(&self) -> &FileDescriptor {
        self
    }
}

/// Represents the scope of a composite type (message or enum type)
pub enum CompositeScope {
    /// A file scope
    File(Ref<FileDescriptor>),
    /// A message scope
    Message(Ref<MessageDescriptor>),
}

impl CompositeScope {
    pub fn file(&self) -> &FileDescriptor {
        let mut scope = self;
        while let CompositeScope::Message(m) = scope {
            scope = m.scope();
        }

        match scope {
            CompositeScope::File(f) => f,
            CompositeScope::Message(_) => unreachable!(),
        }
    }
}

/// A message descriptor
pub struct MessageDescriptor {
    proto: *const DescriptorProto,
    scope: CompositeScope,
    full_name: String,
    fields: Box<[Ref<FieldDescriptor>]>,
    fields_ordered: Box<[Ref<FieldDescriptor>]>,
    extensions: Box<[Ref<FieldDescriptor>]>,
    messages: Box<[Ref<MessageDescriptor>]>,
    enums: Box<[Ref<EnumDescriptor>]>,
    oneofs: Box<[Ref<OneofDescriptor>]>,
}

impl MessageDescriptor {
    pub fn proto(&self) -> &DescriptorProto {
        unsafe { &*self.proto }
    }

    /// Gets the file this descriptor belongs to
    pub fn scope(&self) -> &CompositeScope {
        &self.scope
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn fields(&self) -> &[Ref<FieldDescriptor>] {
        &self.fields
    }

    pub fn extensions(&self) -> &[Ref<FieldDescriptor>] {
        &self.extensions
    }

    pub fn messages(&self) -> &[Ref<MessageDescriptor>] {
        &self.messages
    }

    pub fn enums(&self) -> &[Ref<EnumDescriptor>] {
        &self.enums
    }

    pub fn oneofs(&self) -> &[Ref<OneofDescriptor>] {
        &self.oneofs
    }

    pub fn options(&self) -> Option<&MessageOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    /// Creates a new string with the full name of this descriptor
    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn map_entry(&self) -> bool {
        if let Some(options) = self.options() {
            options.map_entry.unwrap_or(false)
        } else {
            false
        }
    }

    fn new(
        proto: *const DescriptorProto,
        scope: CompositeScope,
        pool: &mut DescriptorPool,
    ) -> Ref<MessageDescriptor> {
        let descriptor_raw: *mut MessageDescriptor;
        let descriptor: &mut MessageDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.scope = scope;

        descriptor.full_name = get_full_type_name(descriptor.name(), descriptor.scope());

        descriptor.messages = descriptor
            .proto()
            .nested_type
            .iter()
            .map(|m| {
                MessageDescriptor::new(
                    &**m as *const DescriptorProto,
                    CompositeScope::Message(Ref::new(descriptor_raw)),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.enums = descriptor
            .proto()
            .enum_type
            .iter()
            .map(|e| {
                EnumDescriptor::new(
                    &**e as *const EnumDescriptorProto,
                    CompositeScope::Message(Ref::new(descriptor_raw)),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.extensions = descriptor
            .proto()
            .extension
            .iter()
            .map(|e| {
                FieldDescriptor::new(
                    &**e as *const FieldDescriptorProto,
                    FieldScope::Message(Ref::new(descriptor_raw)),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.oneofs = descriptor
            .proto()
            .oneof_decl // oneofs before fields since we reference them when determining field scopes
            .iter()
            .map(|o| {
                OneofDescriptor::new(
                    &**o as *const OneofDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.fields = descriptor
            .proto()
            .field
            .iter()
            .map(|f| {
                FieldDescriptor::new(
                    &**f as *const FieldDescriptorProto,
                    if let Some(o) = f.oneof_index {
                        FieldScope::Oneof(Ref::clone(&descriptor.oneofs[o as usize]))
                    } else {
                        FieldScope::Message(Ref::new(descriptor_raw))
                    },
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        let mut number_order = descriptor
            .fields()
            .iter()
            .map(Ref::clone)
            .collect::<Vec<_>>();
        number_order.sort_by_key(|r| FieldDescriptor::number(r));
        descriptor.fields_ordered = number_order.into();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().clone(),
            Symbol::Message(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        for mut message in self.messages.iter_mut() {
            unsafe {
                Ref::get_mut(&mut message).cross_ref(pool);
            }
        }

        for mut field in self.fields.iter_mut() {
            unsafe {
                Ref::get_mut(&mut field).cross_ref(pool);
            }
        }

        for mut oneof in self.oneofs.iter_mut() {
            unsafe {
                Ref::get_mut(&mut oneof).cross_ref();
            }
        }

        for mut extension in self.extensions.iter_mut() {
            unsafe { Ref::get_mut(&mut extension).cross_ref(pool) }
        }
    }
}

impl PartialEq for MessageDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for MessageDescriptor {}

unsafe impl Send for MessageDescriptor {}

unsafe impl Sync for MessageDescriptor {}

impl Descriptor for MessageDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        let mut scope = self.scope();
        while let CompositeScope::Message(m) = scope {
            scope = m.scope();
        }

        match scope {
            CompositeScope::File(f) => f,
            _ => unreachable!(),
        }
    }
}

pub struct EnumDescriptor {
    proto: *const EnumDescriptorProto,
    scope: CompositeScope,
    full_name: String,
    values: Box<[Ref<EnumValueDescriptor>]>,
}

impl EnumDescriptor {
    pub fn proto(&self) -> &EnumDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn scope(&self) -> &CompositeScope {
        &self.scope
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn values(&self) -> &[Ref<EnumValueDescriptor>] {
        &self.values
    }

    pub fn options(&self) -> Option<&EnumOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    fn new(
        proto: *const EnumDescriptorProto,
        scope: CompositeScope,
        pool: &mut DescriptorPool,
    ) -> Ref<EnumDescriptor> {
        let descriptor_raw: *mut EnumDescriptor;
        let descriptor: &mut EnumDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.scope = scope;
        descriptor.full_name = get_full_type_name(descriptor.name(), descriptor.scope());

        descriptor.values = descriptor
            .proto()
            .value
            .iter()
            .map(|v| {
                EnumValueDescriptor::new(
                    &**v as *const EnumValueDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        if let Some(_) = pool
            .symbols
            .insert(descriptor.full_name().clone(), Symbol::Enum(descriptor_raw))
        {
            panic!()
        }

        Ref::new(descriptor_raw)
    }
}

impl PartialEq for EnumDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for EnumDescriptor {}

unsafe impl Send for EnumDescriptor {}

unsafe impl Sync for EnumDescriptor {}

impl Descriptor for EnumDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.scope().file()
    }
}

pub struct EnumValueDescriptor {
    proto: *const EnumValueDescriptorProto,
    enum_type: Ref<EnumDescriptor>,
    full_name: String,
}

impl EnumValueDescriptor {
    pub fn proto(&self) -> &EnumValueDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn enum_type(&self) -> &EnumDescriptor {
        &self.enum_type
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn number(&self) -> i32 {
        validate!(self.proto().number, "number")
    }

    pub fn options(&self) -> Option<&EnumValueOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    fn new(
        proto: *const EnumValueDescriptorProto,
        parent: Ref<EnumDescriptor>,
        pool: &mut DescriptorPool,
    ) -> Ref<EnumValueDescriptor> {
        let descriptor_raw: *mut EnumValueDescriptor;
        let descriptor: &mut EnumValueDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.enum_type = parent;
        descriptor.full_name = format!(
            "{}.{}",
            descriptor.enum_type().full_name().clone(),
            descriptor.name()
        );

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().clone(),
            Symbol::EnumValue(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }
}

impl PartialEq for EnumValueDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for EnumValueDescriptor {}

unsafe impl Send for EnumValueDescriptor {}

unsafe impl Sync for EnumValueDescriptor {}

impl Descriptor for EnumValueDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.enum_type().file()
    }
}

pub struct ServiceDescriptor {
    proto: *const ServiceDescriptorProto,
    full_name: String,
    file: Ref<FileDescriptor>,
    methods: Box<[Ref<MethodDescriptor>]>,
}

impl ServiceDescriptor {
    pub fn proto(&self) -> &ServiceDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn file(&self) -> &FileDescriptor {
        &self.file
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn methods(&self) -> &[Ref<MethodDescriptor>] {
        &self.methods
    }

    pub fn options(&self) -> Option<&ServiceOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    fn new(
        proto: *const ServiceDescriptorProto,
        file: Ref<FileDescriptor>,
        pool: &mut DescriptorPool,
    ) -> Ref<ServiceDescriptor> {
        let descriptor_raw: *mut ServiceDescriptor;
        let descriptor: &mut ServiceDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.file = file;
        descriptor.full_name = format!(".{}.{}", descriptor.file().name(), descriptor.name());

        descriptor.methods = descriptor
            .proto()
            .method
            .iter()
            .map(|m| {
                MethodDescriptor::new(
                    &**m as *const MethodDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                )
            })
            .collect::<Vec<_>>()
            .into();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().clone(),
            Symbol::Service(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        for mut method in self.methods.iter_mut() {
            unsafe {
                Ref::get_mut(&mut method).cross_ref(pool);
            }
        }
    }
}

impl PartialEq for ServiceDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for ServiceDescriptor {}

unsafe impl Send for ServiceDescriptor {}

unsafe impl Sync for ServiceDescriptor {}

impl Descriptor for ServiceDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.file()
    }
}

pub struct MethodDescriptor {
    proto: *const MethodDescriptorProto,
    full_name: String,
    service: Ref<ServiceDescriptor>,
    input_type: Ref<MessageDescriptor>,
    output_type: Ref<MessageDescriptor>,
}

impl MethodDescriptor {
    pub fn proto(&self) -> &MethodDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn service(&self) -> &ServiceDescriptor {
        &self.service
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn input_type(&self) -> &MessageDescriptor {
        &self.input_type
    }

    pub fn output_type(&self) -> &MessageDescriptor {
        &self.output_type
    }

    pub fn client_streaming(&self) -> bool {
        validate!(self.proto().client_streaming, "client_streaming")
    }

    pub fn server_streaming(&self) -> bool {
        validate!(self.proto().server_streaming, "server_streaming")
    }

    pub fn options(&self) -> Option<&MethodOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    fn new(
        proto: *const MethodDescriptorProto,
        service: Ref<ServiceDescriptor>,
        pool: &mut DescriptorPool,
    ) -> Ref<MethodDescriptor> {
        let descriptor_raw: *mut MethodDescriptor;
        let descriptor: &mut MethodDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.service = service;
        descriptor.full_name = format!(
            ".{}.{}",
            descriptor.service().full_name(),
            descriptor.name()
        );

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().clone(),
            Symbol::Method(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        self.input_type = pool.get_message_ref(self.proto().input_type.as_ref().unwrap());
        self.output_type = pool.get_message_ref(self.proto().output_type.as_ref().unwrap());
    }
}

impl PartialEq for MethodDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for MethodDescriptor {}

unsafe impl Send for MethodDescriptor {}

unsafe impl Sync for MethodDescriptor {}

impl Descriptor for MethodDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.service().file()
    }
}

pub enum FieldType {
    Double,
    Float,
    Int64,
    Uint64,
    Sint64,
    Fixed64,
    Sfixed64,
    Int32,
    Uint32,
    Sint32,
    Fixed32,
    Sfixed32,
    Bool,
    String,
    Bytes,
    Enum(Ref<EnumDescriptor>),
    Message(Ref<MessageDescriptor>),
    Group(Ref<MessageDescriptor>),
}

pub enum DefaultValue {
    /// There was no specified default value
    None,
    /// The default value was invalid
    Invalid,
    Bool(bool),
    Double(f64),
    SignedInt(i64),
    UnsignedInt(u64),
    Enum(Ref<EnumValueDescriptor>),
    String(String),
    Bytes(Vec<u8>),
}

pub struct FieldDescriptor {
    proto: *const FieldDescriptorProto,
    full_name: String,
    scope: FieldScope,
    value_type: FieldType,
    default: DefaultValue,
    message: Ref<MessageDescriptor>,
}

impl FieldDescriptor {
    pub fn proto(&self) -> &FieldDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn number(&self) -> i32 {
        validate!(self.proto().number, "number")
    }

    pub fn label(&self) -> FieldLabel {
        validate!(self.proto().label, "label").unwrap()
    }

    pub fn field_type(&self) -> &FieldType {
        &self.value_type
    }

    pub fn default_value(&self) -> &DefaultValue {
        &self.default
    }

    pub fn json_name(&self) -> &String {
        validate!(self.proto().json_name.as_ref(), "json_name")
    }

    pub fn scope(&self) -> &FieldScope {
        &self.scope
    }

    pub fn options(&self) -> Option<&FieldOptions> {
        self.proto().options.as_ref().map(AsRef::as_ref)
    }

    pub fn packed(&self) -> bool {
        if self.label() == FieldLabel::Repeated && self.wire_type().is_packable() {
            if let Some(options) = self.options() {
                if let Some(option) = options.packed {
                    return option;
                }
            }

            self.file().syntax() == Syntax::Proto3
        } else {
            false
        }
    }

    pub fn wire_type(&self) -> WireType {
        match self.field_type() {
            FieldType::Message(_) | FieldType::String | FieldType::Bytes => {
                WireType::LengthDelimited
            }
            FieldType::Group(_) => WireType::StartGroup,
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => WireType::Bit32,
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => WireType::Bit64,
            _ => WireType::Varint,
        }
    }

    /// Gets the message this field applies to.
    ///
    /// For normal fields, this is the message this field is defined in.
    ///
    /// For extension fields, this is the extended message.
    pub fn message(&self) -> &MessageDescriptor {
        &self.message
    }

    fn new(
        proto: *const FieldDescriptorProto,
        scope: FieldScope,
        pool: &mut DescriptorPool,
    ) -> Ref<FieldDescriptor> {
        let descriptor_raw: *mut FieldDescriptor;
        let descriptor: &mut FieldDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.scope = scope;
        descriptor.full_name = match &descriptor.scope {
            FieldScope::File(f) => format!(".{}.{}", f.package(), descriptor.name()),
            FieldScope::Message(m) => format!("{}.{}", m.full_name(), descriptor.name()),
            FieldScope::Oneof(o) => format!("{}.{}", o.message().full_name(), descriptor.name()),
        };

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().clone(),
            Symbol::Field(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        use crate::descriptor::FieldDescriptorProto_Type::*;
        self.value_type = match self.proto().r#type.unwrap().unwrap() {
            Message => {
                FieldType::Message(pool.get_message_ref(self.proto().type_name.as_ref().unwrap()))
            }
            Enum => FieldType::Enum(pool.get_enum_ref(self.proto().type_name.as_ref().unwrap())),
            Group => {
                FieldType::Group(pool.get_message_ref(self.proto().type_name.as_ref().unwrap()))
            }
            Double => FieldType::Double,
            Float => FieldType::Float,
            Int64 => FieldType::Int64,
            Uint64 => FieldType::Uint64,
            Int32 => FieldType::Int32,
            Fixed64 => FieldType::Fixed64,
            Fixed32 => FieldType::Fixed32,
            Bool => FieldType::Bool,
            String => FieldType::String,
            Bytes => FieldType::Bytes,
            Uint32 => FieldType::Uint32,
            Sfixed32 => FieldType::Sfixed32,
            Sfixed64 => FieldType::Sfixed64,
            Sint32 => FieldType::Sint32,
            Sint64 => FieldType::Sint64,
        };

        if let Some(default) = self.proto().default_value.as_ref() {
            self.default = match self.field_type() {
                FieldType::Enum(e) => DefaultValue::Enum(
                    pool.get_enum_value_ref(&(e.full_name().clone() + "." + default)),
                ),
                FieldType::Double | FieldType::Float => match default.parse() {
                    Ok(ok) => DefaultValue::Double(ok),
                    Err(_) => DefaultValue::Invalid,
                },
                FieldType::Int32
                | FieldType::Int64
                | FieldType::Sfixed32
                | FieldType::Sfixed64
                | FieldType::Sint32
                | FieldType::Sint64 => match default.parse() {
                    Ok(ok) => DefaultValue::SignedInt(ok),
                    Err(_) => DefaultValue::Invalid,
                },
                FieldType::Uint32 | FieldType::Uint64 | FieldType::Fixed32 | FieldType::Fixed64 => {
                    match default.parse() {
                        Ok(ok) => DefaultValue::UnsignedInt(ok),
                        Err(_) => DefaultValue::Invalid,
                    }
                }
                FieldType::Bool => match default.parse() {
                    Ok(ok) => DefaultValue::Double(ok),
                    Err(_) => DefaultValue::Invalid,
                },
                FieldType::String => DefaultValue::String(default.clone()),
                FieldType::Bytes => {
                    fn esc_lit(lit: &str) -> u8 {
                        match &lit[0..2] {
                            "\\n" => b'\n',
                            "\\r" => b'\r',
                            "\\t" => b'\t',
                            "\\\"" => b'\"',
                            "\\\'" => b'\'',
                            "\\\\" => b'\\',
                            _ => {
                                let mut chars = lit.chars();
                                assert!(chars.next() == Some('\\'));
                                (chars.next().expect("missing octal digit") as u8 - b'0') * 64
                                    + (chars.next().expect("missing octal digit") as u8 - b'0') * 8
                                    + (chars.next().expect("missing octal digit") as u8 - b'0')
                            }
                        }
                    }

                    let mut result = Vec::with_capacity(default.len());
                    for (i, c) in default.char_indices() {
                        match c {
                            '\\' => result.push(esc_lit(&default[i..])),
                            _ => result.push(c as u8),
                        }
                    }

                    DefaultValue::Bytes(result)
                }
                _ => DefaultValue::Invalid,
            };
        } else {
            self.default = DefaultValue::None;
        }

        if let Some(extendee) = &self.proto().extendee {
            self.message = pool.get_message_ref(extendee);
        } else {
            self.message = match &self.scope {
                FieldScope::Message(m) => Ref::clone(m),
                FieldScope::Oneof(o) => Ref::clone(&o.message),
                _ => panic!(),
            }
        }
    }
}

impl PartialEq for FieldDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for FieldDescriptor {}

unsafe impl Send for FieldDescriptor {}

unsafe impl Sync for FieldDescriptor {}

impl Descriptor for FieldDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        match self.scope() {
            FieldScope::File(f) => f,
            FieldScope::Message(m) => m.file(),
            FieldScope::Oneof(o) => o.message().file(),
        }
    }
}

/// Gets the scope a field is defined in
#[derive(PartialEq, Eq)]
pub enum FieldScope {
    File(Ref<FileDescriptor>),
    Message(Ref<MessageDescriptor>),
    Oneof(Ref<OneofDescriptor>),
}

impl FieldScope {
    /// Gets the message that this field is in or none if this scope is an extension field defined in a file
    pub fn message(&self) -> Option<&MessageDescriptor> {
        match self {
            FieldScope::File(_) => None,
            FieldScope::Message(m) => Some(m),
            FieldScope::Oneof(o) => Some(o.message()),
        }
    }
}

pub struct OneofDescriptor {
    proto: *const OneofDescriptorProto,
    full_name: String,
    message: Ref<MessageDescriptor>,
    fields: Box<[Ref<FieldDescriptor>]>,
}

impl OneofDescriptor {
    pub fn proto(&self) -> &OneofDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn message(&self) -> &MessageDescriptor {
        &self.message
    }

    pub fn name(&self) -> &String {
        validate!(self.proto().name.as_ref(), "name")
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn fields(&self) -> &[Ref<FieldDescriptor>] {
        &self.fields
    }

    fn new(
        proto: *const OneofDescriptorProto,
        message: Ref<MessageDescriptor>,
        pool: &mut DescriptorPool,
    ) -> Ref<OneofDescriptor> {
        let descriptor_raw: *mut OneofDescriptor;
        let descriptor: &mut OneofDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.message = message;
        descriptor.full_name = descriptor.message.full_name().clone() + "." + descriptor.name();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().clone(),
            Symbol::Oneof(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self) {
        self.fields = self
            .message()
            .fields()
            .iter()
            .filter(|s| {
                if let FieldScope::Oneof(o) = s.scope() {
                    o.full_name() == self.full_name()
                } else {
                    false
                }
            })
            .map(Ref::clone)
            .collect::<Vec<_>>()
            .into()
    }
}

impl PartialEq for OneofDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.proto == other.proto
    }
}

impl Eq for OneofDescriptor {}

unsafe impl Send for OneofDescriptor {}

unsafe impl Sync for OneofDescriptor {}

impl Descriptor for OneofDescriptor {
    fn name(&self) -> &String {
        self.name()
    }
    fn full_name(&self) -> &String {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.message().file()
    }
}
