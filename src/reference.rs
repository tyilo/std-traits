use crate::primitive::Primitive;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mutability {
    Const,
    Mut,
}

pub trait Reference<T: ?Sized>: Primitive + Sized {
    fn ref_type() -> Mutability;

    fn as_ref(&self) -> &Self;
}

macro_rules! impl_reference {
    ($ref:tt, $ty:expr) => {
        #[allow(unused_parens)]
        impl<T: ?Sized> Primitive for $ref {}

        #[allow(unused_parens)]
        impl<T: ?Sized> Reference<T> for $ref {
            fn ref_type() -> Mutability {
                $ty
            }

            fn as_ref(&self) -> &Self {
                self
            }
        }
    };
}

impl_reference!((&T), Mutability::Const);
impl_reference!((&mut T), Mutability::Mut);
