//! Provides reflection and dynamic message access to protobuf messages

pub use crate::descriptor::FieldDescriptorProto_Label as FieldLabel;

use crate::descriptor::{
    DescriptorProto, 
    EnumDescriptorProto, 
    EnumOptions, 
    EnumValueDescriptorProto, 
    EnumValueOptions, 
    FieldDescriptorProto, 
    FieldOptions, 
    FileDescriptorProto, 
    FileOptions, 
    MessageOptions, 
    MethodDescriptorProto, 
    MethodOptions, 
    OneofDescriptorProto, 
    ServiceDescriptorProto, 
    ServiceOptions
};
use crate::io::{FieldNumber, WireType};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::mem::zeroed; // zeroed, not uninitialized, since it makes it easier for us to assign values
use std::ops::Deref;

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

/// Represents an immutable reference to a descriptor value. 
/// This structure will always be behind another lifetime, such as a borrowed slice or iterator, and can't be owned.
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

impl<T: Debug> Debug for Ref<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.deref().fmt(fmt)
    }
}

/// Creates a boxed value and turns it into a raw pointer
fn raw_box<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

/// A pool of Descriptor symbols aggregated in via a slice of `FileDescriptorProto`s or a slice of borrowed pools
/// 
/// Unlike Google's C++ implementation of Protocol Buffers, this pool is immutable once created. It is not possible
/// to add, remove, or modify any descriptors once they have been added.
/// 
/// # Examples
/// 
/// ## Building a pool from a selection of files
/// 
/// ```
/// use protrust::reflect::DescriptorPool;
/// 
/// let files = [
///     protrust::descriptor::file().proto().clone(),
///     protrust::plugin::file().proto().clone()
/// ];
/// 
/// let pool = DescriptorPool::build_from_files(&files);
/// ```
/// 
/// ## Using a pool from generated code
/// 
/// ```
/// use protrust::{CodedMessage, LiteMessage, Message};
/// use protrust::descriptor::FileDescriptorProto;
/// use protrust::reflect::AnyMessage;
/// 
/// let file_descriptor = &protrust::descriptor::file().messages()[1];
/// assert!(file_descriptor.full_name() == ".google.protobuf.FileDescriptorProto");
/// 
/// let mut instance = file_descriptor.new_message().unwrap();
/// assert!(instance.calculate_size() == 0);
/// 
/// let other = protrust::descriptor::file().proto();
/// let file_instance = &mut *instance.downcast_mut::<FileDescriptorProto>().expect("Could not unwrap FileDescriptorProto");
/// file_instance.merge(other);
/// 
/// assert_eq!(file_instance, other);
/// ```
/// 
/// ## Creating a pool from multiple existing pools
/// 
/// ```
/// use protrust::reflect::DescriptorPool;
/// 
/// let pools = [protrust::wkt::any::pool(), protrust::wkt::timestamp::pool()];
/// let pool = DescriptorPool::build_from_pools(&pools);
/// 
/// assert!(pool.find_message_by_name(".google.protobuf.Any").is_some());
/// assert!(pool.find_message_by_name(".google.protobuf.Timestamp").is_some());
/// ```
pub struct DescriptorPool<'a> {
    pools: &'a [&'a DescriptorPool<'a>],
    protos: &'a [FileDescriptorProto],
    symbols: HashMap<String, Symbol>,
}

static EMPTY_POOLS: &'static [&'static DescriptorPool<'static>] = &[];
static EMPTY_FILES: &'static [FileDescriptorProto] = &[];

