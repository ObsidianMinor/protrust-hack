use crate::io::OutputError;
use crate::prelude::*;
use crate::reflect::MessageDescriptor;

pub use crate::generated::google_protobuf_any_proto::*;

const DEFAULT_PREFIX: &'static str = "type.googleapis.com";

fn get_type_url(descriptor: &MessageDescriptor, prefix: &str) -> String {
    if prefix.ends_with('/') {
        format!("{}{}", prefix, &descriptor.full_name()[1..])
    } else {
        format!("{}/{}", prefix, &descriptor.full_name()[1..])
    }
}

/// Gets the type name from a type url in an Any value or None if the url is invalid
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
    /// Creates a new `Any` value from a message of type `T`. 
    /// 
    /// This uses the default prefix "type.googleapis.com". A different prefix can be used with `pack_with_prefix`
    /// 
    /// # Examples
    /// ```
    /// use protrust::LiteMessage;
    /// use protrust::wkt::{any::Any, timestamp::Timestamp};
    /// # use std::error::Error;
    /// 
    /// # fn main() -> Result<(), Box<Error>> {
    /// let time = Timestamp::new();
    /// let any = Any::pack(&time)?;
    /// 
    /// assert_eq!(any.type_url(), "type.googleapis.com/google.protobuf.Timestamp");
    /// assert_ne!(any.type_url(), "example.com/google.protobuf.Timestamp");
    /// 
    /// # Ok(())
    /// # }
    /// ```
    pub fn pack<T: Message>(message: &T) -> Result<Any, crate::io::OutputError> {
        Any::pack_with_prefix(message, DEFAULT_PREFIX)
    }

    /// Creates a new `Any` value from a message of type `T` with the specified url prefix
    /// 
    /// # Examples
    /// ```
    /// use protrust::LiteMessage;
    /// use protrust::wkt::{any::Any, timestamp::Timestamp};
    /// # use std::error::Error;
    /// 
    /// # fn main() -> Result<(), Box<Error>> {
    /// let time = Timestamp::new();
    /// let any = Any::pack_with_prefix(&time, "example.com")?;
    ///
    /// assert_eq!(any.type_url(), "example.com/google.protobuf.Timestamp");
    /// assert_ne!(any.type_url(), "type.googleapis.com/google.protobuf.Timestamp");
    /// 
    /// # Ok(())
    /// # }
    /// ```
    pub fn pack_with_prefix<T: Message>(message: &T, prefix: &str) -> Result<Any, OutputError> {
        let mut value = Any::new();
        *value.type_url_mut() = get_type_url(T::descriptor(), prefix);
        *value.value_mut() = message.write_to_vec()?;

        Ok(value)
    }

    /// Returns a bool indicating if this `Any` value is of the specified message type `T`.
    /// 
    /// # Examples
    /// ```
    /// use protrust::LiteMessage;
    /// use protrust::wkt::{any::Any, timestamp::Timestamp};
    /// # use std::error::Error;
    /// 
    /// # fn main() -> Result<(), Box<Error>> {
    /// let time = Timestamp::new();
    /// let any = Any::pack(&time)?;
    /// 
    /// assert!(any.is::<Timestamp>());
    /// assert!(!any.is::<Any>());
    /// 
    /// # Ok(())
    /// # }
    /// ```
    pub fn is<T: Message>(&self) -> bool {
        match get_type_name(self.type_url()) {
            Some(msg_type) => *msg_type == T::descriptor().full_name()[1..],
            None => false,
        }
    }

    /// Unpacks a message of the specified type, returning None if
    /// the message is not of the specified message type `T`.
    /// 
    /// # Examples
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
    pub fn unpack<T: Message>(&self) -> Option<crate::io::InputResult<T>> {
        if self.is::<T>() {
            Some(self.unpack_unchecked::<T>())
        } else {
            None
        }
    }

    /// Unpacks a message of the specified type without 
    /// checking if this Any instance is an instance of the type.
    /// 
    /// This is useful if you've already checked that this is
    /// instance of the specified type
    /// 
    /// # Examples
    /// ```
    /// use protrust::LiteMessage;
    /// use protrust::wkt::{any::Any, timestamp::Timestamp};
    /// 
    /// # fn main() -> Result<(), Box<std::error::Error>> {
    /// let time = Timestamp::new();
    /// let any = Any::pack(&time)?;
    /// 
    /// assert_eq!(any.unpack_unchecked::<Timestamp>()?, time);
    /// # Ok(())
    /// # }
    /// ```
    pub fn unpack_unchecked<T: Message>(&self) -> crate::io::InputResult<T> {
        T::read_new(&mut self.value().as_slice())
    }
}
