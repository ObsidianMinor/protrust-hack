/// Provides runtime support for the `Any` type defined in `google/protobuf/any.proto`
///
/// ### Packing / Unpacking
/// ```text
/// extern crate protrust;
///
/// use protrust::wkt::any::Any;
/// use protrust::wkt::timestamp::Timestamp;
/// use std::time::SystemTime;
///
/// fn main() -> Result<T, E> {
///     let msg = Timestamp::from_system_time(&SystemTime::now())?;
///     let packed = Any::pack(msg);
///     
///     assert!(packed.is<Timestamp>());
///     assert!(packed.unpack<Timestamp>().is_some());
/// }
/// ```
pub mod any;
pub use crate::generated::google_protobuf_api_proto as api;
pub use crate::generated::google_protobuf_empty_proto as empty;
pub mod duration;
pub mod field_mask;
pub use crate::generated::google_protobuf_source_context_proto as source_context;
pub use crate::generated::google_protobuf_struct_proto as r#struct;
pub mod timestamp;
pub use crate::generated::google_protobuf_type_proto as r#type;
pub use crate::generated::google_protobuf_wrappers_proto as wrappers;
