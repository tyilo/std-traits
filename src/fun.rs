use crate::{primitive::Primitive, tuple::Tuple};

pub trait FunctionPointer: Primitive + Copy + Sized {
    type Args: Tuple;
    type Return;

    fn call(self, args: Self::Args) -> Self::Return;
}

impl<R> Primitive for fn() -> R {}
impl<R> FunctionPointer for fn() -> R {
    type Args = ();
    type Return = R;

    fn call(self, _args: Self::Args) -> Self::Return {
        self()
    }
}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for function pointers with up to 16 arguments."
)]
impl<A1, R> Primitive for fn(A1) -> R {}

#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for function pointers with up to 16 arguments."
)]
impl<A1, R> FunctionPointer for fn(A1) -> R {
    type Args = (A1,);
    type Return = R;

    fn call(self, args: Self::Args) -> Self::Return {
        self(args.0)
    }
}

macro_rules! impl_fn {
    ($($args:tt $n:tt),*) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$($args,)* R> Primitive for fn($($args,)*) -> R {}
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$($args,)* R> FunctionPointer for fn($($args,)*) -> R {
            type Args = ($($args,)*);
            type Return = R;

            fn call(self, args: Self::Args) -> Self::Return {
                self($(args.$n),*)
            }
        }
    }
}

/*
for n in range(2, 17):
    print(f"impl_fn!({', '.join(f'A{i + 1} {i}' for i in range(n))});")
*/
impl_fn!(A1 0, A2 1);
impl_fn!(A1 0, A2 1, A3 2);
impl_fn!(A1 0, A2 1, A3 2, A4 3);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9, A11 10);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9, A11 10, A12 11);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9, A11 10, A12 11, A13 12);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9, A11 10, A12 11, A13 12, A14 13);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9, A11 10, A12 11, A13 12, A14 13, A15 14);
impl_fn!(A1 0, A2 1, A3 2, A4 3, A5 4, A6 5, A7 6, A8 7, A9 8, A10 9, A11 10, A12 11, A13 12, A14 13, A15 14, A16 15);
