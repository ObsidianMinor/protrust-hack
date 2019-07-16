use crate::reflect::{AnyMessage, AnyValue};
use crate::ExtendableMessage;
use std::fmt::Debug;

/// Represents a field accessor for a single, repeated, or map value
#[derive(Clone, Copy)]
pub enum FieldAccessor<'a, 'b> {
    Single(&'b dyn SingleFieldAccessor<'a>),
    Repeated(&'b dyn RepeatedFieldAccessor<'a>),
    Map(&'b dyn MapFieldAccessor<'a>),
}

#[derive(Debug)]
pub enum FieldAccessError<'a> {
    InvalidMessage,
    ExtensionNotFound,
    InvalidEntry(Box<dyn AnyValue<'a> + 'a>, Box<dyn AnyValue<'a> + 'a>),
    InvalidKey,
    InvalidValue(Box<dyn AnyValue<'a> + 'a>),
}

/// A result type for accessing message fields
pub type Result<'a, T> = std::result::Result<T, FieldAccessError<'a>>;

pub trait SingleFieldAccessor<'a>: Send + Sync {
    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
    ) -> Result<'a, Option<&'b dyn AnyValue<'a>>>;
    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
    ) -> Result<'a, &'b mut dyn AnyValue<'a>>;
    fn set(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        value: Box<dyn AnyValue<'a> + 'a>,
    ) -> Result<'a, ()>;
    fn take(
        &self,
        instance: &mut dyn AnyMessage<'a>,
    ) -> Result<'a, Option<Box<dyn AnyValue<'a> + 'a>>>;
    fn clear(&self, instance: &mut dyn AnyMessage<'a>) -> Result<'a, ()>;
}

pub trait RepeatedFieldAccessor<'a>: Send + Sync {
    fn len(&self, instance: &dyn AnyMessage<'a>) -> Result<'a, usize>;

    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
        index: usize,
    ) -> Result<'a, Option<&'b dyn AnyValue<'a>>>;
    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
        index: usize,
    ) -> Result<'a, Option<&'b mut dyn AnyValue<'a>>>;

    fn push(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        value: Box<dyn AnyValue<'a>>,
    ) -> Result<'a, ()>;
    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        index: usize,
        value: Box<dyn AnyValue<'a>>,
    ) -> Result<'a, ()>;
    fn pop(
        &self,
        instance: &mut dyn AnyMessage<'a>,
    ) -> Result<'a, Option<Box<dyn AnyValue<'a>>>>;
    fn remove(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        index: usize,
    ) -> Result<'a, Box<dyn AnyValue<'a>>>;

    fn clear(&self, instance: &mut dyn AnyMessage<'a>) -> Result<'a, ()>;
}

// we use this to unconditionally extend our borrow of extensions to be static
// this is only unsafe if a consumer decided to disregard all the hidden types and functions
// and create extensions on their own outside of static items, in which case: fuck 'em
#[inline]
fn unsafe_extend_extension_lifetime<T: crate::ExtensionIdentifier>(t: &T) -> &'static T {
    unsafe { std::mem::transmute(t) }
}

impl<
        T: crate::ExtendableMessage + AnyMessage<'static>,
        V: Clone + PartialEq + Default + AnyValue<'static>,
        D: Debug + Send + Sync + 'static,
    > SingleFieldAccessor<'static> for crate::Extension<T, V, D>
{
    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'static>,
    ) -> Result<'static, Option<&'b dyn AnyValue<'static>>> {
        let instance = instance
            .downcast_ref::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if instance.has_extension(unsafe_extend_extension_lifetime(self)) {
            Ok(instance
                .get_value(unsafe_extend_extension_lifetime(self))
                .map::<&'b dyn AnyValue<'static>, _>(|f| f))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'static>,
    ) -> Result<'static, &'b mut dyn AnyValue<'static>> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(unsafe_extend_extension_lifetime(self))
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .get_mut())
    }

    fn set(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(unsafe_extend_extension_lifetime(self))
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .set(
                *value
                    .downcast::<V>()
                    .map_err(|e| FieldAccessError::InvalidValue(e))?,
            ))
    }

    fn take(
        &self,
        instance: &mut dyn AnyMessage<'static>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(unsafe_extend_extension_lifetime(self))
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .take()
            .map::<Box<dyn AnyValue<'static>>, _>(|v| Box::new(v)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(unsafe_extend_extension_lifetime(self))
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .clear())
    }
}

