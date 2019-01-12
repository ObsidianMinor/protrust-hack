use crate::CodedMessage;
use std::cmp::min;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::mem;
use std::num::NonZeroU32;

/// The wire type of a protobuf value
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum WireType {
    Varint = 0,
    Bit64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bit32 = 5,
}

impl WireType {
    /// Gets the wire type of a constructed tag value or None if the value does not have a valid wire type
    ///
    /// # Examples
    /// ```
    /// # use protrust::io::WireType;
    /// assert_eq!(Some(WireType::Varint), WireType::get_type(8));
    /// assert_eq!(Some(WireType::Bit64), WireType::get_type(8388609));
    /// assert_eq!(Some(WireType::LengthDelimited), WireType::get_type(536870914));
    /// assert_eq!(Some(WireType::StartGroup), WireType::get_type(772603539));
    /// assert_eq!(Some(WireType::EndGroup), WireType::get_type(772603540));
    /// assert_eq!(Some(WireType::Bit32), WireType::get_type(13));
    /// assert_eq!(None, WireType::get_type(14));
    /// assert_eq!(None, WireType::get_type(15));
    /// ```
    pub fn get_type(value: u32) -> Option<WireType> {
        match value & 0b111 {
            0 => Some(WireType::Varint),
            1 => Some(WireType::Bit64),
            2 => Some(WireType::LengthDelimited),
            3 => Some(WireType::StartGroup),
            4 => Some(WireType::EndGroup),
            5 => Some(WireType::Bit32),
            _ => None,
        }
    }

    pub const fn get_num(value: u32) -> i32 {
        (value >> 3) as i32
    }

    pub const fn make_tag(num: i32, wt: WireType) -> u32 {
        (num << 3) as u32 | wt as u32
    }

    /// Gets whether a wire type is eligable for repeated field packing
    pub fn is_packable(self) -> bool {
        return self == WireType::Bit32 || self == WireType::Bit64 || self == WireType::Varint;
    }
}

/// Provides helper functions for calculating the size of values
pub mod sizes {
    use crate::CodedMessage;

    #[inline]
    /// Gets the size of a given int32 value
    pub fn int32(value: i32) -> i32 {
        if value >= 0 {
            raw_varint32_size(value as u32)
        } else {
            10
        }
    }

    #[inline]
    pub fn enum_value<E: Into<i32> + Clone>(value: crate::EnumValue<E>) -> i32 {
        int32(value.into())
    }

    #[inline]
    /// Gets the size of a given
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
    pub fn message(value: &CodedMessage) -> Option<i32> {
        let length = value.calculate_size();
        if let Some(length) = length {
            length.checked_add(int32(length))
        } else {
            None
        }
    }

    #[inline]
    #[cfg(not(checked_size))]
    pub fn message(value: &CodedMessage) -> i32 {
        let length = value.calculate_size();
        length + int32(length)
    }

    #[inline]
    #[cfg(checked_size)]
    pub fn group(value: &CodedMessage) -> Option<i32> {
        value.calculate_size()
    }

