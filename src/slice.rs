use crate::primitive::Primitive;

pub trait Slice: Primitive {
    type Item;
}
impl<T> Primitive for [T] {}
impl<T> Slice for [T] {
    type Item = T;
}
impl Primitive for str {}
impl Slice for str {
    type Item = u8;
}
