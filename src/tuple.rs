use core::mem::{forget, transmute_copy};

use crate::{array::Array, primitive::Primitive};

pub trait Tuple: Primitive {
    const N: usize;
}

pub trait HomogeneousTuple<Item>: Tuple {
    type Array: Array<Item = Item>;

    fn into_array(self) -> Self::Array
    where
        Self: Sized;
    fn from_array(array: Self::Array) -> Self;
}

impl Primitive for () {}
impl Tuple for () {
    const N: usize = 0;
}
impl<T> HomogeneousTuple<T> for () {
    type Array = [T; 0];

    fn into_array(self) -> Self::Array
    where
        Self: Sized,
    {
        []
    }

    fn from_array(_array: Self::Array) -> Self {
        #[allow(clippy::unused_unit)]
        ()
    }
}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T1: ?Sized> Primitive for (T1,) {}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T1: ?Sized> Tuple for (T1,) {
    const N: usize = 1;
}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T> HomogeneousTuple<T> for (T,) {
    type Array = [T; 1];

    fn into_array(self) -> Self::Array
    where
        Self: Sized,
    {
        [self.0]
    }

    fn from_array(array: Self::Array) -> Self {
        let res = unsafe { transmute_copy::<Self::Array, Self>(&array) };
        forget(array);
        res
    }
}

macro_rules! replace_expr {
    ($_t:tt $sub:tt) => {
        $sub
    };
}

macro_rules! homogeneous_tuple {
    ($($types:tt),*) => {
        (
            $(
                replace_expr!(
                    ($types)
                    T
                ),
            )*
        )
    };
}

macro_rules! impl_tuple {
    ($n:expr => $($types:tt $i:tt),*; $last:tt $last_i:tt) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$($types,)* $last: ?Sized> Primitive for ($($types,)* $last,) {}
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$($types,)* $last: ?Sized> Tuple for ($($types,)* $last,) {
            const N: usize = $n;
        }
        #[cfg_attr(docsrs, doc(hidden))]
        impl<T> HomogeneousTuple<T> for homogeneous_tuple!($($types,)* $last) {
            type Array = [T; $n];

            fn into_array(self) -> Self::Array
            where
                Self: Sized
            {
                [$(self.$i,)* self.$last_i]
            }

            fn from_array(array: Self::Array) -> Self {
                let res = unsafe { transmute_copy::<Self::Array, Self>(&array) };
                forget(array);
                res
            }
        }
    }
}

/*
for n in range(2, 17):
    print(f"impl_tuple!({n} => {', '.join(f'T{i} {i - 1}' for i in range(1, n))}; T{n} {n - 1});")
*/
impl_tuple!(2 => T1 0; T2 1);
impl_tuple!(3 => T1 0, T2 1; T3 2);
impl_tuple!(4 => T1 0, T2 1, T3 2; T4 3);
impl_tuple!(5 => T1 0, T2 1, T3 2, T4 3; T5 4);
impl_tuple!(6 => T1 0, T2 1, T3 2, T4 3, T5 4; T6 5);
impl_tuple!(7 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5; T7 6);
impl_tuple!(8 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6; T8 7);
impl_tuple!(9 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7; T9 8);
impl_tuple!(10 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8; T10 9);
impl_tuple!(11 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8, T10 9; T11 10);
impl_tuple!(12 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8, T10 9, T11 10; T12 11);
impl_tuple!(13 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8, T10 9, T11 10, T12 11; T13 12);
impl_tuple!(14 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8, T10 9, T11 10, T12 11, T13 12; T14 13);
impl_tuple!(15 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8, T10 9, T11 10, T12 11, T13 12, T14 13; T15 14);
impl_tuple!(16 => T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7, T9 8, T10 9, T11 10, T12 11, T13 12, T14 13, T15 14; T16 15);

#[cfg(test)]
mod test {
    use super::*;
    extern crate std;
    use std::{format, prelude::rust_2021::*, println};

    macro_rules! test_from_array {
        ($($types:tt),+) => {
            let arr  = std::array::from_fn(|i| format!("{i}"));
            println!("{arr:?}");
            let tuple = <($($types,)+)>::from_array(arr.clone());
            let arr2 = tuple.into_array();
            println!("{arr2:?}");
            assert_eq!(arr2, arr);
        }
    }

    #[test]
    fn test_from_array_0() {
        let arr: [String; 0] = [];
        #[allow(clippy::let_unit_value)]
        let tuple = <()>::from_array(arr);
        let _: [String; 0] = tuple.into_array();
    }

    #[test]
    fn test_into_array_change_type_0_elements() {
        let arr: [String; 0] = [];
        #[allow(clippy::let_unit_value)]
        let tuple = <()>::from_array(arr);
        let _: [u8; 0] = tuple.into_array();
    }

    #[test]
    fn test_from_array_1() {
        test_from_array!(String);
    }

    #[test]
    fn test_from_array_2() {
        test_from_array!(String, String);
    }

    #[test]
    fn test_from_array_3() {
        test_from_array!(String, String, String);
    }

    #[test]
    fn test_from_array_16() {
        test_from_array!(
            String, String, String, String, String, String, String, String, String, String, String,
            String, String, String, String, String
        );
    }
}