    #[inline]
    #[cfg(not(checked_size))]
    pub fn group(value: &CodedMessage) -> i32 {
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

    /// Gets the size of float values
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

/// The error in a [InputResult](#InputResult`1)
#[derive(Debug)]
pub enum InputError {
    MalformedVarint,
    NegativeSize,
    InvalidTag,
    IoError(std::io::Error),
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

/// The result of a read from a [CodedInput](#CodedInput)
pub type InputResult<T> = Result<T, InputError>;
/*
/// A protocol buffer input stream.
pub trait CodedInput {
    /// Reads a double from the coded input
    fn read_double(&mut self) -> InputResult<f64>;
    /// Reads a float from the coded input
    fn read_float(&mut self) -> InputResult<f32>;
    /// Reads an int32 from the coded input
    fn read_int32(&mut self) -> InputResult<i32>;
    /// Reads an int64 from the coded input
    fn read_int64(&mut self) -> InputResult<i64>;
    /// Reads a uint32 from the coded input
    fn read_uint32(&mut self) -> InputResult<u32>;
    /// Reads a uint64 from the coded input
    fn read_uint64(&mut self) -> InputResult<u64>;
    /// Reads an sint32 from the coded input
    fn read_sint32(&mut self) -> InputResult<i32>;
    /// Reads an sint64 from the coded input
    fn read_sint64(&mut self) -> InputResult<i64>;
    /// Reads a fixed32 from the coded input
    fn read_fixed32(&mut self) -> InputResult<u32>;
    /// Reads a fixed64 from the coded input
    fn read_fixed64(&mut self) -> InputResult<u64>;
    /// Reads an sfixed32 from the coded input
    fn read_sfixed32(&mut self) -> InputResult<i32>;
    /// Reads an sfixed64 from the coded input
    fn read_sfixed64(&mut self) -> InputResult<i64>;
    /// Reads a bool from the coded input
    fn read_bool(&mut self) -> InputResult<bool>;
    /// Reads a string from the coded input
    fn read_string(&mut self) -> InputResult<String>;
    /// Reads a bytes value from the coded input
    fn read_bytes(&mut self) -> InputResult<Vec<u8>>;
    /// Merges the coded input into the given message
    fn read_message(&mut self, value: &mut CodedMessage) -> InputResult<()>;
    /// Merges the coded input into the given group
    fn read_group(&mut self, value: &mut CodedMessage) -> InputResult<()>;

    /// Reads a tag from the coded input
    fn read_tag(&mut self) -> InputResult<Option<NonZeroU32>>;

    /// Pushes a limit, returning the previous limit
    fn push_limit(&mut self, limit: i32) -> Option<i32>;
    /// Returns a bool indicating which the read limit has been reached
    fn reached_limit(&self) -> bool;
    /// Pops the current limit, reapplying the previous limit
    fn pop_limit(&mut self, previous: Option<i32>);
}
*/
/// A protocol buffers input stream
pub struct CodedInput<'a> {
    inner: &'a mut Read,
    limit: Option<i32>,
}

impl<'a> CodedInput<'a> {
    pub fn new(inner: &'a mut Read) -> Self {
        CodedInput { inner, limit: None }
    }

    pub fn into_inner(self) -> &'a mut Read {
        self.inner
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
    pub(crate) fn skip(&mut self, tag: u32) -> InputResult<()> {
        match WireType::get_type(tag) {
            Some(WireType::Varint) => {
                self.read_uint64()?;
            }
            Some(WireType::Bit64) => {
                self.read_fixed64()?;
            }
            Some(WireType::LengthDelimited) => {
                self.read_bytes()?;
            }
            Some(WireType::StartGroup) => {
                while let Some(tag) = self.read_tag()? {
                    self.skip(tag.get())?;
                }
            }
            Some(WireType::Bit32) => {
                self.read_fixed32()?;
            }
            _ => return Err(InputError::InvalidTag),
        }

        Ok(())
    }
    pub fn read_bool(&mut self) -> InputResult<bool> {
        Ok(self.read_uint32()? != 0)
    }
    pub fn read_message(&mut self, message: &mut CodedMessage) -> InputResult<()> {
        let len = self.read_int32()?;
        let old = self.push_limit(len);
        message.merge_from(self)?;
        self.pop_limit(old);
        Ok(())
    }
    pub fn read_group(&mut self, message: &mut CodedMessage) -> InputResult<()> {
        message.merge_from(self)
    }
    pub fn read_bytes(&mut self) -> InputResult<Vec<u8>> {
        let len = self.read_uint32()? as usize;
        let mut buf = Vec::with_capacity(len);
        unsafe {
            buf.set_len(len);
        }
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    pub fn read_string(&mut self) -> InputResult<String> {
        let bytes = self.read_bytes()?;
        Ok(String::from_utf8(bytes)?)
    }
    pub fn read_fixed32(&mut self) -> InputResult<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        unsafe { Ok(u32::from_le(mem::transmute(buf))) }
    }
    pub fn read_sfixed32(&mut self) -> InputResult<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        unsafe { Ok(i32::from_le(mem::transmute(buf))) }
    }
    pub fn read_float(&mut self) -> InputResult<f32> {
        Ok(f32::from_bits(self.read_fixed32()?))
    }
    pub fn read_fixed64(&mut self) -> InputResult<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        unsafe { Ok(u64::from_le(mem::transmute(buf))) }
    }
    pub fn read_sfixed64(&mut self) -> InputResult<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        unsafe { Ok(i64::from_le(mem::transmute(buf))) }
    }
    pub fn read_double(&mut self) -> InputResult<f64> {
        Ok(f64::from_bits(self.read_fixed64()?))
    }
    pub fn read_sint32(&mut self) -> InputResult<i32> {
        let val = self.read_uint32()?;
        Ok(((val >> 1) as i32) ^ -((val & 1) as i32))
    }
    pub fn read_sint64(&mut self) -> InputResult<i64> {
        let val = self.read_uint64()?;
        Ok(((val >> 1) as i64) ^ -((val & 1) as i64))
    }
    pub fn read_int32(&mut self) -> InputResult<i32> {
        Ok(self.read_uint32()? as i32)
    }
    pub fn read_int64(&mut self) -> InputResult<i64> {
        Ok(self.read_uint64()? as i64)
    }
    pub fn read_uint32(&mut self) -> InputResult<u32> {
        let mut shift = 0i32;
        let mut result = 0i32;
        let mut buf = [0u8; 1];
        while shift < 32 {
            self.read_exact(&mut buf)?;
            result |= ((buf[0] & 0x7F) as i32) << shift;
            if (buf[0] & 0x80) == 0 {
                return Ok(result as u32);
            }
            shift += 7;
        }
        while shift < 64 {
            self.read_exact(&mut buf)?;
            if (buf[0] & 0x80) == 0 {
                return Ok(result as u32);
            }
            shift += 7;
        }
        Err(InputError::MalformedVarint)
    }
    pub fn read_uint64(&mut self) -> InputResult<u64> {
        let mut shift = 0i32;
        let mut result = 0u64;
        let mut buf = [0u8; 1];
        while shift < 64 {
            self.read_exact(&mut buf)?;
            result |= ((buf[0] & 0x7F) << shift) as u64;
            if (buf[0] & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        Err(InputError::MalformedVarint)
    }
    pub fn read_tag(&mut self) -> InputResult<Option<NonZeroU32>> {
        let mut shift = 0i32;
        let mut result = 0i32;
        let mut buf = [0u8; 1];
        let mut in_tag = false; // the first byte we read we check for eof, after that we're in a tag and "UnexpectedEof" happens on eof
        while shift < 32 {
            if !in_tag {
                let result = self.read(&mut buf)?;
                if result == 0 {
                    return Ok(None);
                }
                in_tag = true;
            } else {
                self.read_exact(&mut buf)?;
            }
            result |= ((buf[0] & 0x7F) as i32) << shift;
            if (buf[0] & 0x80) == 0 {
                if result == 0 {
                    return Err(InputError::InvalidTag);
                } else {
                    return Ok(NonZeroU32::new(result as u32));
                }
            }
            shift += 7;
        }
        while shift < 64 {
            self.read_exact(&mut buf)?;
            if (buf[0] & 0x80) == 0 {
                return Ok(NonZeroU32::new(result as u32));
            }
            shift += 7;
        }
        Err(InputError::MalformedVarint)
    }
    pub fn read_enum_value<E: std::convert::TryFrom<i32, Error = crate::VariantUndefinedError>>(
        &mut self,
    ) -> InputResult<crate::EnumValue<E>> {
        self.read_int32().map(crate::EnumValue::from)
    }
}

/// The error of an [OutputResult](#OutputResult)
#[derive(Debug)]
pub enum OutputError {
    ValueTooLarge,
    IoError(std::io::Error),
}

impl From<std::io::Error> for OutputError {
    fn from(value: std::io::Error) -> OutputError {
        OutputError::IoError(value)
    }
}

/// The result of a write to a [CodedOutput](#CodedOutput)
pub type OutputResult = Result<(), OutputError>;

/*
/// A protocol buffer output stream
pub trait CodedOutput {
    /// Writes a double value to the coded output
    fn write_double(&mut self, value: f64) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_float(&mut self, value: f32) -> OutputResult;
    /// Writes an int32 value to the coded output
    fn write_int32(&mut self, value: i32) -> OutputResult;
    /// Writes an int64 value to the coded output
    fn write_int64(&mut self, value: i64) -> OutputResult;
    /// Writes a uint32 value to the coded output
    fn write_uint32(&mut self, value: u32) -> OutputResult;
    /// Writes a uint64 value to the coded output
    fn write_uint64(&mut self, value: u64) -> OutputResult;
    /// Writes an sint32 value to the coded output
    fn write_sint32(&mut self, value: i32) -> OutputResult;
    /// Writes an sint64 value to the coded output
    fn write_sint64(&mut self, value: i64) -> OutputResult;
    /// Writes a fixed32 value to the coded output
    fn write_fixed32(&mut self, value: u32) -> OutputResult;
    /// Writes a fixed64 value to the coded output
    fn write_fixed64(&mut self, value: u64) -> OutputResult;
    /// Writes an sfixed32 value to the coded output
    fn write_sfixed32(&mut self, value: i32) -> OutputResult;
    /// Writes an sfixed64 value to the coded output
    fn write_sfixed64(&mut self, value: i64) -> OutputResult;
    /// Writes a bool value to the coded output
    fn write_bool(&mut self, value: bool) -> OutputResult;
    /// Writes a string value to the coded output
    fn write_string(&mut self, value: &String) -> OutputResult;
    /// Writes a bytes value to the coded output
    fn write_bytes(&mut self, value: &Vec<u8>) -> OutputResult;
    /// Writes a message to the coded output
    fn write_message(&mut self, value: &CodedMessage) -> OutputResult;
    /// Writes a group to the coded output
    fn write_group(&mut self, value: &CodedMessage) -> OutputResult;

    /// Writes a float value to the coded output
    fn write_tag(&mut self, num: i32, wtype: WireType) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_raw_tag(&mut self, value: u32) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_raw_tag_bytes(&mut self, value: &[u8]) -> OutputResult;
}
*/

pub struct CodedOutput<'a> {
    inner: &'a mut Write,
}

