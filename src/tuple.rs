use crate::primitive::Primitive;

pub trait Tuple: Primitive {
    const N: usize;
}
impl Primitive for () {}
impl Tuple for () {
    const N: usize = 0;
}
impl<T1: ?Sized> Primitive for (T1,) {}
impl<T1: ?Sized> Tuple for (T1,) {
    const N: usize = 1;
}
impl<T1, T2: ?Sized> Primitive for (T1, T2) {}
impl<T1, T2: ?Sized> Tuple for (T1, T2) {
    const N: usize = 2;
}
