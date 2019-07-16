//! Provides reflection and dynamic message access to protobuf messages

pub use crate::descriptor::field_descriptor_proto::Label as FieldLabel;

use crate::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumOptions, EnumValueDescriptorProto, EnumValueOptions,
    FieldDescriptorProto, field_descriptor_proto::Type, FieldOptions, FileDescriptorProto, FileOptions, MessageOptions,
    MethodDescriptorProto, MethodOptions, OneofDescriptorProto, ServiceDescriptorProto,
    ServiceOptions,
};
use crate::io::{FieldNumber, WireType};
use crate::{CodedMessage, Enum, EnumValue::{self, Defined}, ExtendableMessage, ExtensionRegistry, Message};
use std::any::TypeId;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{self, Iterator, IntoIterator};
use std::ptr::NonNull;
use std::ops::{Drop, Deref};
use std::sync::Arc;

/// Contains types and traits for dynamic field access
pub mod access;
mod dynamic;

// macro the methods and impls from std::any::Any
macro_rules! any_extensions {
    ($nm:tt) => {
        impl<'a> dyn $nm<'a> {
            /// Returns `true` if the boxed type is the same as `T`.
            #[inline]
            pub fn is<T: $nm<'static>>(&self) -> bool {
                match self.type_id() {
                    ValueType::Dynamic(_) => false,
                    ValueType::Static(s) => TypeId::of::<T>() == s,
                }
            }

            /// Returns some reference to the boxed value if it is of type `T`, or
            /// `None` if it isn't.
            #[inline]
            pub fn downcast_ref<T: $nm<'static>>(&self) -> Option<&T> {
                if self.is::<T>() {
                    unsafe { Some(&*(self as *const dyn $nm as *const T)) }
                } else {
                    None
                }
            }

            /// Returns some mutable reference to the boxed value if it is of type `T`, or
            /// `None` if it isn't.
            #[inline]
            pub fn downcast_mut<T: $nm<'static>>(&mut self) -> Option<&mut T> {
                if self.is::<T>() {
                    unsafe { Some(&mut *(self as *mut dyn $nm as *mut T)) }
                } else {
                    None
                }
            }
        }
    };
}

/// The type of a protobuf value. Dynamic types cannot be casted to static types however static types can be casted.
#[derive(PartialEq, Eq, Debug, Hash)]
pub enum ValueType {
    /// A static type that can be casted to a concrete type
    Static(TypeId),
    /// A dynamic type that can't be casted to any concrete type
    Dynamic(DynamicType),
}
/// The type of dynamic value
#[derive(PartialEq, Eq, Debug, Hash)]
pub enum DynamicType {
    /// A dynamic enum value
    Enum,
    /// A dynamic message value
    Message,
}

/// Represents a value of any type that can be cloned, compared for partial equvilance,
/// and used in debug formatting.
///
/// It can also be upcasted to an enum value of any type or a message of any type
///
/// # Safety
///
/// These traits should not be implemented by consumers, rather consumers should use
/// the blanket implementations provided by the library. Implementing these traits outside
/// of the crate can cause undefined behavior.
pub trait AnyValue<'a>: 'a + Debug + Send + Sync {
    /// Clones the value, returning a new box containing it
    fn clone(&self) -> Box<dyn AnyValue<'a>>;

    // a PartialEq requirement creates a cycle so we use this method and implement PartialEq on top of it
    /// Compares this value with another value of any type
    fn eq(&self, other: &dyn AnyValue<'a>) -> bool;

    /// Gets the Static type ID of this value, or Dynamic if the value is dynamic
    fn type_id(&self) -> ValueType;

    /// Attempts to cast this value into an enum value of any type
    fn as_enum(&self) -> Option<&dyn AnyEnum<'a>>;

    /// Attempts to cast this value into an enum value of any type
    fn as_enum_mut(&mut self) -> Option<&mut dyn AnyEnum<'a>>;

    /// Attempts to cast this value into a message value of any type
    fn as_message(&self) -> Option<&dyn AnyMessage<'a>>;

    /// Attempts to cast this value into a message value of any type
    fn as_message_mut(&mut self) -> Option<&mut dyn AnyMessage<'a>>;
}

any_extensions!(AnyValue);

// any impl for primitive types
// we impl with an unbounded lifetime so we 
// can bind these static values to other descriptors with
// unbounded lifetimes, ie so Box<String> can cast to Box<dyn AnyValue<'a> + 'a>
macro_rules! any_impl {
    ($($nm:ty),*) => {
        $(impl<'a> AnyValue<'a> for $nm {
            fn clone(&self) -> Box<dyn AnyValue<'a>> {
                Box::new(Clone::clone(self))
            }

            fn eq(&self, other: &dyn AnyValue<'a>) -> bool {
                match other.type_id() {
                    ValueType::Static(s) if s == TypeId::of::<Self>() => {
                        let other = unsafe { &*(other as *const dyn AnyValue as *const Self) };
                        self == other
                    },
                    _ => false
                }
            }
            fn type_id(&self) -> ValueType { ValueType::Static(TypeId::of::<$nm>()) }
            fn as_enum(&self) -> Option<&dyn AnyEnum<'a>> { None }
            fn as_enum_mut(&mut self) -> Option<&mut dyn AnyEnum<'a>> { None }
            fn as_message(&self) -> Option<&dyn AnyMessage<'a>> { None }
            fn as_message_mut(&mut self) -> Option<&mut dyn AnyMessage<'a>> { None }
        }
        )+
    };
}

any_impl!(bool, i32, i64, u32, u64, f32, f64, String, Vec<u8>);

impl<'a> dyn AnyValue<'a> {
    pub fn downcast<T: AnyValue<'static>>(
        self: Box<dyn AnyValue<'a>>,
    ) -> Result<Box<T>, Box<dyn AnyValue<'a>>> {
        if self.is::<T>() {
            unsafe {
                let raw: *mut dyn AnyValue = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }
}

impl<'a> Clone for Box<dyn AnyValue<'a>> {
    fn clone(&self) -> Box<dyn AnyValue<'a>> {
        self.as_ref().clone()
    }
}

impl<'a> PartialEq<dyn AnyValue<'a>> for dyn AnyValue<'a> {
    fn eq(&self, other: &dyn AnyValue<'a>) -> bool {
        AnyValue::eq(self, other)
    }
}

/// Represents an enum value of any type
pub trait AnyEnum<'a>: AnyValue<'a> {
    /// Gets the descriptor of this enum type
    fn descriptor(&self) -> &'a EnumDescriptor<'a>;

    /// Gets the enum value descriptor describing the set value of the enum,
    /// or None if the value is undefined
    fn get(&self) -> Option<&'a EnumValueDescriptor<'a>>;

    /// Gets the value of this enum as an i32
    fn get_i32(&self) -> i32;

    /// Sets the value to the number described by the provided descriptor.
    /// If the enum type of the descriptor value is not the same as this enum, this returns false.
    ///
    /// Consumers should not assume that setting a value to a specified
    /// enum descriptor will return the same descriptor by calling `AnyEnum::get`, only that it
    /// will return a descriptor with the same underlying value.
    fn set(&mut self, value: &'a EnumValueDescriptor<'a>) -> bool;

    /// Sets the value of the enum to the specified 32-bit value. This may be an undefined value
    fn set_i32(&mut self, value: i32);
}

impl<E: Enum> AnyValue<'static> for EnumValue<E> {
    fn clone(&self) -> Box<dyn AnyValue<'static>> {
        Box::new(Clone::clone(self))
    }

    fn eq(&self, other: &dyn AnyValue<'static>) -> bool {
        match other.type_id() {
            ValueType::Static(s) if s == TypeId::of::<Self>() => {
                let other = unsafe { &*(other as *const dyn AnyValue as *const Self) };
                self == other
            },
            _ => false
        }
    }
    fn type_id(&self) -> ValueType {
        ValueType::Static(TypeId::of::<EnumValue<E>>())
    }
    fn as_enum(&self) -> Option<&dyn AnyEnum<'static>> {
        Some(self)
    }
    fn as_enum_mut(&mut self) -> Option<&mut dyn AnyEnum<'static>> {
        Some(self)
    }
    fn as_message(&self) -> Option<&dyn AnyMessage<'static>> {
        None
    }
    fn as_message_mut(&mut self) -> Option<&mut dyn AnyMessage<'static>> {
        None
    }
}

impl<E: Enum> AnyEnum<'static> for EnumValue<E> {
    fn descriptor(&self) -> &'static EnumDescriptor<'static> {
        E::descriptor()
    }

    fn get(&self) -> Option<&'static EnumValueDescriptor<'static>> {
        let value = self.get_i32();
        E::descriptor()
            .values()
            .iter()
            .find(move |e| e.number() == value)
            .map(|r| &**r)
    }

    fn get_i32(&self) -> i32 {
        i32::from(*self)
    }

    fn set(&mut self, value: &'static EnumValueDescriptor<'static>) -> bool {
        if value.enum_type() == E::descriptor() {
            self.set_i32(value.number());
            true
        } else {
            false
        }
    }

    fn set_i32(&mut self, value: i32) {
        *self = EnumValue::from(value)
    }
}

