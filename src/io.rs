//! Contains types for reading and writing Protocol Buffer streams

use crate::CodedMessage;
use std::cmp::min;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Write};
use std::mem;
use std::num::NonZeroU32;

/// The wire type of a protobuf value.
///
/// A wire type is paired with a field number between 1 and 536,870,911 to create a tag,
/// a unique identifier for a field on the wire.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum WireType {
    /// A value read a variable length integer.
    ///
    /// See the protobuf docs for more information on this encoding: https://developers.google.com/protocol-buffers/docs/encoding#varints
    Varint = 0,
    /// A 64-bit value encoded as 8 little endian bytes
    Bit64 = 1,
    /// A length delimited value. The length is encoded as a varint
    LengthDelimited = 2,
    /// A start group tag, deprecated in proto3.
    StartGroup = 3,
    /// An end group tag, deprecated in proto3.
    EndGroup = 4,
    /// A 32-bit value encoded as 4 little endian bytes
    Bit32 = 5,
}

/// The error struct used when trying to convert from an byte to a wire type
#[derive(Debug)]
pub struct InvalidWireType;

impl WireType {
    /// Gets whether a wire type is eligible for repeated field packing
    pub fn is_packable(self) -> bool {
        return self == WireType::Bit32 || self == WireType::Bit64 || self == WireType::Varint;
    }
}

impl TryFrom<u8> for WireType {
    type Error = InvalidWireType;

    fn try_from(value: u8) -> Result<WireType, InvalidWireType> {
        match value & 0b111 {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::Bit64),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::Bit32),
            _ => Err(InvalidWireType),
        }
    }
}

/// A protobuf field number. Its value is known to be less than 536870911 and not 0.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldNumber(NonZeroU32);

impl FieldNumber {
    /// The max value of a field number as a u32
    pub const MAX_RAW: u32 = 536870911;

    /// The max value of a field number
    pub const MAX_VALUE: FieldNumber = unsafe { FieldNumber::new_unchecked(FieldNumber::MAX_RAW) };

    /// Create a field number without checking the value.
    ///
    /// # Safety
    ///
    /// The value must be a valid field number
    #[inline]
    pub const unsafe fn new_unchecked(n: u32) -> FieldNumber {
        FieldNumber(NonZeroU32::new_unchecked(n))
    }

    /// Creates a field number if the given value is not zero or more than 536870911
    #[inline]
    pub fn new(n: u32) -> Option<FieldNumber> {
        if n != 0 && n <= Self::MAX_RAW {
            unsafe { Some(FieldNumber(NonZeroU32::new_unchecked(n))) }
        } else {
            None
        }
    }

    /// Returns the value as a primitive type
    #[inline]
    pub fn get(self) -> u32 {
        self.0.get()
    }
}

impl Display for FieldNumber {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A tag containing a wire type and field number. Its value is known to not be 0, and both field number and wire type are valid values
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(NonZeroU32);

impl Tag {
    /// Create a tag without checking the value.
    ///
    /// # Safety
    ///
    /// The value must be a valid tag
    #[inline]
    pub const unsafe fn new_unchecked(n: u32) -> Tag {
        Tag(NonZeroU32::new_unchecked(n))
    }

    /// Creates a new tag if the value is not zero and has a valid field number and wire type
    ///
    /// # Examples
    ///
    /// ```
    /// use protrust::io::Tag;
    ///
    /// assert!(Tag::new_from_raw(1).is_none());
    /// assert!(Tag::new_from_raw(8).is_some());
    /// assert!(Tag::new_from_raw(16).is_some());
    /// assert!(Tag::new_from_raw(14).is_none());
    /// ```
    #[inline]
    pub fn new_from_raw(n: u32) -> Option<Tag> {
        match (n & 0b111, n >> 3) {
            // (wire type, field number)
            (6, _) | (7, _) | (_, 0) => None,
            _ => unsafe { Some(Tag(NonZeroU32::new_unchecked(n))) },
        }
    }

    /// Creates a new tag value
    #[inline]
    pub fn new(f: FieldNumber, wt: WireType) -> Tag {
        unsafe { Tag(NonZeroU32::new_unchecked((f.get() << 3) | wt as u32)) }
    }

    /// Gets the wire type from this tag
    #[inline]
    pub fn wire_type(self) -> WireType {
        WireType::try_from((self.get() & 0b111) as u8).unwrap()
    }

