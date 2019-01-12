//! Contains protobuf collection types `repeated` and `map` and
//! are represented through the [`RepeatedField`] and [`MapField`] types respectively
//! in generated code.
//!
//! [`RepeatedField`]: collections/struct.RepeatedField.html
//! [`MapField`]: collections/struct.MapField.html

use crate::io::{CodedInput, CodedOutput, InputError, InputResult, OutputResult};
use crate::Codec;
use crate::LiteMessage;
use crate::ValueSize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

/// A `repeated` protobuf field. This type derefs to [`Vec<T>`]
///
/// This type is used in generated code to represent `repeated` fields and is
/// paired with a private static [`Codec`] to perform reading, writing, and size calculations.
///
/// The functions included in this type are made for generated code and as such are
/// rarely used directly by consumers. Most consumers will exclusively use derefed
/// [`Vec<T>`] functions.
///
/// [`Vec<T>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`Codec`]: ../struct.Codec.html
#[derive(Clone, Debug, PartialEq)]
pub struct RepeatedField<T>(Vec<T>);

impl<T> Deref for RepeatedField<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for RepeatedField<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> RepeatedField<T> {
    /// Creates a new RepeatedField
    ///
    /// Consumers will rarely use this outside of generated code
    pub fn new() -> RepeatedField<T> {
        RepeatedField(Vec::new())
    }
}

impl<T: Clone + PartialEq> RepeatedField<T> {
    /// Adds entries from the specified [`CodedInput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedInput`]: ../io/structs.CodedInput.html
    pub fn add_entries(
        &mut self,
        tag: u32,
        input: &mut CodedInput,
        codec: &Codec<T>,
    ) -> InputResult<()> {
        if let Some(packed) = codec.is_packed_tag(tag) {
            if packed {
                let new_limit = input.read_int32()?;
                let old = input.push_limit(new_limit);
                while !input.reached_limit() {
                    self.push(codec.read_from(input)?);
                }
                input.pop_limit(old);
            } else {
                self.push(codec.read_from(input)?);
            }
            Ok(())
        } else {
            Err(InputError::InvalidTag)
        }
    }

    /// Calculates the size it take to serialize this field in a [`CodedOutput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedOutput`]: ../io/structs.CodedOutput.html
    #[cfg(checked_size)]
    pub fn calculate_size(&self, codec: &Codec<T>) -> Option<i32> {
        if self.len() == 0 {
            return Some(0);
        }
        if codec.is_packed() {
            let length = self.calculate_packed_size(codec)?;
            (crate::io::sizes::int32(codec.tag()) + crate::io::sizes::int32(length))
                .checked_add(length)
        } else {
            self.iter().fold(
                crates::io::sizes::uint32(codec.tag()).checked_mul(self.len()),
                |last, value| last.checked_add(codec.calculate_size(value)?),
            )
        }
    }

    #[cfg(checked_size)]
    fn calculate_packed_size(&self, codec: &Codec<T>) -> Option<i32> {
        match codec.size {
            ValueSize::Fixed(s) => self
                .len()
                .checked_mul(s as usize)
                .and_then(|m| m.try_into().ok()),
            ValueSize::Func(fun) => self
                .iter()
                .fold(0, |last, value| last.checked_add(fun(value)?)?),
        }
    }

    /// Calculates the size it take to serialize this field in a [`CodedOutput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedOutput`]: ../io/structs.CodedOutput.html
    #[cfg(not(checked_size))]
    pub fn calculate_size(&self, codec: &Codec<T>) -> i32 {
        if self.len() == 0 {
            return 0;
        }
        if codec.is_packed() {
            let length = self.calculate_packed_size(codec);
            crate::io::sizes::uint32(codec.tag()) + crate::io::sizes::int32(length) + length
        } else {
            self.iter().fold(
                (crate::io::sizes::uint32(codec.tag()) as usize * self.len()) as i32,
                |last, value| last + codec.calculate_size(value),
            )
        }
    }

    #[cfg(not(checked_size))]
    fn calculate_packed_size(&self, codec: &Codec<T>) -> i32 {
        match codec.size {
            ValueSize::Fixed(s) => (self.len() * s as usize) as i32,
            ValueSize::Func(fun) => self.iter().fold(0, |last, value| last + fun(value)),
        }
    }

