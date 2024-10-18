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

impl<A1, R> Primitive for fn(A1) -> R {}
impl<A1, R> FunctionPointer for fn(A1) -> R {
    type Args = (A1,);
    type Return = R;
    fn call(self, args: Self::Args) -> Self::Return {
        self(args.0)
    }
}
impl<A1, A2, R> Primitive for fn(A1, A2) -> R {}
impl<A1, A2, R> FunctionPointer for fn(A1, A2) -> R {
    type Args = (A1, A2);
    type Return = R;
    fn call(self, args: Self::Args) -> Self::Return {
        self(args.0, args.1)
    }
}
