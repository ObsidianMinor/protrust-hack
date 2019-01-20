#![feature(const_fn)]
#![feature(const_string_new)]
#![feature(const_vec_new)]
#![feature(try_from)]
#![feature(get_type_id)]

#[cfg_attr(checked_size, path = "generated/checked/mod.rs")]
#[cfg_attr(not(checked_size), path = "generated/unchecked/mod.rs")]
#[rustfmt::skip]
#[allow(unused_variables, dead_code, non_camel_case_types, non_snake_case, missing_docs)]
pub(crate) mod generated;

/// The protrust prelude
///
/// Alleviates imports of many common protobuf traits and io structs
/// by adding a glob import to the top of protobuf heavy modules
pub mod prelude {
    pub use crate::CodedMessage;
    pub use crate::EnumValue;
    pub use crate::LiteMessage;
    #[cfg(feature = "reflection")]
    pub use crate::Message;
}
pub mod collections;
pub mod io;
#[cfg(all(feature = "reflection"))]
pub mod wkt;
#[cfg(feature = "reflection")]
pub use crate::generated::google_protobuf_compiler_plugin_proto as plugin;
#[cfg(feature = "reflection")]
pub use crate::generated::google_protobuf_descriptor_proto as descriptor;
#[cfg(feature = "reflection")]
pub mod reflect;

use crate::io::{Tag, WireType};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::NonZeroU32;

mod internal {
    pub trait Sealed { }
}

/// A Protocol Buffers message capable of writing itself to a coded output or reading itself from a coded input
pub trait CodedMessage {
    /// Merges fields from the coded input into this message
    fn merge_from(&mut self, input: &mut io::CodedInput) -> io::InputResult<()>;

    /// Merges an instance of self from a Read instance
    #[inline]
    fn merge_from_read(&mut self, read: &mut std::io::Read) -> io::InputResult<()> {
        let mut reader = io::CodedInput::new(read);
        self.merge_from(&mut reader)
    }

    /// Calculates the size of the message and returns it as an 32-bit integer or None if the message is larger than `i32::MAX`
    #[cfg(checked_size)]
    fn calculate_size(&self) -> Option<i32>;

    /// Calculates the size of the message and returns it as an 32-bit integer
    #[cfg(not(checked_size))]
    fn calculate_size(&self) -> i32;

    /// Writes the fields of this message to the coded output
    fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult;

    /// Writes the message to a Write instance
    #[inline]
    fn write(&self, write: &mut std::io::Write) -> io::OutputResult {
        let mut writer = io::CodedOutput::new(write);
        self.write_to(&mut writer)
    }

    /// Writes the message to a new Vec<u8> or Err(io::OutputError::ValueTooLarge) if the message is too large
    #[cfg(checked_size)]
    #[inline]
    fn write_to_vec(&self) -> Result<Vec<u8>, io::OutputError> {
        if let Some(size) = self.calculate_size() {
            let mut out = Vec::with_capacity(size as usize);
            self.write(&mut out)?;
            Ok(out)
        } else {
            Err(io::OutputError::ValueTooLarge)
        }
    }

    /// Writes the message to a new `Vec<u8>`
    #[cfg(not(checked_size))]
    #[inline]
    fn write_to_vec(&self) -> Result<Vec<u8>, io::OutputError> {
        let mut out = Vec::with_capacity(self.calculate_size() as usize);
        self.write(&mut out)?;
        Ok(out)
    }

    /// Gets whether all the required fields and messages are initialized
    #[inline]
    fn is_initialized(&self) -> bool {
        true
    }
}

/// A LITE Protocol Buffers message
pub trait LiteMessage: CodedMessage + Clone + PartialEq {
    /// Creates a new instance of the message
    fn new() -> Self;

    /// Merges the fields from another message into this one via copy assignment
    fn merge(&mut self, other: &Self);

    /// Reads a new instance of Self from the specified Read using a CodedInput
    fn read_new(read: &mut std::io::Read) -> io::InputResult<Self> {
        let mut reader = io::CodedInput::new(read);
        Self::read_new_from_input(&mut reader)
    }

    /// Reads a new instance of Self from the specified CodedInput
    fn read_new_from_input(input: &mut io::CodedInput) -> io::InputResult<Self> {
        let mut instance = Self::new();
        instance.merge_from(input)?;
        Ok(instance)
    }
}

