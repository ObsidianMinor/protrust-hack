use crate::{EnumValue, Enum, Codec, CodedMessage, LiteMessage};
use crate::io::{Tag, CodedInput, CodedOutput, InputResult, OutputResult};
use crate::collections::RepeatedField;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::marker::PhantomData;

/// A message with extensions
pub trait ExtensionMessage: LiteMessage {
    /// Gets the current registry in use by this message
    fn registry(&self) -> Option<&'static ExtensionRegistry>;

    /// Returns if the current registry in use by this message is equal to the specified registry
    #[inline]
    fn has_registry(&self, registry: Option<&'static ExtensionRegistry>) -> bool {
        match (self.registry(), registry) {
            (Some(r), Some(o)) => std::ptr::eq(r, o),
            (None, None) => true,
            _ => false
        }
    }

    /// Replaces the registry used by this message with another, returning the previous registry in use
    fn replace_registry(&mut self, extensions: Option<&'static ExtensionRegistry>) -> Option<&'static ExtensionRegistry>;

    /// Creates a new instance of the message using the specified extension registry
    #[inline]
    fn with_registry(registry: Option<&'static ExtensionRegistry>) -> Self {
        let mut instance = Self::new();
        instance.replace_registry(registry);
        instance
    }

    #[inline]
    fn read_new(input: &mut std::io::Read, registry: Option<&'static ExtensionRegistry>) -> InputResult<Self> {
        let mut reader = CodedInput::new(input).with_registry(registry);
        ExtensionMessage::read_new_from_input(&mut reader, registry)
    }

    #[inline]
    fn read_new_from_input(input: &mut CodedInput, registry: Option<&'static ExtensionRegistry>) -> InputResult<Self> {
        let mut instance = Self::with_registry(registry);
        instance.merge_from(input)?;
        Ok(instance)
    }

    /// Gets the value of the specified extension field or None if it is not registered in this message.
    fn field<V: Clone + PartialEq + PartialEq<D> + Debug + Sync, D: Debug + Sync>(&self, extension: &'static Extension<Self, V, D>) -> Option<&ExtensionField<V, D>>;
    /// Gets the value of the specified extension field mutably or None if it is not registered in this message.
    fn field_mut<V: Clone + PartialEq + PartialEq<D> + Debug + Sync, D: Debug + Sync>(&mut self, extension: &'static Extension<Self, V, D>) -> Option<&mut ExtensionField<V, D>>;
    /// Gets the value of the specified extension repeated field or None if it is not registered in this message.
    fn repeated_field<V: Clone + PartialEq + Debug + Sync + 'static>(&self, extension: &'static RepeatedExtension<Self, V>) -> Option<&RepeatedField<V>>;
    /// Gets the value of a specified extension repeated field mutably or None if it is not registered in this message.
    fn repeated_field_mut<V: Clone + PartialEq + Debug + Sync + 'static>(&mut self, extension: &'static RepeatedExtension<Self, V>) -> Option<&mut RepeatedField<V>>;
}

pub trait ExtensionIdentifier: Sync {
    fn new_value(&'static self) -> Box<dyn AnyExtension>;
    fn tag(&'static self) -> Tag;
}

pub trait AnyExtension: CodedMessage + Any + Debug + Sync {
    fn clone(&self) -> Box<dyn AnyExtension>;
    fn merge(&mut self, other: &dyn AnyExtension);
    fn eq(&self, other: &dyn AnyExtension) -> bool;
    fn tag(&self) -> Tag;
}

pub struct ExtensionRegistry(HashMap<TypeId, Vec<&'static dyn ExtensionIdentifier>>);

const EMPTY_EXTENSIONS: &[&'static dyn ExtensionIdentifier] = &[];

#[doc(hidden)]
impl ExtensionRegistry {
    pub fn new(registries: &[&'static ExtensionRegistry], extensions: &[(TypeId, &[&'static dyn ExtensionIdentifier])]) -> ExtensionRegistry {
        let mut registry = ExtensionRegistry(HashMap::new());
        registry.0.extend(extensions.iter().map(|(t, b)| (*t, b.to_vec())));
        let mut tag_set = std::collections::HashSet::new();
        for (typeid, values) in registry.0.iter_mut() {
            tag_set.extend(values.iter().map(|e| e.tag().number()));
            for extension in registries.iter().flat_map(|r| r.0.get(typeid)).flatten() {
                let tag = extension.tag();
                if tag_set.contains(&tag.number()) {
                    panic!("Extension conflict: an imported registry contained an extension with a field number that already exists in this registry (tag: {}, field number: {})", tag, tag.number());
                } else {
                    values.push(*extension);
                }
            }
            tag_set.clear();
            values.shrink_to_fit();
        }
        registry
    }

    fn for_type<T: 'static>(&self) -> &[&'static dyn ExtensionIdentifier] {
        self.0.get(&TypeId::of::<T>()).map_or(EMPTY_EXTENSIONS, |e| &**e)
    }
}

/// An extension identifier used to retreive an extension value
pub struct Extension<T, V, D = V> {
    t: PhantomData<T>,
    codec: Codec<V>,
    default: Option<D>
}

macro_rules! extension_value_factory {
    ($($t:ty => $($name:ident),+);*) => {
        $(
            #[doc(hidden)]
            impl<T> Extension<T, $t> {
                $(
                    pub const fn $name(tag: u32, default: $t) -> Extension<T, $t> {
                        Extension {
                            t: PhantomData,
                            codec: Codec::$name(tag),
                            default: Some(default)
                        }
                    }
                )+
            }
        )*
    };
}

extension_value_factory!(
    f32 => float;
    f64 => double; 
    i32 => int32, sint32, sfixed32;
    u32 => uint32, fixed32;
    i64 => int64, sint64, sfixed64;
    u64 => uint64, fixed64;
    bool => bool);

#[doc(hidden)]
impl<T> Extension<T, String, &'static str> {
    pub const fn string(tag: u32, default: &'static str) -> Extension<T, String, &'static str> {
        Extension {
            t: PhantomData,
            codec: Codec::string(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, Vec<u8>, &'static [u8]> {
    pub const fn bytes(tag: u32, default: &'static [u8]) -> Extension<T, Vec<u8>, &'static [u8]> {
        Extension {
            t: PhantomData,
            codec: Codec::bytes(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T, E: crate::Enum> Extension<T, EnumValue<E>> {
    pub const fn enum_value(tag: u32, default: EnumValue<E>) -> Extension<T, EnumValue<E>> {
        Extension {
            t: PhantomData,
            codec: Codec::enum_value(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T, M: LiteMessage> Extension<T, M> {
    pub const fn message(tag: u32) -> Extension<T, M> {
        Extension {
            t: PhantomData,
            codec: Codec::message(tag),
            default: None
        }
    }

    pub const fn group(tag: u32, end: u32) -> Extension<T, M> {
        Extension {
            t: PhantomData,
            codec: Codec::group(tag, end),
            default: None
        }
    }
}

impl<T: ExtensionMessage + Sync, V: Clone + PartialEq + PartialEq<D> + Debug + Sync, D: Debug + Sync> ExtensionIdentifier for Extension<T, V, D> {
    fn new_value(&'static self) -> Box<dyn AnyExtension> {
        Box::new(ExtensionField {
            value: None,
            default: &self.default,
            codec: &self.codec
        })
    }
    fn tag(&'static self) -> Tag {
        self.codec.tag()
    }
}

pub struct ExtensionField<V: 'static, D: 'static = V> {
    value: Option<V>,
    default: &'static Option<D>,
    codec: &'static Codec<V>
}

impl<V: PartialEq<D>, D> CodedMessage for ExtensionField<V, D> {
    fn merge_from(&mut self, input: &mut CodedInput) -> InputResult<()> {
        self.codec.merge_from(input, &mut self.value)
    }

    fn write_to(&self, output: &mut CodedOutput) -> OutputResult {
        if let Some(value) = &self.value {
            if !self.default.as_ref().map_or(false, |d| value == d) {
                output.write_tag(self.codec.tag())?;
                self.codec.write_to(output, value)?;
                if let Some(end_tag) = self.codec.end_tag() {
                    output.write_tag(end_tag)?;
                }
            }
        }
        Ok(())
    }

    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        self.value.as_ref().map_or(0, |v| self.codec.calculate_size(v))
    }

    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        self.value.as_ref().map_or(Some(0), |v| self.codec.calculate_size(v))
    }
}

impl<V: Clone + PartialEq + PartialEq<D> + Debug + Sync, D: Debug + Sync> AnyExtension for ExtensionField<V, D> {
    fn clone(&self) -> Box<dyn AnyExtension> {
        Box::new(ExtensionField {
            value: self.value.clone(),
            default: self.default,
            codec: self.codec
        })
    }
    fn merge(&mut self, other: &dyn AnyExtension) {
        if TypeId::of::<Self>() == other.type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                if std::ptr::eq(self.codec, other.codec) {
                    self.codec.merge_values(&mut self.value, &other.value);
                }
            }
        }
    }
    fn eq(&self, other: &dyn AnyExtension) -> bool {
        if TypeId::of::<Self>() == other.type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                std::ptr::eq(self.codec, other.codec) && self.value == other.value
            }
        } else {
            false
        }
    }
    fn tag(&self) -> Tag {
        self.codec.tag()
    }
}

impl<V: Debug, D: Debug> Debug for ExtensionField<V, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(ref value) => value.fmt(fmt),
            None => self.default.as_ref().fmt(fmt)
        }
    }
}

impl<V: 'static, D: 'static> ExtensionField<V, D> {
    pub fn get(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut V> {
        self.value.as_mut()
    }

    pub fn set(&mut self, value: V) {
        self.value = Some(value)
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn take(&mut self) -> Option<V> {
        self.value.take()
    }

    pub fn clear(&mut self) {
        self.value = None
    }
}

macro_rules! extension_field_or_default {
    ($($t:ty),*) => {
        $(impl ExtensionField<$t> {
            pub fn get_or_default(&self) -> $t {
                self.value.unwrap_or_else(|| self.default.unwrap())
            }
        })*
    };
}

macro_rules! extension_field_or_default_static {
    ($($t:ty, $t2:ty),*) => {
        $(impl ExtensionField<$t, &'static $t2> {
            pub fn get_or_default(&self) -> &$t2 {
                self.get().map(|v| &**v).unwrap_or_else(|| self.default.unwrap())
            }
        })*
    };
}

extension_field_or_default!(f32, f64, i32, u32, i64, u64);

impl<E: Enum> ExtensionField<EnumValue<E>> {
    pub fn get_or_default(&self) -> EnumValue<E> {
        self.value.clone().unwrap_or_else(|| self.default.clone().unwrap())
    }
}

extension_field_or_default_static!(String, str, Vec<u8>, [u8]);

pub struct RepeatedExtension<T, V> {
    t: PhantomData<T>,
    codec: Codec<V>
}

macro_rules! repeated_extension_value_factory {
    ($($t:ty => $($name:ident),+);*) => {
        $(
            #[doc(hidden)]
            impl<T> RepeatedExtension<T, $t> {
                $(
                    pub const fn $name(tag: u32) -> RepeatedExtension<T, $t> {
                        RepeatedExtension {
                            t: PhantomData,
                            codec: Codec::$name(tag)
                        }
                    }
                )+
            }
        )*
    };
}

repeated_extension_value_factory!(
    f32 => float;
    f64 => double;
    i32 => int32, sint32, sfixed32;
    u32 => uint32, fixed32;
    i64 => int64, sint64, sfixed64;
    u64 => uint64, fixed64;
    bool => bool);

#[doc(hidden)]
impl<T> RepeatedExtension<T, String> {
    pub const fn string(tag: u32) -> RepeatedExtension<T, String> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::string(tag)
        }
    }
}

#[doc(hidden)]
impl<T> RepeatedExtension<T, Vec<u8>> {
    pub const fn bytes(tag: u32) -> RepeatedExtension<T, Vec<u8>> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::bytes(tag)
        }
    }
}

#[doc(hidden)]
impl<T, E: crate::Enum> RepeatedExtension<T, EnumValue<E>> {
    pub const fn enum_value(tag: u32) -> RepeatedExtension<T, EnumValue<E>> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::enum_value(tag)
        }
    }
}