    /// Gets the field number from this tag
    #[inline]
    pub fn number(self) -> FieldNumber {
        unsafe { FieldNumber::new_unchecked(self.get() >> 3) }
    }

    /// Returns the value as a primitive type
    #[inline]
    pub fn get(self) -> u32 {
        self.0.get()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[doc(hidden)]
pub mod sizes {
    // a helper module for calculating sizes in generated code
    use crate::CodedMessage;

    #[inline]
    pub fn int32(value: i32) -> i32 {
        if value >= 0 {
            raw_varint32_size(value as u32)
        } else {
            10
        }
    }

    #[inline]
    pub fn enum_value<E: crate::Enum>(value: crate::EnumValue<E>) -> i32 {
        int32(value.into())
    }

    #[inline]
    pub fn int64(value: i64) -> i32 {
        raw_varint64_size(value as u64)
    }

    #[inline]
    pub fn uint32(value: u32) -> i32 {
        raw_varint32_size(value)
    }

    #[inline]
    pub fn uint64(value: u64) -> i32 {
        raw_varint64_size(value)
    }

    #[inline]
    pub fn sint32(value: i32) -> i32 {
        raw_varint32_size(zig_zag32(value))
    }

    #[inline]
    pub fn sint64(value: i64) -> i32 {
        raw_varint64_size(zig_zag64(value))
    }

    #[inline]
    pub fn bool(_value: bool) -> i32 {
        1
    }

    #[inline]
    pub fn fixed64(_value: u64) -> i32 {
        8
    }

    #[inline]
    pub fn sfixed64(_value: i64) -> i32 {
        8
    }

    #[inline]
    pub fn double(_value: f64) -> i32 {
        8
    }

    #[inline]
    #[cfg(checked_size)]
    pub fn string(value: &String) -> Option<i32> {
        let size = value.len();
        if size > i32::max_value() as usize {
            None
        } else {
            let length = size as i32;
            length.checked_add(int32(length))
        }
    }

    #[inline]
    #[cfg(not(checked_size))]
    pub fn string(value: &String) -> i32 {
        let size = value.len();
        if size > i32::max_value() as usize {
            panic!("value too large")
        } else {
            let length = size as i32;
            length + int32(length)
        }
    }

    #[inline]
    #[cfg(checked_size)]
    pub fn bytes(value: &Vec<u8>) -> Option<i32> {
        let size = value.len();
        if size > i32::max_value() as usize {
            None
        } else {
            let length = size as i32;
            length.checked_add(int32(length))
        }
    }

    #[inline]
    #[cfg(not(checked_size))]
    pub fn bytes(value: &Vec<u8>) -> i32 {
        let size = value.len();
        if size > i32::max_value() as usize {
            panic!("value too large")
        } else {
            let length = size as i32;
            length + int32(length)
        }
    }

    #[inline]
    #[cfg(checked_size)]
    pub fn message(value: &dyn CodedMessage) -> Option<i32> {
        let length = value.calculate_size();
        if let Some(length) = length {
            length.checked_add(int32(length))
        } else {
            None
        }
    }

    #[inline]
    #[cfg(not(checked_size))]
    pub fn message(value: &dyn CodedMessage) -> i32 {
        let length = value.calculate_size();
        length + int32(length)
    }

    pub use message as extension_message; // for the compiler plugin

    #[inline]
    #[cfg(checked_size)]
    pub fn group(value: &dyn CodedMessage) -> Option<i32> {
        value.calculate_size()
    }

    #[inline]
    #[cfg(not(checked_size))]
    pub fn group(value: &dyn CodedMessage) -> i32 {
        value.calculate_size()
    }

    #[inline]
    pub fn fixed32(_value: u32) -> i32 {
        4
    }

    #[inline]
    pub fn sfixed32(_value: i32) -> i32 {
        4
    }

    #[inline]
    pub fn float(_value: f32) -> i32 {
        4
    }

    #[inline]
    fn zig_zag32(value: i32) -> u32 {
        ((value << 1) ^ (value >> 31)) as u32
    }

    #[inline]
    fn zig_zag64(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    #[inline]
    fn raw_varint32_size(value: u32) -> i32 {
        if (value & (0xffffffff << 7)) == 0 {
            1
        } else if (value & (0xffffffff << 14)) == 0 {
            2
        } else if (value & (0xffffffff << 21)) == 0 {
            3
        } else if (value & (0xffffffff << 28)) == 0 {
            4
        } else {
            5
        }
    }

    #[inline]
    fn raw_varint64_size(value: u64) -> i32 {
        if (value & (0xffffffffffffffff << 7)) == 0 {
            1
        } else if (value & (0xffffffffffffffff << 14)) == 0 {
            2
        } else if (value & (0xffffffffffffffff << 21)) == 0 {
            3
        } else if (value & (0xffffffffffffffff << 28)) == 0 {
            4
        } else if (value & (0xffffffffffffffff << 35)) == 0 {
            5
        } else if (value & (0xffffffffffffffff << 42)) == 0 {
            6
        } else if (value & (0xffffffffffffffff << 49)) == 0 {
            7
        } else if (value & (0xffffffffffffffff << 56)) == 0 {
            8
        } else if (value & (0xffffffffffffffff << 63)) == 0 {
            9
        } else {
            10
        }
    }
}

/// The error type for [`CodedInput`](struct.CodedInput.html) and associated read operations
#[derive(Debug)]
pub enum InputError {
    /// The input contained a malformed variable length integer
    MalformedVarint,
    /// The input contained a length delimited value which reported it had a negative size
    NegativeSize,
    /// The input contained an invalid tag (zero or the tag had an invalid wire format)
    InvalidTag(u32),
    /// An error occured while reading from the underlying `Read` object
    IoError(std::io::Error),
    /// The input contained an invalid UTF8 string
    InvalidString(std::string::FromUtf8Error),
}

impl From<std::io::Error> for InputError {
    fn from(value: std::io::Error) -> InputError {
        InputError::IoError(value)
    }
}

impl From<std::string::FromUtf8Error> for InputError {
    fn from(value: std::string::FromUtf8Error) -> InputError {
        InputError::InvalidString(value)
    }
}

impl Display for InputError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::InputError::*;
        match self {
            MalformedVarint => write!(fmt, "the input contained an invalid variable length integer"),
            NegativeSize => write!(fmt, "the input contained a length delimited value which reported it had a negative size"),
            InvalidTag(val) => write!(fmt, "the input contained an tag that was either invalid or was unexpected at this point in the input: {}", val),
            IoError(_) => write!(fmt, "an error occured in the underlying input"),
            InvalidString(_) => write!(fmt, "the input contained an invalid UTF8 string")
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InputError::IoError(ref e) => Some(e),
            InputError::InvalidString(ref e) => Some(e),
            _ => None,
        }
    }
}

/// The result of a read from a CodedInput
pub type InputResult<T> = Result<T, InputError>;

/// A protocol buffers input stream
pub struct CodedInput<'a> {
    inner: &'a mut dyn Read,
    limit: Option<i32>,
    last_tag: Option<Tag>,
    registry: Option<&'static crate::ExtensionRegistry>,
}

impl<'a> CodedInput<'a> {
    /// Creates a new CodedInput from the specified Read instance
    pub fn new(inner: &'a mut dyn Read) -> Self {
        CodedInput {
            inner,
            limit: None,
            last_tag: None,
            registry: None,
        }
    }

