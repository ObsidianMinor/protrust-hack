use crate::collections::RepeatedField;
use crate::io::{CodedInput, CodedOutput, InputResult, OutputResult, Tag};
use crate::{Codec, CodedMessage, Enum, EnumValue, LiteMessage};
use std::any::{Any, TypeId};
use std::collections::{hash_map::Entry, HashMap};
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::Deref;

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
            _ => false,
        }
    }

    /// Replaces the registry used by this message with another, returning the previous registry in use
    fn replace_registry(
        &mut self,
        extensions: Option<&'static ExtensionRegistry>,
    ) -> Option<&'static ExtensionRegistry>;

    /// Creates a new instance of the message using the specified extension registry
    #[inline]
    fn with_registry(registry: Option<&'static ExtensionRegistry>) -> Self {
        let mut instance = Self::new();
        instance.replace_registry(registry);
        instance
    }

    /// Gets if the specified extension is present in this extendable message
    fn has_extension<T: ExtensionIdentifier>(&self, extension: &'static T) -> bool;

    /// Gets if the specified extension is set in this extendable message by
    /// checking if its tag is contained in value storage, without checking
    /// to make sure the extension is present in the set registry
    fn has_extension_unchecked<T: ExtensionIdentifier>(&self, extension: &'static T) -> bool;

    /// Gets the value of the specified extension or
    /// None if the extension isn't set or isn't present in this message
    fn get_value<V: Clone + PartialEq + Debug + Send + Sync, D: Sync>(
        &self,
        extension: &'static Extension<Self, V, D>,
    ) -> Option<&V>;
    /// Gets the value of the specified extension or it's default or
    /// None if the extension doesn't have a default or it isn't present in this message
    fn get_value_or_default<
        V: Clone + PartialEq + Debug + Send + Sync + Deref<Target = L>,
        D: Sync + Deref<Target = L>,
        L,
    >(
        &self,
        extension: &'static Extension<Self, V, D>,
    ) -> Option<&L>;

    /// Gets the value of the specified repeated extension or None if the extension isn't set or isn't present in this message
    fn get_repeated_value<V: PartialEq + Clone + Debug + Send + Sync>(
        &self,
        extension: &'static RepeatedExtension<Self, V>,
    ) -> Option<&RepeatedField<V>>;

    /// Gets the value of the specified extension field or None if it is not registered in this message.
    fn field<V: Default + Clone + PartialEq + Debug + Send + Sync, D: Sync>(
        &mut self,
        extension: &'static Extension<Self, V, D>,
    ) -> Option<ExtensionField<Self, V, D>>;
    /// Gets the value of the specified extension repeated field or None if it is not registered in this message.
    fn repeated_field<V: Clone + PartialEq + Debug + Send + Sync>(
        &mut self,
        extension: &'static RepeatedExtension<Self, V>,
    ) -> Option<RepeatedExtensionField<Self, V>>;
}

pub struct ExtensionField<'a, T: 'static, V: 'static, D: 'static> {
    entry: Entry<'a, Tag, Box<dyn AnyExtension>>,
    extension: &'static Extension<T, V, D>,
}

impl<
        'a,
        T: ExtensionMessage,
        V: Clone + PartialEq + Default + Debug + Send + Sync + 'static,
        D: Send + Sync + 'static,
    > ExtensionField<'a, T, V, D>
{
    pub fn get(self) -> Option<&'a V> {
        unsafe {
            match self.entry {
                Entry::Occupied(o) => Some(
                    &(*(o.get().as_ref() as *const AnyExtension as *const ExtensionValue<V>)).value,
                ),
                _ => None,
            }
        }
    }

    pub fn get_mut(self) -> &'a mut V {
        unsafe {
            match self.entry {
                Entry::Occupied(mut o) => {
                    let ev =
                        &mut *(o.get_mut().as_mut() as *mut AnyExtension as *mut ExtensionValue<V>);
                    &mut ev.value
                }
                Entry::Vacant(v) => {
                    let ev = &mut *(v
                        .insert(self.extension.new_value(Default::default()))
                        .as_mut() as *mut AnyExtension
                        as *mut ExtensionValue<V>);
                    &mut ev.value
                }
            }
        }
    }

    pub fn has_value(&self) -> bool {
        match self.entry {
            Entry::Occupied(_) => true,
            Entry::Vacant(_) => false,
        }
    }

    pub fn set(self, value: V) {
        unsafe {
            match self.entry {
                Entry::Occupied(mut o) => {
                    let ev =
                        &mut *(o.get_mut().as_mut() as *mut AnyExtension as *mut ExtensionValue<V>);
                    ev.value = value;
                }
                Entry::Vacant(v) => {
                    v.insert(self.extension.new_value(value));
                }
            }
        }
    }

    pub fn take(self) -> Option<V> {
        unsafe {
            match self.entry {
                Entry::Occupied(o) => {
                    Some(Box::from_raw(Box::into_raw(o.remove()) as *mut ExtensionValue<V>).value)
                }
                Entry::Vacant(_) => None,
            }
        }
    }

    pub fn clear(self) {
        match self.entry {
            Entry::Occupied(o) => {
                o.remove();
            }
            _ => {}
        }
    }
}

