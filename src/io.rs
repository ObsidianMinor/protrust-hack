use std::mem;
use std::cmp;
use std::num::NonZeroU32;
use std::io::{Read};

use ::LiteMessage;

/// The wire type of a protobuf value
#[derive(PartialEq)]
pub enum WireType {
    Varint = 0,
    Bit64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bit32 = 5
}

impl WireType {
    pub fn get_type(value: u32) -> Option<WireType> {
        match value | 0b111 {
            0 => Some(WireType::Varint),
            1 => Some(WireType::Bit64),
            2 => Some(WireType::LengthDelimited),
            3 => Some(WireType::StartGroup),
            4 => Some(WireType::EndGroup),
            5 => Some(WireType::Bit32),
            _ => None
        }
    }

    pub const fn get_num(value: u32) -> u32 {
        value >> 3
    }

    pub const fn make_tag(num: u32, wt: WireType) -> u32 {
        (num << 3) as u32 | wt as u32
    }
}

/// Provides helper functions for calculating the size of values
pub mod sizes {
    pub fn int32(value: i32) -> i32 {
        if value >= 0 {
            raw_varint32_size(value as u32)
        } else {
            10
        }
    }

    pub fn int64(value: i64) -> i32 {
        raw_varint64_size(value as u64)
    }

    pub fn uint32(value: u32) -> i32 {
        raw_varint32_size(value)
    }

    pub fn uint64(value: u64) -> i32 {
        raw_varint64_size(value)
    }

    pub fn sint32(value: i32) -> i32 {
        raw_varint32_size(zig_zag32(value))
    }

    pub fn sint64(value: i64) -> i32 {
        raw_varint64_size(zig_zag64(value))
    }

    pub fn bool(_value: bool) -> i32 {
        1 
    }

    pub fn fixed64(_value: u64) -> i32 {
        8
    }

    pub fn sfixed64(_value: i64) -> i32 {
        8
    }

    pub fn double(_value: f64) -> i32 {
        8
    }

    pub fn string(value: &String) -> Option<i32> {
        let size = value.len();
        if size > i32::max_value() as usize {
            None
        } else {
            let length = size as i32;
            length.checked_add(int32(length))
        }
    }

    pub fn bytes(value: &Vec<u8>) -> Option<i32> {
        let size = value.len();
        if size > i32::max_value() as usize {
            None
        } else {
            let length = size as i32;
            length.checked_add(int32(length))
        }
    }

    pub fn message(value: &::LiteMessage) -> Option<i32> {
        let length = value.calculate_size();
        if let Some(length) = length {
            length.checked_add(int32(length))
        } else {
            None
        }
    }

    pub fn group(value: &::LiteMessage) -> Option<i32> {
        value.calculate_size()
    }

    pub fn fixed32(_value: u32) -> i32 {
        4
    }

    pub fn sfixed32(_value: i32) -> i32 {
        4
    }

    pub fn float(_value: f32) -> i32 {
        4
    }

    fn zig_zag32(value: i32) -> u32 {
        ((value << 1) ^ (value >> 31)) as u32
    }

    fn zig_zag64(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    fn raw_varint32_size(value: u32) -> i32 {
        if (value & (0xffffffff << 7)) == 0 {
            1
        } else 
        if (value & (0xffffffff << 14)) == 0 {
            2
        } else
        if (value & (0xffffffff << 21)) == 0 {
            3
        } else
        if (value & (0xffffffff << 28)) == 0 {
            4
        } else {
            5
        }
    }

    fn raw_varint64_size(value: u64) -> i32 {
        if (value & (0xffffffffffffffff << 7)) == 0 {
            1
        } else
        if (value & (0xffffffffffffffff << 14)) == 0 {
            2
        } else
        if (value & (0xffffffffffffffff << 21)) == 0 {
            3
        } else
        if (value & (0xffffffffffffffff << 28)) == 0 {
            4
        } else
        if (value & (0xffffffffffffffff << 35)) == 0 {
            5
        } else
        if (value & (0xffffffffffffffff << 42)) == 0 {
            6
        } else
        if (value & (0xffffffffffffffff << 49)) == 0 {
            7
        } else
        if (value & (0xffffffffffffffff << 56)) == 0 {
            8
        } else
        if (value & (0xffffffffffffffff << 63)) == 0 {
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
    InvalidString(std::string::FromUtf8Error)
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
    fn read_message(&mut self, &mut LiteMessage) -> InputResult<()>;
    /// Merges the coded input into the given group
    fn read_group(&mut self, &mut LiteMessage) -> InputResult<()>;

    /// Reads a tag from the coded input
    fn read_tag(&mut self) -> InputResult<Option<NonZeroU32>>;
}

pub struct CodedInputReader<R> {
    inner: R,
    limit: Option<i32>
}

impl<R> CodedInputReader<R> { 
    pub fn new(inner: R) -> CodedInputReader<R> {
        CodedInputReader { inner, limit: None }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> CodedInputReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(limit) = self.limit {
            if limit == 0 {
                return Ok(0);
            }

            let max = cmp::min(buf.len() as i32, limit) as usize;
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
                Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "the input ended in the middle of a field"))
            } else {
                self.limit = Some(limit - buf.len() as i32);
                self.inner.read_exact(buf)
            }
        } else {
            self.inner.read_exact(buf)
        }
    }
}