/// A message type to emulate dynamic typing.
/// This type is like Any and allows for downcasting the type to a concrete type.
///
/// It also has the methods of CodedMessage, allowing for reading, merging, and calculating the size of an unknown message
pub trait AnyMessage<'a>: CodedMessage + AnyValue<'a> {
    /// Attempts to merge the two messages together.
    /// If the two messages are not of the same type, this does nothing.
    fn merge(&mut self, other: &dyn AnyMessage<'a>);

    /// Gets the descriptor for this message
    fn descriptor(&self) -> &'a MessageDescriptor<'a>;

    /// For extension messages, gets the registry in use by the message.
    /// Dynamic messages don't contain extension registries
    fn registry(&self) -> Option<&'static ExtensionRegistry>;

    /// For extension messages, replaces the registry in use by the message
    fn replace_registry(
        &mut self,
        extensions: Option<&'static ExtensionRegistry>,
    ) -> Option<&'static ExtensionRegistry>;
}

any_extensions!(AnyMessage);

impl<T: Message> AnyValue<'static> for T {
    fn clone(&self) -> Box<dyn AnyValue<'static>> {
        Box::new(Clone::clone(self))
    }

    fn eq(&self, other: &dyn AnyValue<'static>) -> bool {
        match other.type_id() {
            ValueType::Static(s) if s == TypeId::of::<Self>() => {
                let other = unsafe { &*(other as *const dyn AnyValue as *const Self) };
                self == other
            },
            _ => false
        }
    }
    fn type_id(&self) -> ValueType {
        ValueType::Static(TypeId::of::<T>())
    }
    fn as_enum(&self) -> Option<&dyn AnyEnum<'static>> {
        None
    }
    fn as_enum_mut(&mut self) -> Option<&mut dyn AnyEnum<'static>> {
        None
    }
    fn as_message(&self) -> Option<&dyn AnyMessage<'static>> {
        Some(self)
    }
    fn as_message_mut(&mut self) -> Option<&mut dyn AnyMessage<'static>> {
        Some(self)
    }
}

impl<T: Message> AnyMessage<'static> for T {
    fn merge(&mut self, other: &dyn AnyMessage<'static>) {
        match other.type_id() {
            ValueType::Static(s) if s == TypeId::of::<Self>() => {
                let other = unsafe { &*(other as *const dyn AnyMessage as *const Self) };
                self.merge(other);
            },
            _ => { }
        }
    }

    fn descriptor(&self) -> &'static MessageDescriptor<'static> {
        T::descriptor()
    }

    default fn registry(&self) -> Option<&'static ExtensionRegistry> {
        None
    }

    default fn replace_registry(
        &mut self,
        _: Option<&'static ExtensionRegistry>,
    ) -> Option<&'static ExtensionRegistry> {
        None
    }
}

impl<T: Message + ExtendableMessage> AnyMessage<'static> for T {
    fn registry(&self) -> Option<&'static ExtensionRegistry> {
        ExtendableMessage::registry(self)
    }

    fn replace_registry(
        &mut self,
        extensions: Option<&'static ExtensionRegistry>,
    ) -> Option<&'static ExtensionRegistry> {
        ExtendableMessage::replace_registry(self, extensions)
    }
}

mod internal {
    use super::{AnyMessage, FileDescriptor, MessageDescriptor, OneofDescriptor, FieldDescriptor, EnumDescriptor, EnumValueDescriptor, ServiceDescriptor, MethodDescriptor};

    #[derive(PartialEq, Eq, Hash)]
    pub enum Symbol<'a> {
        File(FileDescriptor<'a>),
        Message(MessageDescriptor<'a>),
        Field(FieldDescriptor<'a>),
        Oneof(OneofDescriptor<'a>),
        Enum(EnumDescriptor<'a>),
        EnumValue(EnumValueDescriptor<'a>),
        Service(ServiceDescriptor<'a>),
        Method(MethodDescriptor<'a>),
    }

    impl<'a> Symbol<'a> {
        pub fn proto(&self) -> &'a dyn AnyMessage<'static> {
            match self {
                Symbol::File(s) => s.proto(),
                Symbol::Message(s) => s.proto(),
                Symbol::Field(s) => s.proto(),
                Symbol::Oneof(s) => s.proto(),
                Symbol::Enum(s) => s.proto(),
                Symbol::EnumValue(s) => s.proto(),
                Symbol::Service(s) => s.proto(),
                Symbol::Method(s) => s.proto(),
            }
        }
        pub fn unwrap<T: SymbolType<'a>>(&self) -> &T {
            T::get(self)
        }
        pub fn name(&self) -> &'static str {
            match self {
                Symbol::File(_) => "File",
                Symbol::Message(_) => "Message",
                Symbol::Field(_) => "Field",
                Symbol::Oneof(_) => "Oneof",
                Symbol::Enum(_) => "Enum",
                Symbol::EnumValue(_) => "EnumValue",
                Symbol::Service(_) => "Service",
                Symbol::Method(_) => "Method"
            }
        }
    }

    pub trait SymbolType<'a> {
        fn get<'b>(_: &'b Symbol<'a>) -> &'b Self;
        fn get_mut<'b>(_: &'b mut Symbol<'a>) -> &'b mut Self;
    }

    macro_rules! unwrapper_impl {
        ($s:ident, $t:ident) => {
            impl<'a> SymbolType<'a> for $t<'a> {
                fn get<'b>(s: &'b Symbol<'a>) -> &'b $t<'a> {
                    match s {
                        Symbol::$s(ref s) => s,
                        s => unreachable!("{}{}", concat!("unexpected symbol: expected \"", stringify!($s), "\" got "), s.name())
                    }
                }
                fn get_mut<'b>(s: &'b mut Symbol<'a>) -> &'b mut $t<'a> {
                    match s {
                        Symbol::$s(ref mut s) => s,
                        s => unreachable!("{}{}", concat!("unexpected symbol: expected \"", stringify!($s), "\" got "), s.name())
                    }
                }
            }
        };
    }

    unwrapper_impl!(File, FileDescriptor);
    unwrapper_impl!(Message, MessageDescriptor);
    unwrapper_impl!(Oneof, OneofDescriptor);
    unwrapper_impl!(Field, FieldDescriptor);
    unwrapper_impl!(Enum, EnumDescriptor);
    unwrapper_impl!(EnumValue, EnumValueDescriptor);
    unwrapper_impl!(Service, ServiceDescriptor);
    unwrapper_impl!(Method, MethodDescriptor);
}

use internal::Symbol;

/// A reference to a symbol in a descriptor pool
pub struct Ref<T>(NonNull<T>);

impl<T> Ref<T> {
    fn clone(this: &Self) -> Self {
        Ref(this.0)
    }
    fn from(b: &T) -> Ref<T> {
        Ref(NonNull::from(b))
    }
    const fn dangling() -> Self {
        Ref(NonNull::dangling())
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsRef<T> for Ref<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> std::borrow::Borrow<T> for Ref<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T: Debug> Debug for Ref<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
    fn ne(&self, other: &Self) -> bool {
        self.deref().ne(other.deref())
    }
}

impl<T: Eq> Eq for Ref<T> { }

impl<T: Hash> Hash for Ref<T> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.deref().hash(h)
    }
}

// we only use these after construction when the pool is immutable, so Send and Sync is valid here
unsafe impl<T: Send> Send for Ref<T> { }
unsafe impl<T: Sync> Sync for Ref<T> { }