impl<
        'a,
        T: ExtensionMessage,
        V: Clone + PartialEq + Default + Debug + Send + Sync + Deref<Target = L> + 'static,
        D: Send + Sync + Deref<Target = L> + 'static,
        L: 'static,
    > ExtensionField<'a, T, V, D>
{
    pub fn get_or_default(self) -> Option<&'a L> {
        let default = self.extension.default.as_ref();
        self.get().map(|r| &**r).or(default.map(|r| &**r))
    }
}

pub struct RepeatedExtensionField<'a, T: 'static, V: 'static> {
    entry: Entry<'a, Tag, Box<dyn AnyExtension>>,
    extension: &'static RepeatedExtension<T, V>,
}

impl<'a, T: ExtensionMessage, V: Clone + PartialEq + Debug + Send + Sync + 'static>
    RepeatedExtensionField<'a, T, V>
{
    pub fn get(self) -> Option<&'a RepeatedField<V>> {
        unsafe {
            match self.entry {
                Entry::Occupied(o) => Some(
                    &(*(o.get().as_ref() as *const AnyExtension
                        as *const RepeatedExtensionValue<V>))
                        .value,
                ),
                Entry::Vacant(_) => None,
            }
        }
    }

    pub fn get_mut(self) -> &'a mut RepeatedField<V> {
        unsafe {
            match self.entry {
                Entry::Occupied(mut o) => {
                    &mut (*(o.get_mut().as_mut() as *mut AnyExtension
                        as *mut RepeatedExtensionValue<V>))
                        .value
                }
                Entry::Vacant(v) => {
                    &mut (*(v.insert(self.extension.new_value()).as_mut() as *mut AnyExtension
                        as *mut RepeatedExtensionValue<V>))
                        .value
                }
            }
        }
    }

    pub fn has_entry(&self) -> bool {
        match self.entry {
            Entry::Occupied(_) => true,
            Entry::Vacant(_) => false,
        }
    }

    pub fn set(self, value: RepeatedField<V>) {
        unsafe {
            match self.entry {
                Entry::Occupied(mut o) => {
                    (*(o.get_mut().as_mut() as *mut AnyExtension
                        as *mut RepeatedExtensionValue<V>))
                        .value = value
                }
                Entry::Vacant(v) => {
                    v.insert(Box::new(RepeatedExtensionValue {
                        value,
                        codec: &self.extension.codec,
                    }));
                }
            }
        }
    }

    pub fn take(self) -> Option<RepeatedField<V>> {
        unsafe {
            match self.entry {
                Entry::Occupied(o) => Some(
                    Box::from_raw(Box::into_raw(o.remove()) as *mut RepeatedExtensionValue<V>)
                        .value,
                ),
                Entry::Vacant(_) => None,
            }
        }
    }

    pub fn clear(self) {
        match self.entry {
            Entry::Occupied(o) => {
                o.remove();
            }
            Entry::Vacant(_) => {}
        }
    }
}

