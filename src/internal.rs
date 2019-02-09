pub trait Primitive {
    fn is_default(&self) -> bool;
}

impl Primitive for i32 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}

impl Primitive for u32 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}

impl Primitive for f32 {
    fn is_default(&self) -> bool {
        *self == 0.0
    }
}

impl Primitive for f64 {
    fn is_default(&self) -> bool {
        *self == 0.0
    }
}

impl Primitive for i64 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}

impl Primitive for u64 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}

impl Primitive for bool {
    fn is_default(&self) -> bool {
        !self
    }
}

impl Primitive for String {
    fn is_default(&self) -> bool {
        self.len() == 0
    }
}

impl Primitive for Vec<u8> {
    fn is_default(&self) -> bool {
        self.len() == 0
    }
}

impl<T: crate::CodedMessage> Primitive for T {
    fn is_default(&self) -> bool {
        false
    }
}

impl<E: Into<i32> + Clone> Primitive for crate::EnumValue<E> {
    fn is_default(&self) -> bool {
        i32::from(self.clone()) == 0
    }
}

pub trait Sealed {}
