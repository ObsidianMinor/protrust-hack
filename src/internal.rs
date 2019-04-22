pub trait Primitive {
    fn is_default(&self) -> bool;
    fn is_packable() -> bool;
}

impl Primitive for i32 {
    fn is_default(&self) -> bool {
        *self == 0
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for u32 {
    fn is_default(&self) -> bool {
        *self == 0
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for f32 {
    fn is_default(&self) -> bool {
        *self == 0.0
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for f64 {
    fn is_default(&self) -> bool {
        *self == 0.0
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for i64 {
    fn is_default(&self) -> bool {
        *self == 0
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for u64 {
    fn is_default(&self) -> bool {
        *self == 0
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for bool {
    fn is_default(&self) -> bool {
        !self
    }

    fn is_packable() -> bool {
        true
    }
}

impl Primitive for String {
    fn is_default(&self) -> bool {
        self.len() == 0
    }

    fn is_packable() -> bool {
        false
    }
}

impl Primitive for Vec<u8> {
    fn is_default(&self) -> bool {
        self.len() == 0
    }

    fn is_packable() -> bool {
        false
    }
}

impl<T: crate::CodedMessage> Primitive for T {
    fn is_default(&self) -> bool {
        false
    }

    fn is_packable() -> bool {
        false
    }
}

impl<E: crate::Enum> Primitive for crate::EnumValue<E> {
    fn is_default(&self) -> bool {
        i32::from(self.clone()) == 0
    }

    fn is_packable() -> bool {
        true
    }
}

pub trait Sealed {}