pub trait ExtensionIdentifier: Sync {
    /// Gets the tag of the value this extension creates
    fn tag(&'static self) -> Tag;
    /// Gets the TypeId of the message this extension is extending
    fn message_type(&'static self) -> TypeId;
    #[doc(hidden)]
    fn read_value(&'static self, input: &mut CodedInput) -> InputResult<Box<dyn AnyExtension>>;
}

#[doc(hidden)]
pub trait AnyExtension: CodedMessage + Any + Debug + Send + Sync {
    fn clone(&self) -> Box<dyn AnyExtension>;
    fn merge(&mut self, other: &dyn AnyExtension);
    fn eq(&self, other: &dyn AnyExtension) -> bool;
    fn tag(&self) -> Tag;
}

pub struct ExtensionRegistry(HashMap<(TypeId, Tag), &'static dyn ExtensionIdentifier>);

#[doc(hidden)]
impl ExtensionRegistry {
    pub fn new(
        registries: &[&'static ExtensionRegistry],
        extensions: &[(TypeId, &[&'static dyn ExtensionIdentifier])],
    ) -> ExtensionRegistry {
        let mut registry = ExtensionRegistry(HashMap::new());
        let iterator = registries
            .iter()
            .flat_map(|r| r.0.iter().map(|(k, v)| (*k, *v)))
            .chain(
                extensions
                    .iter()
                    .flat_map(|pair| pair.1.iter().map(move |r| ((pair.0, r.tag()), *r))),
            );
        for entry in iterator {
            match registry.0.insert(entry.0, entry.1) {
                Some(e) if !std::ptr::eq(e, entry.1) => panic!("Tag conflict: two extension for the same type had a tag with the same value (value: {:?})", entry.0),
                Some(_) | None => { },
            }
        }
        registry
    }

    fn for_tag<T: 'static>(&self, tag: Tag) -> Option<&'static dyn ExtensionIdentifier> {
        self.0.get(&(TypeId::of::<T>(), tag)).map(|r| *r)
    }
}

impl ExtensionRegistry {
    /// Gets if the specified extension is present in this registry
    pub fn has_extension<T: ExtensionIdentifier>(&'static self, extension: &'static T) -> bool {
        self.0
            .get(&(extension.message_type(), extension.tag()))
            .map_or(false, |r| std::ptr::eq(*r, extension))
    }
}

/// An extension identifier used to retreive an extension value
pub struct Extension<T, V, D = V> {
    t: PhantomData<T>,
    codec: Codec<V>,
    default: Option<D>,
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
    bool => bool
);

#[doc(hidden)]
impl<T> Extension<T, String, &'static str> {
    pub const fn string(tag: u32, default: &'static str) -> Extension<T, String, &'static str> {
        Extension {
            t: PhantomData,
            codec: Codec::string(tag),
            default: Some(default),
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, Vec<u8>, &'static [u8]> {
    pub const fn bytes(tag: u32, default: &'static [u8]) -> Extension<T, Vec<u8>, &'static [u8]> {
        Extension {
            t: PhantomData,
            codec: Codec::bytes(tag),
            default: Some(default),
        }
    }
}

#[doc(hidden)]
impl<T, E: Enum> Extension<T, EnumValue<E>> {
    pub const fn enum_value(tag: u32, default: EnumValue<E>) -> Extension<T, EnumValue<E>> {
        Extension {
            t: PhantomData,
            codec: Codec::enum_value(tag),
            default: Some(default),
        }
    }
}

#[doc(hidden)]
impl<T, M: LiteMessage> Extension<T, M> {
    pub const fn message(tag: u32) -> Extension<T, M> {
        Extension {
            t: PhantomData,
            codec: Codec::message(tag),
            default: None,
        }
    }

    pub const fn group(tag: u32, end: u32) -> Extension<T, M> {
        Extension {
            t: PhantomData,
            codec: Codec::group(tag, end),
            default: None,
        }
    }
}

impl<T: ExtensionMessage, V: Clone + PartialEq + Debug + Send + Sync, D: Sync> ExtensionIdentifier
    for Extension<T, V, D>
{
    fn tag(&'static self) -> Tag {
        self.codec.tag()
    }
    fn message_type(&'static self) -> TypeId {
        TypeId::of::<T>()
    }
    fn read_value(&'static self, input: &mut CodedInput) -> InputResult<Box<dyn AnyExtension>> {
        Ok(Box::new(ExtensionValue {
            value: self.codec.read_from(input)?,
            codec: &self.codec
        }))
    }
}

impl<T: ExtensionMessage, V: Clone + PartialEq + Debug + Send + Sync, D: Send + Sync>
    Extension<T, V, D>
{
    fn new_value(&'static self, value: V) -> Box<dyn AnyExtension> {
        Box::new(ExtensionValue {
            value,
            codec: &self.codec,
        })
    }
}

pub struct ExtensionValue<V: 'static> {
    value: V,
    codec: &'static Codec<V>,
}

impl<V> CodedMessage for ExtensionValue<V> {
    fn merge_from(&mut self, input: &mut CodedInput) -> InputResult<()> {
        self.value = self.codec.read_from(input)?;
        Ok(())
    }

    fn write_to(&self, output: &mut CodedOutput) -> OutputResult {
        output.write_tag(self.codec.tag())?;
        self.codec.write_to(output, &self.value)?;
        if let Some(end_tag) = self.codec.end_tag() {
            output.write_tag(end_tag)?;
        }
        Ok(())
    }

    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        self.codec.calculate_size(&self.value)
    }

    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        self.codec.calculate_size(&self.value)
    }
}

impl<V: Clone + PartialEq + Debug + Send + Sync> AnyExtension for ExtensionValue<V> {
    fn clone(&self) -> Box<dyn AnyExtension> {
        Box::new(ExtensionValue {
            value: self.value.clone(),
            codec: self.codec,
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

impl<V: Debug> Debug for ExtensionValue<V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(fmt)
    }
}

pub struct RepeatedExtension<T, V> {
    t: PhantomData<T>,
    codec: Codec<V>,
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
            codec: Codec::string(tag),
        }
    }
}

#[doc(hidden)]
impl<T> RepeatedExtension<T, Vec<u8>> {
    pub const fn bytes(tag: u32) -> RepeatedExtension<T, Vec<u8>> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::bytes(tag),
        }
    }
}

#[doc(hidden)]
impl<T, E: crate::Enum> RepeatedExtension<T, EnumValue<E>> {
    pub const fn enum_value(tag: u32) -> RepeatedExtension<T, EnumValue<E>> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::enum_value(tag),
        }
    }
}

#[doc(hidden)]
impl<T, M: LiteMessage> RepeatedExtension<T, M> {
    pub const fn message(tag: u32) -> RepeatedExtension<T, M> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::message(tag),
        }
    }

    pub const fn group(tag: u32, end: u32) -> RepeatedExtension<T, M> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::group(tag, end),
        }
    }
}

