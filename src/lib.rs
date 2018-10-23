#![feature(const_fn)]
#![feature(const_string_new)]
#![feature(const_vec_new)]
#![feature(try_from)]

use std::num::NonZeroU32;
use std::convert::TryFrom;
use std::hash::Hash;

/// Provides modules for using vectors and hashmaps as repeated fields and map fields
pub mod collections;
/// Provides input output types for reading and writing protobuf streams
pub mod io;
/// Provides runtime support for well known types
pub mod wkt;
pub mod descriptor;
/// Provides reflection acccess for messages
pub mod reflect;
/// Provides types for LITE_RUNTIME optimized proto files
pub mod litegen {
    pub use LiteMessage;
    pub use GeneratedLiteMessage;
    pub use reflect::LiteDescriptor;
    pub use io::CodedInput;
    pub use io::InputResult;
    pub use io::CodedOutput;
    pub use io::OutputResult;
    pub use io::sizes;
    pub use collections;
    pub use Codec;
    pub use EnumValue;
    pub use VariantUndefinedError;
    pub use std::convert::TryFrom;
    pub use std::convert::From;
    pub use UnknownFieldSet;
}
/// Provides types for CODE_SIZE and SPEED optimized proto files
pub mod codegen {
    pub use ::litegen::*;
    pub use Message;
    pub use GeneratedMessage;
    pub use reflect::Descriptor;
}

/// A message with all the required information to merge, write, size, and get basic reflection information
pub trait LiteMessage {
    fn is_initialized(&self) -> bool { true }
    /// Gets a lite descriptor of this message
    fn descriptor(&self) -> &reflect::LiteDescriptor { unimplemented!() }
    /// Merges fields from the coded input into this message
    fn merge_from(&mut self, input: &mut io::CodedInput) -> io::InputResult<()>;
    /// Calculates the size of the message and returns it as an 32-bit integer or None if the message is larger than `i32::MAX`
    fn calculate_size(&self) -> Option<i32>;
    /// Writes the fields of this message to the coded output
    fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult;
}

/// A generated lite message
pub trait GeneratedLiteMessage : LiteMessage + Clone + PartialEq {
    /// Creates a new instance of the message
    fn new() -> Self;
    /// Gets the lite descriptor for this message
    fn descriptor() -> &'static reflect::LiteDescriptor { unimplemented!() }
    /// Merges two messages of the same type, copying fields from the other message
    fn merge(&mut self, other: &Self);

    /// Creates a new instance of the message and then merges from the specified coded input
    fn merge_new(input: &mut io::CodedInput) -> io::InputResult<Self> {
        let mut val = Self::new();
        val.merge_from(input)?;
        Ok(val)
    }
}

/// A message allowing access to its descriptor
pub trait Message : LiteMessage {
    /// Gets the descriptor for this message
    fn descriptor(&self) -> &reflect::Descriptor { unimplemented!() }
}

/// A generated message allowing full static type access to its descriptor
pub trait GeneratedMessage : Message + GeneratedLiteMessage {
    /// Gets the descriptor for this message
    fn descriptor() -> &'static reflect::Descriptor { unimplemented!() }
}

#[derive(Clone, PartialEq, Hash)]
pub enum EnumValue<E: Clone + PartialEq + Hash + Into<i32> + TryFrom<i32, Error = VariantUndefinedError>> {
    Defined(E),
    Undefined(i32)
}

/// The error result for when an enum value is undefined
pub struct VariantUndefinedError;

impl<E: Clone + PartialEq + Hash + Into<i32> + TryFrom<i32, Error = VariantUndefinedError>> From<i32> for EnumValue<E> {
    fn from(value: i32) -> EnumValue<E> {
        if let Ok(e) = E::try_from(value) {
            EnumValue::Defined(e)
        } else {
            EnumValue::Undefined(value)
        }
    }
}

impl<E: Clone + PartialEq + Hash + Into<i32> + TryFrom<i32, Error = VariantUndefinedError>> From<EnumValue<E>> for i32 {
    fn from(value: EnumValue<E>) -> i32 {
        match value {
            EnumValue::Defined(ref e) => Into::<i32>::into(e.clone()),
            EnumValue::Undefined(v) => v
        }
    }
}

