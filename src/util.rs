use core::mem::{ManuallyDrop, transmute_copy};

pub(crate) unsafe fn transmute_unchecked<Src, Dst>(v: Src) -> Dst {
    unsafe { transmute_copy::<ManuallyDrop<Src>, Dst>(&ManuallyDrop::new(v)) }
}

macro_rules! cast_generic {
    ($value:expr => $ty:ty; $($tys:ty)*) => {
        loop {
            let type_id = ::core::any::TypeId::of::<$ty>();
            $(
                if type_id == ::core::any::TypeId::of::<$tys>() {
                    #[allow(clippy::fn_to_numeric_cast)]
                    #[allow(clippy::fn_to_numeric_cast_with_truncation)]
                    break unsafe { $crate::util::transmute_unchecked::<$tys, $ty>($value as _) };
                }
            )*
            panic!("unexpected cast target type: {}", ::core::any::type_name::<$ty>());
        }
    }
}

macro_rules! cast_int {
    ($value:expr => $ty:ty) => {
        $crate::util::cast_generic!($value => $ty;
            i8 i16 i32 i64 i128 isize
            u8 u16 u32 u64 u128 usize
        )
    }
}

pub(crate) use cast_generic;
pub(crate) use cast_int;