/// A Protocol Buffers message
#[cfg(feature = "reflection")]
pub trait Message: LiteMessage {
    /// Gets a static reference to the descriptor describing this message type
    fn descriptor() -> &'static reflect::MessageDescriptor;
}

/// The error result for when an enum value is undefined
pub struct VariantUndefinedError;

#[doc(hidden)]
pub unsafe trait Enum { }

/// Represents a Protocol Buffer enum value that can be a defined enum value or an undefined integer
///
/// In Rust, an enum value without an associated discriminant is undefined behavior.
/// In Protocol Buffers, there is no guarantee that an enum value will be valid.
/// Thus, this union is introduced to allow for both undefined enum values and defined enum values.
#[derive(Copy, Debug, Clone)]
pub enum EnumValue<E> {
    /// A defined enum value
    Defined(E),
    /// An undefined enum value
    Undefined(i32),
}

impl<E> EnumValue<E> {
    /// Converts from an EnumValue<E> to Option<E>, discarding the undefined value if it exists
    pub fn defined(self) -> Option<E> {
        match self {
            EnumValue::Defined(e) => Some(e),
            EnumValue::Undefined(_) => None
        }
    }

    /// Converts from an EnumValue<E> to Option<i32>, discarding the defined value if it exists
    pub fn undefined(self) -> Option<i32> {
        match self {
            EnumValue::Defined(_) => None,
            EnumValue::Undefined(u) => Some(u)
        }
    }

    /// Returns a Defined value, panics if Undefined
    pub fn unwrap(self) -> E {
        match self {
            EnumValue::Defined(e) => e,
            EnumValue::Undefined(u) => panic!("Undefined enum value {}", u),
        }
    }

    /// Returns a Defined value, panics with the specified message if Undefined
    pub fn expect(self, msg: &str) -> E {
        match self {
            EnumValue::Defined(e) => e,
            EnumValue::Undefined(_) => expect_failed(msg),
        }
    }
}

#[inline(never)]
#[cold]
fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

impl<E: Into<i32> + Clone> PartialEq for EnumValue<E> {
    fn eq(&self, other: &Self) -> bool {
        Into::<i32>::into(self.clone()) == Into::<i32>::into(other.clone())
    }
}

impl<E: Into<i32> + Clone> Eq for EnumValue<E> {}

impl<E: TryFrom<i32, Error = VariantUndefinedError>> From<i32> for EnumValue<E> {
    fn from(value: i32) -> EnumValue<E> {
        if let Ok(e) = E::try_from(value) {
            EnumValue::Defined(e)
        } else {
            EnumValue::Undefined(value)
        }
    }
}

impl<E: Into<i32> + Clone> From<EnumValue<E>> for i32 {
    fn from(value: EnumValue<E>) -> i32 {
        match value {
            EnumValue::Defined(ref e) => e.clone().into(),
            EnumValue::Undefined(v) => v,
        }
    }
}

impl<E: Enum> From<E> for EnumValue<E> {
    fn from(value: E) -> EnumValue<E> {
        EnumValue::Defined(value)
    }
}

#[doc(hidden)]
pub struct Codec<T> {
    default: Option<T>,
    start: u32,
    end: Option<NonZeroU32>,
    size: ValueSize<T>,
    merge: fn(&mut io::CodedInput, &mut Option<T>) -> io::InputResult<()>,
    write: fn(&mut io::CodedOutput, &T) -> io::OutputResult,
    packed: bool,
    packable: bool,
}

enum ValueSize<T> {
    Fixed(i32),
    #[cfg(checked_size)]
    Func(fn(&T) -> Option<i32>),
    #[cfg(not(checked_size))]
    Func(fn(&T) -> i32),
}

const fn is_packed(tag: u32) -> bool {
    (tag & 0b111) == 2
}

impl<T: Clone + PartialEq> Codec<T> {
    fn is_packed_tag(&self, tag: u32) -> Option<bool> {
        Some(self.packable && io::WireType::get_type(tag)? == io::WireType::LengthDelimited)
    }

    #[inline]
    fn is_packed(&self) -> bool {
        self.packed
    }