pub struct Codec<T: Clone + PartialEq> {
    default: Option<T>,
    start: u32,
    end: Option<NonZeroU32>,
    size: ValueSize<T>,
    merge: fn(&mut io::CodedInput, &mut Option<T>) -> io::InputResult<()>,
    write: fn(&mut io::CodedOutput, &T) -> io::OutputResult,
    packed: bool,
    packable: bool
}

enum ValueSize<T: Clone + PartialEq> {
    Fixed(i32),
    Func(fn(&T) -> Option<i32>)
}

const fn is_packed(tag: u32) -> bool {
    (tag | 0b111) == 2
}

impl<T: Clone + PartialEq> Codec<T> {
    /// Gets a Some bool indicating if this type is packed or not, or None if the tag is invalid
    pub fn packed(&self, tag: u32) -> Option<bool> {
        Some(self.packable && io::WireType::get_type(tag)? == io::WireType::LengthDelimited)
    }

    /// Gets the tag of the codec or the start tag for groups
    pub fn tag(&self) -> &u32 { &self.start }

    /// Gets the end tag of the codec (groups only)
    pub fn end_tag(&self) -> &Option<NonZeroU32> { &self.end }

    /// Gets whether the value is default and should be written to an output
    pub fn is_default(&self, value: &T) -> bool {
        if let Some(ref default) = self.default {
            default == value
        } else {
            false
        }
    }

    /// Calculates the size of the value
    pub fn calculate_size(&self, value: &T) -> Option<i32> {
        match self.size {
            ValueSize::Fixed(s) => Some(s),
            ValueSize::Func(f) => (f)(value)
        }
    }

    pub fn write_to(&self, output: &mut io::CodedOutput, value: &T) -> io::OutputResult {
        (self.write)(output, value)
    }

    pub fn read_from(&self, input: &mut io::CodedInput) -> io::InputResult<T> {
        let mut value = None;
        self.merge_from(input, &mut value)?;
        if let Some(value) = value {
            Ok(value)
        } else {
            panic!("codec did not read and assign value from coded input")
        }
    }

    pub fn merge_from(&self, input: &mut io::CodedInput, value: &mut Option<T>) -> io::InputResult<()> {
        (self.merge)(input, value)
    }

