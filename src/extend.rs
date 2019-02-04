use crate::{Codec, CodedMessage, LiteMessage};
use crate::io::{Tag, CodedInput, CodedOutput, InputResult, OutputResult};
use crate::collections::RepeatedField;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;

/// A message with extensions
pub trait ExtensionMessage: LiteMessage {
    /// Clears this message's extensions and configures the message to use extensions from the specified registry.
    fn with_extensions(&mut self, extensions: &ExtensionRegistry);

    /// Gets the value of the specified extension field or None if it is not registered in this message.
    fn field<V, D>(&self, extension: &Extension<Self, V, D>) -> Option<&ExtensionField<V, D>>;
    /// Gets the value of the specified extension field mutably or None if it is not registered in this message.
    fn field_mut<V, D>(&mut self, extension: &Extension<Self, V, D>) -> Option<&mut ExtensionField<V, D>>;
    /// Gets the value of the specified extension repeated field or None if it is not registered in this message.
    fn repeated_field<V>(&self, extension: &RepeatedExtension<Self, V>) -> Option<&RepeatedField<V>>;
    /// Gets the value of a specified extension repeated field mutably or None if it is not registered in this message.
    fn repeated_field_mut<V>(&mut self, extension: &RepeatedExtension<Self, V>) -> Option<&mut RepeatedField<V>>;
}

