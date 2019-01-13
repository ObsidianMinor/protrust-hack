// DO NOT EDIT!
// Generated by protoc-gen-rust, part of the protrust crate.
//
// Source: google/protobuf/field_mask.proto

static FILE_ONCE: ::std::sync::Once = ::std::sync::Once::new();
static mut FILE_POOL: ::std::option::Option<crate::reflect::DescriptorPool<'static>> = ::std::option::Option::None;
static mut FILE_PROTO: ::std::option::Option<[crate::descriptor::FileDescriptorProto; 1]> = ::std::option::Option::None;
static mut FILE_DESCRIPTOR: ::std::option::Option<&'static crate::reflect::FileDescriptor> = ::std::option::Option::None;
static mut FILE_DEPS: ::std::option::Option<[&'static crate::reflect::DescriptorPool<'static>; 0]> = ::std::option::Option::None;
static FILE_BINARY: &'static [u8] = &[
    10, 32, 103, 111, 111, 103, 108, 101, 47, 112, 114, 111, 116, 111, 98, 117, 102, 47, 102, 105, 
    101, 108, 100, 95, 109, 97, 115, 107, 46, 112, 114, 111, 116, 111, 18, 15, 103, 111, 111, 103, 
    108, 101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 34, 33, 10, 9, 70, 105, 101, 108, 100, 
    77, 97, 115, 107, 18, 20, 10, 5, 112, 97, 116, 104, 115, 24, 1, 32, 3, 40, 9, 82, 
    5, 112, 97, 116, 104, 115, 66, 137, 1, 10, 19, 99, 111, 109, 46, 103, 111, 111, 103, 108, 
    101, 46, 112, 114, 111, 116, 111, 98, 117, 102, 66, 14, 70, 105, 101, 108, 100, 77, 97, 115, 
    107, 80, 114, 111, 116, 111, 80, 1, 90, 57, 103, 111, 111, 103, 108, 101, 46, 103, 111, 108, 
    97, 110, 103, 46, 111, 114, 103, 47, 103, 101, 110, 112, 114, 111, 116, 111, 47, 112, 114, 111, 
    116, 111, 98, 117, 102, 47, 102, 105, 101, 108, 100, 95, 109, 97, 115, 107, 59, 102, 105, 101, 
    108, 100, 95, 109, 97, 115, 107, 162, 2, 3, 71, 80, 66, 170, 2, 30, 71, 111, 111, 103, 
    108, 101, 46, 80, 114, 111, 116, 111, 98, 117, 102, 46, 87, 101, 108, 108, 75, 110, 111, 119, 
    110, 84, 121, 112, 101, 115, 98, 6, 112, 114, 111, 116, 111, 51, 
];