#[derive(PartialEq, Eq, Hash)]
enum SymbolRef<'a> {
    File(FileRef<'a>),
    Message(MessageRef<'a>),
    Field(FieldRef<'a>),
    Oneof(OneofRef<'a>),
    Enum(EnumRef<'a>),
    EnumValue(EnumValueRef<'a>),
    Service(ServiceRef<'a>),
    Method(MethodRef<'a>)
}

impl<'a> From<RefSymbol<'a>> for SymbolRef<'a> {
    fn from(symbol: RefSymbol<'a>) -> Self {
        match symbol.deref() {
            Symbol::File(ref f) => SymbolRef::File(Ref(NonNull::from(f))),
            Symbol::Message(ref f) => SymbolRef::Message(Ref(NonNull::from(f))),
            Symbol::Field(ref f) => SymbolRef::Field(Ref(NonNull::from(f))),
            Symbol::Oneof(ref f) => SymbolRef::Oneof(Ref(NonNull::from(f))),
            Symbol::Enum(ref f) => SymbolRef::Enum(Ref(NonNull::from(f))),
            Symbol::EnumValue(ref f) => SymbolRef::EnumValue(Ref(NonNull::from(f))),
            Symbol::Service(ref f) => SymbolRef::Service(Ref(NonNull::from(f))),
            Symbol::Method(ref f) => SymbolRef::Method(Ref(NonNull::from(f))),
        }
    }
}

type RefSymbol<'a> = Ref<Symbol<'a>>;
/// A reference to a file in a descriptor pool
pub type FileRef<'a> = Ref<FileDescriptor<'a>>;
/// A reference to a message in a descriptor pool
pub type MessageRef<'a> = Ref<MessageDescriptor<'a>>;
/// A reference to a field in a descriptor pool
pub type FieldRef<'a> = Ref<FieldDescriptor<'a>>;
/// A reference to a oneof in a descriptor pool
pub type OneofRef<'a> = Ref<OneofDescriptor<'a>>;
/// A reference to a enum in a descriptor pool
pub type EnumRef<'a> = Ref<EnumDescriptor<'a>>;
/// A reference to a enum value in a descriptor pool
pub type EnumValueRef<'a> = Ref<EnumValueDescriptor<'a>>;
/// A reference to a service in a descriptor pool
pub type ServiceRef<'a> = Ref<ServiceDescriptor<'a>>;
/// A reference to a method in a descriptor pool
pub type MethodRef<'a> = Ref<MethodDescriptor<'a>>;

static UNCHECKED_BORROW_MSG: &str = "symbol database should be immutable after construction";

struct SymbolDatabase<'a> {
    backups: Box<[Arc<SymbolDatabase<'a>>]>, // if we don't find a symbol we search a set of backups. this currently only used with external modules
    symbols: RefCell<Box<[Symbol<'a>]>>,
    full_map: RefCell<HashMap<Box<str>, RefSymbol<'a>>>, // a map of the full names of each symbol
    num_symbol_map: RefCell<HashMap<(i32, MessageRef<'a>), FieldRef<'a>>>, // a map of numbers and symbols to other symbols (used for fields)
    ext_symbol_map: RefCell<HashMap<MessageRef<'a>, HashSet<FieldRef<'a>>>>, // a map of extensions in this set that extend the message at a specified index
    str_symbol_map: RefCell<HashMap<(&'a str, SymbolRef<'a>), RefSymbol<'a>>>, // a map of strings and symbols to other symbols (used for named items)
}

impl<'a> SymbolDatabase<'a> {
    fn new(backups: Box<[Arc<SymbolDatabase<'a>>]>, files: &'a [FileDescriptorProto], code_info: Option<Box<[GeneratedCodeInfo<'a>]>>) -> Result<Arc<SymbolDatabase<'a>>, PoolError<'a>> {
        let db = Arc::new(SymbolDatabase {
            backups,
            symbols: RefCell::default(),
            full_map: RefCell::default(),
            num_symbol_map: RefCell::default(),
            ext_symbol_map: RefCell::default(),
            str_symbol_map: RefCell::default(),
        });

        let mut symbols = Vec::new();
        for (proto, code_info) in files.iter().zip(code_info.map(Vec::from).into_iter().flatten().map(Some).chain(iter::repeat_with(|| None))) {
            FileDescriptor::new(proto, &db, &mut symbols, code_info)?;
        }
        db.symbols.replace(symbols.into_boxed_slice());

        let mut symbols_borrow = db.symbols.borrow_mut();
        let mut iter = symbols_borrow.iter_mut();
        while let Some(symbol) = iter.next() {
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::File(f) => f.map_symbols(symbol_ref, &mut iter)?,
                _ => unreachable!("matched symbol should always be a file")
            }
        }

        for symbol in symbols_borrow.iter_mut() {
            match symbol {
                Symbol::File(f) => f.cross_ref()?,
                Symbol::Field(f) => f.cross_ref()?,
                Symbol::Oneof(o) => o.cross_ref()?,
                Symbol::Method(m) => m.cross_ref()?,
                _ => { }
            }
        }

        drop(symbols_borrow);

        Ok(db)
    }

    fn find_symbol(&self, name: &'a str, relative_symbol: SymbolRef<'a>) -> Result<RefSymbol<'a>, PoolError<'a>> {
        if name.starts_with('.') {
            self.find_full_symbol(&name[1..]).ok_or_else(|| PoolError::MissingSymbol(Cow::Borrowed(name)))
        } else {
            unimplemented!()
        }
    }

    fn find_full_symbol(&self, name: &str) -> Option<RefSymbol<'a>> {
        self.full_map
            .borrow()
            .get(name)
            .map(Ref::clone)
            .or_else(|| 
                self.backups
                    .iter()
                    .filter_map(|b| b.find_full_symbol(name))
                    .nth(0))
    }

    fn borrow_symbol(&self, name: &str) -> Option<&Symbol<'a>> {
        unsafe {
            self.full_map
                .try_borrow_unguarded()
                .expect(UNCHECKED_BORROW_MSG)
                .get(name)
                .map(Deref::deref)
                .or_else(|| self.backups.iter().flat_map(|db| db.borrow_symbol(name)).nth(0))
        }
    }

    fn find_extensions_for_message_by_index<'b>(&'b self, r: MessageRef<'a>) -> impl Iterator<Item = &'b FieldDescriptor<'a>> + 'b {
        unsafe {
            self.ext_symbol_map.try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&r)
                .into_iter()
                .flat_map(move |s| s.iter().map(|s| &**s))
                .chain(
                    self.backups
                        .iter()
                        .flat_map::<Box<dyn Iterator<Item = &'b FieldDescriptor<'a>> + 'b>, _>(
                            move |db| Box::new(db.find_extensions_for_message_by_index(Ref::clone(&r)))))
        }
    }
}

unsafe impl Sync for SymbolDatabase<'_> { } // after construction, the db is immutable, so we can mark it as sync safely

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
///     protrust::plugin::proto::file().proto().clone()
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
/// assert!(file_descriptor.full_name() == "google.protobuf.FileDescriptorProto");
///
/// let mut instance = file_descriptor.new_instance().unwrap();
/// assert!(instance.as_message().unwrap().calculate_size() == 0);
///
/// let other = protrust::descriptor::file().proto();
/// let file_instance = instance.downcast_mut::<FileDescriptorProto>().expect("Could not unwrap FileDescriptorProto");
/// LiteMessage::merge(file_instance, other);
///
/// assert_eq!(file_instance, other);
/// ```
pub struct DescriptorPool<'a> {
    db: Arc<SymbolDatabase<'a>>
}

impl<'a> DescriptorPool<'a> {
    /// Builds a descriptor pool from the slice of file descriptors
    ///
    /// These files can be defined in any order
    pub fn build_from_files(files: &'a [FileDescriptorProto]) -> DescriptorPool<'a> {
        DescriptorPool { db: SymbolDatabase::new(Box::default(), files, None).unwrap() }
    }

    #[doc(hidden)]
    pub fn build_from_generated_code(
        file: &'static [FileDescriptorProto],
        extern_pools: &'static [&'static DescriptorPool<'static>],
        info: Box<[GeneratedCodeInfo<'static>]>,
    ) -> DescriptorPool<'static> {
        DescriptorPool { db: SymbolDatabase::new(extern_pools.iter().map(|p| Arc::clone(&p.db)).collect(), file, Some(info)).unwrap() }
    }

    pub fn find_file_by_name(&self, name: &str) -> Option<&FileDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::File(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_message_by_name(&self, name: &str) -> Option<&MessageDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::Message(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_field_by_name(&self, name: &str) -> Option<&FieldDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::Field(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_oneof_by_name(&self, name: &str) -> Option<&OneofDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::Oneof(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&EnumDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::Enum(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_enum_value_by_name(&self, name: &str) -> Option<&EnumValueDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::EnumValue(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_service_by_name(&self, name: &str) -> Option<&ServiceDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::Service(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_method_by_name(&self, name: &str) -> Option<&MethodDescriptor<'a>> {
        match self.db.borrow_symbol(name) {
            Some(Symbol::Method(ref f)) => Some(f),
            _ => None
        }
    }

    pub fn find_extensions_for_message_by_name<'b>(
        &'b self,
        name: &str,
    ) -> Option<impl Iterator<Item = &'b FieldDescriptor<'a>> + 'b> {
        let symbol = self.db.borrow_symbol(name)?;
        match symbol {
            Symbol::Message(m) => Some(self.db.find_extensions_for_message_by_index(Ref::from(m))),
            _ => None
        }
    }
}

// the problem with Arc cycles is simply dropping an Arc won't drop the 
// underlying data because the reference cycles. however if we drop the 
// underlying data with the arcs "owner" manually we can remove the 
// reference cycles while dropping so when the arc drops it can drop normally
impl Drop for DescriptorPool<'_> {
    fn drop(&mut self) {
        self.db.ext_symbol_map.replace(HashMap::new());
        self.db.num_symbol_map.replace(HashMap::new());
        self.db.str_symbol_map.replace(HashMap::new());
        self.db.full_map.replace(HashMap::new());
        self.db.symbols.replace(Box::default());
    }
}

#[derive(Debug)]
pub enum PoolError<'a> {
    Validation(ValidationError<'a>),
    Conflict(ConflictError<'a>),
    MissingSymbol(Cow<'a, str>),
}

#[derive(Debug)]
pub struct ConflictError<'a> {
    inserted: &'a dyn AnyMessage<'static>,
    conflict: &'a dyn AnyMessage<'static>,
    kind: ConflictKind
}

impl<'a> ConflictError<'a> {
    fn new(inserted: &'a dyn AnyMessage<'static>, conflict: &'a dyn AnyMessage<'static>, kind: ConflictKind) -> Self {
        Self { inserted, conflict, kind }
    }
}

/// The kind of conflict that creates an error
#[derive(Debug)]
pub enum ConflictKind {
    /// Name conflicts (for example when a message and field name conflict)
    Name,
    /// Field number conflicts
    Number
}

/// Describes a validation error that occured when constructing a descriptor pool.
/// This occurs when an input file descriptor / child descriptor has an invalid value,
/// preventing the descriptor itself from being built
#[derive(Debug)]
pub struct ValidationError<'a> {
    proto: &'a dyn AnyMessage<'static>,
    field: u32,
    index: Option<usize>,
    err: Box<str>,
}

impl<'a> ValidationError<'a> {
    fn new(
        proto: &'a dyn AnyMessage<'static>,
        field: u32,
        index: Option<usize>,
        err: Box<str>,
    ) -> ValidationError<'a> {
        ValidationError { proto, field, index, err }
    }
}

impl Display for ValidationError<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let descriptor = self.proto.descriptor();
        let field = descriptor
            .fields()
            .iter()
            .find(|f| f.number().get() == self.field)
            .ok_or(fmt::Error)?; // it shouldn't fail anyway
        let value = match field.accessor().unwrap() {
            access::FieldAccessor::Single(s) => s.get(self.proto).map_err(|_| fmt::Error)?,
            access::FieldAccessor::Repeated(r) => r.get(self.proto, self.index.unwrap()).map_err(|_| fmt::Error)?,
            _ => unreachable!()
        };
        write!(
            fmt,
            "An error occured while validating {}.{}: {}\nValue: {:#?}",
            descriptor.name(),
            field.name(),
            self.err,
            value
        )
    }
}

impl Error for ValidationError<'_> {}

/// A trait containing all the shared items of a descriptor
pub trait Descriptor<'a> {
    /// Gets the underlying message that created this descriptor
    fn proto(&self) -> &'a dyn AnyMessage<'static>;
    /// Gets the name of this descriptor
    fn name(&self) -> &str;
    /// Gets the full name of this descriptor
    fn full_name(&self) -> &str;
    /// Gets the file that this descriptor is contained in
    fn file(&self) -> &FileDescriptor<'a>;
}

macro_rules! descriptor_comparison_impls {
    ($t:tt) => {
        impl PartialEq for $t<'_> {
            fn eq(&self, other: &Self) -> bool {
                std::ptr::eq(self.proto, other.proto) && Arc::ptr_eq(&self.db, &other.db)
            }
        }
        impl Eq for $t<'_> { }
        impl Hash for $t<'_> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                std::ptr::hash(self.proto(), state)
            }
        }
    };
}

descriptor_comparison_impls!(FileDescriptor);
descriptor_comparison_impls!(MessageDescriptor);
descriptor_comparison_impls!(EnumDescriptor);
descriptor_comparison_impls!(ServiceDescriptor);
descriptor_comparison_impls!(FieldDescriptor);
descriptor_comparison_impls!(OneofDescriptor);
descriptor_comparison_impls!(EnumValueDescriptor);
descriptor_comparison_impls!(MethodDescriptor);

/// A structure containing the comments for a particular file's message, field, oneof, service, or method definition
pub struct SourceCodeInfo<'a> {
    leading_comments: Option<&'a String>,
    trailing_comments: Option<&'a String>,
    leading_detached_comments: &'a [String],
}

