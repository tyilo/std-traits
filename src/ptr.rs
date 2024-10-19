use core::fmt::Debug;

use crate::primitive::Primitive;

pub trait Pointer: Primitive + Copy + Debug + Sized {}

impl<T: ?Sized> Primitive for *const T {}
impl<T: ?Sized> Pointer for *const T {}
impl<T: ?Sized> Primitive for *mut T {}
impl<T: ?Sized> Pointer for *mut T {}
