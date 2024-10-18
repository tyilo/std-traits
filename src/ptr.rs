use crate::primitive::Primitive;

use core::fmt::Debug;

pub trait Pointer: Primitive + Copy + Debug + Sized {}

impl<T: ?Sized> Primitive for *const T {}
impl<T: ?Sized> Pointer for *const T {}
impl<T: ?Sized> Primitive for *mut T {}
impl<T: ?Sized> Pointer for *mut T {}