    /// Writes this field to a [`CodedOutput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedOutput`]: ../io/structs.CodedOutput.html
    pub fn write_to(&self, output: &mut CodedOutput, codec: &Codec<T>) -> OutputResult {
        if !self.is_empty() {
            if codec.is_packed() {
                #[cfg(checked_size)]
                let size = self.calculate_packed_size(codec)?;

                #[cfg(not(checked_size))]
                let size = self.calculate_packed_size(codec);

                output.write_raw_tag(codec.tag())?;
                output.write_int32(size)?;
                for value in self.iter() {
                    codec.write_to(output, value)?;
                }
            } else {
                for value in self.iter() {
                    output.write_raw_tag(codec.tag())?;
                    codec.write_to(output, value)?;
                    if let Some(end_tag) = codec.end_tag() {
                        output.write_raw_tag(end_tag.get())?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Merges this field with another field of the same type
    ///
    /// Consumers will rarely use this outside of generated code
    pub fn merge(&mut self, other: &Self) {
        self.extend_from_slice(other.as_slice())
    }
}

impl<T: LiteMessage> RepeatedField<T> {
    /// Checks each message in this field to see if it is initialized
    ///
    /// Consumers will rarely use this outside of generated code
    pub fn is_initialized(&self) -> bool {
        for msg in self.iter() {
            if !msg.is_initialized() {
                return false;
            }
        }
        true
    }
}

/// A `map` protobuf field. This type derefs to [`HashMap<K, V>`]
///
/// This type is used in generated code to represent `map` fields and is
/// paired with a private static [`MapCodec`] to perform reading, writing, and size calculations.
///
/// The functions included in this type are made for generated code and as such are
/// rarely used directly by consumers. Most consumers will exclusively use derefed
/// [`HashMap<K, V>`] functions.
///
/// [`HashMap<K, V>`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`MapCodec`]: struct.MapCodec.html
#[derive(Clone)]
pub struct MapField<K, V>(HashMap<K, V>);

impl<K: Eq + Hash, V: PartialEq> MapField<K, V> {
    pub fn new() -> MapField<K, V> {
        MapField(HashMap::new())
    }
}

impl<K: Eq + Hash, V: PartialEq> PartialEq for MapField<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Eq + Hash + Debug, V: PartialEq + Debug> Debug for MapField<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<K, V> Deref for MapField<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &HashMap<K, V> {
        &self.0
    }
}

impl<K, V> DerefMut for MapField<K, V> {
    fn deref_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.0
    }
}

impl<K: Eq + Hash + Clone, V: PartialEq + Clone> MapField<K, V> {
    /// Adds entries from the specified [`CodedInput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedInput`]: ../io/structs.CodedInput.html
    pub fn add_entries(
        &mut self,
        input: &mut CodedInput,
        codec: &MapCodec<K, V>,
    ) -> InputResult<()> {
        let mut adapter = MapReadAdapter::new(codec);
        input.read_message(&mut adapter)?;
        if let Some(key) = adapter.key {
            if let Some(value) = adapter.value {
                self.insert(key, value);
            }
        }
        Ok(())
    }

    /// Calculates the size it take to serialize this field in a [`CodedOutput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedOutput`]: ../io/structs.CodedOutput.html
    #[cfg(checked_size)]
    pub fn calculate_size(&self, codec: &MapCodec<K, V>) -> Option<i32> {
        if self.is_empty() {
            return Some(0);
        }

        let mut size = 0i32;
        let mut adapter = MapWriteAdapter::new(codec);
        for field in &self.0 {
            adapter.key = Some(&field.0);
            adapter.value = Some(&field.1);
            size = size.checked_add(crate::io::sizes::uint32(codec.tag))?;
            size = size.checked_add(crate::io::sizes::message(&adapter)?)?;
        }
        Some(size)
    }

    /// Calculates the size it take to serialize this field in a [`CodedOutput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedOutput`]: ../io/structs.CodedOutput.html
    #[cfg(not(checked_size))]
    pub fn calculate_size(&self, codec: &MapCodec<K, V>) -> i32 {
        if self.is_empty() {
            return 0;
        }

        let mut size = 0i32;
        let mut adapter = MapWriteAdapter::new(codec);
        for field in &self.0 {
            adapter.key = Some(&field.0);
            adapter.value = Some(&field.1);
            size += crate::io::sizes::uint32(codec.tag);
            size += crate::io::sizes::message(&adapter);
        }
        size
    }

    /// Writes this field to a [`CodedOutput`]
    ///
    /// Consumers will rarely use this outside of generated code
    ///
    /// [`CodedOutput`]: ../io/structs.CodedOutput.html
    pub fn write_to(&self, output: &mut CodedOutput, codec: &MapCodec<K, V>) -> OutputResult {
        let mut adapter = MapWriteAdapter::new(codec);
        for field in &self.0 {
            adapter.key = Some(&field.0);
            adapter.value = Some(&field.1);
            output.write_raw_tag(codec.tag)?;
            output.write_message(&adapter)?;
        }
        Ok(())
    }

    /// Merges this field with another field of the same type
    ///
    /// Consumers will rarely use this outside of generated code
    pub fn merge(&mut self, other: &Self) {
        for entry in &other.0 {
            match self.get_mut(entry.0) {
                Some(value) => *value = entry.1.clone(),
                None => {
                    self.insert(entry.0.clone(), entry.1.clone());
                }
            }
        }
    }
}

impl<K: Eq + Hash + Clone, V: LiteMessage> MapField<K, V> {
    /// Checks each message in this field to see if it is initialized
    ///
    /// Consumers will rarely use this outside of generated code
    pub fn is_initialized(&self) -> bool {
        for msg in self.values() {
            if !msg.is_initialized() {
                return false;
            }
        }
        true
    }
}

struct MapReadAdapter<'a, K, V> {
    key: Option<K>,
    value: Option<V>,
    codec: &'a MapCodec<K, V>,
}

impl<'a, K, V> MapReadAdapter<'a, K, V> {
    pub fn new(codec: &'a MapCodec<K, V>) -> MapReadAdapter<'a, K, V> {
        MapReadAdapter {
            codec,
            key: None,
            value: None,
        }
    }
}

impl<K: Clone + PartialEq, V: Clone + PartialEq> crate::CodedMessage for MapReadAdapter<'_, K, V> {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let Some(tag) = input.read_tag()? {
            match tag.get() {
                tag if tag == self.codec.key.tag() => {
                    self.codec.key.merge_from(input, &mut self.key)?
                }
                tag if tag == self.codec.value.tag() => {
                    self.codec.value.merge_from(input, &mut self.value)?
                }
                tag => input.skip(tag)?,
            }
        }
        Ok(())
    }
    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        unreachable!()
    }
    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        unreachable!()
    }
    fn write_to(&self, _output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        unreachable!()
    }
}

