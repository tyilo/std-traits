#![no_std]
#![cfg_attr(docsrs, feature(rustdoc_internals))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod array;
pub mod fun;
pub mod num;
pub mod primitive;
pub mod ptr;
pub mod reference;
pub mod slice;
pub mod tuple;