impl<
        T: ExtendableMessage + AnyMessage<'static>,
        V: AnyValue<'static> + Clone + PartialEq,
    > RepeatedFieldAccessor<'static> for crate::RepeatedExtension<T, V>
{
    fn len(&self, instance: &dyn AnyMessage<'static>) -> Result<'static, usize> {
        let instance = instance
            .downcast_ref::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            Ok(instance.get_repeated_value(unsafe_extend_extension_lifetime(self)).map_or(0, |f| f.len()))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'static>,
        index: usize,
    ) -> Result<'static, Option<&'b dyn AnyValue<'static>>> {
        let instance = instance
            .downcast_ref::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            Ok(instance
                .get_repeated_value(unsafe_extend_extension_lifetime(self))
                .map::<&'b dyn AnyValue<'static>, _>(move |f| &f[index]))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'static>,
        index: usize,
    ) -> Result<'static, Option<&'b mut dyn AnyValue<'static>>> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            let field = instance.repeated_field(unsafe_extend_extension_lifetime(self)).unwrap();
            if field.has_entry() {
                Ok(field
                    .get_mut()
                    .get_mut(index)
                    .map::<&'b mut dyn AnyValue<'static>, _>(|v| v))
            } else {
                Ok(None)
            }
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn push(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            let value = *value
                .downcast::<V>()
                .map_err(|e| FieldAccessError::InvalidValue(e))?;
            Ok(instance
                .repeated_field(unsafe_extend_extension_lifetime(self))
                .unwrap()
                .get_mut()
                .push(value))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        index: usize,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            let value = *value
                .downcast::<V>()
                .map_err(|e| FieldAccessError::InvalidValue(e))?;
            Ok(instance
                .repeated_field(unsafe_extend_extension_lifetime(self))
                .unwrap()
                .get_mut()
                .insert(index, value))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn pop(
        &self,
        instance: &mut dyn AnyMessage<'static>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            let value = instance.repeated_field(unsafe_extend_extension_lifetime(self)).unwrap();
            if value.has_entry() {
                Ok(value
                    .get_mut()
                    .pop()
                    .map::<Box<dyn AnyValue<'static>>, _>(|v| Box::new(v)))
            } else {
                Ok(None)
            }
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn remove(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        index: usize,
    ) -> Result<'static, Box<dyn AnyValue<'static>>> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            let value = instance.repeated_field(unsafe_extend_extension_lifetime(self)).unwrap();
            if value.has_entry() {
                Ok(Box::new(value.get_mut().remove(index)))
            } else {
                panic!("Index is out of bounds")
            }
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if ExtendableMessage::registry(instance).map_or(false, |r| r.has_extension(unsafe_extend_extension_lifetime(self))) {
            Ok(instance.repeated_field(unsafe_extend_extension_lifetime(self)).unwrap().clear())
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }
}

/// An accessor for accessing fields with a shared reference getter and unique reference getter
pub struct SimpleFieldAccessor<T, V> {
    pub get: fn(&T) -> &V,
    pub get_mut: fn(&mut T) -> &mut V,
}

impl<T: AnyMessage<'static>, V: AnyValue<'static> + Default>
    SingleFieldAccessor<'static> for SimpleFieldAccessor<T, V>
{
    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage<'static>,
    ) -> Result<'static, Option<&'a dyn AnyValue<'static>>> {
        Ok(Some((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage<'static>,
    ) -> Result<'static, &'a mut dyn AnyValue<'static>> {
        Ok((self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
    }

    fn set(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let instance: &mut T = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value: Box<V> = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;

        *(self.get_mut)(instance) = *value;

        Ok(())
    }

    fn take(
        &self,
        instance: &mut dyn AnyMessage<'static>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value = std::mem::replace((self.get_mut)(instance), Default::default());
        Ok(Some(Box::new(value)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        *(self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ) = Default::default();
        Ok(())
    }
}

impl<T: AnyMessage<'static> + 'static, V: AnyValue<'static> + 'static>
    RepeatedFieldAccessor<'static>
    for SimpleFieldAccessor<T, crate::collections::RepeatedField<V>>
{
    fn len(&self, instance: &dyn AnyMessage<'static>) -> Result<'static, usize> {
        Ok((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )
        .len())
    }

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage<'static>,
        index: usize,
    ) -> Result<'static, Option<&'a dyn AnyValue<'static>>> {
        let field = (self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(field
            .get(index)
            .map::<&'a dyn AnyValue<'static>, _>(|v| v))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage<'static>,
        index: usize,
    ) -> Result<'static, Option<&'a mut dyn AnyValue<'static>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(field
            .get_mut(index)
            .map::<&'a mut dyn AnyValue<'static>, _>(|v| v))
    }

    fn push(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let value = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;
        field.push(*value);
        Ok(())
    }

    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        index: usize,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let value = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;
        field.insert(index, *value);
        Ok(())
    }

    fn pop(
        &self,
        instance: &mut dyn AnyMessage<'static>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(field
            .pop()
            .map::<Box<dyn AnyValue<'static>>, _>(|v| Box::new(v)))
    }

    fn remove(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        index: usize,
    ) -> Result<'static, Box<dyn AnyValue<'static>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(Box::new(field.remove(index)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        field.clear();
        Ok(())
    }
}

/// An accessor for accessing fields with a getter that returns an optional shared reference and unique reference getter
pub struct SimpleOptionFieldAccessor<T, V> {
    pub get: fn(&T) -> Option<&V>,
    pub get_mut: fn(&mut T) -> &mut V,
}

impl<T, V> SingleFieldAccessor<'static> for SimpleOptionFieldAccessor<T, V>
where
    T: AnyMessage<'static>,
    V: AnyValue<'static> + Default,
{
    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage<'static>,
    ) -> Result<'static, Option<&'a dyn AnyValue<'static>>> {
        Ok((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )
        .map::<&'a dyn AnyValue<'static>, _>(|v| v))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage<'static>,
    ) -> Result<'static, &'a mut dyn AnyValue<'static>> {
        Ok((self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
    }

    fn set(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let instance: &mut T = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value: Box<V> = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;

        *(self.get_mut)(instance) = *value;

        Ok(())
    }

    fn take(
        &self,
        instance: &mut dyn AnyMessage<'static>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value = std::mem::replace((self.get_mut)(instance), Default::default());
        Ok(Some(Box::new(value)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        *(self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ) = Default::default();
        Ok(())
    }
}

pub struct VerboseFieldAccessor<T, V> {
    pub get_option: fn(&T) -> Option<&V>,
    pub get_mut: fn(&mut T) -> &mut V,
    pub set: fn(&mut T, V),
    pub take: fn(&mut T) -> Option<V>,
    pub clear: fn(&mut T),
}

impl<T: AnyMessage<'static>, V: AnyValue<'static>> SingleFieldAccessor<'static>
    for VerboseFieldAccessor<T, V>
{
    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage<'static>,
    ) -> Result<'static, Option<&'a dyn AnyValue<'static>>> {
        Ok(((self.get_option)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
        .map::<&'a dyn AnyValue<'static>, _>(|v| v))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage<'static>,
    ) -> Result<'static, &'a mut dyn AnyValue<'static>> {
        Ok((self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
    }

    fn set(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, ()> {
        let instance: &mut T = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value: Box<V> = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;

        (self.set)(instance, *value);

        Ok(())
    }

    fn take(
        &self,
        instance: &mut dyn AnyMessage<'static>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        match (self.take)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ) {
            Some(v) => Ok(Some(Box::new(v))),
            None => Ok(None),
        }
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        (self.clear)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(())
    }
}

pub trait MapFieldAccessor<'a>: Send + Sync {
    fn len(&self, instance: &dyn AnyMessage<'a>) -> Result<'a, usize>;

    fn get<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
        key: &dyn AnyValue<'a>,
    ) -> Result<'a, Option<&'b dyn AnyValue<'a>>>;
    fn get_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
        key: &dyn AnyValue<'a>,
    ) -> Result<'a, Option<&'b mut dyn AnyValue<'a>>>;

    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        key: Box<dyn AnyValue<'a> + 'a>,
        value: Box<dyn AnyValue<'a> + 'a>,
    ) -> Result<'a, Option<Box<dyn AnyValue<'a> + 'a>>>;
    fn remove(
        &self,
        instance: &mut dyn AnyMessage<'a>,
        key: &dyn AnyValue<'a>,
    ) -> Result<'a, Option<Box<dyn AnyValue<'a> + 'a>>>;

    fn clear(&self, instance: &mut dyn AnyMessage<'a>) -> Result<'a, ()>;

    fn iter<'b>(
        &self,
        instance: &'b dyn AnyMessage<'a>,
    ) -> Result<'a, Box<dyn Iterator<Item = (&'b dyn AnyValue<'a>, &'b dyn AnyValue<'a>)> + 'b>>;
    fn iter_mut<'b>(
        &self,
        instance: &'b mut dyn AnyMessage<'a>,
    ) -> Result<'a, Box<dyn Iterator<Item = (&'b dyn AnyValue<'a>, &'b mut dyn AnyValue<'a>)> + 'b>>;
}

impl<
        T: AnyMessage<'static>,
        K: AnyValue<'static> + Eq + std::hash::Hash,
        V: AnyValue<'static>,
    > MapFieldAccessor<'static> for SimpleFieldAccessor<T, crate::collections::MapField<K, V>>
{
    fn len(&self, instance: &dyn AnyMessage<'static>) -> Result<'static, usize> {
        Ok((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )
        .len())
    }

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage<'static>,
        key: &dyn AnyValue<'static>,
    ) -> Result<'static, Option<&'a dyn AnyValue<'static>>> {
        let field = (self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let key = key
            .downcast_ref::<K>()
            .ok_or(FieldAccessError::InvalidKey)?;
        Ok(field
            .get(key)
            .map::<&'a dyn AnyValue<'static>, _>(|v| v))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage<'static>,
        key: &dyn AnyValue<'static>,
    ) -> Result<'static, Option<&'a mut dyn AnyValue<'static>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let key = key
            .downcast_ref::<K>()
            .ok_or(FieldAccessError::InvalidKey)?;
        Ok(field
            .get_mut(key)
            .map::<&'a mut dyn AnyValue<'static>, _>(|v| v))
    }

    fn insert(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        key: Box<dyn AnyValue<'static>>,
        value: Box<dyn AnyValue<'static>>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let key = match key.downcast::<K>() {
            Ok(k) => k,
            Err(k) => return Err(FieldAccessError::InvalidEntry(k, value)),
        };
        let value = match value.downcast::<V>() {
            Ok(v) => v,
            Err(v) => return Err(FieldAccessError::InvalidEntry(key, v)),
        };
        Ok(field
            .insert(*key, *value)
            .map::<Box<dyn AnyValue<'static>>, _>(|v| Box::new(v)))
    }

    fn remove<'a>(
        &self,
        instance: &mut dyn AnyMessage<'static>,
        key: &dyn AnyValue<'_>,
    ) -> Result<'static, Option<Box<dyn AnyValue<'static>>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let key = key
            .downcast_ref::<K>()
            .ok_or(FieldAccessError::InvalidKey)?;
        Ok(field
            .remove(key)
            .map::<Box<dyn AnyValue<'static>>, _>(|v| Box::new(v)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage<'static>) -> Result<'static, ()> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        field.clear();
        Ok(())
    }

    fn iter<'a>(
        &self,
        instance: &'a dyn AnyMessage<'static>,
    ) -> Result<
        'static,
        Box<
            dyn Iterator<
                    Item = (
                        &'a dyn AnyValue<'static>,
                        &'a dyn AnyValue<'static>,
                    ),
                > + 'a,
        >,
    > {
        Ok(Box::new(
            (self.get)(
                instance
                    .downcast_ref::<T>()
                    .ok_or(FieldAccessError::InvalidMessage)?,
            )
            .iter()
            .map::<(
                &'a dyn AnyValue<'static>,
                &'a dyn AnyValue<'static>,
            ), _>(|(k, v)| (k, v)),
        ))
    }

    fn iter_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage<'static>,
    ) -> Result<
        'static,
        Box<
            dyn Iterator<
                    Item = (
                        &'a dyn AnyValue<'static>,
                        &'a mut dyn AnyValue<'static>,
                    ),
                > + 'a,
        >,
    > {
        Ok(Box::new(
            (self.get_mut)(
                instance
                    .downcast_mut::<T>()
                    .ok_or(FieldAccessError::InvalidMessage)?,
            )
            .iter_mut()
            .map::<(
                &'a dyn AnyValue<'static>,
                &'a mut dyn AnyValue<'static>,
            ), _>(|(k, v)| (k, v)),
        ))
    }
}