impl<'a> SourceCodeInfo<'a> {
    /// Gets the leading comments of a descriptor
    pub fn leading_comments(&self) -> Option<&'a String> {
        self.leading_comments
    }

    /// Gets the trailing comments of a descriptor
    pub fn trailing_comments(&self) -> Option<&'a String> {
        self.trailing_comments
    }

    /// Gets the leading detached comments of a descriptor
    pub fn leading_detached_comments(&self) -> &'a [String] {
        self.leading_detached_comments
    }
}

#[doc(hidden)]
pub struct GeneratedCodeInfo<'a> {
    pub structs: Option<Box<[GeneratedStructInfo<'a>]>>,
    pub extensions: Option<Box<[access::FieldAccessor<'a, 'a>]>>,
}

#[doc(hidden)]
pub struct GeneratedStructInfo<'a> {
    pub new: fn() -> Box<dyn AnyValue<'a>>,
    pub structs: Option<Box<[GeneratedStructInfo<'a>]>>,
    pub fields: Option<Box<[access::FieldAccessor<'a, 'a>]>>,
    pub extensions: Option<Box<[access::FieldAccessor<'a, 'a>]>>
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

impl Display for Syntax {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Syntax::Proto2 => write!(f, "proto2"),
            Syntax::Proto3 => write!(f, "proto3"),
            Syntax::Unknown => write!(f, "unknown"),
        }
    }
}

fn get_full_name(name: &str, scope: &CompositeScope) -> Box<str> {
    let scope_name = 
        match scope {
            CompositeScope::Message(m) => m.full_name(),
            CompositeScope::File(f) => f.package(),
        };
    format!("{}.{}", scope_name, name).into_boxed_str()
}

/// Describes a complete .proto file
pub struct FileDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a FileDescriptorProto,
    dependencies: Box<[FileRef<'a>]>,
    public_dependencies: Box<[FileRef<'a>]>,
    messages: Box<[MessageRef<'a>]>,
    enums: Box<[EnumRef<'a>]>,
    services: Box<[ServiceRef<'a>]>,
    extensions: Box<[FieldRef<'a>]>,
    syntax: Syntax,
}

impl<'a> FileDescriptor<'a> {
    /// Gets the underlying FileDescriptorProto that this descriptor represents
    pub fn proto(&self) -> &'a FileDescriptorProto {
        self.proto
    }

    pub fn dependencies(&self) -> &[FileRef<'a>] {
        &self.dependencies
    }

    pub fn public_dependencies(&self) -> &[FileRef<'a>] {
        &self.public_dependencies
    }

    /// Gets the name of this file
    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    /// Gets the package
    pub fn package(&self) -> &'a str {
        self.proto().package()
    }

    /// Gets the top-level messages defined in this file
    pub fn messages(&self) -> &[MessageRef<'a>] {
        &self.messages
    }

    pub fn find_message_by_name(&self, name: &str) -> Option<&MessageDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::File(Ref::from(self))))
                .and_then(|r| {
                    match &*r.0.as_ptr() {
                        Symbol::Message(ref m) => Some(m),
                        _ => None
                    }
                })
        }
    }

    /// Flattens all the messages in this file as an iterator
    pub fn flatten_messages<'b>(
        &'b self,
    ) -> impl Iterator<Item = &'b MessageDescriptor<'a>> {
        self.messages().iter().map(Deref::deref).chain(
            self.messages().iter().flat_map::<Box<dyn Iterator<Item = &'b MessageDescriptor<'a>> + 'b>, _>(|m| Box::new(m.flatten_messages())),
        )
    }

    /// Gets the top-level enums defined in this file
    pub fn enums(&self) -> &[EnumRef<'a>] {
        &self.enums
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&EnumDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::File(Ref::from(self))))
                .and_then(|r| 
                    match &*r.0.as_ptr() {
                        Symbol::Enum(ref e) => Some(e),
                        _ => None,
                    })
        }
    }

    /// Gets the services defined in this file
    pub fn services(&self) -> &[ServiceRef<'a>] {
        &self.services
    }

    pub fn find_service_by_name(&self, name: &str) -> Option<&ServiceDescriptor<'a>> {
        unsafe {   
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::File(Ref::from(self))))
                .and_then(|r| match &*r.0.as_ptr() {
                    Symbol::Service(ref s) => Some(s),
                    _ => None,
                })
        }
    }

    /// Gets the top-level extensions defined in this file
    pub fn extensions(&self) -> &[FieldRef<'a>] {
        &self.extensions
    }

    pub fn find_extension_by_name(&self, name: &str) -> Option<&FieldDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::File(Ref::from(self))))
                .and_then(|r| match &*r.0.as_ptr() {
                    Symbol::Field(ref f) => Some(f),
                    _ => None,
                })
        }
    }

    pub fn options(&self) -> Option<&'a FileOptions> {
        self.proto().options()
    }

    pub fn syntax(&self) -> Syntax {
        self.syntax
    }

    fn new(proto: &'a FileDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>, code_info: Option<GeneratedCodeInfo<'a>>) -> Result<(), PoolError<'a>> {
        let file = FileDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            dependencies: Box::default(),
            public_dependencies: Box::default(),
            messages: Box::default(),
            enums: Box::default(),
            services: Box::default(),
            extensions: Box::default(),
            syntax: match proto.syntax_option().map(AsRef::as_ref) {
                None | Some("proto2") => Syntax::Proto2,
                Some("proto3") => Syntax::Proto3,
                _ => Syntax::Unknown
            }
        };
        symbol_vec.push(Symbol::File(file));

        let (structs, fields) = code_info.map(|GeneratedCodeInfo { structs, extensions }| (structs, extensions)).unwrap_or((None, None));

        for (msg, info) in proto.message_type().iter().zip(structs.map(Vec::from).into_iter().flatten().map(Some).chain(iter::repeat_with(|| None))) {
            MessageDescriptor::new(msg, symbol_db, symbol_vec, info)?;
        }

        for enum_type in proto.enum_type().iter() {
            EnumDescriptor::new(enum_type, symbol_db, symbol_vec)?;
        }

        for service in proto.service().iter() {
            ServiceDescriptor::new(service, symbol_db, symbol_vec)?;
        }

        for (extension, accessor) in proto.extension().iter().zip(fields.map(Vec::from).into_iter().flatten().map(Some).chain(iter::repeat_with(|| None))) {
            FieldDescriptor::new(extension, symbol_db, symbol_vec, accessor)?;
        }

        Ok(())
    }

    fn map_symbols<'b>(&'b mut self, self_ref: RefSymbol<'a>, symbols: &mut impl Iterator<Item = &'b mut Symbol<'a>>) -> Result<(), PoolError<'a>> {
        if let Some(conflict) = self.db.full_map.borrow_mut().insert(Box::from(self.proto().name()), Ref::clone(&self_ref)) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        let mut messages = Vec::new();
        for (i, proto) in self.proto().message_type().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Message(m) => {
                    messages.push(Ref::from(m));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    m.map_symbols(Ref::clone(&symbol_ref), symbols, CompositeScope::File(Ref::from(self)), i)?;
                },
                _ => unreachable!()
            }
        }
        self.messages = messages.into_boxed_slice();

        let mut enums = Vec::new();
        for (i, proto) in self.proto().enum_type().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Enum(e) => {
                    enums.push(Ref::from(e));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    e.map_symbols(Ref::clone(&symbol_ref), symbols, CompositeScope::File(Ref::from(self)), i)?;
                },
                _ => unreachable!()
            }
        }
        self.enums = enums.into_boxed_slice();

        let mut services = Vec::new();
        for (i, proto) in self.proto().service().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Service(s) => {
                    services.push(Ref::from(s));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    s.map_symbols(Ref::clone(&symbol_ref), symbols, Ref::from(self), i)?;
                },
                _ => unreachable!()
            }
        }
        self.services = services.into_boxed_slice();

        let mut extensions = Vec::new();
        for (i, proto) in self.proto().extension().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Field(f) => {
                    extensions.push(Ref::from(f));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    f.map_symbols(Ref::clone(&symbol_ref), CompositeScope::File(Ref::from(self)), i, FieldScope::File(Ref::from(self)))?;
                },
                _ => unreachable!()
            }
        }
        self.extensions = extensions.into_boxed_slice();

        Ok(())
    }

    fn cross_ref(&mut self) -> Result<(), PoolError<'a>> {
        self.dependencies = 
            self.proto().dependency().iter().map(|s| {
                match self.db.find_full_symbol(s).map(SymbolRef::from) {
                    Some(SymbolRef::File(f)) => Ok(f),
                    _ => Err(PoolError::MissingSymbol(Cow::Borrowed(s)))
                }
            }).collect::<Result<_, _>>()?;

        self.public_dependencies = 
            self.proto().public_dependency().iter().enumerate().map(|(i, e)| {
                Ok(Ref::clone(
                    self.dependencies()
                        .get(
                            usize::try_from(*e)
                                .map_err(|_| PoolError::Validation(ValidationError::new(self.proto, 10, Some(i), Box::from("index of public dependency was negative"))))?)
                        .ok_or_else(|| PoolError::Validation(ValidationError::new(self.proto, 10, Some(i), Box::from("index of public dependency was out of range"))))?))
            }).collect::<Result<_, _>>()?;

        Ok(())
    }
}

impl<'a> Descriptor<'a> for FileDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self
    }
}