#[doc(hidden)]
impl<T, M: ExtensionMessage> RepeatedExtension<T, M> {
    pub const fn extension_message(tag: u32) -> RepeatedExtension<T, M> {
        RepeatedExtension {
            t: PhantomData,
            codec: Codec::extension_message(tag),
        }
    }
}

impl<T: ExtensionMessage, V: Clone + PartialEq + Debug + Send + Sync> ExtensionIdentifier
    for RepeatedExtension<T, V>
{
    fn tag(&'static self) -> Tag {
        self.codec.tag()
    }
    fn message_type(&'static self) -> TypeId {
        TypeId::of::<T>()
    }
    fn read_value(&'static self, input: &mut CodedInput) -> InputResult<Box<dyn AnyExtension>> {
        let mut value = self.new_value();
        value.merge_from(input)?;
        Ok(value)
    }
}

impl<T: ExtensionMessage, V: Clone + PartialEq + Debug + Send + Sync> RepeatedExtension<T, V> {
    fn new_value(&'static self) -> Box<dyn AnyExtension> {
        Box::new(RepeatedExtensionValue {
            value: RepeatedField::new(),
            codec: &self.codec,
        })
    }
}

#[doc(hidden)]
pub struct RepeatedExtensionValue<V: 'static> {
    value: RepeatedField<V>,
    codec: &'static Codec<V>,
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