#[doc(hidden)]
impl<T, M: LiteMessage> RepeatedExtension<T, M> {
    pub const fn message(tag: u32) -> RepeatedExtension<T, M> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::message(tag)
        }
    }

    pub const fn group(tag: u32, end: u32) -> RepeatedExtension<T, M> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::group(tag, end)
        }
    }
}

#[doc(hidden)]
impl<T, M: ExtensionMessage> RepeatedExtension<T, M> {
    pub const fn extension_message(tag: u32) -> RepeatedExtension<T, M> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::extension_message(tag)
        }
    }
}

impl<T: ExtensionMessage + Sync, V: Clone + PartialEq + Debug + Sync + 'static> ExtensionIdentifier for RepeatedExtension<T, V> {
    fn new_value(&'static self) -> Box<dyn AnyExtension> {
        Box::new(RepeatedExtensionValue {
            value: RepeatedField::new(),
            codec: &self.codec
        })
    }
    fn tag(&'static self) -> Tag {
        self.codec.tag()
    }
}

#[doc(hidden)]
pub struct RepeatedExtensionValue<V: 'static> {
    value: RepeatedField<V>,
    codec: &'static Codec<V>
}

#[doc(hidden)]
impl<V: Clone + 'static> CodedMessage for RepeatedExtensionValue<V> {
    fn merge_from(&mut self, input: &mut CodedInput) -> InputResult<()> {
        self.value.add_entries(input, self.codec)
    }

    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        self.value.calculate_size(&self.codec)
    }

    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        self.value.calculate_size(&self.codec)
    }

    fn write_to(&self, output: &mut CodedOutput) -> OutputResult {
        self.value.write_to(output, self.codec)
    }
}

