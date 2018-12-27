use crate::Codec;
use crate::ValueSize;
use crate::io::*;
use std::convert::TryInto;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct RepeatedField<T>(Vec<T>);

impl<T> std::ops::Deref for RepeatedField<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> std::ops::DerefMut for RepeatedField<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> RepeatedField<T> {
    pub fn new() -> RepeatedField<T> {
        RepeatedField(Vec::new())
    }
}

impl<T: Clone + PartialEq> RepeatedField<T> {
    pub fn add_entries(&mut self, tag: u32, input: &mut CodedInput, codec: &Codec<T>) -> InputResult<()> {
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

    #[allow(unused_variables)]
    pub fn calculate_size(&self, codec: &Codec<T>) -> Option<i32> {
        unimplemented!()
    }

    pub fn write_to(&self, output: &mut CodedOutput, codec: &Codec<T>) -> OutputResult {
        if !self.is_empty() {
            if codec.is_packed() {
                let size = match codec.size {
                    ValueSize::Fixed(s) => self.len().checked_mul(s as usize).and_then(|m| m.try_into().ok()),
                    ValueSize::Func(fun) => self.iter().fold(Some(0i32), |last,value| last?.checked_add(fun(value)?)),
                }.ok_or(OutputError::ValueTooLarge)?;

                output.write_raw_tag(*codec.tag())?;
                output.write_int32(size)?;
                for value in self.iter() {
                    codec.write_to(output, value)?;
                }
            } else {
                for value in self.iter() {
                    output.write_raw_tag(*codec.tag())?;
                    codec.write_to(output, value)?;
                    if let Some(end_tag) = codec.end_tag() {
                        output.write_raw_tag(end_tag.get())?;
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn is_initialized(&self) -> bool {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn merge(&mut self, other: &Self) {
        self.extend_from_slice(other.as_slice())
    }
}

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

impl<K, V> std::ops::Deref for MapField<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &HashMap<K, V> {
        &self.0
    }
}

impl<K, V> std::ops::DerefMut for MapField<K, V> {
    fn deref_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.0
    }
}

impl<K, V> MapField<K, V> {
    pub fn add_entries(&mut self, tag: u32, input: &mut CodedInput, codec: &(Codec<K>, Codec<V>)) -> InputResult<()> {
        unimplemented!()
    }
    pub fn calculate_size(&self, codec: &(Codec<K>, Codec<V>)) -> Option<i32> {
        unimplemented!()
    }
    pub fn write_to(&self, output: &mut CodedOutput, codec: &(Codec<K>, Codec<V>)) -> OutputResult {
        unimplemented!()
    }
    pub fn merge(&mut self, other: &Self) {
        unimplemented!()
    }
}