use crate::prelude::*;
use crate::io::OutputError;
use crate::reflect::MessageDescriptor;

pub use crate::generated::google_protobuf_any_proto::*;

const DEFAULT_PREFIX: &'static str = "type.googleapis.com";

fn get_type_url(descriptor: &MessageDescriptor, prefix: &str) -> String {
    if prefix.ends_with('/') {
        format!("{}{}", prefix, descriptor.full_name())
    } else {
        format!("{}/{}", prefix, descriptor.full_name())
    }
}

/// Gets the type name from a type url in an Any value
/// 
/// # Examples
/// ```
/// # use protrust::wkt::any::get_type_name;
/// assert_eq!(get_type_name("foobar"), None);
/// assert_eq!(get_type_name("/foo/bar/baz"), Some("baz"));
/// assert_eq!(get_type_name("proto://foo.bar"), Some("foo.bar"));
/// assert_eq!(get_type_name("type.googleapis.com/google.protobuf.Empty"), Some("google.protobuf.Empty"));
/// ```
pub fn get_type_name(url: &str) -> Option<&str> {
    let pos = url.rfind('/')?;
    Some(&url[(pos + 1)..])
}

impl Any {
    pub fn pack<T: Message>(message: &T) -> Result<Any, crate::io::OutputError> {
        Any::pack_with_prefix(message, DEFAULT_PREFIX)
    }

    pub fn pack_with_prefix<T: Message>(message: &T, prefix: &str) -> Result<Any, OutputError> {
        let mut value = Any::new();
        value.type_url = get_type_url(T::descriptor(), prefix);
        value.value = message.write_to_vec()?;

        Ok(value)
    }

    pub fn is<T: Message>(&self) -> bool {
        match get_type_name(&self.type_url) {
            Some(msg_type) => *msg_type == T::descriptor().full_name()[1..],
            None => false
        }
    }

    /// Unpacks a message of the specified type
    /// 
    /// # Examples
    /// ```text
    /// let msg = Timestamp::from_system_time(&SystemTime::now())?;
    /// let packed = Any::pack(msg);
    /// 
    /// if let Some(timestamp) = packed.unpack<Timestamp>()? {
    ///     assert!(timestamp.eq(msg));
    /// }
    /// ```
    pub fn unpack<T: Message>(&self) -> Result<Option<T>, crate::io::InputError> {
        if self.is::<T>() {
            Ok(Some(self.unpack_unchecked::<T>()?))
        } else {
            Ok(None)
        }
    }

    pub fn unpack_unchecked<T: Message>(&self) -> Result<T, crate::io::InputError> {
        let mut slice = self.value.as_slice();
        T::read_new(&mut slice)
    }
}