struct MapWriteAdapter<'a, K, V> {
    key: Option<&'a K>,
    value: Option<&'a V>,
    codec: &'a MapCodec<K, V>,
}

impl<'a, K, V> MapWriteAdapter<'a, K, V> {
    pub fn new(codec: &'a MapCodec<K, V>) -> MapWriteAdapter<'a, K, V> {
        MapWriteAdapter {
            codec,
            key: None,
            value: None,
        }
    }
}

impl<K: Clone + PartialEq, V: Clone + PartialEq> crate::CodedMessage for MapWriteAdapter<'_, K, V> {
    fn merge_from(&mut self, _input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        unreachable!()
    }
    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32> {
        let mut size = 0i32;
        size = size.checked_add(crate::io::sizes::uint32(self.codec.key.tag()))?;
        size = size.checked_add(self.codec.key.calculate_size(self.key.unwrap())?)?;
        size = size.checked_add(crate::io::sizes::uint32(self.codec.key.tag()))?;
        size = size.checked_add(self.codec.value.calculate_size(self.value.unwrap())?)?;

        Some(size)
    }
    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += crate::io::sizes::uint32(self.codec.key.tag());
        size += self.codec.key.calculate_size(self.key.unwrap());
        size += crate::io::sizes::uint32(self.codec.key.tag());
        size += self.codec.value.calculate_size(self.value.unwrap());
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        output.write_raw_tag(self.codec.key.tag())?;
        self.codec.key.write_to(output, self.key.unwrap())?;
        output.write_raw_tag(self.codec.value.tag())?;
        self.codec.value.write_to(output, self.value.unwrap())?;

        Ok(())
    }
}

/// A codec containing the required information to read, write, and calculate the size of a map field
pub struct MapCodec<K, V> {
    key: Codec<K>,
    value: Codec<V>,
    tag: u32,
}

impl<K, V> MapCodec<K, V> {
    /// Creates a new MapCodec for static storage
    ///
    /// Consumers will rarely use this outside of generated code
    pub const fn new(key: Codec<K>, value: Codec<V>, tag: u32) -> MapCodec<K, V> {
        MapCodec { key, value, tag }
    }
}
