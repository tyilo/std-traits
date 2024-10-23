use core::{
    borrow::{Borrow, BorrowMut},
    ops::{
        Bound, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

use crate::primitive::Primitive;

pub trait Array:
    Primitive
    + Sized
    + IntoIterator // Contains `Self::Item`
    + AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Borrow<[Self::Item]>
    + BorrowMut<[Self::Item]>
    + Index<usize, Output = Self::Item>
    + Index<(Bound<usize>, Bound<usize>), Output = [Self::Item]>
    + Index<Range<usize>, Output = [Self::Item]>
    + Index<RangeInclusive<usize>, Output = [Self::Item]>
    + Index<RangeFrom<usize>, Output = [Self::Item]>
    + Index<RangeTo<usize>, Output = [Self::Item]>
    + Index<RangeToInclusive<usize>, Output = [Self::Item]>
    + Index<RangeFull, Output = [Self::Item]>
    + IndexMut<usize, Output = Self::Item>
    + IndexMut<(Bound<usize>, Bound<usize>), Output = [Self::Item]>
    + IndexMut<Range<usize>, Output = [Self::Item]>
    + IndexMut<RangeInclusive<usize>, Output = [Self::Item]>
    + IndexMut<RangeFrom<usize>, Output = [Self::Item]>
    + IndexMut<RangeTo<usize>, Output = [Self::Item]>
    + IndexMut<RangeToInclusive<usize>, Output = [Self::Item]>
    + IndexMut<RangeFull, Output = [Self::Item]>
{
    const N: usize;

    fn as_slice(&self) -> &[Self::Item];
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<const N: usize, T> Primitive for [T; N] {}
impl<const N: usize, T> Array for [T; N] {
    const N: usize = N;

    fn as_slice(&self) -> &[Self::Item] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut_slice()
    }
}
