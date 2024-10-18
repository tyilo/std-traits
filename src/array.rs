use crate::primitive::Primitive;

pub trait Array: Primitive + Sized {
    const N: usize;
    type Item;

    fn as_slice(&self) -> &[Self::Item];
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<const N: usize, T> Primitive for [T; N] {}
impl<const N: usize, T> Array for [T; N] {
    const N: usize = N;
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut_slice()
    }
}
