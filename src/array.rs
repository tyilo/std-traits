use core::{
    borrow::{Borrow, BorrowMut},
    ops::{
        Bound, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

use crate::primitive::Primitive;

macro_rules! array_trait {
    (($($bounds:tt +)*); ($($alloc_bounds:tt +)*) $impl:tt) => {
        #[cfg(not(feature = "alloc"))]
        pub trait Array: $($bounds +)* $impl

        #[cfg(feature = "alloc")]
        pub trait Array: $($bounds +)* $($alloc_bounds +)* $impl
    }
}

array_trait!(
    (
        Primitive
        + Sized
        + IntoIterator // Contains `Self::Item`
        + (AsRef<[Self::Item]>)
        + (AsMut<[Self::Item]>)
        + (Borrow<[Self::Item]>)
        + (BorrowMut<[Self::Item]>)
        + (Index<usize, Output = Self::Item>)
        + (Index<(Bound<usize>, Bound<usize>), Output = [Self::Item]>)
        + (Index<Range<usize>, Output = [Self::Item]>)
        + (Index<RangeInclusive<usize>, Output = [Self::Item]>)
        + (Index<RangeFrom<usize>, Output = [Self::Item]>)
        + (Index<RangeTo<usize>, Output = [Self::Item]>)
        + (Index<RangeToInclusive<usize>, Output = [Self::Item]>)
        + (Index<RangeFull, Output = [Self::Item]>)
        + (IndexMut<usize, Output = Self::Item>)
        + (IndexMut<(Bound<usize>, Bound<usize>), Output = [Self::Item]>)
        + (IndexMut<Range<usize>, Output = [Self::Item]>)
        + (IndexMut<RangeInclusive<usize>, Output = [Self::Item]>)
        + (IndexMut<RangeFrom<usize>, Output = [Self::Item]>)
        + (IndexMut<RangeTo<usize>, Output = [Self::Item]>)
        + (IndexMut<RangeToInclusive<usize>, Output = [Self::Item]>)
        + (IndexMut<RangeFull, Output = [Self::Item]>)
    +);
    (
        (TryFrom<alloc::vec::Vec<Self::Item>>)
        + (Into<alloc::boxed::Box<[Self::Item]>>)
        + (Into<alloc::rc::Rc<[Self::Item]>>)
        + (Into<alloc::sync::Arc<[Self::Item]>>)
        + (Into<alloc::vec::Vec<Self::Item>>)
        + (Into<alloc::collections::VecDeque<Self::Item>>)
        + (Into<alloc::collections::LinkedList<Self::Item>>)
    +) {
        const N: usize;

        fn as_slice(&self) -> &[Self::Item];
        fn as_mut_slice(&mut self) -> &mut [Self::Item];
        fn map<F, U>(self, f: F) -> impl Array<Item = U>
        where
            F: FnMut(Self::Item) -> U;
        fn each_ref(&self) -> impl Array<Item = &Self::Item>;
        fn each_mut(&mut self) -> impl Array<Item = &mut Self::Item>;
    }
);

impl<const N: usize, T> Primitive for [T; N] {}
impl<const N: usize, T> Array for [T; N] {
    const N: usize = N;

    fn as_slice(&self) -> &[Self::Item] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut_slice()
    }

    fn map<F, U>(self, f: F) -> impl Array<Item = U>
    where
        F: FnMut(Self::Item) -> U,
    {
        self.map(f)
    }

    fn each_ref(&self) -> impl Array<Item = &Self::Item> {
        self.each_ref()
    }

    fn each_mut(&mut self) -> impl Array<Item = &mut Self::Item> {
        self.each_mut()
    }
}