    #[inline]
    fn tag(&self) -> u32 {
        self.start
    }

    #[inline]
    fn end_tag(&self) -> Option<NonZeroU32> {
        self.end
    }

    #[inline]
    fn is_default(&self, value: &T) -> bool {
        match &self.default {
            None => false,
            Some(default) => default == value,
        }
    }

    #[cfg(checked_size)]
    #[inline]
    fn calculate_size(&self, value: &T) -> Option<i32> {
        match self.size {
            ValueSize::Fixed(s) => Some(s),
            ValueSize::Func(f) => (f)(value),
        }
    }

    #[cfg(not(checked_size))]
    #[inline]
    fn calculate_size(&self, value: &T) -> i32 {
        match self.size {
            ValueSize::Fixed(s) => s,
            ValueSize::Func(f) => (f)(value),
        }
    }

    #[inline]
    fn write_to(&self, output: &mut io::CodedOutput, value: &T) -> io::OutputResult {
        (self.write)(output, value)
    }

    fn read_from(&self, input: &mut io::CodedInput) -> io::InputResult<T> {
        let mut value = None;
        self.merge_from(input, &mut value)?;
        if let Some(value) = value {
            Ok(value)
        } else {
            panic!("codec did not read and assign value from coded input")
        }
    }

    #[inline]
    fn merge_from(
        &self,
        input: &mut io::CodedInput,
        value: &mut Option<T>,
    ) -> io::InputResult<()> {
        (self.merge)(input, value)
    }
}