impl<V: Clone + PartialEq + Debug + Sync + 'static> AnyExtension for RepeatedExtensionValue<V> {
    fn clone(&self) -> Box<dyn AnyExtension> {
        Box::new(RepeatedExtensionValue {
            value: self.value.clone(),
            codec: self.codec
        })
    }
    fn merge(&mut self, other: &dyn AnyExtension) {
        if TypeId::of::<Self>() == other.type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                if std::ptr::eq(self.codec, other.codec) {
                    self.value.merge(&other.value);
                }
            }
        }
    }
    fn tag(&self) -> Tag {
        self.codec.tag()
    }
    fn eq(&self, other: &dyn AnyExtension) -> bool {
        if TypeId::of::<Self>() == other.type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                std::ptr::eq(self.codec, other.codec) && self.value == other.value
            }
        } else {
            false
        }
    }
}

impl<V: Debug> Debug for RepeatedExtensionValue<V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(fmt)
    }
}

#[doc(hidden)]
pub struct ExtensionSet<T> {
    t: PhantomData<T>,
    registry: Option<&'static ExtensionRegistry>,
    values_by_tag: HashMap<Tag, Box<dyn AnyExtension>>
}

impl<T: ExtensionMessage + Sync + 'static> ExtensionSet<T> {
    pub fn new() -> Self {
        Self {
            t: PhantomData,
            registry: None,
            values_by_tag: HashMap::new()
        }
    }

    pub fn registry(&self) -> Option<&'static ExtensionRegistry> {
        self.registry
    }

    pub fn replace_registry(&mut self, registry: Option<&'static ExtensionRegistry>) -> Option<&'static ExtensionRegistry> {
        self.values_by_tag.clear();

        if let Some(registry) = registry {
            for extension in registry.for_type::<T>() {
                let value = extension.new_value();
                self.values_by_tag.insert(extension.tag(), value);
            }
        }

        std::mem::replace(&mut self.registry, registry)
    }

    pub fn field<V: Clone + PartialEq + PartialEq<D> + Debug + Sync, D: Debug + Sync>(&self, extension: &'static Extension<T, V, D>) -> Option<&ExtensionField<V, D>> {
        unsafe {
            self.values_by_tag.get(&extension.tag())
                .map(|rc| &*(rc.as_ref() as *const dyn AnyExtension as *const ExtensionField<V, D>))
        }
    }

    pub fn field_mut<V: Clone + PartialEq + PartialEq<D> + Debug + Sync, D: Debug + Sync>(&mut self, extension: &'static Extension<T, V, D>) -> Option<&mut ExtensionField<V, D>> {
        unsafe {
            self.values_by_tag.get_mut(&extension.tag())
                .map(|rc| &mut *(rc.as_mut() as *mut dyn AnyExtension as *mut ExtensionField<V, D>))
        }
    }

    pub fn repeated_field<V: Clone + PartialEq + Debug + Sync + 'static>(&self, extension: &'static RepeatedExtension<T, V>) -> Option<&RepeatedField<V>> {
        unsafe {
            self.values_by_tag.get(&extension.tag())
                .map(|rc| &(*(rc.as_ref() as *const dyn AnyExtension as *const RepeatedExtensionValue<V>)).value)
        }
    }

    pub fn repeated_field_mut<V: Clone + PartialEq + Debug + Sync + 'static>(&mut self, extension: &'static RepeatedExtension<T, V>) -> Option<&mut RepeatedField<V>> {
        unsafe {
            self.values_by_tag.get_mut(&extension.tag())
                .map(|rc| &mut (*(rc.as_mut() as *mut dyn AnyExtension as *mut RepeatedExtensionValue<V>)).value)
        }
    }

    pub fn merge_from(&mut self, input: &mut CodedInput) -> InputResult<bool> {
        if let Some(last_tag) = input.last_tag() {
            if let Some(value) = self.values_by_tag.get_mut(&last_tag) {
                value.merge_from(input)?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    #[cfg(not(checked_size))]
    pub fn calculate_size(&self) -> i32 {
        self.values_by_tag
            .iter()
            .map(|(_, field)| field.calculate_size())
            .sum()
    }

    #[cfg(checked_size)]
    pub fn calculate_size(&self) -> Option<i32> {
        let mut len = 0;

        for (_, field) in self.values_by_tag.iter() {
            len.checked_add(field.calculate_size())?;
        }

        Some(len)
    }

    pub fn write_to(&self, output: &mut CodedOutput) -> OutputResult {
        for (_, field) in self.values_by_tag.iter() {
            field.write_to(output)?;
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) {
        match (self.registry(), other.registry()) {
            (None, None) => { },
            (Some(r), Some(o)) if std::ptr::eq(r, o) => {
                for (tag, value) in self.values_by_tag.iter_mut() {
                    value.merge(&**other.values_by_tag.get(tag).unwrap())
                }
            },
            _ => *self = other.clone(),
        }
    }
}

impl<T> Clone for ExtensionSet<T> {
    fn clone(&self) -> Self {
        let mut new_tag_map: HashMap<Tag, Box<dyn AnyExtension>> = HashMap::new();

        for (tag, value) in self.values_by_tag.iter() {
            new_tag_map.insert(*tag, value.as_ref().clone());
        }

        ExtensionSet {
            t: PhantomData,
            registry: self.registry,
            values_by_tag: new_tag_map,
        }
    }
}

impl<T> PartialEq for ExtensionSet<T> {
    fn eq(&self, other: &Self) -> bool {
        for (tag, value) in self.values_by_tag.iter() {
            if let Some(other_value) = other.values_by_tag.get(tag) {
                if !value.eq(&**other_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl<T> Debug for ExtensionSet<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut map = fmt.debug_map();
        for (tag, value) in &self.values_by_tag {
            map.entry(tag, value);
        }
        Ok(())
    }
}