pub trait ExtensionIdentifier {
    fn new_value(&'static self) -> Rc<dyn AnyExtension>;
    fn tag(&'static self) -> Tag;
}

pub trait AnyExtension: CodedMessage + Any {
    fn clone(&self) -> Rc<dyn AnyExtension>;
    fn merge(&mut self, other: &dyn AnyExtension);
    fn eq(&self, other: &dyn AnyExtension) -> bool;
    fn tag(&self) -> Tag;
}

pub struct ExtensionRegistry(HashMap<TypeId, Box<[&'static dyn ExtensionIdentifier]>>);

/// An extension identifier used to retreive an extension value
pub struct Extension<T, V, D = V> {
    t: PhantomData<T>,
    codec: Codec<V>,
    default: Option<D>
}

#[doc(hidden)]
impl<T> Extension<T, f32> {
    pub const fn float(tag: u32, default: f32) -> Extension<T, f32> {
        Extension {
            t: PhantomData,
            codec: Codec::float(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, f64> {
    pub const fn double(tag: u32, default: f64) -> Extension<T, f64> {
        Extension {
            t: PhantomData,
            codec: Codec::double(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, i32> {
    pub const fn int32(tag: u32, default: i32) -> Extension<T, i32> {
        Extension {
            t: PhantomData,
            codec: Codec::int32(tag),
            default: Some(default)
        }
    }

    pub const fn sint32(tag: u32, default: i32) -> Extension<T, i32> {
        Extension {
            t: PhantomData,
            codec: Codec::sint32(tag),
            default: Some(default)
        }
    }

    pub const fn sfixed32(tag: u32, default: i32) -> Extension<T, i32> {
        Extension {
            t: PhantomData,
            codec: Codec::sfixed32(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, u32> {
    pub const fn uint32(tag: u32, default: u32) -> Extension<T, u32> {
        Extension {
            t: PhantomData,
            codec: Codec::uint32(tag),
            default: Some(default)
        }
    }

    pub const fn fixed32(tag: u32, default: u32) -> Extension<T, u32> {
        Extension {
            t: PhantomData,
            codec: Codec::fixed32(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, i64> {
    pub const fn int64(tag: u32, default: i64) -> Extension<T, i64> {
        Extension {
            t: PhantomData,
            codec: Codec::int64(tag),
            default: Some(default)
        }
    }

    pub const fn sint64(tag: u32, default: i64) -> Extension<T, i64> {
        Extension {
            t: PhantomData,
            codec: Codec::sint64(tag),
            default: Some(default)
        }
    }

    pub const fn sfixed64(tag: u32, default: i64) -> Extension<T, i64> {
        Extension {
            t: PhantomData,
            codec: Codec::sfixed64(tag),
            default: Some(default)
        }
    }
}

#[doc(hidden)]
impl<T> Extension<T, u64> {
    pub const fn uint64(tag: u32, default: u64) -> Extension<T, u64> {
        Extension {
            t: PhantomData,
            codec: Codec::uint64(tag),
            default: Some(default)
        }
    }

    pub const fn fixed64(tag: u32, default: u64) -> Extension<T, u64> {
        Extension {
            t: PhantomData,
            codec: Codec::fixed64(tag),
            default: Some(default)
        }
    }
}

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
impl<T, M: LiteMessage> Extension<T, M> {
    pub const fn message(tag: u32) -> Extension<T, M> {
        Extension {
            t: PhantomData,
            codec: Codec::message(tag),
            default: None
        }
    }

    pub const fn group(tag: u32, end: NonZeroU32) -> Extension<T, M> {
        Extension {
            t: PhantomData,
            codec: Codec::group(tag, end),
            default: None
        }
    }
}

impl<T: LiteMessage, V: Clone + PartialEq + PartialEq<D>, D> ExtensionIdentifier for Extension<T, V, D> {
    fn new_value(&'static self) -> Rc<dyn AnyExtension> {
        Rc::new(ExtensionField {
            value: None,
            default: &self.default,
            codec: &self.codec
        })
    }
    fn tag(&'static self) -> Tag {
        self.codec.tag()
    }
}

pub struct ExtensionField<V: 'static, D: 'static> {
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

impl<V: Clone + PartialEq + PartialEq<D>, D> AnyExtension for ExtensionField<V, D> {
    fn clone(&self) -> Rc<dyn AnyExtension> {
        Rc::new(ExtensionField {
            value: self.value.clone(),
            default: self.default,
            codec: self.codec
        })
    }
    fn merge(&mut self, other: &dyn AnyExtension) {
        if TypeId::of::<Self>() == other.get_type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                if self.codec as *const Codec<V> == other.codec as *const Codec<V> {
                    self.codec.merge_values(&mut self.value, &other.value);
                }
            }
        }
    }
    fn eq(&self, other: &dyn AnyExtension) -> bool {
        if TypeId::of::<Self>() == other.get_type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                self.codec as *const Codec<V> == other.codec as *const Codec<V> && self.value == other.value
            }
        } else {
            false
        }
    }
    fn tag(&self) -> Tag {
        self.codec.tag()
    }
}

pub struct RepeatedExtension<T, V> {
    t: PhantomData<T>,
    codec: Codec<V>
}

impl<T: LiteMessage, V: Clone + PartialEq + 'static> ExtensionIdentifier for RepeatedExtension<T, V> {
    fn new_value(&'static self) -> Rc<dyn AnyExtension> {
        Rc::new(RepeatedExtensionValue {
            value: RepeatedField::new(),
            codec: &self.codec
        })
    }
    fn tag(&'static self) -> Tag {
        self.codec.tag()
    }
}

pub struct RepeatedExtensionValue<V: 'static> {
    value: RepeatedField<V>,
    codec: &'static Codec<V>
}

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

impl<V: Clone + PartialEq + 'static> AnyExtension for RepeatedExtensionValue<V> {
    fn clone(&self) -> Rc<dyn AnyExtension> {
        Rc::new(RepeatedExtensionValue {
            value: self.value.clone(),
            codec: self.codec
        })
    }
    fn merge(&mut self, other: &dyn AnyExtension) {
        if TypeId::of::<Self>() == other.get_type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                if self.codec as *const Codec<V> == other.codec as *const Codec<V> {
                    self.value.merge(&other.value);
                }
            }
        }
    }
    fn tag(&self) -> Tag {
        self.codec.tag()
    }
    fn eq(&self, other: &dyn AnyExtension) -> bool {
        if TypeId::of::<Self>() == other.get_type_id() {
            unsafe {
                let other = &*(other as *const dyn AnyExtension as *const Self);
                self.codec as *const Codec<V> == other.codec as *const Codec<V> && self.value == other.value
            }
        } else {
            false
        }
    }
}

pub struct ExtensionSet<T> {
    t: PhantomData<T>,
    // each extension is statically unique, so we can make a const pointer to keep track of them
    values_by_type: HashMap<*const (), Rc<dyn AnyExtension>>,
    values_by_tag: HashMap<Tag, Rc<dyn AnyExtension>>
}

impl<T: LiteMessage + 'static> ExtensionSet<T> {
    #[doc(hidden)]
    pub fn new(identifiers: &[&'static dyn ExtensionIdentifier]) -> Self {
        let mut by_types: HashMap<*const (), Rc<dyn AnyExtension>> = HashMap::new();
        let mut by_tags: HashMap<Tag, Rc<dyn AnyExtension>> = HashMap::new();

        for identifier in identifiers.iter() {
            let value = identifier.new_value();
            by_types.insert(*identifier as *const dyn ExtensionIdentifier as *const (), Rc::clone(&value));
            by_tags.insert(identifier.tag(), value);
        }

        Self {
            t: PhantomData,
            values_by_type: by_types,
            values_by_tag: by_tags
        }
    }

    pub fn field<V: Clone + PartialEq<D>, D>(&self, extension: &'static Extension<T, V, D>) -> Option<&ExtensionField<V, D>> {
        unsafe {
            self.values_by_type.get(&(extension as *const Extension<T, V, D> as *const ()))
                .map(|rc| &*(rc.as_ref() as *const dyn AnyExtension as *const ExtensionField<V, D>))
        }
    }

    pub fn field_mut<V: Clone + PartialEq<D>, D>(&mut self, extension: &'static Extension<T, V, D>) -> Option<&mut ExtensionField<V, D>> {
        unsafe {
            self.values_by_type.get_mut(&(extension as *const Extension<T, V, D> as *const ()))
                .map(|rc| &mut *(Rc::get_mut(rc).unwrap() as *mut dyn AnyExtension as *mut ExtensionField<V, D>))
        }
    }

    pub fn repeated_field<V: Clone>(&self, extension: &'static RepeatedExtension<T, V>) -> Option<&RepeatedField<V>> {
        unsafe {
            self.values_by_type.get(&(extension as *const RepeatedExtension<T, V> as *const ()))
                .map(|rc| &(*(rc.as_ref() as *const dyn AnyExtension as *const RepeatedExtensionValue<V>)).value)
        }
    }

    pub fn repeated_field_mut<V: Clone>(&mut self, extension: &'static RepeatedExtension<T, V>) -> Option<&mut RepeatedField<V>> {
        unsafe {
            self.values_by_type.get_mut(&(extension as *const RepeatedExtension<T, V> as *const ()))
                .map(|rc| &mut (*(Rc::get_mut(rc).unwrap() as *mut dyn AnyExtension as *mut RepeatedExtensionValue<V>)).value)
        }
    }

    #[doc(hidden)]
    pub fn merge_from(&mut self, input: &mut CodedInput) -> InputResult<bool> {
        if let Some(last_tag) = input.last_tag() {
            if let Some(value) = self.values_by_tag.get_mut(&last_tag) {
                Rc::get_mut(value).unwrap().merge_from(input)?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    #[doc(hidden)]
    #[cfg(not(checked_size))]
    pub fn calculate_size(&self) -> i32 {
        self.values_by_tag
            .iter()
            .map(|(_, field)| field.calculate_size())
            .sum()
    }

    #[doc(hidden)]
    #[cfg(checked_size)]
    pub fn calculate_size(&self) -> Option<i32> {
        let mut len = 0;

        for (_, field) in self.values_by_tag.iter() {
            len.checked_add(field.calculate_size())?;
        }

        Some(len)
    }

    #[doc(hidden)]
    pub fn write_to(&self, output: &mut CodedOutput) -> OutputResult {
        for (_, field) in self.values_by_tag.iter() {
            field.write_to(output)?;
        }
        Ok(())
    }
}

impl<T> Clone for ExtensionSet<T> {
    fn clone(&self) -> Self {
        let mut new_type_map = HashMap::new();
        let mut new_tag_map = HashMap::new();

        for (ptr, value) in self.values_by_type.iter() {
            let new_value = value.clone();
            new_type_map.insert(*ptr, Rc::clone(&new_value));
            new_tag_map.insert(new_value.tag(), new_value);
        }

        ExtensionSet {
            t: PhantomData,
            values_by_tag: new_tag_map,
            values_by_type: new_type_map
        }
    }
}

impl<T> PartialEq for ExtensionSet<T> {
    fn eq(&self, other: &Self) -> bool {
        for (ptr, value) in self.values_by_type.iter() {
            if let Some(other_value) = other.values_by_type.get(ptr) {
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