#[doc(hidden)]
impl Codec<f32> {
    pub const fn float(tag: u32) -> Codec<f32> {
        Codec {
            default: Some(0.0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::float(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::float(*i)),
            merge: |i, v| {
                *v = Some(i.read_float()?);
                Ok(())
            },
            write: |o, v| o.write_float(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<f64> {
    pub const fn double(tag: u32) -> Codec<f64> {
        Codec {
            default: Some(0.0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::double(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::double(*i)),
            merge: |i, v| {
                *v = Some(i.read_double()?);
                Ok(())
            },
            write: |o, v| o.write_double(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<i32> {
    pub const fn int32(tag: u32) -> Codec<i32> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::int32(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::int32(*i)),
            merge: |i, v| {
                *v = Some(i.read_int32()?);
                Ok(())
            },
            write: |o, v| o.write_int32(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }

    pub const fn sint32(tag: u32) -> Codec<i32> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::sint32(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::sint32(*i)),
            merge: |i, v| {
                *v = Some(i.read_sint32()?);
                Ok(())
            },
            write: |o, v| o.write_sint32(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }

    pub const fn sfixed32(tag: u32) -> Codec<i32> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            size: ValueSize::Fixed(4),
            merge: |i, v| {
                *v = Some(i.read_sfixed32()?);
                Ok(())
            },
            write: |o, v| o.write_sfixed32(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<u32> {
    pub const fn uint32(tag: u32) -> Codec<u32> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::uint32(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::uint32(*i)),
            merge: |i, v| {
                *v = Some(i.read_uint32()?);
                Ok(())
            },
            write: |o, v| o.write_uint32(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }

    pub const fn fixed32(tag: u32) -> Codec<u32> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            size: ValueSize::Fixed(4),
            merge: |i, v| {
                *v = Some(i.read_fixed32()?);
                Ok(())
            },
            write: |o, v| o.write_fixed32(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<i64> {
    pub const fn int64(tag: u32) -> Codec<i64> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::int64(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::int64(*i)),
            merge: |i, v| {
                *v = Some(i.read_int64()?);
                Ok(())
            },
            write: |o, v| o.write_int64(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }

    pub const fn sint64(tag: u32) -> Codec<i64> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::sint64(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::sint64(*i)),
            merge: |i, v| {
                *v = Some(i.read_sint64()?);
                Ok(())
            },
            write: |o, v| o.write_sint64(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }

    pub const fn sfixed64(tag: u32) -> Codec<i64> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            size: ValueSize::Fixed(8),
            merge: |i, v| {
                *v = Some(i.read_sfixed64()?);
                Ok(())
            },
            write: |o, v| o.write_sfixed64(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<u64> {
    pub const fn uint64(tag: u32) -> Codec<u64> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|i| Some(io::sizes::uint64(*i))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|i| io::sizes::uint64(*i)),
            merge: |i, v| {
                *v = Some(i.read_uint64()?);
                Ok(())
            },
            write: |o, v| o.write_uint64(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }

    pub const fn fixed64(tag: u32) -> Codec<u64> {
        Codec {
            default: Some(0),
            start: tag,
            end: None,
            size: ValueSize::Fixed(8),
            merge: |i, v| {
                *v = Some(i.read_fixed64()?);
                Ok(())
            },
            write: |o, v| o.write_fixed64(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<bool> {
    pub const fn bool(tag: u32) -> Codec<bool> {
        Codec {
            default: Some(false),
            start: tag,
            end: None,
            size: ValueSize::Fixed(1),
            merge: |i, v| {
                *v = Some(i.read_bool()?);
                Ok(())
            },
            write: |o, v| o.write_bool(*v),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

#[doc(hidden)]
impl Codec<String> {
    pub const fn string(tag: u32) -> Codec<String> {
        Codec {
            default: Some(String::new()),
            start: tag,
            end: None,
            size: ValueSize::Func(|s| io::sizes::string(s)),
            merge: |i, v| {
                *v = Some(i.read_string()?);
                Ok(())
            },
            write: |o, v| o.write_string(v),
            packed: false,
            packable: false,
        }
    }
}

#[doc(hidden)]
impl Codec<Vec<u8>> {
    pub const fn bytes(tag: u32) -> Codec<Vec<u8>> {
        Codec {
            default: Some(Vec::new()),
            start: tag,
            end: None,
            size: ValueSize::Func(|b| io::sizes::bytes(b)),
            merge: |i, v| {
                *v = Some(i.read_bytes()?);
                Ok(())
            },
            write: |o, v| o.write_bytes(v),
            packed: false,
            packable: false,
        }
    }
}

#[doc(hidden)]
impl<M: LiteMessage> Codec<M> {
    pub const fn message(tag: u32) -> Codec<M> {
        Codec {
            default: None,
            start: tag,
            end: None,
            size: ValueSize::Func(|m| io::sizes::message(m)),
            merge: |i, v| {
                if let Some(v) = v {
                    i.read_message(v)?;
                } else {
                    let mut new = M::new();
                    i.read_message(&mut new)?;
                    *v = Some(new);
                }
                Ok(())
            },
            write: |o, v| o.write_message(v),
            packed: false,
            packable: false,
        }
    }

    pub const fn group(start: u32, end: NonZeroU32) -> Codec<M> {
        Codec {
            default: None,
            start,
            end: Some(end),
            size: ValueSize::Func(|m| io::sizes::group(m)),
            merge: |i, v| {
                if let Some(v) = v {
                    i.read_group(v)?;
                } else {
                    let mut new = M::new();
                    i.read_group(&mut new)?;
                    *v = Some(new);
                }
                Ok(())
            },
            write: |o, v| o.write_group(v),
            packed: false,
            packable: false,
        }
    }
}

#[doc(hidden)]
impl<E: Clone + Into<i32> + TryFrom<i32, Error = VariantUndefinedError>> Codec<EnumValue<E>> {
    pub const fn enum_value(tag: u32) -> Codec<EnumValue<E>> {
        Codec {
            default: Some(EnumValue::Undefined(0)),
            start: tag,
            end: None,
            #[cfg(checked_size)]
            size: ValueSize::Func(|e| Some(io::sizes::enum_value(e.clone()))),
            #[cfg(not(checked_size))]
            size: ValueSize::Func(|e| io::sizes::enum_value(e.clone())),
            merge: |i, v| {
                *v = Some(EnumValue::from(i.read_int32()?));
                Ok(())
            },
            write: |o, v| o.write_int32(Into::<i32>::into(v.clone())),
            packed: is_packed(tag),
            packable: true,
        }
    }
}

/// Contains a collection of unknown fields encountered when reading a protobuf message
#[derive(Clone, Debug, PartialEq)]
pub struct UnknownFieldSet(HashMap<Tag, UnknownField>);

#[derive(Clone, Debug, PartialEq)]
enum UnknownField {
    Varint(u64),
    Bit64(u64),
    LengthDelimited(Vec<u8>),
    Group(UnknownFieldSet),
    Bit32(u32),
}

impl UnknownFieldSet {
    #[doc(hidden)]
    pub fn new() -> UnknownFieldSet {
        UnknownFieldSet(HashMap::new())
    }

    #[doc(hidden)]
    pub fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult {
        for field in &self.0 {
            output.write_tag(*field.0)?;
            match field.1 {
                UnknownField::Varint(v) => {
                    output.write_uint64(*v)?;
                }
                UnknownField::Bit64(v) => {
                    output.write_fixed64(*v)?;
                }
                UnknownField::LengthDelimited(v) => {
                    output.write_bytes(v)?;
                }
                UnknownField::Group(v) => {
                    v.write_to(output)?;
                    output.write_tag(Tag::new(field.0.number(), WireType::EndGroup))?;
                }
                UnknownField::Bit32(v) => {
                    output.write_tag(*field.0)?;
                    output.write_fixed32(*v)?;
                }
            }
        }
        Ok(())
    }

    #[doc(hidden)]
    #[cfg(checked_size)]
    pub fn calculate_size(&self) -> Option<i32> {
        let mut size = 0i32;
        for field in &self.0 {
            size = size.checked_add(io::sizes::uint32(field.0.get()))?;
            match field.1 {
                UnknownField::Varint(v) => {
                    size = size.checked_add(io::sizes::uint64(*v))?;
                }
                UnknownField::Bit64(v) => {
                    size = size.checked_add(io::sizes::fixed64(*v))?;
                }
                UnknownField::LengthDelimited(v) => {
                    size = size.checked_add(io::sizes::bytes(v)?)?;
                }
                UnknownField::Group(v) => {
                    size = size.checked_add(v.calculate_size()?)?;
                    size = size.checked_add(io::sizes::uint32(Tag::new(field.0.number(), WireType::EndGroup)))?;
                }
                UnknownField::Bit32(v) => {
                    size = size.checked_add(io::sizes::fixed32(*v))?;
                }
            }
        }
        Some(size)
    }

    #[doc(hidden)]
    #[cfg(not(checked_size))]
    pub fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        for field in &self.0 {
            size += io::sizes::uint32(field.0.get());
            match field.1 {
                UnknownField::Varint(v) => {
                    size += io::sizes::uint64(*v);
                }
                UnknownField::Bit64(v) => {
                    size += io::sizes::fixed64(*v);
                }
                UnknownField::LengthDelimited(v) => {
                    size += io::sizes::bytes(v);
                }
                UnknownField::Group(v) => {
                    size += v.calculate_size();
                    size += io::sizes::uint32(Tag::new(field.0.number(), WireType::EndGroup).get());
                }
                UnknownField::Bit32(v) => {
                    size += io::sizes::fixed32(*v);
                }
            }
        }
        size
    }

    #[doc(hidden)]
    pub fn merge_from(&mut self, tag: Tag, input: &mut io::CodedInput) -> io::InputResult<()> {
        let wt = tag.wire_type();
        match wt {
            WireType::Varint => {
                self.0
                    .insert(tag, UnknownField::Varint(input.read_uint64()?));
            }
            WireType::Bit64 => {
                self.0
                    .insert(tag, UnknownField::Bit64(input.read_fixed64()?));
            }
            WireType::LengthDelimited => {
                self.0
                    .insert(tag, UnknownField::LengthDelimited(input.read_bytes()?));
            }
            WireType::StartGroup => {
                let end = Tag::new(tag.number(), WireType::EndGroup);
                let mut set = UnknownFieldSet::new();
                while let Some(tag) = input.read_tag()? {
                    match tag.get() {
                        _ if end == tag => break,
                        _ => set.merge_from(tag, input)?,
                    }
                }
                self.0.insert(tag, UnknownField::Group(set));
            }
            WireType::EndGroup => return Err(io::InputError::InvalidTag(tag.get())),
            WireType::Bit32 => {
                self.0
                    .insert(tag, UnknownField::Bit32(input.read_fixed32()?));
            }
        }

        Ok(())
    }

    #[doc(hidden)]
    pub fn merge(&mut self, other: &Self) {
        for field in &other.0 {
            self.0.insert(*field.0, field.1.clone());
        }
    }
}

#[cfg(test)]
mod tests {}
