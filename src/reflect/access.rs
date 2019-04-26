use crate::ExtensionMessage;
use crate::reflect::{AnyMessage, AnyValue};
use std::fmt::Debug;

/// Represents a field accessor for a single, repeated, or map value
pub enum FieldAccessor<'a> {
    Single(&'a SingleFieldAccessor),
    Repeated(&'a RepeatedFieldAccessor),
    Map(&'a MapFieldAccessor),
}

pub enum FieldAccessError {
    InvalidMessage,
    ExtensionNotFound,
    InvalidEntry(Box<dyn AnyValue>, Box<dyn AnyValue>),
    InvalidKey,
    InvalidValue(Box<dyn AnyValue>),
}

/// A result type for accessing message fields
pub type Result<T> = std::result::Result<T, FieldAccessError>;

pub trait SingleFieldAccessor {
    fn get<'a>(&self, instance: &'a dyn AnyMessage) -> Result<Option<&'a dyn AnyValue>>;
    fn get_mut<'a>(&self, instance: &'a mut dyn AnyMessage) -> Result<&'a mut dyn AnyValue>;
    fn set(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()>;
    fn take(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>>;
    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()>;
}

impl<
        T: crate::ExtensionMessage + AnyMessage,
        V: Clone + PartialEq + Default + Debug + Send + Sync + AnyValue,
        D: Debug + Send + Sync,
    > SingleFieldAccessor for &'static crate::Extension<T, V, D>
{
    fn get<'a>(&self, instance: &'a dyn AnyMessage) -> Result<Option<&'a dyn AnyValue>> {
        let instance = instance
            .downcast_ref::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        if instance.has_extension(*self) {
            Ok(instance.get_value(self).map::<&'a dyn AnyValue, _>(|f| f))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn get_mut<'a>(&self, instance: &'a mut dyn AnyMessage) -> Result<&'a mut dyn AnyValue> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(self)
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .get_mut())
    }

    fn set(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(self)
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .set(*value.downcast::<V>()
                .map_err(|e| FieldAccessError::InvalidValue(e))?))
    }

    fn take(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(self)
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .take()
            .map::<Box<dyn AnyValue>, _>(|v| Box::new(v)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()> {
        Ok(instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?
            .field(self)
            .ok_or(FieldAccessError::ExtensionNotFound)?
            .clear())
    }
}

impl<T: ExtensionMessage + AnyMessage, V: AnyValue + Clone + PartialEq + Debug + Send + Sync> RepeatedFieldAccessor
    for &'static crate::RepeatedExtension<T, V>
{
    fn len(&self, instance: &dyn AnyMessage) -> Result<usize> {
        let instance = instance.downcast_ref::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            Ok(instance.get_repeated_value(self).map_or(0, |f| f.len()))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage,
        index: usize,
    ) -> Result<Option<&'a dyn AnyValue>> {
        let instance = instance.downcast_ref::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            Ok(instance.get_repeated_value(*self).map::<&'a dyn AnyValue, _>(move |f| &f[index]))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage,
        index: usize,
    ) -> Result<Option<&'a mut dyn AnyValue>> {
        let instance = instance.downcast_mut::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            let field = instance.repeated_field(*self).unwrap();
            if field.has_entry() {
                Ok(field.get_mut().get_mut(index).map::<&'a mut dyn AnyValue, _>(|v| v))
            } else {
                Ok(None)
            }
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn push(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()> {
        let instance = instance.downcast_mut::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            let value = *value.downcast::<V>().map_err(|e| FieldAccessError::InvalidValue(e))?;
            Ok(instance.repeated_field(*self).unwrap().get_mut().push(value))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn insert(
        &self,
        instance: &mut dyn AnyMessage,
        index: usize,
        value: Box<dyn AnyValue>,
    ) -> Result<()> {
        let instance = instance.downcast_mut::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            let value = *value.downcast::<V>().map_err(|e| FieldAccessError::InvalidValue(e))?;
            Ok(instance.repeated_field(*self).unwrap().get_mut().insert(index, value))
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn pop(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>> {
        let instance = instance.downcast_mut::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            let value = instance.repeated_field(*self).unwrap();
            if value.has_entry() {
                Ok(value.get_mut().pop().map::<Box<dyn AnyValue>, _>(|v| Box::new(v)))
            } else {
                Ok(None)
            }
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn remove(&self, instance: &mut dyn AnyMessage, index: usize) -> Result<Box<dyn AnyValue>> {
        let instance = instance.downcast_mut::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            let value = instance.repeated_field(*self).unwrap();
            if value.has_entry() {
                Ok(Box::new(value.get_mut().remove(index)))
            } else {
                panic!("Index is out of bounds")
            }
        } else {
            Err(FieldAccessError::ExtensionNotFound)
        }
    }

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()> {
        let instance = instance.downcast_mut::<T>().ok_or(FieldAccessError::InvalidMessage)?;
        if ExtensionMessage::registry(instance).map_or(false, |r| r.has_extension(*self)) {
            Ok(instance.repeated_field(*self).unwrap().clear())
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

impl<T: AnyMessage, V: AnyValue + Default> SingleFieldAccessor for SimpleFieldAccessor<T, V> {
    fn get<'a>(&self, instance: &'a dyn AnyMessage) -> Result<Option<&'a dyn AnyValue>> {
        Ok(Some((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )))
    }

    fn get_mut<'a>(&self, instance: &'a mut dyn AnyMessage) -> Result<&'a mut dyn AnyValue> {
        Ok((self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
    }

    fn set(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()> {
        let instance: &mut T = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value: Box<V> = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;

        *(self.get_mut)(instance) = *value;

        Ok(())
    }

    fn take(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>> {
        let instance = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value = std::mem::replace((self.get_mut)(instance), Default::default());
        Ok(Some(Box::new(value)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()> {
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

impl<T: AnyMessage, V: AnyValue> SingleFieldAccessor for VerboseFieldAccessor<T, V> {
    fn get<'a>(&self, instance: &'a dyn AnyMessage) -> Result<Option<&'a dyn AnyValue>> {
        Ok(((self.get_option)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
        .map::<&'a dyn AnyValue, _>(|v| v))
    }

    fn get_mut<'a>(&self, instance: &'a mut dyn AnyMessage) -> Result<&'a mut dyn AnyValue> {
        Ok((self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ))
    }

    fn set(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()> {
        let instance: &mut T = instance
            .downcast_mut::<T>()
            .ok_or(FieldAccessError::InvalidMessage)?;
        let value: Box<V> = value
            .downcast::<V>()
            .map_err(|v| FieldAccessError::InvalidValue(v))?;

        (self.set)(instance, *value);

        Ok(())
    }

    fn take(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>> {
        match (self.take)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        ) {
            Some(v) => Ok(Some(Box::new(v))),
            None => Ok(None),
        }
    }

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()> {
        (self.clear)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(())
    }
}

pub trait RepeatedFieldAccessor {
    fn len(&self, instance: &dyn AnyMessage) -> Result<usize>;

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage,
        index: usize,
    ) -> Result<Option<&'a dyn AnyValue>>;
    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage,
        index: usize,
    ) -> Result<Option<&'a mut dyn AnyValue>>;

    fn push(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()>;
    fn insert(
        &self,
        instance: &mut dyn AnyMessage,
        index: usize,
        value: Box<dyn AnyValue>,
    ) -> Result<()>;
    fn pop(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>>;
    fn remove(&self, instance: &mut dyn AnyMessage, index: usize) -> Result<Box<dyn AnyValue>>;

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()>;
}

impl<T: AnyMessage, V: AnyValue> RepeatedFieldAccessor
    for SimpleFieldAccessor<T, crate::collections::RepeatedField<V>>
{
    fn len(&self, instance: &dyn AnyMessage) -> Result<usize> {
        Ok((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )
        .len())
    }

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage,
        index: usize,
    ) -> Result<Option<&'a dyn AnyValue>> {
        let field = (self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(field.get(index).map::<&'a dyn AnyValue, _>(|v| v))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage,
        index: usize,
    ) -> Result<Option<&'a mut dyn AnyValue>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(field.get_mut(index).map::<&'a mut dyn AnyValue, _>(|v| v))
    }

    fn push(&self, instance: &mut dyn AnyMessage, value: Box<dyn AnyValue>) -> Result<()> {
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
        instance: &mut dyn AnyMessage,
        index: usize,
        value: Box<dyn AnyValue>,
    ) -> Result<()> {
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

    fn pop(&self, instance: &mut dyn AnyMessage) -> Result<Option<Box<dyn AnyValue>>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(field.pop().map::<Box<dyn AnyValue>, _>(|v| Box::new(v)))
    }

    fn remove(&self, instance: &mut dyn AnyMessage, index: usize) -> Result<Box<dyn AnyValue>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        Ok(Box::new(field.remove(index)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        field.clear();
        Ok(())
    }
}

pub trait MapFieldAccessor {
    fn len(&self, instance: &dyn AnyMessage) -> Result<usize>;

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage,
        key: &dyn AnyValue,
    ) -> Result<Option<&'a dyn AnyValue>>;
    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage,
        key: &dyn AnyValue,
    ) -> Result<Option<&'a mut dyn AnyValue>>;

    fn insert(
        &self,
        instance: &mut dyn AnyMessage,
        key: Box<dyn AnyValue>,
        value: Box<dyn AnyValue>,
    ) -> Result<Option<Box<dyn AnyValue>>>;
    fn remove(
        &self,
        instance: &mut dyn AnyMessage,
        key: &dyn AnyValue,
    ) -> Result<Option<Box<dyn AnyValue>>>;

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()>;

    fn iter<'a>(&self, instance: &'a dyn AnyMessage) -> Result<Box<(dyn Iterator<Item = (&'a AnyValue, &'a AnyValue)> + 'a)>>;
    fn iter_mut<'a>(&self, instance: &'a mut dyn AnyMessage) -> Result<Box<(dyn Iterator<Item = (&'a AnyValue, &'a mut AnyValue)> + 'a)>>;
}

impl<T: AnyMessage, K: AnyValue + Eq + std::hash::Hash, V: AnyValue> MapFieldAccessor
    for SimpleFieldAccessor<T, crate::collections::MapField<K, V>>
{
    fn len(&self, instance: &dyn AnyMessage) -> Result<usize> {
        Ok((self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        )
        .len())
    }

    fn get<'a>(
        &self,
        instance: &'a dyn AnyMessage,
        key: &dyn AnyValue,
    ) -> Result<Option<&'a dyn AnyValue>> {
        let field = (self.get)(
            instance
                .downcast_ref::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let key = key
            .downcast_ref::<K>()
            .ok_or(FieldAccessError::InvalidKey)?;
        Ok(field.get(key).map::<&'a dyn AnyValue, _>(|v| v))
    }

    fn get_mut<'a>(
        &self,
        instance: &'a mut dyn AnyMessage,
        key: &dyn AnyValue,
    ) -> Result<Option<&'a mut dyn AnyValue>> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        let key = key
            .downcast_ref::<K>()
            .ok_or(FieldAccessError::InvalidKey)?;
        Ok(field.get_mut(key).map::<&'a mut dyn AnyValue, _>(|v| v))
    }

    fn insert(
        &self,
        instance: &mut dyn AnyMessage,
        key: Box<dyn AnyValue>,
        value: Box<dyn AnyValue>,
    ) -> Result<Option<Box<dyn AnyValue>>> {
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
            .map::<Box<dyn AnyValue>, _>(|v| Box::new(v)))
    }

    fn remove(
        &self,
        instance: &mut dyn AnyMessage,
        key: &dyn AnyValue,
    ) -> Result<Option<Box<dyn AnyValue>>> {
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
            .map::<Box<dyn AnyValue>, _>(|v| Box::new(v)))
    }

    fn clear(&self, instance: &mut dyn AnyMessage) -> Result<()> {
        let field = (self.get_mut)(
            instance
                .downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?,
        );
        field.clear();
        Ok(())
    }

    fn iter<'a>(&self, instance: &'a dyn AnyMessage) -> Result<Box<(dyn Iterator<Item = (&'a AnyValue, &'a AnyValue)> + 'a)>> {
        Ok(
            Box::new((self.get)(
                instance.downcast_ref::<T>()
                    .ok_or(FieldAccessError::InvalidMessage)?)
                    .iter()
                    .map::<(&'a dyn AnyValue, &'a dyn AnyValue), _>(|(k, v)| (k, v))))
    }

    fn iter_mut<'a>(&self, instance: &'a mut dyn AnyMessage) -> Result<Box<(dyn Iterator<Item = (&'a AnyValue, &'a mut AnyValue)> + 'a)>> {
        Ok(
            Box::new((self.get_mut)(
                instance.downcast_mut::<T>()
                .ok_or(FieldAccessError::InvalidMessage)?)
                .iter_mut()
                .map::<(&'a dyn AnyValue, &'a mut dyn AnyValue), _>(|(k, v)| (k, v))))
    }
}