impl Debug for FileDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("FileDescriptor")
            .field("name", &self.name())
            .field("package", &self.package())
            .field("syntax", &self.syntax())
            .field(
                "public_dependencies",
                &self.public_dependencies().iter().map(|d| d.name()),
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
#[derive(Debug, PartialEq)]
pub enum CompositeScope<'a> {
    /// A file scope
    File(FileRef<'a>),
    /// A message scope
    Message(MessageRef<'a>),
}

impl<'a> CompositeScope<'a> {
    pub fn file(&self) -> &FileDescriptor<'a> {
        let mut scope = self;
        loop {
            match scope {
                CompositeScope::Message(ref m) => scope = m.scope(),
                CompositeScope::File(ref f) => return f,
            }
        }
    }
}

/// A message descriptor
pub struct MessageDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a DescriptorProto,
    new: Option<fn() -> Box<dyn AnyValue<'a>>>,
    scope: CompositeScope<'a>,
    scope_index: usize,
    full_name: Box<str>,
    fields: Box<[FieldRef<'a>]>,
    fields_ordered: Box<[FieldRef<'a>]>,
    message_fields: Box<[FieldRef<'a>]>,
    extensions: Box<[FieldRef<'a>]>,
    messages: Box<[MessageRef<'a>]>,
    enums: Box<[EnumRef<'a>]>,
    oneofs: Box<[OneofRef<'a>]>,
    info: Option<SourceCodeInfo<'a>>,
}

impl<'a> MessageDescriptor<'a> {
    pub fn proto(&self) -> &'a DescriptorProto {
        self.proto
    }

    /// Gets the scope this descriptor was defined in
    pub fn scope(&self) -> &CompositeScope<'a> {
        &self.scope
    }

    /// Gets the index of this descriptor in its parent descriptor
    pub fn scope_index(&self) -> usize {
        self.scope_index
    }

    /// Creates a new instance of the type represented by this descriptor.
    ///
    /// Generated pools will create an instance of the message as its defined in the binary,
    /// while other pools will create a dynamic instance of the message.
    ///
    /// Map entries don't return an instance
    pub fn new_instance<'b: 'a>(&'b self) -> Option<Box<dyn AnyValue<'b> + 'b>> {
        if !self.is_map_entry() {
            if let Some(new) = self.new {
                Some((new)())
            } else {
                Some(Box::new(dynamic::DynamicMessage::new(self)))
            }
        } else {
            None
        }
    }

    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    pub fn fields(&self) -> &[FieldRef<'a>] {
        &self.fields
    }

    pub fn fields_ordered(&self) -> &[FieldRef<'a>] {
        &self.fields_ordered
    }

    pub fn find_field_by_name(&self, name: &str) -> Option<&FieldDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Message(Ref::from(self))))
                .and_then(|r| match &*r.0.as_ptr() {
                    Symbol::Field(ref r) if !r.is_extension() => Some(r),
                    _ => None,
                })
        }
    }

    pub fn find_field_by_number(&self, num: FieldNumber) -> Option<&FieldDescriptor<'a>> {
        unsafe {
            self.db.num_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(num.get() as i32, Ref::from(self)))
                .and_then(|r| if !r.is_extension() { Some(&*r.0.as_ptr()) } else { None })
        }
    }

    /// This doesn't filter out extension fields and is used in dynamic messages. 
    /// Check `find_extension_by_number` for info about why this is safe
    pub(crate) fn find_any_field_by_number(&self, num: FieldNumber) -> Option<&FieldDescriptor<'a>> {
        unsafe {
            self.db.num_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(num.get() as i32, Ref::from(self)))
                .map(|r| &**r)
        }
    }

    /// this only returns extension fields and is used in dynamic messages; it works with dynamic messages since:
    /// 
    /// 1. pools with backups are only valid for static generated pools, which won't make dynamic messages with new_instance
    /// 
    /// 2. pools that create dynamic messages can't use backups, so every field will be included in our index db's num_symbol_map
    pub(crate) fn find_extension_by_number(&self, num: FieldNumber) -> Option<&FieldDescriptor<'a>> {
        unsafe {
            self.db.num_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(num.get() as i32, Ref::from(self)))
                .and_then(|r| if !r.is_extension() { Some(&*r.0.as_ptr()) } else { None })
        }
    }

    /// Gets all the fields in this message except those contained within oneofs
    pub fn message_fields(&self) -> &[FieldRef<'a>] {
        &self.message_fields
    }

    pub fn extensions(&self) -> &[FieldRef<'a>] {
        &self.extensions
    }

    pub fn messages(&self) -> &[MessageRef<'a>] {
        &self.messages
    }

    pub fn find_message_by_name(&self, name: &str) -> Option<&MessageDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Message(Ref::from(self))))
                .and_then(|r| match &*r.0.as_ptr() {
                    Symbol::Message(ref r) => Some(r),
                    _ => None,
                })
        }
    }

    /// Flattens the submessages declared in this message
    pub fn flatten_messages<'b>(
        &'b self,
    ) -> impl Iterator<Item = &'b MessageDescriptor<'a>> {
        self.messages.iter().map(|r| &**r).chain(
            self.messages
                .iter()
                .flat_map::<Box<dyn Iterator<Item = &'b MessageDescriptor<'a>> + 'b>, _>(|m| Box::new(m.flatten_messages())),
        )
    }

    pub fn enums(&self) -> &[EnumRef<'a>] {
        &self.enums
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&EnumDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Message(Ref::from(self))))
                .and_then(|r| match &*r.0.as_ptr() {
                    Symbol::Enum(ref r) => Some(r),
                    _ => None,
                })
        }
    }

    pub fn oneofs(&self) -> &[OneofRef<'a>] {
        &self.oneofs
    }

    pub fn find_oneof_by_name(&self, name: &str) -> Option<&OneofDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Message(Ref::from(self))))
                .and_then(|r| match &*r.0.as_ptr() {
                    Symbol::Oneof(ref r) => Some(r),
                    _ => None,
                })
        }
    }

    pub fn options(&self) -> Option<&'a MessageOptions> {
        self.proto().options()
    }

    /// Creates a new string with the full name of this descriptor
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    pub fn is_map_entry(&self) -> bool {
        if let Some(options) = self.options() {
            options.map_entry()
        } else {
            false
        }
    }

    fn new(proto: &'a DescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>, code_info: Option<GeneratedStructInfo<'a>>) -> Result<(), PoolError<'a>> {
        let (new, structs, fields, extensions) = code_info.map(|GeneratedStructInfo { new, structs, fields, extensions }| (Some(new), structs, fields, extensions)).unwrap_or_else(|| (None, None, None, None));

        let message = MessageDescriptor {
            db: Arc::clone(&symbol_db),
            proto,
            new,
            scope: CompositeScope::File(Ref::dangling()),
            scope_index: 0,
            full_name: Box::default(),
            fields: Box::default(),
            fields_ordered: Box::default(),
            message_fields: Box::default(),
            extensions: Box::default(),
            messages: Box::default(),
            enums: Box::default(),
            oneofs: Box::default(),
            info: None
        };
        symbol_vec.push(Symbol::Message(message));

        for (proto, accessor) in proto.field().iter().zip(fields.map(Vec::from).into_iter().flatten().map(Some).chain(iter::repeat_with(|| None))) {
            FieldDescriptor::new(proto, symbol_db, symbol_vec, accessor)?;
        }

        for (proto, accessor) in proto.extension().iter().zip(extensions.map(Vec::from).into_iter().flatten().map(Some).chain(iter::repeat_with(|| None))) {
            FieldDescriptor::new(proto, symbol_db, symbol_vec, accessor)?;
        }

        let mut iter = structs.map(Vec::from).into_iter().flatten();
        for proto in proto.nested_type().iter() {
            let code_info =
                if !proto.options().map(MessageOptions::map_entry).unwrap_or_default() {
                    iter.next()
                } else {
                    None
                };
            MessageDescriptor::new(proto, symbol_db, symbol_vec, code_info)?;
        }

        for enum_type in proto.enum_type().iter() {
            EnumDescriptor::new(enum_type, symbol_db, symbol_vec)?;
        }

        for oneof in proto.oneof_decl().iter() {
            OneofDescriptor::new(oneof, symbol_db, symbol_vec)?;
        }

        Ok(())
    }

    fn map_symbols<'b>(&'b mut self, self_ref: RefSymbol<'a>, symbols: &mut impl Iterator<Item = &'b mut Symbol<'a>>, scope: CompositeScope<'a>, scope_index: usize) -> Result<(), PoolError<'a>> {
        self.scope = scope;
        self.scope_index = scope_index;
        self.full_name = get_full_name(self.name(), self.scope());

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), Ref::clone(&self_ref)) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        let mut fields = Vec::new();
        for (i, proto) in self.proto().field().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Field(f) => {
                    fields.push(Ref::from(f));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    f.map_symbols(Ref::clone(&symbol_ref), CompositeScope::Message(Ref::from(self)), i, FieldScope::Message(Ref::from(self)))?;
                },
                _ => unreachable!()
            }
        }
        self.fields = fields.into_boxed_slice();

        self.fields_ordered = self.fields.iter().map(Ref::clone).collect();
        self.fields_ordered.sort_unstable_by_key(|f| f.number());

        self.message_fields = self.fields.iter().filter(|f| !f.proto().has_oneof_index()).map(Ref::clone).collect();

        let mut extensions = Vec::new();
        for (i, proto) in self.proto().extension().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Field(f) => {
                    extensions.push(Ref::from(f));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    f.map_symbols(Ref::clone(&symbol_ref), CompositeScope::Message(Ref::from(self)), i, FieldScope::Message(Ref::from(self)))?;
                },
                _ => unreachable!()
            }
        }
        self.extensions = extensions.into_boxed_slice();

        let mut nested_types = Vec::new();
        for (i, proto) in self.proto().nested_type().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Message(m) => {
                    nested_types.push(Ref::from(m));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    m.map_symbols(Ref::clone(&symbol_ref), symbols, CompositeScope::Message(Ref::from(self)), i)?;
                },
                _ => unreachable!()
            }
        }
        self.messages = nested_types.into_boxed_slice();

        let mut nested_enum = Vec::new();
        for (i, proto) in self.proto().enum_type().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Enum(e) => {
                    nested_enum.push(Ref::from(e));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    e.map_symbols(Ref::clone(&symbol_ref), symbols, CompositeScope::Message(Ref::from(self)), i)?;
                },
                _ => unreachable!()
            }
        }
        self.enums = nested_enum.into_boxed_slice();

        let mut oneofs = Vec::new();
        for (i, proto) in self.proto().oneof_decl().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Oneof(o) => {
                    oneofs.push(Ref::from(o));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    o.map_symbols(Ref::clone(&symbol_ref), Ref::from(self), i)?;
                },
                _ => unreachable!()
            }
        }
        self.oneofs = oneofs.into_boxed_slice();

        Ok(())
    }
}

