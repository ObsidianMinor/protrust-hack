// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/timestamp.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 0]> = ::std::option::Option::None;

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut [
            10, 31, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 116, 105, 
            109, 101, 115, 116, 97, 109, 112, 46, 112, 114, 111, 116, 111, 18, 15, 103, 111, 111, 103, 108, 
            101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 34, 59, 10, 9, 84, 105, 109, 101, 115, 116, 
            97, 109, 112, 18, 24, 10, 7, 115, 101, 99, 111, 110, 100, 115, 24, 1, 32, 1, 40, 3, 
            82, 7, 115, 101, 99, 111, 110, 100, 115, 18, 20, 10, 5, 110, 97, 110, 111, 115, 24, 2, 
            32, 1, 40, 5, 82, 5, 110, 97, 110, 111, 115, 66, 126, 10, 19, 99, 111, 109, 46, 103, 
            111, 111, 103, 108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 66, 14, 84, 105, 109, 101, 
            115, 116, 97, 109, 112, 80, 114, 111, 116, 111, 80, 1, 90, 43, 103, 105, 116, 104, 117, 98, 
            46, 99, 111, 109, 47, 103, 111, 108, 97, 110, 103, 47, 112, 114, 111, 116, 111, 98, 117, 102, 
            47, 112, 116, 121, 112, 101, 115, 47, 116, 105, 109, 101, 115, 116, 97, 109, 112, 248, 1, 1, 
            162, 2, 3, 71, 80, 66, 170, 2, 30, 71, 111, 111, 103, 108, 101, 46, 80, 114, 111, 116, 
            111, 98, 117, 102, 46, 87, 101, 108, 108, 75, 110, 111, 119, 110, 84, 121, 112, 101, 115, 98, 
            6, 112, 114, 111, 116, 111, 51, 
        ].as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap()
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/timestamp.proto").unwrap());
    }
}

/// Gets the pool containing all the symbols in this proto file and its dependencies
pub fn pool() -> &'static crate::reflect::DescriptorPool<'static> {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_POOL.as_ref().unwrap()
    }
}
/// Gets the file descriptor representing the proto that created this generated file
pub fn file() -> &'static crate::reflect::FileDescriptor {
    unsafe {
        FILE_ONCE.call_once(file_once_init);
        FILE_DESCRIPTOR.as_ref().unwrap()
    }
}
/// A Timestamp represents a point in time independent of any time zone
/// or calendar, represented as seconds and fractions of seconds at
/// nanosecond resolution in UTC Epoch time. It is encoded using the
/// Proleptic Gregorian Calendar which extends the Gregorian calendar
/// backwards to year one. It is encoded assuming all minutes are 60
/// seconds long, i.e. leap seconds are "smeared" so that no leap second
/// table is needed for interpretation. Range is from
/// 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z.
/// By restricting to that range, we ensure that we can convert to
/// and from  RFC 3339 date strings.
/// See [https://www.ietf.org/rfc/rfc3339.txt](https://www.ietf.org/rfc/rfc3339.txt).
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
///     Timestamp timestamp;
///     timestamp.set_seconds(time(NULL));
///     timestamp.set_nanos(0);
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
///     struct timeval tv;
///     gettimeofday(&tv, NULL);
///
///     Timestamp timestamp;
///     timestamp.set_seconds(tv.tv_sec);
///     timestamp.set_nanos(tv.tv_usec * 1000);
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
///     FILETIME ft;
///     GetSystemTimeAsFileTime(&ft);
///     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
///     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
///     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
///     Timestamp timestamp;
///     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
///     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
///     long millis = System.currentTimeMillis();
///
///     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///         .setNanos((int) ((millis % 1000) * 1000000)).build();
///
///
/// Example 5: Compute Timestamp from current time in Python.
///
///     timestamp = Timestamp()
///     timestamp.GetCurrentTime()
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString]
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using [`strftime`](https://docs.python.org/2/library/time.html#time.strftime)
/// with the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one
/// can use the Joda Time's [`ISODateTimeFormat.dateTime()`](
/// http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime--
/// ) to obtain a formatter capable of generating timestamps in this format.
///
///
#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Timestamp {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.seconds = input.read_int64()?,
                16 => self.nanos = input.read_int32()?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let seconds = self.seconds;
        if seconds != Self::SECONDS_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int64(seconds);
        }
        let nanos = self.nanos;
        if nanos != Self::NANOS_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int32(nanos);
        }
        size += self.unknown_fields.calculate_size();
        size
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
impl crate::LiteMessage for self::Timestamp {
    fn new() -> Self {
        Self {
            seconds: Self::SECONDS_DEFAULT_VALUE,
            nanos: Self::NANOS_DEFAULT_VALUE,
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
}
impl ::std::clone::Clone for self::Timestamp {
    fn clone(&self) -> Self {
        Self {
            seconds: self.seconds.clone(),
            nanos: self.nanos.clone(),
            unknown_fields: self.unknown_fields.clone()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        self.seconds = other.seconds;
        self.nanos = other.nanos;
        self.unknown_fields.clone_from(&other.unknown_fields);
    }
}
impl crate::Message for self::Timestamp {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Timestamp {
    /// Gets the field number of the [`seconds`] field
    ///
    /// [`seconds`]: #method.seconds
    pub const SECONDS_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`seconds`] field
    ///
    /// [`seconds`]: #method.seconds
    pub const SECONDS_DEFAULT_VALUE: i64 = 0;
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
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
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
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