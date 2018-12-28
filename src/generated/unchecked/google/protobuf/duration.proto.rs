//! DO NOT EDIT!
//! Generated by protoc-gen-rust, part of the protrust crate.
//! 
//! Source: google/protobuf/duration.proto


#[derive(Clone, PartialEq)]
pub struct Duration {
    pub seconds: i64,
    pub nanos: i32,
    _unknown_fields: crate::UnknownFieldSet
}
static DURATION_SECONDS_DEFAULT_VALUE: i64 = 0;
static DURATION_NANOS_DEFAULT_VALUE: i32 = 0;
impl crate::CodedMessage for self::Duration {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                8 => self.seconds = input.read_int64()?,
                16 => self.nanos = input.read_int32()?,
                tag => self._unknown_fields.merge_from(tag, input)?
            }
        }
        std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let seconds = self.seconds;
        if seconds != DURATION_SECONDS_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int64(seconds);
        }
        let nanos = self.nanos;
        if nanos != DURATION_NANOS_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::int32(nanos);
        }
        size += self._unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let seconds = self.seconds;
        if seconds != DURATION_SECONDS_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[8])?;
            output.write_int64(seconds)?;
        }
        let nanos = self.nanos;
        if nanos != DURATION_NANOS_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[16])?;
            output.write_int32(nanos)?;
        }
        self._unknown_fields.write_to(output)?;
        std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Duration {
    fn new() -> Self {
        Self {
            seconds: DURATION_SECONDS_DEFAULT_VALUE,
            nanos: DURATION_NANOS_DEFAULT_VALUE,
            _unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.seconds = other.seconds;
        self.nanos = other.nanos;
        self._unknown_fields.merge(&other._unknown_fields);
    }
}
impl crate::Message for self::Duration {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        unimplemented!()
    }
}
impl self::Duration {
    /// Gets the field number of the 'seconds' field
    pub const SECONDS_FIELD_NUMBER: i32 = 1;
    /// Gets the field number of the 'nanos' field
    pub const NANOS_FIELD_NUMBER: i32 = 2;
}