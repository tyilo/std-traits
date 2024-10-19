use crate::primitive::Primitive;

pub trait Tuple: Primitive {
    const N: usize;
}

impl Primitive for () {}
impl Tuple for () {
    const N: usize = 0;
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

macro_rules! impl_tuple {
    ($n:expr => $($types:tt),*; $last:tt) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$($types,)* $last: ?Sized> Primitive for ($($types,)* $last,) {}
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$($types,)* $last: ?Sized> Tuple for ($($types,)* $last,) {
            const N: usize = $n;
        }
    }
}

/*
for n in range(2, 17):
    print(f"impl_tuple!({n} => {', '.join(f'T{i}' for i in range(1, n))}; T{n});")
*/
impl_tuple!(2 => T1; T2);
impl_tuple!(3 => T1, T2; T3);
impl_tuple!(4 => T1, T2, T3; T4);
impl_tuple!(5 => T1, T2, T3, T4; T5);
impl_tuple!(6 => T1, T2, T3, T4, T5; T6);
impl_tuple!(7 => T1, T2, T3, T4, T5, T6; T7);
impl_tuple!(8 => T1, T2, T3, T4, T5, T6, T7; T8);
impl_tuple!(9 => T1, T2, T3, T4, T5, T6, T7, T8; T9);
impl_tuple!(10 => T1, T2, T3, T4, T5, T6, T7, T8, T9; T10);
impl_tuple!(11 => T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; T11);
impl_tuple!(12 => T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; T12);
impl_tuple!(13 => T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; T13);
impl_tuple!(14 => T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13; T14);
impl_tuple!(15 => T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14; T15);
impl_tuple!(16 => T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15; T16);