impl<'a> Descriptor<'a> for MessageDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self.scope().file()
    }
}

impl Debug for MessageDescriptor<'_> {
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

pub struct EnumDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a EnumDescriptorProto,
    scope: CompositeScope<'a>,
    scope_index: usize,
    full_name: Box<str>,
    values: Box<[EnumValueRef<'a>]>,
    new_from: Option<fn(i32) -> Box<dyn AnyValue<'a>>>,
    info: Option<SourceCodeInfo<'a>>,
}

impl<'a> EnumDescriptor<'a> {
    pub fn proto(&self) -> &'a EnumDescriptorProto {
        self.proto
    }

    pub fn scope(&self) -> &CompositeScope<'a> {
        &self.scope
    }

    pub fn scope_index(&self) -> usize {
        self.scope_index
    }

    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn new_from<'b: 'a>(&'b self, value: i32) -> Box<dyn AnyValue<'b> + 'b> {
        self.new_from.map_or_else::<Box<dyn AnyValue<'b> + 'b>, _, _>(
            move || Box::new(dynamic::DynamicEnum::new(self, value)),
            move |f| (f)(value))
    }

    pub fn values(&self) -> &[EnumValueRef<'a>] {
        &self.values
    }

    pub fn find_enum_value_by_name(&self, name: &str) -> Option<&EnumValueDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Enum(Ref::from(self))))
                .map(|r| (&*r.0.as_ptr()).unwrap())
        }
    }

    pub fn options(&self) -> Option<&'a EnumOptions> {
        self.proto().options()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    fn new(proto: &'a EnumDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>) -> Result<(), PoolError<'a>> {
        let enum_type = EnumDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            scope: CompositeScope::File(Ref::dangling()),
            scope_index: 0,
            full_name: Box::default(),
            values: Box::default(),
            new_from: None,
            info: None
        };
        symbol_vec.push(Symbol::Enum(enum_type));

        for proto in proto.value().iter() {
            EnumValueDescriptor::new(proto, symbol_db, symbol_vec)?;
        }

        Ok(())
    }

    fn map_symbols<'b>(&'b mut self, self_ref: RefSymbol<'a>, symbols: &mut impl Iterator<Item = &'b mut Symbol<'a>>, scope: CompositeScope<'a>, scope_index: usize) -> Result<(), PoolError<'a>> {
        self.scope = scope;
        self.scope_index = scope_index;
        self.full_name = get_full_name(self.name(), self.scope());

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), Ref::clone(&self_ref)) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        let mut values = Vec::new();
        for (i, proto) in self.proto().value().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::EnumValue(v) => {
                    values.push(Ref::from(v));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    v.map_symbols(Ref::clone(&symbol_ref), Ref::from(self), i)?;
                },
                _ => unreachable!()
            }
        }
        self.values = values.into_boxed_slice();

        Ok(())
    }
}

impl<'a> Descriptor<'a> for EnumDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self.scope().file()
    }
}

impl Debug for EnumDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("EnumDescriptor")
            .field("name", &self.name())
            .field("values", &self.values())
            .finish()
    }
}

pub struct EnumValueDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a EnumValueDescriptorProto,
    enum_type: EnumRef<'a>,
    enum_type_index: usize,
    full_name: Box<str>,
    info: Option<SourceCodeInfo<'a>>,
}

impl<'a> EnumValueDescriptor<'a> {
    pub fn proto(&self) -> &'a EnumValueDescriptorProto {
        self.proto
    }

    pub fn enum_type(&self) -> &EnumDescriptor<'a> {
        &self.enum_type
    }

    /// Gets the index of this enum value in its parent enum
    pub fn index(&self) -> usize {
        self.enum_type_index
    }

    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn number(&self) -> i32 {
        self.proto().number()
    }

    pub fn options(&self) -> Option<&'a EnumValueOptions> {
        self.proto().options()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    fn new(proto: &'a EnumValueDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>) -> Result<(), PoolError<'a>> {
        let value = EnumValueDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            enum_type: Ref::dangling(),
            enum_type_index: 0,
            full_name: Box::default(),
            info: None
        };
        symbol_vec.push(Symbol::EnumValue(value));
        Ok(())
    }

    fn map_symbols(&mut self, self_ref: RefSymbol<'a>, enum_type: EnumRef<'a>, enum_type_index: usize) -> Result<(), PoolError<'a>> {
        self.enum_type = enum_type;
        self.enum_type_index = enum_type_index;
        self.full_name = format!("{}.{}", self.enum_type().full_name(), self.name()).into_boxed_str();

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), self_ref) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        Ok(())
    }
}

impl<'a> Descriptor<'a> for EnumValueDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self.enum_type().file()
    }
}

impl Debug for EnumValueDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("EnumValueDescriptor")
            .field("name", &self.name())
            .field("number", &self.number())
            .finish()
    }
}

pub struct ServiceDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a ServiceDescriptorProto,
    full_name: Box<str>,
    file: FileRef<'a>,
    file_index: usize,
    methods: Box<[MethodRef<'a>]>,
    info: Option<SourceCodeInfo<'a>>,
}

impl<'a> ServiceDescriptor<'a> {
    pub fn proto(&self) -> &'a ServiceDescriptorProto {
        self.proto
    }

    pub fn file(&self) -> &FileDescriptor<'a> {
        &self.file
    }

    pub fn index(&self) -> usize {
        self.file_index
    }

    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn methods(&self) -> &[MethodRef<'a>] {
        &self.methods
    }

    pub fn find_method_by_name(&self, name: &str) -> Option<&MethodDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Service(Ref::from(self))))
                .map(|r| (&*r.0.as_ptr()).unwrap())
        }
    }

    pub fn options(&self) -> Option<&'a ServiceOptions> {
        self.proto().options()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    fn new(proto: &'a ServiceDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>) -> Result<(), PoolError<'a>> {
        let service = ServiceDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            full_name: Box::default(),
            file: Ref::dangling(),
            file_index: 0,
            methods: Box::default(),
            info: None,
        };
        symbol_vec.push(Symbol::Service(service));

        for proto in proto.method().iter() {
            MethodDescriptor::new(proto, symbol_db, symbol_vec)?;
        }

        Ok(())
    }

    fn map_symbols<'b>(&'b mut self, self_ref: RefSymbol<'a>, symbols: &mut impl Iterator<Item = &'b mut Symbol<'a>>, file: FileRef<'a>, file_index: usize) -> Result<(), PoolError<'a>> {
        self.file = file;
        self.file_index = file_index;
        self.full_name = format!("{}.{}", self.file().name(), self.name()).into_boxed_str();

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), Ref::clone(&self_ref)) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        let mut methods = Vec::new();
        for (i, proto) in self.proto().method().iter().enumerate() {
            let symbol = symbols.next().unwrap();
            let symbol_ref = Ref::from(symbol);
            match symbol {
                Symbol::Method(m) => {
                    methods.push(Ref::from(m));
                    if let Some(conflict) = self.db.str_symbol_map.borrow_mut().insert((proto.name(), Ref::clone(&self_ref).into()), Ref::clone(&symbol_ref)) {
                        return Err(PoolError::Conflict(ConflictError::new(proto, conflict.proto(), ConflictKind::Name)))
                    }
                    m.map_symbols(Ref::clone(&symbol_ref), Ref::from(self), i)?;
                },
                _ => unreachable!()
            }
        }
        self.methods = methods.into_boxed_slice();

        Ok(())
    }
}

impl<'a> Descriptor<'a> for ServiceDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self.file()
    }
}

impl Debug for ServiceDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("ServiceDescriptor")
            .field("name", &self.name())
            .field("methods", &self.methods())
            .finish()
    }
}

pub struct MethodDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a MethodDescriptorProto,
    full_name: Box<str>,
    service: ServiceRef<'a>,
    service_index: usize,
    input_type: MessageRef<'a>,
    output_type: MessageRef<'a>,
    info: Option<SourceCodeInfo<'a>>,
}

impl<'a> MethodDescriptor<'a> {
    pub fn proto(&self) -> &'a MethodDescriptorProto {
        self.proto
    }

    pub fn service(&self) -> &ServiceDescriptor<'a> {
        &self.service
    }

    pub fn index(&self) -> usize {
        self.service_index
    }

    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn input_type(&self) -> &MessageDescriptor<'a> {
        &self.input_type
    }

    pub fn output_type(&self) -> &MessageDescriptor<'a> {
        &self.output_type
    }

    pub fn client_streaming(&self) -> bool {
        self.proto().client_streaming()
    }

    pub fn server_streaming(&self) -> bool {
        self.proto().server_streaming()
    }

    pub fn options(&self) -> Option<&'a MethodOptions> {
        self.proto().options()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    fn new(proto: &'a MethodDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>) -> Result<(), PoolError<'a>> {
        let method = MethodDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            full_name: Box::default(),
            service: Ref::dangling(),
            service_index: 0,
            input_type: Ref::dangling(),
            output_type: Ref::dangling(),
            info: None
        };
        symbol_vec.push(Symbol::Method(method));

        Ok(())
    }

    fn map_symbols(&mut self, self_ref: RefSymbol<'a>, service: ServiceRef<'a>, service_index: usize) -> Result<(), PoolError<'a>> {
        self.service = service;
        self.service_index = service_index;
        self.full_name = format!("{}.{}", self.service().full_name(), self.name()).into_boxed_str();

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), self_ref) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        Ok(())
    }

    fn cross_ref(&mut self) -> Result<(), PoolError<'a>> {
        self.input_type = 
            match SymbolRef::from(self.db.find_symbol(self.proto().input_type(), SymbolRef::Method(Ref::from(self)))?) {
                SymbolRef::Message(m) => m,
                _ => return Err(PoolError::MissingSymbol(Cow::Borrowed(self.proto().input_type())))
            };
        self.output_type = 
            match SymbolRef::from(self.db.find_symbol(self.proto().output_type(), SymbolRef::Method(Ref::from(self)))?) {
                SymbolRef::Message(m) => m,
                _ => return Err(PoolError::MissingSymbol(Cow::Borrowed(self.proto().output_type())))
            };
        Ok(())
    }
}

