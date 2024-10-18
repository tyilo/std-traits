use crate::primitive::Primitive;

pub trait Reference: Primitive + Sized {
    fn as_ref(&self) -> &Self;
}

impl<T: ?Sized> Primitive for &T {}
impl<T: ?Sized> Reference for &T {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl<T: ?Sized> Primitive for &mut T {}
impl<T: ?Sized> Reference for &mut T {
    fn as_ref(&self) -> &Self {
        self
    }
}
