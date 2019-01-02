/// Provides runtime support for the `Any` type defined in `google/protobuf/any.proto`
///
/// ### Packing / Unpacking
/// ```
/// # use protrust::prelude::*;
/// # use protrust::wkt::any::Any;
/// # use protrust::wkt::timestamp::Timestamp;
/// # fn main() {
/// let mut msg = Timestamp::new();
/// msg.seconds = 25;
/// msg.nanos = 56;
/// let packed = Any::pack(&msg).expect("Could not pack");
/// 
/// assert!(packed.is::<Timestamp>());
/// assert_eq!(packed.unpack::<Timestamp>().unwrap(), Some(msg));
/// # }
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