impl<'a> Descriptor<'a> for MethodDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self.service().file()
    }
}

impl Debug for MethodDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("MethodDescriptor")
            .field("name", &self.name())
            .field("input_type", &self.input_type().full_name())
            .field("output_type", &self.output_type().full_name())
            .finish()
    }
}

#[derive(PartialEq)]
pub enum FieldType<'a> {
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
    Enum(EnumRef<'a>),
    Message(MessageRef<'a>),
    Group(MessageRef<'a>),
}

impl FieldType<'_> {
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

    #[inline]
    pub fn is_message(&self) -> bool {
        match self {
            FieldType::Message(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_group(&self) -> bool {
        match self {
            FieldType::Group(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_enum(&self) -> bool {
        match self {
            FieldType::Enum(_) => true,
            _ => false,
        }
    }
}

impl Debug for FieldType<'_> {
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

pub enum DefaultValue<'a> {
    /// There was no specified default value
    None,
    Bool(bool),
    Double(f64),
    SignedInt(i64),
    UnsignedInt(u64),
    Enum(EnumValueRef<'a>),
    String(String),
    Bytes(Vec<u8>),
}

impl Debug for DefaultValue<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            DefaultValue::None => fmt.write_str("None"),
            DefaultValue::Bool(b) => fmt.write_fmt(format_args!("Bool({})", b)),
            DefaultValue::Double(d) => fmt.write_fmt(format_args!("Double({})", d)),
            DefaultValue::SignedInt(s) => fmt.write_fmt(format_args!("SignedInt({})", s)),
            DefaultValue::UnsignedInt(u) => fmt.write_fmt(format_args!("UnsignedInt({})", u)),
            DefaultValue::Enum(e) => fmt.write_fmt(format_args!("Enum({})", e.full_name())),
            DefaultValue::String(s) => fmt.write_fmt(format_args!("String({})", s)),
            DefaultValue::Bytes(b) => fmt.write_fmt(format_args!("Bytes({:?})", b)),
        }
    }
}

trait AnyExtension<'a> {
    fn into_accessor<'b>(&'b self) -> access::FieldAccessor<'a, 'b>;
}

pub struct FieldDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a FieldDescriptorProto,
    full_name: Box<str>,
    number: FieldNumber,
    scope: CompositeScope<'a>,
    scope_index: usize,
    field_scope: FieldScope<'a>,
    value_type: FieldType<'a>,
    default_value: DefaultValue<'a>,
    message: MessageRef<'a>,
    info: Option<SourceCodeInfo<'a>>,
    accessor: Option<access::FieldAccessor<'a, 'a>>,
}

impl<'a> FieldDescriptor<'a> {
    pub fn proto(&self) -> &'a FieldDescriptorProto {
        self.proto
    }

    pub fn name(&self) -> &'a str {
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

    pub fn field_type(&self) -> &FieldType<'a> {
        &self.value_type
    }

    pub fn default_value(&self) -> &DefaultValue<'a> {
        &self.default_value
    }

    pub fn json_name(&self) -> Option<&'a String> {
        self.proto().json_name_option()
    }

    pub fn composite_scope(&self) -> &CompositeScope<'a> {
        &self.scope
    }

    pub fn composite_scope_index(&self) -> usize {
        self.scope_index
    }

    pub fn scope(&self) -> &FieldScope<'a> {
        &self.field_scope
    }

    pub fn accessor<'b>(&'b self) -> Option<access::FieldAccessor<'a, 'b>> {
        if !self.message().is_map_entry() {
            self.accessor.or_else(|| {
                Some(match (self.label(), self.field_type()) {
                    (FieldLabel::Optional, _) | (FieldLabel::Required, _) => access::FieldAccessor::Single(self),
                    (FieldLabel::Repeated, FieldType::Message(m)) if m.is_map_entry() => access::FieldAccessor::Map(self),
                    (FieldLabel::Repeated, _) => access::FieldAccessor::Repeated(self),
                })
            })
        } else {
            None
        }
    }

    pub fn options(&self) -> Option<&'a FieldOptions> {
        self.proto().options()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    pub fn is_extension(&self) -> bool {
        self.proto().has_extendee()
    }

    pub fn is_packed(&self) -> bool {
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
        if self.is_packed() {
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
    pub fn message(&self) -> &MessageDescriptor<'a> {
        &self.message
    }

    fn new(proto: &'a FieldDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>, accessor: Option<access::FieldAccessor<'a, 'a>>) -> Result<(), PoolError<'a>> {
        let field = FieldDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            full_name: Box::default(),
            number: 
                FieldNumber::new(
                    proto.number()
                        .try_into()
                        .map_err(|_| PoolError::Validation(ValidationError::new(proto, 3, None, Box::from("invalid field number"))))?)
                        .ok_or_else(|| PoolError::Validation(ValidationError::new(proto, 3, None, Box::from("invalid field number"))))?,
            scope: CompositeScope::File(Ref::dangling()),
            scope_index: 0,
            field_scope: FieldScope::File(Ref::dangling()),
            value_type: FieldType::Bool,
            default_value: DefaultValue::None,
            message: Ref::dangling(),
            info: None,
            accessor
        };
        symbol_vec.push(Symbol::Field(field));

        Ok(())
    }

    fn map_symbols(&mut self, self_ref: RefSymbol<'a>, scope: CompositeScope<'a>, scope_index: usize, field_scope: FieldScope<'a>) -> Result<(), PoolError<'a>> {
        self.scope = scope;
        self.scope_index = scope_index;
        self.field_scope = field_scope;
        self.full_name = get_full_name(self.name(), self.composite_scope());

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), self_ref) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        Ok(())
    }

    fn cross_ref(&mut self) -> Result<(), PoolError<'a>> {
        self.message = 
            if let Some(extendee) = self.proto().extendee_option() {
                match self.db.find_symbol(extendee, SymbolRef::Field(Ref::from(self))).map(Into::<SymbolRef>::into)? {
                    SymbolRef::Message(m) => m,
                    _ => return Err(PoolError::MissingSymbol(Cow::Borrowed(extendee)))
                }
            } else {
                match self.composite_scope() {
                    CompositeScope::Message(m) => Ref::clone(m),
                    CompositeScope::File(f) => return Err(PoolError::Validation(ValidationError::new(f.proto(), 7, Some(self.composite_scope_index()), Box::from("cannot have non-extension field in file scope"))))
                }
            };
        
        if let Some(conflict) = self.db.num_symbol_map.borrow_mut().insert((self.proto().number(), Ref::clone(&self.message)), Ref::from(self)) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Number)))
        }

        if self.is_extension() {
            self.db.ext_symbol_map.borrow_mut().entry(Ref::clone(&self.message)).or_insert_with(HashSet::new).insert(Ref::from(self));
        }

        if let Some(oneof) = self.proto().oneof_index_option() {
            if self.is_extension() {
                return Err(PoolError::Validation(ValidationError::new(self.proto(), 9, None, Box::from("extension field can't be in oneof"))))
            } else {
                let index = usize::try_from(*oneof).map_err(|_| PoolError::Validation(ValidationError::new(self.proto(), 9, None, Box::from("negative oneof index"))))?;
                self.field_scope = FieldScope::Oneof(Ref::clone(&self.message().oneofs()[index]));
            }
        }

        self.value_type =
            match self.proto().r#type() {
                Defined(Type::Bool) => FieldType::Bool,
                Defined(Type::Bytes) => FieldType::Bytes,
                Defined(Type::Double) => FieldType::Double,
                Defined(Type::Enum) => {
                    match self.db.find_symbol(self.proto().type_name(), SymbolRef::Field(Ref::from(self))).map(Into::into)? {
                        SymbolRef::Enum(m) => FieldType::Enum(m),
                        _ => return Err(PoolError::MissingSymbol(Cow::Borrowed(self.proto().type_name())))
                    }
                },
                Defined(Type::Fixed32) => FieldType::Fixed32,
                Defined(Type::Fixed64) => FieldType::Fixed64,
                Defined(Type::Float) => FieldType::Float,
                Defined(Type::Group) => {
                    match self.db.find_symbol(self.proto().type_name(), SymbolRef::Field(Ref::from(self))).map(Into::into)? {
                        SymbolRef::Message(m) => FieldType::Group(m),
                        _ => return Err(PoolError::MissingSymbol(Cow::Borrowed(self.proto().type_name())))
                    }
                },
                Defined(Type::Int32) => FieldType::Int32,
                Defined(Type::Int64) => FieldType::Int64,
                Defined(Type::Message) => {
                    match self.db.find_symbol(self.proto().type_name(), SymbolRef::Field(Ref::from(self))).map(Into::into)? {
                        SymbolRef::Message(m) => FieldType::Message(m),
                        _ => return Err(PoolError::MissingSymbol(Cow::Borrowed(self.proto().type_name())))
                    }
                },
                Defined(Type::Sfixed32) => FieldType::Sfixed32,
                Defined(Type::Sfixed64) => FieldType::Sfixed64,
                Defined(Type::Sint32) => FieldType::Sint32,
                Defined(Type::Sint64) => FieldType::Sint64,
                Defined(Type::String) => FieldType::String,
                Defined(Type::Uint32) => FieldType::Uint32,
                Defined(Type::Uint64) => FieldType::Uint64,
                _ => return Err(PoolError::Validation(ValidationError::new(self.proto(), 5, None, Box::from("invalid field type"))))
            };

        self.default_value = 
            match self.proto().default_value_option() {
                Some(value) => {
                    match self.field_type() {
                        FieldType::Double => {
                            let value = 
                                if value.eq_ignore_ascii_case("NaN") {
                                    "NaN"
                                } else {
                                    value
                                };
                            DefaultValue::Double(value.parse::<f64>().map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?)
                        },
                        FieldType::Float => {
                            let value = 
                                if value.eq_ignore_ascii_case("NaN") {
                                    "NaN"
                                } else {
                                    value
                                };
                            DefaultValue::Double(value.parse::<f32>().map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?.into())
                        },
                        FieldType::Int64 | FieldType::Sfixed64 | FieldType::Sint64 => DefaultValue::SignedInt(value.parse::<i64>()
                            .map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?),
                        FieldType::Int32 | FieldType::Sfixed32 | FieldType::Sint32 => DefaultValue::SignedInt(value.parse::<i32>()
                            .map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?.into()),
                        FieldType::Uint64 | FieldType::Fixed64 => DefaultValue::UnsignedInt(value.parse::<u64>()
                            .map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?),
                        FieldType::Uint32 | FieldType::Fixed32 => DefaultValue::UnsignedInt(value.parse::<u32>()
                            .map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?.into()),
                        FieldType::Bool => DefaultValue::Bool(value.parse::<bool>()
                            .map_err(|e| PoolError::Validation(ValidationError::new(self.proto(), 7, None, e.to_string().into_boxed_str())))?),
                        FieldType::String => DefaultValue::String(Clone::clone(value)),
                        FieldType::Bytes => {
                            fn esc_lit<'a>(s: &FieldDescriptor<'a>, lit: &str) -> Result<u8, PoolError<'a>> {
                                match &lit[0..2] {
                                    "\\n" => Ok(b'\n'),
                                    "\\r" => Ok(b'\r'),
                                    "\\t" => Ok(b'\t'),
                                    "\\\"" => Ok(b'\"'),
                                    "\\\'" => Ok(b'\''),
                                    "\\\\" => Ok(b'\\'),
                                    _ => {
                                        fn fail<'a>(proto: &'a dyn AnyMessage<'static>, reason: &str) -> PoolError<'a> {
                                            PoolError::Validation(ValidationError::new(proto, 7, None, Box::from(reason)))
                                        }
                                        let mut chars = lit.chars();
                                        match chars.next() {
                                            Some('\\') => { },
                                            _ => return Err(fail(s.proto(), "expected \\"))
                                        }
                                        Ok((chars.next().ok_or_else(|| fail(s.proto(), "missing octal digit"))? as u8 - b'0') * 64
                                            + (chars.next().ok_or_else(|| fail(s.proto(), "missing octal digit"))? as u8 - b'0') * 8
                                            + (chars.next().ok_or_else(|| fail(s.proto(), "missing octal digit"))? as u8 - b'0'))
                                    }
                                }
                            }

                            let mut result = Vec::with_capacity(self.proto().default_value().len());
                            for (i, c) in self.proto().default_value().char_indices() {
                                match c {
                                    '\\' => result.push(esc_lit(&self, &self.proto().default_value()[i..])?),
                                    _ => result.push(c as u8),
                                }
                            }

                            DefaultValue::Bytes(result)
                        },
                        FieldType::Enum(e) => 
                            DefaultValue::Enum(
                                e.find_enum_value_by_name(value)
                                    .map(Ref::from)
                                    .ok_or_else(|| PoolError::MissingSymbol(Cow::Owned(format!("{}.{}", e.full_name(), value))))?),
                        _ => return Err(PoolError::Validation(ValidationError::new(self.proto(), 5, None, Box::from("invalid field type for default values"))))
                    }
                },
                _ => DefaultValue::None
            };

        Ok(())
    }
}