impl DescriptorPool<'_> {
    /// Builds a descriptor pool from the slice of file descriptors
    pub fn build_from_files(files: &[FileDescriptorProto]) -> DescriptorPool {
        let mut pool = DescriptorPool {
            pools: EMPTY_POOLS,
            protos: files,
            symbols: HashMap::new(),
        };
        pool.build(None);
        pool
    }

    pub fn build_generated_pool(
        file: &'static [FileDescriptorProto; 1],
        pools: &'static [&'static DescriptorPool],
        info: GeneratedCodeInfo
    ) -> DescriptorPool<'static> {
        let mut pool = DescriptorPool {
            pools,
            protos: file,
            symbols: HashMap::new(),
        };
        pool.build(Some(info));
        pool
    }

    fn build(&mut self, code_info: Option<GeneratedCodeInfo>) {
        // insert the symbol for each file
        if code_info.is_some() && self.protos.len() == 1 {
            let file = FileDescriptor::new(&self.protos[0] as *const FileDescriptorProto, self);
            unsafe {
                (*file).cross_ref(self, code_info);
            }
        } else {
            for file in self.protos.iter() {
                let file = FileDescriptor::new(file as *const FileDescriptorProto, self);
                unsafe {
                    (*file).cross_ref(self, None);
                }
            }
        }
    }

    fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols
            .get(name)
            .or_else(|| self.pools.iter().find_map(|p| p.find_symbol(name)))
    }

    pub fn find_file_by_name(&self, name: &str) -> Option<&FileDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::File(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_message_by_name(&self, name: &str) -> Option<&MessageDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Message(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_field_by_name(&self, name: &str) -> Option<&FieldDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Field(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_oneof_by_name(&self, name: &str) -> Option<&OneofDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Oneof(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&EnumDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Enum(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_enum_value_by_name(&self, name: &str) -> Option<&EnumValueDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::EnumValue(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_service_by_name(&self, name: &str) -> Option<&ServiceDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Service(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    pub fn find_method_by_name(&self, name: &str) -> Option<&MethodDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Method(symbol)) => unsafe { Some(&**symbol) },
            _ => None,
        }
    }

    fn get_file_ref(&self, name: &str) -> Ref<FileDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::File(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }

    fn get_message_ref(&self, name: &str) -> Ref<MessageDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Message(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }

    fn get_enum_ref(&self, name: &str) -> Ref<EnumDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::Enum(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }

    fn get_enum_value_ref(&self, name: &str) -> Ref<EnumValueDescriptor> {
        match self.find_symbol(name) {
            Some(Symbol::EnumValue(symbol)) => Ref::new(*symbol),
            _ => panic!("Pool did not contain referenced symbol"),
        }
    }
}

impl<'a> DescriptorPool<'a> {
    pub fn build_from_pools(pools: &'a [&'a DescriptorPool<'a>]) -> DescriptorPool<'a> {
        DescriptorPool {
            pools,
            protos: EMPTY_FILES,
            symbols: HashMap::new(),
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

/// A trait containing all the shared items of a descriptor
pub trait Descriptor {
    /// Gets the name of this descriptor
    fn name(&self) -> &str;
    /// Gets the full name of this descriptor
    fn full_name(&self) -> &str;
    /// Gets the file that defined this descriptor
    fn file(&self) -> &FileDescriptor;
}

/// A structure containing the comments for a particular file's message, field, oneof, service, or method definition
pub struct SourceCodeInfo {
    leading_comments: Option<*const str>,
    trailing_comments: Option<*const str>,
    leading_detached_comments: *const [String],
}

impl SourceCodeInfo {
    /// Gets the leading comments of a descriptor
    pub fn leading_comments(&self) -> Option<&str> {
        unsafe { self.leading_comments.map(|s| &*s) }
    }

    /// Gets the trailing comments of a descriptor
    pub fn trailing_comments(&self) -> Option<&str> {
        unsafe { self.trailing_comments.map(|s| &*s) }
    }

    /// Gets the leading detached comments of a descriptor
    pub fn leading_detached_comments(&self) -> &[String] {
        unsafe { &*self.leading_detached_comments }
    }
}

#[doc(hidden)]
pub struct GeneratedCodeInfo {
    pub structs: Option<Box<[GeneratedStructInfo]>>
}

#[doc(hidden)]
pub struct GeneratedStructInfo {
    pub new: fn() -> Box<dyn AnyMessage>,
    pub structs: Option<Box<[GeneratedStructInfo]>>
}

/// Specifies the syntax of a proto file
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Syntax {
    /// Proto2 syntax. See the [official Google docs](https://developers.google.com/protocol-buffers/docs/proto) for more information about this syntax
    Proto2,
    /// Proto3 syntax. See the [official Google docs](https://developers.google.com/protocol-buffers/docs/proto3) for more information about this syntax
    Proto3,
    /// Unknown syntax. This may be a new version, or the proto file may be invalid
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

fn get_full_type_name(name: &str, scope: &CompositeScope) -> String {
    let mut name = name.to_string();
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

    /// Gets the dependencies of this file
    pub fn dependencies(&self) -> &[Ref<FileDescriptor>] {
        &self.dependencies
    }

    /// Gets the dependencies in this file that were marked as `public`
    pub fn public_dependencies(&self) -> &[Ref<FileDescriptor>] {
        &self.public_dependencies
    }

    /// Gets the name of this file
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn package(&self) -> &str {
        self.proto().package()
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
        self.proto().options_option()
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
            .dependency()
            .iter()
            .map(|f| pool.get_file_ref(f))
            .collect::<Vec<_>>()
            .into();

        descriptor.public_dependencies = descriptor
            .proto()
            .public_dependency()
            .iter()
            .map(|f| Ref::clone(&descriptor.dependencies[*f as usize]))
            .collect::<Vec<_>>()
            .into();

        descriptor.messages = descriptor
            .proto()
            .message_type()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                MessageDescriptor::new(
                    m as *const DescriptorProto,
                    CompositeScope::File(Ref::new(descriptor_raw)),
                    pool,
                    i,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.enums = descriptor
            .proto()
            .enum_type()
            .iter()
            .enumerate()
            .map(|(i, e)| {
                EnumDescriptor::new(
                    e as *const EnumDescriptorProto,
                    CompositeScope::File(Ref::new(descriptor_raw)),
                    pool,
                    i,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.services = descriptor
            .proto()
            .service()
            .iter()
            .enumerate()
            .map(|(i, s)| {
                ServiceDescriptor::new(
                    s as *const ServiceDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                    i,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.extensions = descriptor
            .proto()
            .extension()
            .iter()
            .enumerate()
            .map(|(i, e)| {
                FieldDescriptor::new(
                    e as *const FieldDescriptorProto,
                    FieldScope::File(Ref::new(descriptor_raw)),
                    pool,
                    i,
                )
            })
            .collect::<Vec<_>>()
            .into();

        descriptor.syntax = 
            if !descriptor.proto().has_syntax() {
                Syntax::Proto2
            } else {
                match descriptor.proto().syntax() {
                    "proto3" => Syntax::Proto3,
                    "proto2" => Syntax::Proto2,
                    _ => Syntax::Unknown,
                }
            };

        if let Some(_) = pool
            .symbols
            .insert(descriptor.name().to_string(), Symbol::File(descriptor_raw))
        {
            panic!()
        }

        descriptor_raw
    }

    unsafe fn cross_ref(&mut self, pool: &mut DescriptorPool, code_info: Option<GeneratedCodeInfo>) {
        if let Some(code_info) = code_info {
            if let Some(structs) = code_info.structs {
                for (message, message_info) in self.messages.iter_mut().zip(structs.iter()) {
                    Ref::get_mut(message).cross_ref(pool, Some(message_info));
                }
            }
        } else {
            for message in self.messages.iter_mut() {
                Ref::get_mut(message).cross_ref(pool, None);
            }
        }

        for mut service in self.services.iter_mut() {
            Ref::get_mut(&mut service).cross_ref(pool);
        }

        for mut extension in self.extensions.iter_mut() {
            Ref::get_mut(&mut extension).cross_ref(pool);
        }

        self.parse_source_code_info();
    }

    unsafe fn parse_source_code_info(&mut self) {
        if let Some(source_code_info) = &(*self.proto).source_code_info_option() {
            for location in source_code_info.location().iter() {
                if location.path().is_empty() || location.path().len() % 2 != 0 {
                    continue;
                }

                let info = match location.path()[0] {
                    4 => Ref::get_mut(&mut self.messages[location.path()[1] as usize])
                        .get_source_code_info(&location.path()[2..]),
                    5 => Ref::get_mut(&mut self.enums[location.path()[1] as usize])
                        .get_source_code_info(&location.path()[2..]),
                    6 => Ref::get_mut(&mut self.services[location.path()[1] as usize])
                        .get_source_code_info(&location.path()[2..]),
                    7 => Ref::get_mut(&mut self.extensions[location.path()[1] as usize])
                        .get_source_code_info(&location.path()[2..]),
                    _ => continue,
                };

                if let Some(info) = info {
                    *info = Some(SourceCodeInfo {
                        leading_comments: if location.has_leading_comments() {
                            Some(location.leading_comments() as *const str)
                        } else {
                            None
                        },
                        trailing_comments: if location.has_trailing_comments() {
                            Some(location.trailing_comments() as *const str)
                        } else {
                            None
                        },
                        leading_detached_comments: location.leading_detached_comments().as_slice()
                            as *const [String],
                    });
                }
            }
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.name()
    }
    fn file(&self) -> &FileDescriptor {
        self
    }
}

impl Debug for FileDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("FileDescriptor")
            .field("name", &self.name())
            .field("package", &self.package())
            .field("syntax", &self.syntax())
            .field(
                "public_dependencies",
                &self
                    .public_dependencies()
                    .iter()
                    .map(|d| d.name())
                    .collect::<Box<[_]>>(),
            )
            .field("dependencies", &self.dependencies())
            .field("messages", &self.messages())
            .field("enums", &self.enums())
            .field("services", &self.services())
            .field("extensions", &self.extensions())
            .finish()
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

/// A message type to emulate dynamic typing.
/// This type is like Any and allows for downcasting the type to a concrete type.
/// 
/// It also has the methods of CodedMessage, allowing for reading, merging, and calculating the size of an unknown message
pub trait AnyMessage : crate::CodedMessage + std::any::Any { }

impl<T: crate::CodedMessage + std::any::Any> AnyMessage for T { }

impl dyn AnyMessage {
    #[inline]
    pub fn is<T: AnyMessage>(&self) -> bool {
        let t = std::any::TypeId::of::<T>();
        let boxed = self.get_type_id();
        t == boxed
    }

    #[inline]
    pub fn downcast_ref<T: AnyMessage>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const dyn AnyMessage as *const T))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: AnyMessage>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                Some(&mut *(self as *mut dyn AnyMessage as *mut T))
            }
        } else {
            None
        }
    }
}

/// Provides the [`downcast`](#method.downcast) extension method for `Box`
pub trait AnyBoxExt : Sized + crate::internal::Sealed {
    fn downcast<T: AnyMessage>(self) -> Result<Box<T>, Self>;
}

impl crate::internal::Sealed for Box<dyn AnyMessage + 'static> { }

impl AnyBoxExt for Box<dyn AnyMessage + 'static> {
    #[inline]
    fn downcast<T: AnyMessage>(self) -> Result<Box<T>, Box<dyn AnyMessage>> {
        if self.is::<T>() {
            unsafe {
                let raw: *mut dyn AnyMessage = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }
}

impl crate::internal::Sealed for Box<dyn AnyMessage + 'static + Send> { }

impl AnyBoxExt for Box<dyn AnyMessage + 'static + Send> {
    #[inline]
    fn downcast<T: AnyMessage>(self) -> Result<Box<T>, Box<dyn AnyMessage + Send>> {
        <Box<dyn AnyMessage>>::downcast(self).map_err(|s| unsafe {
            Box::from_raw(Box::into_raw(s) as *mut (dyn AnyMessage + Send))
        })
    }
}

/// A message descriptor
pub struct MessageDescriptor {
    proto: *const DescriptorProto,
    new: Option<fn() -> Box<AnyMessage>>,
    scope: CompositeScope,
    scope_index: usize,
    full_name: String,
    fields: Box<[Ref<FieldDescriptor>]>,
    fields_ordered: Box<[Ref<FieldDescriptor>]>,
    message_fields: Box<[Ref<FieldDescriptor>]>,
    extensions: Box<[Ref<FieldDescriptor>]>,
    messages: Box<[Ref<MessageDescriptor>]>,
    enums: Box<[Ref<EnumDescriptor>]>,
    oneofs: Box<[Ref<OneofDescriptor>]>,
    info: Option<SourceCodeInfo>,
}

impl MessageDescriptor {
    pub fn proto(&self) -> &DescriptorProto {
        unsafe { &*self.proto }
    }

    /// Gets the file this descriptor belongs to
    pub fn scope(&self) -> &CompositeScope {
        &self.scope
    }

    /// Gets the index of this descriptor in its parent descriptor
    pub fn scope_index(&self) -> usize {
        self.scope_index
    }

    /// Creates a new instance of the type represented by this descriptor
    pub fn new_message(&self) -> Option<Box<AnyMessage>> {
        Some((self.new?)())
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn fields(&self) -> &[Ref<FieldDescriptor>] {
        &self.fields
    }

    /// Gets all the fields in this message except those contained within oneofs
    pub fn message_fields(&self) -> &[Ref<FieldDescriptor>] {
        &self.message_fields
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
        self.proto().options_option()
    }

    /// Creates a new string with the full name of this descriptor
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
    }

    pub fn map_entry(&self) -> bool {
        if let Some(options) = self.options() {
            options.map_entry()
        } else {
            false
        }
    }

    fn new(
        proto: *const DescriptorProto,
        scope: CompositeScope,
        pool: &mut DescriptorPool,
        index: usize,
    ) -> Ref<MessageDescriptor> {
        let descriptor_raw: *mut MessageDescriptor;
        let descriptor: &mut MessageDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.scope = scope;
        descriptor.scope_index = index;

        descriptor.full_name = get_full_type_name(descriptor.name(), descriptor.scope());

        descriptor.messages = descriptor
            .proto()
            .nested_type()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                MessageDescriptor::new(
                    m as *const DescriptorProto,
                    CompositeScope::Message(Ref::new(descriptor_raw)),
                    pool,
                    i,
                )
            })
            .collect::<Box<[_]>>();

        descriptor.enums = descriptor
            .proto()
            .enum_type()
            .iter()
            .enumerate()
            .map(|(i, e)| {
                EnumDescriptor::new(
                    e as *const EnumDescriptorProto,
                    CompositeScope::Message(Ref::new(descriptor_raw)),
                    pool,
                    i,
                )
            })
            .collect::<Box<[_]>>();

        descriptor.extensions = descriptor
            .proto()
            .extension()
            .iter()
            .enumerate()
            .map(|(i, e)| {
                FieldDescriptor::new(
                    &*e as *const FieldDescriptorProto,
                    FieldScope::Message(Ref::new(descriptor_raw)),
                    pool,
                    i,
                )
            })
            .collect::<Box<[_]>>();

        descriptor.oneofs = descriptor
            .proto()
            .oneof_decl() // oneofs before fields since we reference them when determining field scopes
            .iter()
            .map(|o| {
                OneofDescriptor::new(
                    o as *const OneofDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                )
            })
            .collect::<Box<[_]>>();

        descriptor.fields = descriptor
            .proto()
            .field()
            .iter()
            .enumerate()
            .map(|(i, f)| {
                FieldDescriptor::new(
                    f as *const FieldDescriptorProto,
                    if f.has_oneof_index() {
                        FieldScope::Oneof(Ref::clone(&descriptor.oneofs[f.oneof_index() as usize]))
                    } else {
                        FieldScope::Message(Ref::new(descriptor_raw))
                    },
                    pool,
                    i,
                )
            })
            .collect::<Box<[_]>>();

        let mut number_order = descriptor
            .fields()
            .iter()
            .map(Ref::clone)
            .collect::<Vec<_>>();
        number_order.sort_by_key(|r| r.number());
        descriptor.fields_ordered = number_order.into();
        descriptor.message_fields = descriptor
            .fields()
            .iter()
            .filter(|f| !f.proto().has_oneof_index())
            .map(Ref::clone)
            .collect::<Box<[_]>>();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
            Symbol::Message(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool, struct_info: Option<&GeneratedStructInfo>) {
        if let Some(struct_info) = struct_info {
            self.new = Some(struct_info.new);
            if let Some(structs) = &struct_info.structs {
                for (message, message_info) in self.messages.iter_mut().zip(structs.iter()) {
                    unsafe {
                        Ref::get_mut(message).cross_ref(pool, Some(message_info));
                    }
                }
            }
        } else {
            for message in self.messages.iter_mut() {
                unsafe {
                    Ref::get_mut(message).cross_ref(pool, None);
                }
            }
        }

        for field in self.fields.iter_mut() {
            unsafe {
                Ref::get_mut(field).cross_ref(pool);
            }
        }

        for oneof in self.oneofs.iter_mut() {
            unsafe {
                Ref::get_mut(oneof).cross_ref();
            }
        }

        for extension in self.extensions.iter_mut() {
            unsafe {
                Ref::get_mut(extension).cross_ref(pool);
            }
        }
    }

    fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            unsafe {
                match path[0] {
                    2 => Ref::get_mut(&mut self.fields[path[1] as usize])
                        .get_source_code_info(&path[2..]),
                    3 => Ref::get_mut(&mut self.messages[path[1] as usize])
                        .get_source_code_info(&path[2..]),
                    4 => Ref::get_mut(&mut self.enums[path[1] as usize])
                        .get_source_code_info(&path[2..]),
                    6 => Ref::get_mut(&mut self.extensions[path[1] as usize])
                        .get_source_code_info(&path[2..]),
                    8 => Ref::get_mut(&mut self.oneofs[path[1] as usize])
                        .get_source_code_info(&path[2..]),
                    _ => None,
                }
            }
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
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

impl Debug for MessageDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("MessageDescriptor")
            .field("name", &self.name())
            .field("fields", &self.fields())
            .field("oneofs", &self.oneofs())
            .field("messages", &self.messages())
            .field("enums", &self.enums())
            .field("extensions", &self.extensions())
            .finish()
    }
}

pub struct EnumDescriptor {
    proto: *const EnumDescriptorProto,
    scope: CompositeScope,
    scope_index: usize,
    full_name: String,
    values: Box<[Ref<EnumValueDescriptor>]>,
    info: Option<SourceCodeInfo>,
}

impl EnumDescriptor {
    pub fn proto(&self) -> &EnumDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn scope(&self) -> &CompositeScope {
        &self.scope
    }

    pub fn scope_index(&self) -> usize {
        self.scope_index
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn values(&self) -> &[Ref<EnumValueDescriptor>] {
        &self.values
    }

    pub fn options(&self) -> Option<&EnumOptions> {
        self.proto().options_option()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
    }

    fn new(
        proto: *const EnumDescriptorProto,
        scope: CompositeScope,
        pool: &mut DescriptorPool,
        index: usize,
    ) -> Ref<EnumDescriptor> {
        let descriptor_raw: *mut EnumDescriptor;
        let descriptor: &mut EnumDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.scope = scope;
        descriptor.scope_index = index;
        descriptor.full_name = get_full_type_name(descriptor.name(), descriptor.scope());

        descriptor.values = descriptor
            .proto()
            .value()
            .iter()
            .enumerate()
            .map(|(i, v)| {
                EnumValueDescriptor::new(
                    &*v as *const EnumValueDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                    i,
                )
            })
            .collect::<Vec<_>>()
            .into();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
            Symbol::Enum(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    unsafe fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            match path[0] {
                2 => Ref::get_mut(&mut self.values[path[1] as usize])
                    .get_source_code_info(&path[2..]),
                _ => None,
            }
        }
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.scope().file()
    }
}

impl Debug for EnumDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("EnumDescriptor")
            .field("name", &self.name())
            .field("values", &self.values())
            .finish()
    }
}

pub struct EnumValueDescriptor {
    proto: *const EnumValueDescriptorProto,
    index: usize,
    enum_type: Ref<EnumDescriptor>,
    full_name: String,
    info: Option<SourceCodeInfo>,
}

impl EnumValueDescriptor {
    pub fn proto(&self) -> &EnumValueDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn enum_type(&self) -> &EnumDescriptor {
        &self.enum_type
    }

    /// Gets the index of this enum value in its parent enum
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn number(&self) -> i32 {
        self.proto().number()
    }

    pub fn options(&self) -> Option<&EnumValueOptions> {
        self.proto().options_option()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
    }

    fn new(
        proto: *const EnumValueDescriptorProto,
        parent: Ref<EnumDescriptor>,
        pool: &mut DescriptorPool,
        index: usize,
    ) -> Ref<EnumValueDescriptor> {
        let descriptor_raw: *mut EnumValueDescriptor;
        let descriptor: &mut EnumValueDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.enum_type = parent;
        descriptor.index = index;
        descriptor.full_name = format!(
            "{}.{}",
            descriptor.enum_type().full_name().clone(),
            descriptor.name()
        );

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
            Symbol::EnumValue(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            None
        }
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.enum_type().file()
    }
}

impl Debug for EnumValueDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("EnumValueDescriptor")
            .field("name", &self.name())
            .field("number", &self.number())
            .finish()
    }
}

pub struct ServiceDescriptor {
    proto: *const ServiceDescriptorProto,
    full_name: String,
    file: Ref<FileDescriptor>,
    index: usize,
    methods: Box<[Ref<MethodDescriptor>]>,
    info: Option<SourceCodeInfo>,
}

impl ServiceDescriptor {
    pub fn proto(&self) -> &ServiceDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn file(&self) -> &FileDescriptor {
        &self.file
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn methods(&self) -> &[Ref<MethodDescriptor>] {
        &self.methods
    }

    pub fn options(&self) -> Option<&ServiceOptions> {
        self.proto().options_option()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
    }

    fn new(
        proto: *const ServiceDescriptorProto,
        file: Ref<FileDescriptor>,
        pool: &mut DescriptorPool,
        index: usize,
    ) -> Ref<ServiceDescriptor> {
        let descriptor_raw: *mut ServiceDescriptor;
        let descriptor: &mut ServiceDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.file = file;
        descriptor.index = index;
        descriptor.full_name = format!(".{}.{}", descriptor.file().name(), descriptor.name());

        descriptor.methods = descriptor
            .proto()
            .method()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                MethodDescriptor::new(
                    &*m as *const MethodDescriptorProto,
                    Ref::new(descriptor_raw),
                    pool,
                    i,
                )
            })
            .collect::<Box<[_]>>();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
            Symbol::Service(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        for method in self.methods.iter_mut() {
            unsafe {
                Ref::get_mut(method).cross_ref(pool);
            }
        }
    }

    unsafe fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            match path[0] {
                2 => Ref::get_mut(&mut self.methods[path[1] as usize])
                    .get_source_code_info(&path[2..]),
                _ => None,
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.file()
    }
}

impl Debug for ServiceDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("ServiceDescriptor")
            .field("name", &self.name())
            .field("methods", &self.methods())
            .finish()
    }
}

pub struct MethodDescriptor {
    proto: *const MethodDescriptorProto,
    full_name: String,
    service: Ref<ServiceDescriptor>,
    index: usize,
    input_type: Ref<MessageDescriptor>,
    output_type: Ref<MessageDescriptor>,
    info: Option<SourceCodeInfo>,
}

impl MethodDescriptor {
    pub fn proto(&self) -> &MethodDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn service(&self) -> &ServiceDescriptor {
        &self.service
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn input_type(&self) -> &MessageDescriptor {
        &self.input_type
    }

    pub fn output_type(&self) -> &MessageDescriptor {
        &self.output_type
    }

    pub fn client_streaming(&self) -> bool {
        self.proto().client_streaming()
    }

    pub fn server_streaming(&self) -> bool {
        self.proto().server_streaming()
    }

    pub fn options(&self) -> Option<&MethodOptions> {
        self.proto().options_option()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
    }

    fn new(
        proto: *const MethodDescriptorProto,
        service: Ref<ServiceDescriptor>,
        pool: &mut DescriptorPool,
        index: usize,
    ) -> Ref<MethodDescriptor> {
        let descriptor_raw: *mut MethodDescriptor;
        let descriptor: &mut MethodDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.service = service;
        descriptor.index = index;
        descriptor.full_name = format!(
            ".{}.{}",
            descriptor.service().full_name(),
            descriptor.name()
        );

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
            Symbol::Method(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        self.input_type = pool.get_message_ref(self.proto().input_type());
        self.output_type = pool.get_message_ref(self.proto().output_type());
    }

    fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            None
        }
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.service().file()
    }
}

impl Debug for MethodDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("MethodDescriptor")
            .field("name", &self.name())
            .field("input_type", &self.input_type().full_name())
            .field("output_type", &self.output_type().full_name())
            .finish()
    }
}

#[derive(PartialEq, Eq)]
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

impl FieldType {
    /// Gets the wire type of this field type.
    /// 
    /// This function does not consider if the field is packed.
    /// For the wire type of fields considering packed, use `FieldDescriptor::wire_type`
    pub fn wire_type(&self) -> WireType {
        match self {
            FieldType::Message(_) | FieldType::String | FieldType::Bytes => {
                WireType::LengthDelimited
            }
            FieldType::Group(_) => WireType::StartGroup,
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => WireType::Bit32,
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => WireType::Bit64,
            _ => WireType::Varint,
        }
    }
}

impl Debug for FieldType {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use crate::reflect::FieldType::*;
        match self {
            Double => fmt.write_str("Double"),
            Float => fmt.write_str("Float"),
            Int64 => fmt.write_str("Int64"),
            Uint64 => fmt.write_str("Uint64"),
            Sint64 => fmt.write_str("Sint64"),
            Fixed64 => fmt.write_str("Fixed64"),
            Sfixed64 => fmt.write_str("Sfixed64"),
            Int32 => fmt.write_str("Int32"),
            Uint32 => fmt.write_str("Uint32"),
            Sint32 => fmt.write_str("Sint32"),
            Fixed32 => fmt.write_str("Fixed32"),
            Sfixed32 => fmt.write_str("Sfixed32"),
            Bool => fmt.write_str("Bool"),
            String => fmt.write_str("String"),
            Bytes => fmt.write_str("Bytes"),
            Enum(e) => fmt.write_fmt(format_args!("Enum({})", e.full_name())),
            Message(m) => fmt.write_fmt(format_args!("Message({})", m.full_name())),
            Group(g) => fmt.write_fmt(format_args!("Group({})", g.full_name())),
        }
    }
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

impl Debug for DefaultValue {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use crate::reflect::DefaultValue::*;
        match self {
            None => fmt.write_str("None"),
            Invalid => fmt.write_str("Invalid"),
            Bool(b) => fmt.write_fmt(format_args!("Bool({})", b)),
            Double(d) => fmt.write_fmt(format_args!("Double({})", d)),
            SignedInt(s) => fmt.write_fmt(format_args!("SignedInt({})", s)),
            UnsignedInt(u) => fmt.write_fmt(format_args!("UnsignedInt({})", u)),
            Enum(e) => fmt.write_fmt(format_args!("Enum({})", e.full_name())),
            String(s) => fmt.write_fmt(format_args!("String({})", s)),
            Bytes(b) => fmt.write_fmt(format_args!("Bytes({:?})", b)),
        }
    }
}

pub struct FieldDescriptor {
    proto: *const FieldDescriptorProto,
    full_name: String,
    number: FieldNumber,
    scope: FieldScope,
    scope_index: usize,
    value_type: FieldType,
    default: DefaultValue,
    message: Ref<MessageDescriptor>,
    info: Option<SourceCodeInfo>,
}

impl FieldDescriptor {
    pub fn proto(&self) -> &FieldDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn number(&self) -> FieldNumber {
        self.number
    }

    pub fn label(&self) -> FieldLabel {
        self.proto().label().expect("Undefined enum value")
    }

    pub fn field_type(&self) -> &FieldType {
        &self.value_type
    }

    pub fn default_value(&self) -> &DefaultValue {
        &self.default
    }

    pub fn json_name(&self) -> &str {
        self.proto().json_name()
    }

    pub fn scope(&self) -> &FieldScope {
        &self.scope
    }

    /// Gets the index of this field in its parent descriptor
    pub fn scope_index(&self) -> usize {
        self.scope_index
    }

    pub fn options(&self) -> Option<&FieldOptions> {
        self.proto().options_option()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
    }

    pub fn packed(&self) -> bool {
        if self.label() == FieldLabel::Repeated && self.field_type().wire_type().is_packable() {
            if let Some(options) = self.options() {
                if options.has_packed() {
                    return options.packed();
                }
            }
            self.file().syntax() == Syntax::Proto3
        } else {
            false
        }
    }

    pub fn wire_type(&self) -> WireType {
        if self.packed() {
            WireType::LengthDelimited
        } else {
            self.field_type().wire_type()
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
        index: usize,
    ) -> Ref<FieldDescriptor> {
        let descriptor_raw: *mut FieldDescriptor;
        let descriptor: &mut FieldDescriptor;
        unsafe {
            descriptor_raw = raw_box(zeroed());
            descriptor = &mut *descriptor_raw;
        }

        descriptor.proto = proto;
        descriptor.scope = scope;
        descriptor.scope_index = index;
        descriptor.number = FieldNumber::new(descriptor.proto().number() as u32).expect("invalid field number");
        descriptor.full_name = match &descriptor.scope {
            FieldScope::File(f) => format!(".{}.{}", f.package(), descriptor.name()),
            FieldScope::Message(m) => format!("{}.{}", m.full_name(), descriptor.name()),
            FieldScope::Oneof(o) => format!("{}.{}", o.message().full_name(), descriptor.name()),
        };

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
            Symbol::Field(descriptor_raw),
        ) {
            panic!()
        }

        Ref::new(descriptor_raw)
    }

    fn cross_ref(&mut self, pool: &mut DescriptorPool) {
        use crate::descriptor::FieldDescriptorProto_Type::*;
        self.value_type = match self.proto().r#type().expect("Undefined enum value") {
            Message => FieldType::Message(pool.get_message_ref(self.proto().type_name())),
            Enum => FieldType::Enum(pool.get_enum_ref(self.proto().type_name())),
            Group => FieldType::Group(pool.get_message_ref(self.proto().type_name())),
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

        if self.proto().has_default_value() {
            self.default = match self.field_type() {
                FieldType::Enum(e) => DefaultValue::Enum(pool.get_enum_value_ref(
                    &(e.full_name().to_string() + "." + self.proto().default_value()),
                )),
                FieldType::Double | FieldType::Float => {
                    match self.proto().default_value().parse() {
                        Ok(ok) => DefaultValue::Double(ok),
                        Err(_) => DefaultValue::Invalid,
                    }
                }
                FieldType::Int32
                | FieldType::Int64
                | FieldType::Sfixed32
                | FieldType::Sfixed64
                | FieldType::Sint32
                | FieldType::Sint64 => match self.proto().default_value().parse() {
                    Ok(ok) => DefaultValue::SignedInt(ok),
                    Err(_) => DefaultValue::Invalid,
                },
                FieldType::Uint32 | FieldType::Uint64 | FieldType::Fixed32 | FieldType::Fixed64 => {
                    match self.proto().default_value().parse() {
                        Ok(ok) => DefaultValue::UnsignedInt(ok),
                        Err(_) => DefaultValue::Invalid,
                    }
                }
                FieldType::Bool => match self.proto().default_value().parse() {
                    Ok(ok) => DefaultValue::Double(ok),
                    Err(_) => DefaultValue::Invalid,
                },
                FieldType::String => DefaultValue::String(self.proto().default_value().to_string()),
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

                    let mut result = Vec::with_capacity(self.proto().default_value().len());
                    for (i, c) in self.proto().default_value().char_indices() {
                        match c {
                            '\\' => result.push(esc_lit(&self.proto().default_value()[i..])),
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

        if self.proto().has_extendee() {
            self.message = pool.get_message_ref(self.proto().extendee());
        } else {
            self.message = match &self.scope {
                FieldScope::Message(m) => Ref::clone(m),
                FieldScope::Oneof(o) => Ref::clone(&o.message),
                _ => panic!(),
            }
        }
    }

    fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            None
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
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

impl Debug for FieldDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("FieldDescriptor")
            .field("label", &self.label())
            .field("name", &self.name())
            .field("number", &self.number())
            .field("field_type", self.field_type())
            .field("default_value", self.default_value())
            .finish()
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
    info: Option<SourceCodeInfo>,
}

impl OneofDescriptor {
    pub fn proto(&self) -> &OneofDescriptorProto {
        unsafe { &*self.proto }
    }

    pub fn message(&self) -> &MessageDescriptor {
        &self.message
    }

    pub fn name(&self) -> &str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn fields(&self) -> &[Ref<FieldDescriptor>] {
        &self.fields
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo> {
        self.info.as_ref()
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
        descriptor.full_name = descriptor.message.full_name().to_string() + "." + descriptor.name();

        if let Some(_) = pool.symbols.insert(
            descriptor.full_name().to_string(),
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

    fn get_source_code_info(&mut self, path: &[i32]) -> Option<&mut Option<SourceCodeInfo>> {
        if path.is_empty() {
            Some(&mut self.info)
        } else {
            None
        }
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
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor {
        self.message().file()
    }
}

impl Debug for OneofDescriptor {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(self.name())
    }
}