impl<R: Read> CodedInput for CodedInputReader<R> {
    fn read_bool(&mut self) -> InputResult<bool> {
        Ok(self.read_uint32()? != 0)
    }
    fn read_message(&mut self, message: &mut LiteMessage) -> InputResult<()> {
        let len = self.read_int32()?;
        match self.limit {
            Some(existing) => {
                self.limit = Some(len);
                message.merge_from(self)?;
                self.limit = Some(existing - len);
            },
            None => {
                self.limit = Some(len);
                message.merge_from(self)?;
                self.limit = None;
            }
        }
        Ok(())
    }
    fn read_group(&mut self, message: &mut LiteMessage) -> InputResult<()> { 
        // jk we just pass-through to merge_from, add verification in your own impl
        message.merge_from(self)
    }
    fn read_bytes(&mut self) -> InputResult<Vec<u8>> {
        let len = self.read_uint32()? as usize;
        let mut buf = Vec::with_capacity(len);
        unsafe { 
            buf.set_len(len);
        }
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn read_string(&mut self) -> InputResult<String> {
        let bytes = self.read_bytes()?;
        Ok(String::from_utf8(bytes)?)
    }
    fn read_fixed32(&mut self) -> InputResult<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        unsafe {
            Ok(u32::from_le(mem::transmute(buf)))
        }
    }
    fn read_sfixed32(&mut self) -> InputResult<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        unsafe {
            Ok(i32::from_le(mem::transmute(buf)))
        }
    }
    fn read_float(&mut self) -> InputResult<f32> {
        Ok(f32::from_bits(self.read_fixed32()?))
    }
    fn read_fixed64(&mut self) -> InputResult<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        unsafe {
            Ok(u64::from_le(mem::transmute(buf)))
        }
    }
    fn read_sfixed64(&mut self) -> InputResult<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        unsafe {
            Ok(i64::from_le(mem::transmute(buf)))
        }
    }
    fn read_double(&mut self) -> InputResult<f64> {
        Ok(f64::from_bits(self.read_fixed64()?))
    }
    fn read_sint32(&mut self) -> InputResult<i32> {
        let val = self.read_uint32()?;
        Ok(((val >> 1) as i32) ^ -((val & 1) as i32))
    }
    fn read_sint64(&mut self) -> InputResult<i64> {
        let val = self.read_uint64()?;
        Ok(((val >> 1) as i64) ^ -((val & 1) as i64))
    }
    fn read_int32(&mut self) -> InputResult<i32> {
        Ok(self.read_uint32()? as i32)
    }
    fn read_int64(&mut self) -> InputResult<i64> {
        Ok(self.read_uint64()? as i64)
    }
    fn read_uint32(&mut self) -> InputResult<u32> {
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
    fn read_uint64(&mut self) -> InputResult<u64> {
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
    fn read_tag(&mut self) -> InputResult<Option<NonZeroU32>> {
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
}

/// The error of an [OutputResult](#OutputResult)
pub struct OutputError;

/// The result of a write to a [CodedOutput](#CodedOutput)
pub type OutputResult = Result<(), OutputError>;

/// A protocol buffer output stream
pub trait CodedOutput {
    /// Writes a double value to the coded output
    fn write_double(&mut self, value: &f64) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_float(&mut self, value: &f32) -> OutputResult;
    /// Writes an int32 value to the coded output
    fn write_int32(&mut self, value: &i32) -> OutputResult;
    /// Writes an int64 value to the coded output
    fn write_int64(&mut self, value: &i64) -> OutputResult;
    /// Writes a uint32 value to the coded output
    fn write_uint32(&mut self, value: &u32) -> OutputResult;
    /// Writes a uint64 value to the coded output
    fn write_uint64(&mut self, value: &u64) -> OutputResult;
    /// Writes an sint32 value to the coded output
    fn write_sint32(&mut self, value: &i32) -> OutputResult;
    /// Writes an sint64 value to the coded output
    fn write_sint64(&mut self, value: &i64) -> OutputResult;
    /// Writes a fixed32 value to the coded output
    fn write_fixed32(&mut self, value: &u32) -> OutputResult;
    /// Writes a fixed64 value to the coded output
    fn write_fixed64(&mut self, value: &u64) -> OutputResult;
    /// Writes an sfixed32 value to the coded output
    fn write_sfixed32(&mut self, value: &i32) -> OutputResult;
    /// Writes an sfixed64 value to the coded output
    fn write_sfixed64(&mut self, value: &i64) -> OutputResult;
    /// Writes a bool value to the coded output
    fn write_bool(&mut self, value: &bool) -> OutputResult;
    /// Writes a string value to the coded output
    fn write_string(&mut self, value: &String) -> OutputResult;
    /// Writes a bytes value to the coded output
    fn write_bytes(&mut self, value: &Vec<u8>) -> OutputResult;
    /// Writes a message to the coded output
    fn write_message(&mut self, &LiteMessage) -> OutputResult;
    /// Writes a group to the coded output
    fn write_group(&mut self, &LiteMessage) -> OutputResult;

    /// Writes a float value to the coded output
    fn write_tag(&mut self, num: i32, wtype: WireType) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_raw_tag(&mut self, value: u32) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_raw_tag_bytes(&mut self, value: &[u8]) -> OutputResult;
}