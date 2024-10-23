use crate::primitive::Primitive;

pub trait Slice: Primitive + AsRef<[Self::Item]> {
    type Item;

    fn as_slice(&self) -> &[Self::Item];
}

impl<T> Primitive for [T] {}
impl<T> Slice for [T] {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl Primitive for str {}
impl Slice for str {
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        self.as_bytes()
    }
}
