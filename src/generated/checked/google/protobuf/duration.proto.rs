// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/duration.proto


pub fn file() -> &'static crate::reflect::FileDescriptor {
    super::pool().find_file_by_name("google/protobuf/duration.proto").unwrap()
}

/// A Duration represents a signed, fixed-length span of time represented
/// as a count of seconds and fractions of seconds at nanosecond
/// resolution. It is independent of any calendar and concepts like "day"
/// or "month". It is related to Timestamp in that the difference between
/// two Timestamp values is a Duration and it can be added or subtracted
/// from a Timestamp. Range is approximately +-10,000 years.
/// # Examples
/// 
/// Example 1: Compute Duration from two Timestamps in pseudo code.
/// ```text
///  Timestamp start = ...;
///  Timestamp end = ...;
///  Duration duration = ...;
/// 
///  duration.seconds = end.seconds - start.seconds;
///  duration.nanos = end.nanos - start.nanos;
/// 
///  if (duration.seconds < 0 && duration.nanos > 0) {
///    duration.seconds += 1;
///    duration.nanos -= 1000000000;
///  } else if (durations.seconds > 0 && duration.nanos < 0) {
///    duration.seconds -= 1;
///    duration.nanos += 1000000000;
///  }
/// ```
/// 
/// Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.
/// ```text
///  Timestamp start = ...;
///  Duration duration = ...;
///  Timestamp end = ...;
/// 
///  end.seconds = start.seconds + duration.seconds;
///  end.nanos = start.nanos + duration.nanos;
/// 
///  if (end.nanos < 0) {
///    end.seconds -= 1;
///    end.nanos += 1000000000;
///  } else if (end.nanos >= 1000000000) {
///    end.seconds += 1;
///    end.nanos -= 1000000000;
///  }
/// ```
/// 
/// Example 3: Compute Duration from datetime.timedelta in Python.
/// ```text
///  td = datetime.timedelta(days=3, minutes=10)
///  duration = Duration()
///  duration.FromTimedelta(td)
/// ```
/// # JSON Mapping
/// 
/// In JSON format, the Duration type is encoded as a string rather than an
/// object, where the string ends in the suffix "s" (indicating seconds) and
/// is preceded by the number of seconds, with nanoseconds expressed as
/// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
/// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
/// be expressed in JSON format as "3.000000001s", and 3 seconds and 1
/// microsecond should be expressed in JSON format as "3.000001s".
#[derive(Clone, Debug, PartialEq)]
pub struct Duration {
    seconds: i64,
    nanos: i32,
    unknown_fields: crate::UnknownFieldSet,
}
impl crate::CodedMessage for self::Duration {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.seconds = input.read_int64()?,
                16 => self.nanos = input.read_int32()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> ::std::option::Option<i32> {
        let mut size = 0i32;
        let seconds = self.seconds;
        if seconds != Self::SECONDS_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::int64(seconds));
        }
        let nanos = self.nanos;
        if nanos != Self::NANOS_DEFAULT_VALUE {
            size = size.checked_add(1)?;
            size = size.checked_add(crate::io::sizes::int32(nanos));
        }
        size = size.checked_add(self.unknown_fields.calculate_size()?)?;
        ::std::option::Option::Some(size)
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let seconds = self.seconds;
        if seconds != Self::SECONDS_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_int64(seconds)?;
        }
        let nanos = self.nanos;
        if nanos != Self::NANOS_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[16])?;
            output.write_int32(nanos)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Duration {
    fn new() -> Self {
        Self {
            seconds: Self::SECONDS_DEFAULT_VALUE,
            nanos: Self::NANOS_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new(),
        }
    }
    fn merge(&mut self, other: &Self) {
        self.seconds = other.seconds;
        self.nanos = other.nanos;
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Duration {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Duration {
    /// Gets the field number of the [`seconds`] field
    ///
    /// [`seconds`]: #method.seconds
    pub const SECONDS_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`seconds`] field
    ///
    /// [`seconds`]: #method.seconds
    pub const SECONDS_DEFAULT_VALUE: i64 = 0;
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    pub fn seconds(&self) -> i64 {
        self.seconds
    }
    /// Returns a unique reference to the [`seconds`] field
    ///
    /// [`seconds`]: #method.seconds
    pub fn seconds_mut(&mut self) -> &mut i64 {
        &mut self.seconds
    }
    /// Gets the field number of the [`nanos`] field
    ///
    /// [`nanos`]: #method.nanos
    pub const NANOS_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`nanos`] field
    ///
    /// [`nanos`]: #method.nanos
    pub const NANOS_DEFAULT_VALUE: i32 = 0;
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    pub fn nanos(&self) -> i32 {
        self.nanos
    }
    /// Returns a unique reference to the [`nanos`] field
    ///
    /// [`nanos`]: #method.nanos
    pub fn nanos_mut(&mut self) -> &mut i32 {
        &mut self.nanos
    }
}