    /// Sets the registry in use by this input
    pub fn with_registry(mut self, registry: Option<&'static crate::ExtensionRegistry>) -> Self {
        self.registry = registry;
        self
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(limit) = self.limit {
            if limit == 0 {
                return Ok(0);
            }

            let max = min(buf.len() as i32, limit) as usize;
            let n = self.inner.read(&mut buf[..max])?;
            self.limit = Some(limit - n as i32);
            Ok(n)
        } else {
            self.inner.read(buf)
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if let Some(limit) = self.limit {
            if buf.len() > limit as usize {
                Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "the input ended in the middle of a field",
                ))
            } else {
                self.limit = Some(limit - buf.len() as i32);
                self.inner.read_exact(buf)
            }
        } else {
            self.inner.read_exact(buf)
        }
    }

    pub(crate) fn read_length(&mut self) -> InputResult<i32> {
        let length = self.read_int32()?;
        if length < 0 {
            Err(InputError::NegativeSize)
        } else {
            Ok(length)
        }
    }

    pub(crate) fn registry(&self) -> Option<&'static crate::ExtensionRegistry> {
        self.registry
    }

    pub(crate) fn last_tag(&self) -> Option<Tag> {
        self.last_tag
    }

    pub(crate) fn push_limit(&mut self, limit: i32) -> Option<i32> {
        let old = {
            if let Some(existing) = self.limit {
                Some(existing - limit)
            } else {
                None
            }
        };
        self.limit = Some(limit);
        old
    }

    pub(crate) fn reached_limit(&self) -> bool {
        self.limit == Some(0)
    }

    pub(crate) fn pop_limit(&mut self, previous: Option<i32>) {
        mem::replace(&mut self.limit, previous);
    }

    pub(crate) fn skip(&mut self, tag: Tag) -> InputResult<()> {
        match tag.wire_type() {
            WireType::Varint => {
                self.read_uint64()?;
            }
            WireType::Bit64 => {
                self.read_fixed64()?;
            }
            WireType::LengthDelimited => {
                self.read_bytes()?;
            }
            WireType::StartGroup => {
                let end = Tag::new(tag.number(), WireType::EndGroup);
                while let Some(tag) = self.read_tag()? {
                    if tag == end {
                        break;
                    } else {
                        self.skip(tag)?;
                    }
                }
            }
            WireType::Bit32 => {
                self.read_fixed32()?;
            }
            WireType::EndGroup => return Err(InputError::InvalidTag(tag.get())),
        }

        Ok(())
    }

    /// Reads a bool value from the input
    pub fn read_bool(&mut self) -> InputResult<bool> {
        Ok(self.read_uint32()? != 0)
    }
    /// Reads a message from the input, merging it with an existing coded message
    pub fn read_message(&mut self, message: &mut dyn CodedMessage) -> InputResult<()> {
        let len = self.read_length()?;
        let old = self.push_limit(len);
        message.merge_from(self)?;
        if !self.reached_limit() {
            Err(InputError::IoError(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "the input ended in the middle of a field",
            )))
        } else {
            self.pop_limit(old);
            Ok(())
        }
    }
    #[inline(always)]
    #[doc(hidden)]
    pub fn read_extension_message(&mut self, message: &mut dyn CodedMessage) -> InputResult<()> {
        self.read_message(message)
    }
    /// Reads a group message from the input, merging it with an existing coded message
    pub fn read_group(&mut self, message: &mut dyn CodedMessage) -> InputResult<()> {
        message.merge_from(self)
    }
    /// Reads a length delimited `bytes` value from the input
    pub fn read_bytes(&mut self) -> InputResult<Vec<u8>> {
        let len = self.read_length()? as usize;
        let mut buf = Vec::with_capacity(len);
        unsafe {
            buf.set_len(len);
        }
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    /// Reads a length delimited `string` value from the input
    pub fn read_string(&mut self) -> InputResult<String> {
        let bytes = self.read_bytes()?;
        Ok(String::from_utf8(bytes)?)
    }
    /// Reads a `fixed32` value from the input
    pub fn read_fixed32(&mut self) -> InputResult<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    /// Reads an `sfixed32` value from the input
    pub fn read_sfixed32(&mut self) -> InputResult<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
    /// Reads a `float` value from the input
    pub fn read_float(&mut self) -> InputResult<f32> {
        Ok(f32::from_bits(self.read_fixed32()?))
    }
    /// Reads a `fixed64` value from the input
    pub fn read_fixed64(&mut self) -> InputResult<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    /// Reads an `sfixed64` value from the input
    pub fn read_sfixed64(&mut self) -> InputResult<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
    /// Reads a `double` value from the input
    pub fn read_double(&mut self) -> InputResult<f64> {
        Ok(f64::from_bits(self.read_fixed64()?))
    }
    /// Reads an `sint32` value from the input
    pub fn read_sint32(&mut self) -> InputResult<i32> {
        let val = self.read_uint32()?;
        Ok(((val >> 1) as i32) ^ -((val & 1) as i32))
    }
    /// Reads an `sint64` value from the input
    pub fn read_sint64(&mut self) -> InputResult<i64> {
        let val = self.read_uint64()?;
        Ok(((val >> 1) as i64) ^ -((val & 1) as i64))
    }
    /// Reads an `int32` value from the input
    pub fn read_int32(&mut self) -> InputResult<i32> {
        Ok(self.read_uint32()? as i32)
    }
    /// Reads an `int64` value from the input
    pub fn read_int64(&mut self) -> InputResult<i64> {
        Ok(self.read_uint64()? as i64)
    }
    /// Reads a `uint32` value from the input
    pub fn read_uint32(&mut self) -> InputResult<u32> {
        let mut shift = 0u32;
        let mut result = 0u32;
        let mut buf = [0u8; 1];
        while shift < 32 {
            self.read_exact(&mut buf)?;
            result |= ((buf[0] & 0x7F) as u32) << shift;
            if (buf[0] & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        while shift < 64 {
            self.read_exact(&mut buf)?;
            if (buf[0] & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        Err(InputError::MalformedVarint)
    }
    /// Reads a `uint64` value from the input
    pub fn read_uint64(&mut self) -> InputResult<u64> {
        let mut shift = 0u32;
        let mut result = 0u64;
        let mut buf = [0u8; 1];
        while shift < 64 {
            self.read_exact(&mut buf)?;
            result |= ((buf[0] & 0x7F) as u64) << shift;
            if (buf[0] & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        Err(InputError::MalformedVarint)
    }
    /// Reads a tag from the input
    pub fn read_tag(&mut self) -> InputResult<Option<Tag>> {
        let mut shift = 0u32;
        let mut result = 0u32;
        let mut buf = [0u8; 1];
        let mut in_tag = false; // the first byte we read we check for eof, after that we're in a tag and "UnexpectedEof" happens on eof
        while shift < 32 {
            if !in_tag {
                let result = self.read(&mut buf)?;
                if result == 0 {
                    let tag = None;
                    self.last_tag = tag;
                    return Ok(tag);
                }
                in_tag = true;
            } else {
                self.read_exact(&mut buf)?;
            }
            result |= ((buf[0] & 0x7F) as u32) << shift;
            if (buf[0] & 0x80) == 0 {
                return match Tag::new_from_raw(result) {
                    None => {
                        self.last_tag = None;
                        Err(InputError::InvalidTag(result))
                    }
                    tag => {
                        self.last_tag = tag;
                        Ok(tag)
                    }
                };
            }
            shift += 7;
        }
        while shift < 64 {
            self.read_exact(&mut buf)?;
            if (buf[0] & 0x80) == 0 {
                return match Tag::new_from_raw(result) {
                    None => {
                        self.last_tag = None;
                        Err(InputError::InvalidTag(result))
                    }
                    tag => {
                        self.last_tag = tag;
                        Ok(tag)
                    }
                };
            }
            shift += 7;
        }
        Err(InputError::MalformedVarint)
    }
    /// Reads an enum value from the input
    pub fn read_enum_value<E: crate::Enum>(&mut self) -> InputResult<crate::EnumValue<E>> {
        self.read_int32().map(crate::EnumValue::from)
    }
}

/// The error of an [OutputResult](#OutputResult)
#[derive(Debug)]
pub enum OutputError {
    /// The input message contained a length delimited field that was larger than the max value
    ValueTooLarge,
    /// An error occured while writing to the underlying `Write` object
    IoError(std::io::Error),
}

impl From<std::io::Error> for OutputError {
    fn from(value: std::io::Error) -> OutputError {
        OutputError::IoError(value)
    }
}

impl Display for OutputError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::OutputError::*;
        match self {
            ValueTooLarge => write!(fmt, "a contained value reported it's length in bytes exceeds 2147483647 and is too large to write as an length delimited field"),
            IoError(e) => write!(fmt, "{}", e)
        }
    }
}

impl Error for OutputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OutputError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

/// The result of a write to a [CodedOutput](#CodedOutput)
pub type OutputResult = Result<(), OutputError>;

/// A protocol buffers output stream
pub struct CodedOutput<'a> {
    inner: &'a mut dyn Write,
}

impl<'a> CodedOutput<'a> {
    /// Creates a new CodedOutput using the specified Write object
    pub fn new(inner: &'a mut dyn Write) -> Self {
        CodedOutput { inner }
    }

    #[doc(hidden)]
    pub fn write_raw_tag_bytes(&mut self, value: &[u8]) -> OutputResult {
        Ok(self.inner.write_all(value)?)
    }

    #[doc(hidden)]
    pub fn write_raw_tag(&mut self, value: u32) -> OutputResult {
        self.write_uint32(value)
    }

    /// Writes a `Tag` value to the output
    pub fn write_tag(&mut self, tag: Tag) -> OutputResult {
        self.write_uint32(tag.get())
    }

    /// Writes a group message to the output
    pub fn write_group(&mut self, value: &dyn CodedMessage) -> OutputResult {
        value.write_to(self)
    }

    /// Writes a message value to the output
    #[cfg(checked_size)]
    pub fn write_message(&mut self, value: &dyn CodedMessage) -> OutputResult {
        if let Some(len) = value.calculate_size() {
            self.write_int32(len)?;
            value.write_to(self)
        } else {
            Err(OutputError::ValueTooLarge)
        }
    }

    /// Writes a message value to the output
    #[cfg(not(checked_size))]
    pub fn write_message(&mut self, value: &dyn CodedMessage) -> OutputResult {
        self.write_int32(value.calculate_size())?;
        value.write_to(self)
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn write_extension_message(&mut self, message: &dyn CodedMessage) -> OutputResult {
        self.write_message(message)
    }

    /// Writes a length delimited `bytes` value to the output
    pub fn write_bytes(&mut self, value: &Vec<u8>) -> OutputResult {
        if let Some(len) = value.len().try_into().ok() {
            self.write_int32(len)?;
            Ok(self.inner.write_all(value)?)
        } else {
            Err(OutputError::ValueTooLarge)
        }
    }

    /// Writes a length delimited `string` value to the output
    pub fn write_string(&mut self, value: &String) -> OutputResult {
        let slice = value.as_bytes();
        if let Some(len) = slice.len().try_into().ok() {
            self.write_int32(len)?;
            Ok(self.inner.write_all(slice)?)
        } else {
            Err(OutputError::ValueTooLarge)
        }
    }

    /// Writes a `bool` value to the output
    pub fn write_bool(&mut self, value: bool) -> OutputResult {
        Ok(self.inner.write_all(&[if value { 1 } else { 0 }])?)
    }

    /// Writes a `sfixed64` value to the output
    pub fn write_sfixed64(&mut self, value: i64) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    /// Writes a `sfixed32` value to the output
    pub fn write_sfixed32(&mut self, value: i32) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    /// Writes a `fixed64` value to the output
    pub fn write_fixed64(&mut self, value: u64) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    /// Writes a `fixed32` value to the output
    pub fn write_fixed32(&mut self, value: u32) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    /// Writes a `sint64` value to the output
    pub fn write_sint64(&mut self, value: i64) -> OutputResult {
        self.write_uint64(((value << 1) ^ (value >> 63)) as u64)
    }

    /// Writes a `sint32` value to the output
    pub fn write_sint32(&mut self, value: i32) -> OutputResult {
        self.write_uint32(((value << 1) ^ (value >> 31)) as u32)
    }

    /// Writes a `uint64` value to the output
    pub fn write_uint64(&mut self, value: u64) -> OutputResult {
        let mut value = value.to_le();
        let mut buf: [u8; 1] = [0];
        while value > 127 {
            buf[0] = ((value & 0x7F) | 0x80) as u8;
            self.inner.write_all(&buf)?;
            value >>= 7;
        }
        buf[0] = value as u8;
        self.inner.write_all(&buf)?;
        Ok(())
    }

    /// Writes a `uint32` value to the output
    pub fn write_uint32(&mut self, value: u32) -> OutputResult {
        let mut value = value.to_le();
        let mut buf: [u8; 1] = [0];
        while value > 127 {
            buf[0] = ((value & 0x7F) | 0x80) as u8;
            self.inner.write_all(&buf)?;
            value >>= 7;
        }
        buf[0] = value as u8;
        self.inner.write_all(&buf)?;
        Ok(())
    }

    /// Writes an `int64` value to the output
    pub fn write_int64(&mut self, value: i64) -> OutputResult {
        unsafe { self.write_uint64(mem::transmute(value)) }
    }

    /// Writes an `int32` value to the output
    pub fn write_int32(&mut self, value: i32) -> OutputResult {
        if value >= 0 {
            self.write_uint32(value as u32)
        } else {
            unsafe { self.write_uint64(mem::transmute(value as i64)) }
        }
    }

    /// Writes a `float` value to the output
    pub fn write_float(&mut self, value: f32) -> OutputResult {
        self.write_fixed32(value.to_bits())
    }

    /// Writes a `double` value to the output
    pub fn write_double(&mut self, value: f64) -> OutputResult {
        self.write_fixed64(value.to_bits())
    }

    /// Writes an enum value to the output
    pub fn write_enum_value<E: crate::Enum>(&mut self, value: crate::EnumValue<E>) -> OutputResult {
        self.write_int32(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{CodedInput, CodedOutput, FieldNumber, InputError, InputResult, OutputResult, Tag};
    use std::fmt::Debug;

    #[test]
    fn field_number_checks() {
        fn go_round(v: u32) -> Option<u32> {
            FieldNumber::new(v).map(FieldNumber::get)
        }

        assert_eq!(go_round(FieldNumber::MAX_RAW), Some(FieldNumber::MAX_RAW));
        assert_eq!(go_round(1), Some(1));
        assert_eq!(go_round(2), Some(2));
        assert_eq!(go_round(0), None);
        assert_eq!(go_round(FieldNumber::MAX_RAW + 1), None);
    }

    #[test]
    fn tag_checks() {
        fn go_round_raw(v: u32) -> Option<u32> {
            Tag::new_from_raw(v).map(Tag::get)
        }

        fn go_round_tag(t: Tag) -> Tag {
            Tag::new(t.number(), t.wire_type())
        }

        assert_eq!(go_round_raw(0), None);
        assert_eq!(go_round_raw(1), None);
        assert_eq!(go_round_raw(6), None);
        assert_eq!(go_round_raw(33), Some(33));
        let mut t;
        unsafe {
            for value in &[33, 8] {
                t = Tag::new_unchecked(*value);
                assert_eq!(go_round_tag(t), t);
            }
        }
    }

    // fuzzy roundtrips

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    fn roundtrip<T: Debug + PartialEq, W, R>(values: &[T], write: W, read: R) -> Result
    where
        W: Fn(&mut CodedOutput, &T) -> OutputResult,
        R: Fn(&mut CodedInput) -> InputResult<T>,
    {
        let mut coded_vec = Vec::new();
        let mut output = CodedOutput::new(&mut coded_vec);

        for value in values {
            write(&mut output, value)?;
        }

        let mut slice = coded_vec.as_slice();
        let mut input = CodedInput::new(&mut slice);
        let mut returned_values = std::collections::VecDeque::with_capacity(values.len());

        for _ in 0..values.len() {
            returned_values.push_back(read(&mut input)?);
        }

        assert_eq!(returned_values, values);

        Ok(())
    }

    #[test]
    fn roundtrip_double() -> Result {
        roundtrip(
            &[
                0.0,
                1.0,
                0.1,
                -1.0,
                std::f64::INFINITY,
                std::f64::MAX,
                std::f64::MIN,
                std::f64::NEG_INFINITY,
            ],
            |o, v| o.write_double(*v),
            |i| i.read_double(),
        )
    }

    #[test]
    fn roundtrip_float() -> Result {
        roundtrip(
            &[
                0.0,
                1.0,
                0.1,
                -1.0,
                std::f32::INFINITY,
                std::f32::MAX,
                std::f32::MIN,
                std::f32::NEG_INFINITY,
            ],
            |o, v| o.write_float(*v),
            |i| i.read_float(),
        )
    }

    #[test]
    fn roundtrip_int32() -> Result {
        roundtrip(
            &[0, 1, 2, 3, -1, std::i32::MIN, std::i32::MAX],
            |o, v| o.write_int32(*v),
            |i| i.read_int32(),
        )
    }

    #[test]
    fn roundtrip_int64() -> Result {
        roundtrip(
            &[0, 1, 2, 3, -1, std::i64::MIN, std::i64::MAX],
            |o, v| o.write_int64(*v),
            |i| i.read_int64(),
        )
    }

    #[test]
    fn roundtrip_uint32() -> Result {
        roundtrip(
            &[0, 1, 2, 3, std::u32::MAX],
            |o, v| o.write_uint32(*v),
            |i| i.read_uint32(),
        )
    }

    #[test]
    fn roundtrip_uint64() -> Result {
        roundtrip(
            &[0, 1, 2, 3, std::u64::MAX],
            |o, v| o.write_uint64(*v),
            |i| i.read_uint64(),
        )
    }

    #[test]
    fn roundtrip_sint32() -> Result {
        roundtrip(
            &[0, 1, 2, 3, -1, std::i32::MIN, std::i32::MAX],
            |o, v| o.write_sint32(*v),
            |i| i.read_sint32(),
        )
    }

    #[test]
    fn roundtrip_sint64() -> Result {
        roundtrip(
            &[0, 1, 2, 3, -1, std::i64::MIN, std::i64::MAX],
            |o, v| o.write_sint64(*v),
            |i| i.read_sint64(),
        )
    }

    #[test]
    fn roundtrip_fixed32() -> Result {
        roundtrip(
            &[0, 1, 2, 3, std::u32::MAX],
            |o, v| o.write_fixed32(*v),
            |i| i.read_fixed32(),
        )
    }

    #[test]
    fn roundtrip_fixed64() -> Result {
        roundtrip(
            &[0, 1, 2, 3, std::u64::MAX],
            |o, v| o.write_fixed64(*v),
            |i| i.read_fixed64(),
        )
    }

    #[test]
    fn roundtrip_sfixed32() -> Result {
        roundtrip(
            &[0, 1, 2, 3, -1, std::i32::MIN, std::i32::MAX],
            |o, v| o.write_sfixed32(*v),
            |i| i.read_sfixed32(),
        )
    }

    #[test]
    fn roundtrip_sfixed64() -> Result {
        roundtrip(
            &[0, 1, 2, 3, -1, std::i64::MIN, std::i64::MAX],
            |o, v| o.write_sfixed64(*v),
            |i| i.read_sfixed64(),
        )
    }

    #[test]
    fn roundtrip_bool() -> Result {
        roundtrip(&[true, false], |o, v| o.write_bool(*v), |i| i.read_bool())
    }

    #[test]
    fn roundtrip_string() -> Result {
        roundtrip(
            &["Hello world".to_string(), "".to_string()],
            |o, v| o.write_string(v),
            |i| i.read_string(),
        )
    }

    #[test]
    fn roundtrip_bytes() -> Result {
        roundtrip(
            &[vec![1, 2, 3]],
            |o, v| o.write_bytes(v),
            |i| i.read_bytes(),
        )
    }

    #[test]
    fn malformed_varint_returns_err() {
        let data = [255; 11];
        let mut slice: &[u8] = &data;
        let mut input = CodedInput::new(&mut slice);

        match input.read_int32() {
            Err(InputError::MalformedVarint) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn negative_size_returns_err() -> Result {
        let mut data = Vec::new();
        let mut output = CodedOutput::new(&mut data);
        output.write_int32(-1)?;

        let mut slice = data.as_slice();
        let mut input = CodedInput::new(&mut slice);

        match input.read_bytes() {
            Err(InputError::NegativeSize) => Ok(()),
            Ok(_) => {
                assert!(false, "read_bytes returned true");
                unreachable!()
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    #[test]
    fn bad_tag_returns_err() {
        let data = [0u8];
        let mut slice: &[u8] = &data;
        let mut input = CodedInput::new(&mut slice);

        match input.read_tag() {
            Err(InputError::InvalidTag(0)) => {}
            _ => assert!(false, "read_tag didn't return an invalid tag error"),
        }
    }

    #[test]
    fn eof_tag_returns_none() {
        let data: [u8; 0] = [];
        let mut slice: &[u8] = &data;
        let mut input = CodedInput::new(&mut slice);

        match input.read_tag() {
            Ok(None) => {}
            _ => assert!(false, "read_tag didn't return none"),
        }
    }

    #[test]
    fn eof_in_tag_returns_err() {
        let data = [255u8];
        let mut slice: &[u8] = &data;
        let mut input = CodedInput::new(&mut slice);

        match input.read_tag() {
            Err(InputError::IoError(_)) => {}
            _ => assert!(false, "read_tag didn't error out"),
        }
    }

    #[test]
    fn eof_with_limit_in_varint_value_returns_err() {
        let data = [255u8];
        let mut slice: &[u8] = &data;
        let mut input = CodedInput::new(&mut slice);
        input.limit = Some(1);

        match input.read_uint64() {
            Err(InputError::IoError(_)) => {}
            _ => assert!(false, "read_uint64 didn't error out"),
        }
    }
}
