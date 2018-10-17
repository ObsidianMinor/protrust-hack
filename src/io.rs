use std::num::NonZeroU32;

use ::{MessageLite};

/// The wire type of a protobuf value
pub enum WireType {
    Varint = 0,
    Bit64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bit32 = 5
}

/// The error in a [InputResult](#InputResult`1)
pub struct InputError;

/// The result of a read from a [CodedInput](#CodedInput)
pub type InputResult<T> = Result<T, InputError>;

/// A protocol buffer input stream
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
    fn read_message(&mut self, &mut MessageLite) -> InputResult<()>;
    /// Merges the coded input into the given group
    fn read_group(&mut self, &mut MessageLite) -> InputResult<()>;

    /// Reads a tag from the coded input
    fn read_tag(&mut self) -> InputResult<Option<NonZeroU32>>;
}

/// The error of an [OutputResult](#OutputResult)
pub struct OutputError;

pub enum OutputErrorKind {
    InvalidTag,
}

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
    fn write_message(&mut self, &MessageLite) -> OutputResult;
    /// Writes a group to the coded output
    fn write_group(&mut self, &MessageLite) -> OutputResult;

    /// Writes a float value to the coded output
    fn write_tag(&mut self, num: i32, wtype: WireType) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_raw_tag(&mut self, value: u32) -> OutputResult;
    /// Writes a float value to the coded output
    fn write_raw_tag_bytes(&mut self, value: &[u8]) -> OutputResult;
}