impl<'a> CodedOutput<'a> {
    pub fn new(inner: &'a mut Write) -> Self {
        CodedOutput { inner }
    }

    pub fn into_inner(self) -> &'a mut Write {
        self.inner
    }

    pub fn write_raw_tag_bytes(&mut self, value: &[u8]) -> OutputResult {
        Ok(self.inner.write_all(value)?)
    }

    pub fn write_raw_tag(&mut self, value: u32) -> OutputResult {
        self.write_uint32(value)
    }

    pub fn write_tag(&mut self, num: i32, wtype: WireType) -> OutputResult {
        self.write_uint32(WireType::make_tag(num, wtype))
    }

    pub fn write_group(&mut self, value: &CodedMessage) -> OutputResult {
        value.write_to(self)
    }

    #[cfg(checked_size)]
    pub fn write_message(&mut self, value: &CodedMessage) -> OutputResult {
        if let Some(len) = value.calculate_size() {
            self.write_int32(len)?;
            value.write_to(self)
        } else {
            Err(OutputError::ValueTooLarge)
        }
    }

    #[cfg(not(checked_size))]
    pub fn write_message(&mut self, value: &CodedMessage) -> OutputResult {
        self.write_int32(value.calculate_size())?;
        value.write_to(self)
    }

    pub fn write_bytes(&mut self, value: &Vec<u8>) -> OutputResult {
        if let Some(len) = value.len().try_into().ok() {
            self.write_int32(len)?;
            Ok(self.inner.write_all(value)?)
        } else {
            Err(OutputError::ValueTooLarge)
        }
    }

    pub fn write_string(&mut self, value: &String) -> OutputResult {
        let slice = value.as_bytes();
        if let Some(len) = slice.len().try_into().ok() {
            self.write_int32(len)?;
            Ok(self.inner.write_all(slice)?)
        } else {
            Err(OutputError::ValueTooLarge)
        }
    }

    pub fn write_bool(&mut self, value: bool) -> OutputResult {
        Ok(self.inner.write_all(&[if value { 1 } else { 0 }])?)
    }

    pub fn write_sfixed64(&mut self, value: i64) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    pub fn write_sfixed32(&mut self, value: i32) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    pub fn write_fixed64(&mut self, value: u64) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    pub fn write_fixed32(&mut self, value: u32) -> OutputResult {
        Ok(self.inner.write_all(&value.to_le_bytes())?)
    }

    pub fn write_sint64(&mut self, value: i64) -> OutputResult {
        unsafe { self.write_int64(mem::transmute((value << 1) ^ (value >> 63))) }
    }

    pub fn write_sint32(&mut self, value: i32) -> OutputResult {
        unsafe { self.write_int32(mem::transmute((value << 1) ^ (value >> 31))) }
    }

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

    pub fn write_int64(&mut self, value: i64) -> OutputResult {
        unsafe { self.write_uint64(mem::transmute(value)) }
    }

    pub fn write_int32(&mut self, value: i32) -> OutputResult {
        if value >= 0 {
            self.write_uint32(value as u32)
        } else {
            unsafe { self.write_uint64(mem::transmute(value as i64)) }
        }
    }

    pub fn write_float(&mut self, value: f32) -> OutputResult {
        self.write_fixed32(value.to_bits())
    }

    pub fn write_double(&mut self, value: f64) -> OutputResult {
        self.write_fixed64(value.to_bits())
    }

    pub fn write_enum_value<E: Into<i32> + Clone>(
        &mut self,
        value: crate::EnumValue<E>,
    ) -> OutputResult {
        self.write_int32(value.into())
    }
}