fn file_once_init() {
    unsafe {
        FILE_PROTO = ::std::option::Option::Some([crate::LiteMessage::read_new(&mut FILE_BINARY.as_ref()).expect("Could not read file descriptor")]);
        FILE_DEPS = ::std::option::Option::Some([]);
        FILE_POOL = ::std::option::Option::Some(crate::reflect::DescriptorPool::build_generated_pool(
            FILE_PROTO.as_ref().unwrap(),
            FILE_DEPS.as_ref().unwrap(),
            crate::reflect::GeneratedCodeInfo {
                structs: ::std::option::Option::Some(::std::boxed::Box::new([
                    crate::reflect::GeneratedStructInfo {
                        new: || ::std::boxed::Box::new(<self::FieldMask as crate::LiteMessage>::new()),
                        structs: ::std::option::Option::None,
                    },
                ])),
            }
        ));
        FILE_DESCRIPTOR = ::std::option::Option::Some(FILE_POOL.as_ref().unwrap().find_file_by_name("google/protobuf/field_mask.proto").unwrap());
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
/// `FieldMask` represents a set of symbolic field paths, for example:
/// ```text
///  paths: "f.a"
///  paths: "f.b.d"
/// ```
/// 
/// Here `f` represents a field in some root message, `a` and `b`
/// fields in the message found in `f`, and `d` a field found in the
/// message in `f.b`.
/// 
/// Field masks are used to specify a subset of fields that should be
/// returned by a get operation or modified by an update operation.
/// Field masks also have a custom JSON encoding (see below).
/// #Field Masks in Projections
/// 
/// When used in the context of a projection, a response message or
/// sub-message is filtered by the API to only contain those fields as
/// specified in the mask. For example, if the mask in the previous
/// example is applied to a response message as follows:
/// ```text
///  f {
///    a : 22
///    b {
///      d : 1
///      x : 2
///    }
///    y : 13
///  }
///  z: 8
/// ```
/// 
/// The result will not contain specific values for fields x,y and z
/// (their value will be set to the default, and omitted in proto text
/// output):
/// ```text
///  f {
///    a : 22
///    b {
///      d : 1
///    }
///  }
/// ```
/// 
/// A repeated field is not allowed except at the last position of a
/// paths string.
/// 
/// If a FieldMask object is not present in a get operation, the
/// operation applies to all fields (as if a FieldMask of all fields
/// had been specified).
/// 
/// Note that a field mask does not necessarily apply to the
/// top-level response message. In case of a REST get operation, the
/// field mask applies directly to the response, but in case of a REST
/// list operation, the mask instead applies to each individual message
/// in the returned resource list. In case of a REST custom method,
/// other definitions may be used. Where the mask applies will be
/// clearly documented together with its declaration in the API.  In
/// any case, the effect on the returned resource/resources is required
/// behavior for APIs.
/// #Field Masks in Update Operations
/// 
/// A field mask in update operations specifies which fields of the
/// targeted resource are going to be updated. The API is required
/// to only change the values of the fields as specified in the mask
/// and leave the others untouched. If a resource is passed in to
/// describe the updated values, the API ignores the values of all
/// fields not covered by the mask.
/// 
/// If a repeated field is specified for an update operation, the existing
/// repeated values in the target resource will be overwritten by the new values.
/// Note that a repeated field is only allowed in the last position of a `paths`
/// string.
/// 
/// If a sub-message is specified in the last position of the field mask for an
/// update operation, then the existing sub-message in the target resource is
/// overwritten. Given the target message:
/// ```text
///  f {
///    b {
///      d : 1
///      x : 2
///    }
///    c : 1
///  }
/// ```
/// 
/// And an update message:
/// ```text
///  f {
///    b {
///      d : 10
///    }
///  }
/// ```
/// 
/// then if the field mask is:
/// 
/// paths: "f.b"
/// 
/// then the result will be:
/// ```text
///  f {
///    b {
///      d : 10
///    }
///    c : 1
///  }
/// ```
/// 
/// However, if the update mask was:
/// 
/// paths: "f.b.d"
/// 
/// then the result would be:
/// ```text
///  f {
///    b {
///      d : 10
///      x : 2
///    }
///    c : 1
///  }
/// ```
/// 
/// In order to reset a field's value to the default, the field must
/// be in the mask and set to the default value in the provided resource.
/// Hence, in order to reset all fields of a resource, provide a default
/// instance of the resource and set all fields in the mask, or do
/// not provide a mask as described below.
/// 
/// If a field mask is not present on update, the operation applies to
/// all fields (as if a field mask of all fields has been specified).
/// Note that in the presence of schema evolution, this may mean that
/// fields the client does not know and has therefore not filled into
/// the request will be reset to their default. If this is unwanted
/// behavior, a specific service may require a client to always specify
/// a field mask, producing an error if not.
/// 
/// As with get operations, the location of the resource which
/// describes the updated values in the request message depends on the
/// operation kind. In any case, the effect of the field mask is
/// required to be honored by the API.
/// ##Considerations for HTTP REST
/// 
/// The HTTP kind of an update operation which uses a field mask must
/// be set to PATCH instead of PUT in order to satisfy HTTP semantics
/// (PUT must only be used for full updates).
/// #JSON Encoding of Field Masks
/// 
/// In JSON, a field mask is encoded as a single string where paths are
/// separated by a comma. Fields name in each path are converted
/// to/from lower-camel naming conventions.
/// 
/// As an example, consider the following message declarations:
/// ```text
///  message Profile {
///    User user = 1;
///    Photo photo = 2;
///  }
///  message User {
///    string display_name = 1;
///    string address = 2;
///  }
/// ```
/// 
/// In proto a field mask for `Profile` may look as such:
/// ```text
///  mask {
///    paths: "user.display_name"
///    paths: "photo"
///  }
/// ```
/// 
/// In JSON, the same mask is represented as below:
/// ```text
///  {
///    mask: "user.displayName,photo"
///  }
/// ```
/// #Field Masks and Oneof Fields
/// 
/// Field masks treat fields in oneofs just as regular fields. Consider the
/// following message:
/// ```text
///  message SampleMessage {
///    oneof test_oneof {
///      string name = 4;
///      SubMessage sub_message = 9;
///    }
///  }
/// ```
/// 
/// The field mask can be:
/// ```text
///  mask {
///    paths: "name"
///  }
/// ```
/// 
/// Or:
/// ```text
///  mask {
///    paths: "sub_message"
///  }
/// ```
/// 
/// Note that oneof type names ("test_oneof" in this case) cannot be used in
/// paths.
/// ##Field Mask Verification
/// 
/// The implementation of any API method which has a FieldMask type field in the
/// request should verify the included field paths, and return an
/// `INVALID_ARGUMENT` error if any path is duplicated or unmappable.
/// 
#[derive(Clone, Debug, PartialEq)]
pub struct FieldMask {
    paths: crate::collections::RepeatedField<::std::string::String>,
    unknown_fields: crate::UnknownFieldSet
}
static FIELD_MASK_PATHS_CODEC: crate::Codec<::std::string::String> = crate::Codec::string(10);
impl crate::CodedMessage for self::FieldMask {
    fn merge_from(&mut self, input: &mut crate::io::CodedInput) -> crate::io::InputResult<()> {
        while let ::std::option::Option::Some(tag) = input.read_tag()? {
            match tag.get() {
                10 => self.paths.add_entries(tag.get(), input, &FIELD_MASK_PATHS_CODEC)?,
                tag => self.unknown_fields.merge_from(tag, input)?
            }
        }
        ::std::result::Result::Ok(())
    }
    fn calculate_size(&self) -> i32 {
        let mut size = 0i32;
        size += self.paths.calculate_size(&FIELD_MASK_PATHS_CODEC);
        size += self.unknown_fields.calculate_size();
        size
    }
    fn write_to(&self, output: &mut crate::io::CodedOutput) -> crate::io::OutputResult {
        self.paths.write_to(output, &FIELD_MASK_PATHS_CODEC)?;
        self.unknown_fields.write_to(output)?;
        ::std::result::Result::Ok(())
    }
}
impl crate::LiteMessage for self::FieldMask {
    fn new() -> Self {
        Self {
            paths: crate::collections::RepeatedField::new(),
            unknown_fields: crate::UnknownFieldSet::new()
        }
    }
    fn merge(&mut self, other: &Self) {
        self.paths.merge(&other.paths);
        self.unknown_fields.merge(&other.unknown_fields);
    }
}
impl crate::Message for self::FieldMask {
    fn descriptor() -> &'static crate::reflect::MessageDescriptor {
        &self::file().messages()[0]
    }
}
impl self::FieldMask {
    /// Gets the field number of the [`paths`] field
    ///
    /// [`paths`]: #method.paths
    pub const PATHS_FIELD_NUMBER: i32 = 1;
        /// The set of field mask paths.
        /// 
    pub fn paths(&self) -> &crate::collections::RepeatedField<::std::string::String> {
        &self.paths
    }
    /// Returns a unique reference to the [`paths`] field
    ///
    /// [`paths`]: #method.paths
    pub fn paths_mut(&mut self) -> &mut crate::collections::RepeatedField<::std::string::String> {
        &mut self.paths
    }
}