impl<V: Clone + PartialEq + Debug + Send + Sync> AnyExtension for RepeatedExtensionValue<V> {
    fn clone(&self) -> Box<dyn AnyExtension> {
        Box::new(RepeatedExtensionValue {
            value: self.value.clone(),
            codec: self.codec,
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
#[derive(Default)]
pub struct ExtensionSet<T> {
    t: PhantomData<T>,
    registry: Option<&'static ExtensionRegistry>,
    values_by_tag: HashMap<Tag, Box<dyn AnyExtension>>,
}

impl<T: ExtensionMessage> ExtensionSet<T> {
    fn registry_has_extension<I: ExtensionIdentifier>(&self, extension: &'static I) -> bool {
        self.registry.map_or(false, |r| r.has_extension(extension))
    }

    pub fn new() -> Self {
        Self {
            t: PhantomData,
            registry: None,
            values_by_tag: HashMap::new(),
        }
    }

    pub fn registry(&self) -> Option<&'static ExtensionRegistry> {
        self.registry
    }

    pub fn replace_registry(
        &mut self,
        registry: Option<&'static ExtensionRegistry>,
    ) -> Option<&'static ExtensionRegistry> {
        self.values_by_tag.clear();
        std::mem::replace(&mut self.registry, registry)
    }

    pub fn has_extension<I: ExtensionIdentifier>(&self, extension: &'static I) -> bool {
        self.registry_has_extension(extension) && self.has_extension_unchecked(extension)
    }

    pub fn has_extension_unchecked<I: ExtensionIdentifier>(&self, extension: &'static I) -> bool {
        self.values_by_tag.contains_key(&extension.tag())
    }

    pub fn get_value<V: Clone + PartialEq + Debug + Send + Sync, D: Sync>(
        &self,
        extension: &'static Extension<T, V, D>,
    ) -> Option<&V> {
        if self.registry_has_extension(extension) {
            unsafe {
                self.values_by_tag.get(&extension.tag()).map(|b| {
                    &(*(b.as_ref() as *const AnyExtension as *const ExtensionValue<V>)).value
                })
            }
        } else {
            None
        }
    }

    pub fn get_value_or_default<
        V: Clone + PartialEq + Debug + Send + Sync + Deref<Target = L>,
        D: Sync + Deref<Target = L>,
        L,
    >(
        &self,
        extension: &'static Extension<T, V, D>,
    ) -> Option<&L> {
        if self.registry_has_extension(extension) {
            unsafe {
                self.values_by_tag
                    .get(&extension.tag())
                    .map(|b| {
                        &*(*(b.as_ref() as *const AnyExtension as *const ExtensionValue<V>)).value
                    })
                    .or(extension.default.as_ref().map(|r| &**r))
            }
        } else {
            None
        }
    }

    pub fn get_repeated_value<V: PartialEq + Clone + Debug + Send + Sync>(
        &self,
        extension: &'static RepeatedExtension<T, V>,
    ) -> Option<&RepeatedField<V>> {
        if self.registry_has_extension(extension) {
            unsafe {
                self.values_by_tag.get(&extension.tag()).map(|b| {
                    &(*(b.as_ref() as *const AnyExtension as *const RepeatedExtensionValue<V>))
                        .value
                })
            }
        } else {
            None
        }
    }

    pub fn field<V: Default + Clone + PartialEq + Debug + Send + Sync, D: Sync>(
        &mut self,
        extension: &'static Extension<T, V, D>,
    ) -> Option<ExtensionField<T, V, D>> {
        if self.registry_has_extension(extension) {
            Some(ExtensionField {
                entry: self.values_by_tag.entry(extension.tag()),
                extension,
            })
        } else {
            None
        }
    }

    pub fn repeated_field<V: Clone + PartialEq + Debug + Send + Sync>(
        &mut self,
        extension: &'static RepeatedExtension<T, V>,
    ) -> Option<RepeatedExtensionField<T, V>> {
        if self.registry_has_extension(extension) {
            Some(RepeatedExtensionField {
                entry: self.values_by_tag.entry(extension.tag()),
                extension,
            })
        } else {
            None
        }
    }

    pub fn merge_from(&mut self, input: &mut CodedInput) -> InputResult<bool> {
        if let Some(last_tag) = input.last_tag() {
            if let Some(value) = self.values_by_tag.get_mut(&last_tag) {
                value.merge_from(input)?;
                return Ok(true);
            } else
            if let Some(id) = self.registry.and_then(|r| r.for_tag::<T>(last_tag)) {
                let value = id.read_value(input)?;
                self.values_by_tag.insert(last_tag, value);
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
            (None, None) => {}
            (Some(r), Some(o)) if std::ptr::eq(r, o) => {
                for (tag, value) in self.values_by_tag.iter_mut() {
                    value.merge(&**other.values_by_tag.get(tag).unwrap())
                }
            }
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
        match (self.registry, other.registry) {
            (Some(r), Some(o)) if std::ptr::eq(r, o) => {
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
            _ => false,
        }
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