impl<'a> Descriptor<'a> for FieldDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        match self.scope() {
            FieldScope::File(f) => f,
            FieldScope::Message(m) => m.file(),
            FieldScope::Oneof(o) => o.message().file(),
        }
    }
}

impl Debug for FieldDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("FieldDescriptor")
            .field("label", &self.label())
            .field("name", &self.name())
            .field("number", &self.number())
            .field("field_type", &self.field_type())
            .field("default_value", &self.default_value())
            .finish()
    }
}

/// Gets the scope a field is defined in
#[derive(PartialEq)]
pub enum FieldScope<'a> {
    File(FileRef<'a>),
    Message(MessageRef<'a>),
    Oneof(OneofRef<'a>),
}

impl<'a> FieldScope<'a> {
    /// Gets the message that this field is in or none if this scope is an extension field defined in a file
    pub fn message(&self) -> Option<&MessageDescriptor<'a>> {
        match self {
            FieldScope::File(_) => None,
            FieldScope::Message(m) => Some(m),
            FieldScope::Oneof(o) => Some(o.message()),
        }
    }
}

pub struct OneofDescriptor<'a> {
    db: Arc<SymbolDatabase<'a>>,
    proto: &'a OneofDescriptorProto,
    full_name: Box<str>,
    message: MessageRef<'a>,
    message_index: usize,
    get_set_case: Option<fn(&dyn AnyMessage<'a>) -> Option<usize>>,
    fields: Box<[FieldRef<'a>]>,
    info: Option<SourceCodeInfo<'a>>,
}

impl<'a> OneofDescriptor<'a> {
    pub fn proto(&self) -> &'a OneofDescriptorProto {
        self.proto
    }

    pub fn message(&self) -> &MessageDescriptor<'a> {
        &self.message
    }

    pub fn message_index(&self) -> usize {
        self.message_index
    }

    pub fn name(&self) -> &'a str {
        self.proto().name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    /// Gets the set case of this oneof in the specified message. 
    /// If the message isn't an instance of the message that contains this oneof 
    /// or the oneof doesn't have a set field, this returns None
    pub fn get_set_case(&self, message: &dyn AnyMessage<'a>) -> Option<&FieldDescriptor<'a>> {
        if let Some(get_set_case) = self.get_set_case {
            get_set_case(message).map(|i| &*self.fields()[i])
        } else {
            if let Some(message) = dynamic::DynamicMessage::downcast_any_ref(message) {
                message.get_set_case(self)
            } else {
                None
            }
        }
    }

    pub fn fields(&self) -> &[FieldRef<'a>] {
        &self.fields
    }

    pub fn find_field_by_name<'b>(&'b self, name: &'b str) -> Option<&'b FieldDescriptor<'a>> {
        unsafe {
            self.db.str_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(name, SymbolRef::Message(Ref::clone(&self.message))))
                .and_then(|r| match r.deref() {
                    Symbol::Field(ref r) if {
                        match r.scope() {
                            FieldScope::Oneof(oneof) => *self == **oneof,
                            _ => false
                        }
                    } => {
                        Some(r)
                    }
                    _ => None,
                })
        }
    }

    pub fn find_field_by_number<'b>(&'b self, num: FieldNumber) -> Option<&'b FieldDescriptor<'a>> {
        unsafe {
            self.db.num_symbol_map
                .try_borrow_unguarded().expect(UNCHECKED_BORROW_MSG)
                .get(&(num.get() as i32, Ref::clone(&self.message)))
                .and_then(|r| 
                    match r.scope() {
                        FieldScope::Oneof(oneof) if *self == **oneof => Some(&**r),
                        _ => None
                    })
        }
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.info.as_ref()
    }

    fn new(proto: &'a OneofDescriptorProto, symbol_db: &Arc<SymbolDatabase<'a>>, symbol_vec: &mut Vec<Symbol<'a>>) -> Result<(), PoolError<'a>> {
        let oneof = OneofDescriptor {
            db: Arc::clone(symbol_db),
            proto,
            full_name: Box::default(),
            message: Ref::dangling(),
            message_index: 0,
            get_set_case: None,
            fields: Box::default(),
            info: None
        };
        symbol_vec.push(Symbol::Oneof(oneof));

        Ok(())
    }

    fn map_symbols(&mut self, self_ref: RefSymbol<'a>, message: MessageRef<'a>, message_index: usize) -> Result<(), PoolError<'a>> {
        self.message = message;
        self.message_index = message_index;
        self.full_name = format!("{}.{}", self.message().full_name(), self.name()).into_boxed_str();

        if let Some(conflict) = self.db.full_map.borrow_mut().insert(self.full_name.clone(), self_ref) {
            return Err(PoolError::Conflict(ConflictError::new(self.proto(), conflict.proto(), ConflictKind::Name)))
        }

        Ok(())
    }

    fn cross_ref(&mut self) -> Result<(), PoolError<'a>> {
        self.fields = 
            self.message()
                .fields()
                .iter()
                .filter_map(|f| {
                    if f.proto().has_oneof_index() { 
                        match usize::try_from(f.proto().oneof_index()) {
                            Ok(i) => {
                                if i == self.message_index {
                                    Some(Ok(Ref::clone(f)))
                                } else {
                                    None
                                }
                            },
                            Err(_) => {
                                Some(Err(PoolError::Validation(ValidationError::new(f.proto(), 9, None, Box::from("oneof index can't be negative")))))
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect::<Result<_,_>>()?;

        Ok(())
    }
}

impl<'a> Descriptor<'a> for OneofDescriptor<'a> {
    fn proto(&self) -> &'a dyn AnyMessage<'static> {
        self.proto()
    }
    fn name(&self) -> &str {
        self.name()
    }
    fn full_name(&self) -> &str {
        self.full_name()
    }
    fn file(&self) -> &FileDescriptor<'a> {
        self.message().file()
    }
}

impl Debug for OneofDescriptor<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(self.name())
    }
}