    pub const fn double(def: f64, tag: u32) -> Codec<f64> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::double(*i))),
            merge: |i,v| { *v = Some(i.read_double()?); Ok(()) },
            write: |o,v| o.write_double(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn float(def: f32, tag: u32) -> Codec<f32> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::float(*i))),
            merge: |i,v| { *v = Some(i.read_float()?); Ok(()) },
            write: |o,v| o.write_float(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn int32(def: i32, tag: u32) -> Codec<i32> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::int32(*i))),
            merge: |i,v| { *v = Some(i.read_int32()?); Ok(()) },
            write: |o,v| o.write_int32(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn int64(def: i64, tag: u32) -> Codec<i64> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::int64(*i))),
            merge: |i,v| { *v = Some(i.read_int64()?); Ok(()) },
            write: |o,v| o.write_int64(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn uint32(def: u32, tag: u32) -> Codec<u32> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::uint32(*i))),
            merge: |i,v| { *v = Some(i.read_uint32()?); Ok(()) },
            write: |o,v| o.write_uint32(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn uint64(def: u64, tag: u32) -> Codec<u64> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::uint64(*i))),
            merge: |i,v| { *v = Some(i.read_uint64()?); Ok(()) },
            write: |o,v| o.write_uint64(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn sint32(def: i32, tag: u32) -> Codec<i32> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::sint32(*i))),
            merge: |i,v| { *v = Some(i.read_sint32()?); Ok(()) },
            write: |o,v| o.write_sint32(v),
            packed: is_packed(tag),
            packable: true
        }
    }
    
    pub const fn sint64(def: i64, tag: u32) -> Codec<i64> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|i| Some(io::sizes::sint64(*i))),
            merge: |i,v| { *v = Some(i.read_sint64()?); Ok(()) },
            write: |o,v| o.write_sint64(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn fixed32(def: u32, tag: u32) -> Codec<u32> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Fixed(4),
            merge: |i,v| { *v = Some(i.read_fixed32()?); Ok(()) },
            write: |o,v| o.write_fixed32(v),
            packed: is_packed(tag),
            packable: true
        }
    }
    
    pub const fn fixed64(def: u64, tag: u32) -> Codec<u64> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Fixed(8),
            merge: |i,v| { *v = Some(i.read_fixed64()?); Ok(()) },
            write: |o,v| o.write_fixed64(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn sfixed32(def: i32, tag: u32) -> Codec<i32> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Fixed(4),
            merge: |i,v| { *v = Some(i.read_sfixed32()?); Ok(()) },
            write: |o,v| o.write_sfixed32(v),
            packed: is_packed(tag),
            packable: true
        }
    }
    
    pub const fn sfixed64(def: i64, tag: u32) -> Codec<i64> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Fixed(8),
            merge: |i,v| { *v = Some(i.read_sfixed64()?); Ok(()) },
            write: |o,v| o.write_sfixed64(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn bool(def: bool, tag: u32) -> Codec<bool> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Fixed(1),
            merge: |i,v| { *v = Some(i.read_bool()?); Ok(()) },
            write: |o,v| o.write_bool(v),
            packed: is_packed(tag),
            packable: true
        }
    }

    pub const fn string(def: String, tag: u32) -> Codec<String> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|s| io::sizes::string(s)),
            merge: |i,v| { *v = Some(i.read_string()?); Ok(()) },
            write: |o,v| o.write_string(v),
            packed: false,
            packable: false
        }
    }

    pub const fn bytes(def: Vec<u8>, tag: u32) -> Codec<Vec<u8>> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|b| io::sizes::bytes(b)),
            merge: |i,v| { *v = Some(i.read_bytes()?); Ok(()) },
            write: |o,v| o.write_bytes(v),
            packed: false,
            packable: false
        }
    }

    pub const fn message<M: GeneratedLiteMessage>(tag: u32) -> Codec<M> {
        Codec {
            default: None,
            start: tag,
            end: None,
            size: ValueSize::Func(|m| io::sizes::message(m)),
            merge: |i,v| {
                if let Some(v) = v {
                    i.read_message(v)?;
                } else {
                    let mut new = M::new();
                    i.read_message(&mut new)?;
                    *v = Some(new);
                }
                Ok(())
            },
            write: |o,v| o.write_message(v),
            packed: false,
            packable: false
        }
    }

    pub const fn group<M: GeneratedLiteMessage>(start: u32, end: NonZeroU32) -> Codec<M> {
        Codec {
            default: None,
            start,
            end: Some(end),
            size: ValueSize::Func(|m| io::sizes::group(m)),
            merge: |i,v| {
                if let Some(v) = v {
                    i.read_group(v)?;
                } else {
                    let mut new = M::new();
                    i.read_group(&mut new)?;
                    *v = Some(new);
                }
                Ok(())
            },
            write: |o,v| o.write_group(v),
            packed: false,
            packable: false
        }
    }

    pub const fn enum_value<E: Clone + PartialEq + Hash + Into<i32> + TryFrom<i32, Error = VariantUndefinedError>>(def: EnumValue<E>, tag: u32) -> Codec<EnumValue<E>> {
        Codec {
            default: Some(def),
            start: tag,
            end: None,
            size: ValueSize::Func(|e| Some(io::sizes::int32(Into::<i32>::into(e.clone())))),
            merge: |i,v| { *v = Some(EnumValue::from(i.read_int32()?)); Ok(()) },
            write: |o,v| { o.write_int32(&Into::<i32>::into(v.clone())) },
            packed: is_packed(tag),
            packable: true
        }
    }
}

pub enum UnknownField {
    Varint(u64),
    Bit64(u64),
    LengthDelimited(Vec<u8>),
    Group(UnknownFieldSet),
    Bit32(u32)
}

#[derive(Clone, PartialEq)]
pub struct UnknownFieldSet;

impl UnknownFieldSet {
    pub fn new() -> UnknownFieldSet {
        UnknownFieldSet { }
    }
}

#[cfg(test)]
mod tests {

}
