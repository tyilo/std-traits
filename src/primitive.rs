/// A useless(?) trait for all primitive types in Rust.
pub trait Primitive: private::Sealed {}

mod private {
    pub trait Sealed {}

    impl<T: ?Sized + super::Primitive> Sealed for T {}
}
