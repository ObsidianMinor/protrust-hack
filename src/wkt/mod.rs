//! Provides runtime support for [well-known types](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf)

/// Provides runtime support for the `Any` type defined in `google/protobuf/any.proto`
///
/// ### Packing / Unpacking
/// ```
/// use protrust::LiteMessage;
/// use protrust::wkt::{any::Any, timestamp::Timestamp};
///
/// # fn main() -> Result<(), Box<std::error::Error>> {
/// let time = Timestamp::new();
/// let any = Any::pack(&time)?;
///
/// if let Some(result) = any.unpack::<Timestamp>() {
///     assert_eq!(result?, time);
/// }
///
/// assert!(any.unpack::<Any>().is_none());
/// # Ok(())
/// # }
/// ```
pub mod any;
pub use crate::generated::google_protobuf_api_proto as api;
pub use crate::generated::google_protobuf_empty_proto as empty;
/// Provides runtime support for the `Duration` type defined in `google/protobuf/duration.proto`
pub mod duration;
/// Provides runtime support for the `FieldMask` type defined in `google/protobuf/field_mask.proto`
pub mod field_mask;
pub use crate::generated::google_protobuf_source_context_proto as source_context;
pub use crate::generated::google_protobuf_struct_proto as r#struct;
/// Provides runtime support for the `Timestamp` type defined in `google/protobuf/timestamp.proto`
pub mod timestamp;
pub use crate::generated::google_protobuf_type_proto as r#type;
pub use crate::generated::google_protobuf_wrappers_proto as wrappers;
