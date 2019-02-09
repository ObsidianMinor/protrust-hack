// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/any.proto


pub fn file() -> &'static crate::reflect::FileDescriptor {
    super::pool().find_file_by_name("google/protobuf/any.proto").unwrap()
}

/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.
/// 
/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.
/// 
/// Example 1: Pack and unpack a message in C++.
/// ```text
///  Foo foo = ...;
///  Any any;
///  any.PackFrom(foo);
///  ...
///  if (any.UnpackTo(&foo)) {
///    ...
///  }
/// ```
/// 
/// Example 2: Pack and unpack a message in Java.
/// ```text
///  Foo foo = ...;
///  Any any = Any.pack(foo);
///  ...
///  if (any.is(Foo.class)) {
///    foo = any.unpack(Foo.class);
///  }
/// ```
/// 
/// Example 3: Pack and unpack a message in Python.
/// ```text
///  foo = Foo(...)
///  any = Any()
///  any.Pack(foo)
///  ...
///  if any.Is(Foo.DESCRIPTOR):
///    any.Unpack(foo)
///    ...
/// ```
/// 
/// Example 4: Pack and unpack a message in Go
/// ```text
///   foo := &pb.Foo{...}
///   any, err := ptypes.MarshalAny(foo)
///   ...
///   foo := &pb.Foo{}
///   if err := ptypes.UnmarshalAny(any, foo); err != nil {
///     ...
///   }
/// ```
/// 
/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".
/// # JSON
/// 
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:
/// ```text
///  package google.profile;
///  message Person {
///    string first_name = 1;
///    string last_name = 2;
///  }
/// 
///  {
///    "@type": "type.googleapis.com/google.profile.Person",
///    "firstName": <string>,
///    "lastName": <string>
///  }
/// ```
/// 
/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message [google.protobuf.Duration][]):
/// ```text
///  {
///    "@type": "type.googleapis.com/google.protobuf.Duration",
///    "value": "1.212s"
///  }
/// ```
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct Any {
    type_url: ::std::string::String,
    value: ::std::vec::Vec<u8>,
    unknown_fields: crate::UnknownFieldSet
}
impl crate::CodedMessage for self::Any {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.type_url = input.read_string()?,
                18 => self.value = input.read_bytes()?,
                _ => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        let type_url = &self.type_url;
        if type_url != Self::TYPE_URL_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::string(type_url);
        }
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            size += 1;
            size += crate::io::sizes::bytes(value);
        }
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        let type_url = &self.type_url;
        if type_url != Self::TYPE_URL_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[10])?;
            output.write_string(type_url)?;
        }
        let value = &self.value;
        if value.as_slice() != Self::VALUE_DEFAULT_VALUE {
            output.write_raw_tag_bytes(&[18])?;
            output.write_bytes(value)?;
        }
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::Any {
    fn new() -> Self {
        Self {
            type_url: ::std::string::String::new(),
            value: ::std::vec::Vec::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.type_url = other.type_url.clone();
        self.value = other.value.clone();
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::Any {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::Any {
    /// Gets the field number of the [`type_url`] field
    ///
    /// [`type_url`]: #method.type_url
    pub const TYPE_URL_FIELD_NUMBER: i32 = 1;
    /// A constant value representing the default value of the [`type_url`] field
    ///
    /// [`type_url`]: #method.type_url
    pub const TYPE_URL_DEFAULT_VALUE: &'static str = "";
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    /// 
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    /// 
    ///   * If no scheme is provided, `https` is assumed.
    ///   * An HTTP GET on the URL must yield a [google.protobuf.Type][]
    ///   value in binary format, or produce an error.
    ///   * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///   
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    /// 
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    pub fn type_url(&self) -> &::std::string::String {
        &self.type_url
    }
    /// Returns a unique reference to the [`type_url`] field
    ///
    /// [`type_url`]: #method.type_url
    pub fn type_url_mut(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }
    /// Gets the field number of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_FIELD_NUMBER: i32 = 2;
    /// A constant value representing the default value of the [`value`] field
    ///
    /// [`value`]: #method.value
    pub const VALUE_DEFAULT_VALUE: &'static [u8] = &[];
    /// Must be a valid serialized protocol buffer of the above specified type.
    pub fn value(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }
    /// Returns a unique reference to the [`value`] field
    ///
    /// [`value`]: #method.value
    pub fn value_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}