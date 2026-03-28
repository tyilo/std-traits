use core::fmt::Debug;

use crate::{num::Integer, primitive::Primitive, reference::Mutability, util::cast_int};

macro_rules! cast_ptr {
    ($T:ty, $P:ty, $value:expr) => {
        match <$P>::ptr_type() {
            $crate::reference::Mutability::Const => unsafe {
                $crate::util::transmute_unchecked::<*const $T, $P>($value as *const $T)
            },
            $crate::reference::Mutability::Mut => unsafe {
                $crate::util::transmute_unchecked::<*mut $T, $P>($value as *mut $T)
            },
        }
    };
}

pub(crate) use cast_ptr;

pub trait Pointer<T: ?Sized>: Primitive + Copy + Debug + Sized {
    fn ptr_type() -> Mutability;

    fn cast_ptr<U: Sized, P: Pointer<U>>(self) -> P
    where
        Self: Sized;

    fn cast_int<I: Integer>(self) -> I
    where
        T: Sized;
}

macro_rules! impl_pointer {
    ($mut:tt, $ty:expr) => {
        impl<T: ?Sized> Primitive for *$mut T {}

        impl<T: ?Sized> Pointer<T> for *$mut T {
            fn ptr_type() -> Mutability {
                $ty
            }

            fn cast_ptr<U: Sized, P: Pointer<U>>(self) -> P where Self: Sized {
                cast_ptr!(T, P, self)
            }

            fn cast_int<I: Integer>(self) -> I where T: Sized {
                cast_int!(self => I)
            }
        }
    }
}

impl_pointer!(const, Mutability::Const);
impl_pointer!(mut, Mutability::Mut);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cast() {
        let ptr: *const u8 = "foo".as_ptr();
        let int: usize = ptr.cast_int();
        let ptr2: *const u8 = int.cast_ptr();

        assert_eq!(ptr, ptr2);
    }
}
