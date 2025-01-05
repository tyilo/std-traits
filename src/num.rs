use core::{
    cmp::Ordering,
    fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex},
    hash::Hash,
    iter::{Product, Sum},
    mem::{size_of, transmute},
    num::FpCategory,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
    str::FromStr,
};

use crate::{array::Array, primitive::Primitive};

pub trait NumberLike:
    Primitive
    + Copy
    + Default
    + FromStr
    + PartialEq
    + PartialOrd
    + Debug
    + Display
    + Unpin
    + UnwindSafe
    + RefUnwindSafe
    + Send
    + Sync
    + Sized
    + 'static
{
    /// Same as the builtin `MIN` associated constant, except that this is `NEG_INFINITY` for
    /// floats instead of the minimum finite value.
    const MIN: Self;
    /// Same as the builtin `MAX` associated constant, except that this is `INFINITY` for
    /// floats instead of the maximum finite value.
    const MAX: Self;

    type Underlying: Number;
    type ByteArray: Array<Item = u8>;

    fn to_underlying(self) -> Self::Underlying;
    fn try_from_underlying(underlying: Self::Underlying) -> Option<Self>;
    fn to_bytes(self) -> Self::ByteArray;
    fn try_from_bytes(bytes: Self::ByteArray) -> Option<Self>;

    fn to_be_bytes(self) -> Self::ByteArray;
    fn to_le_bytes(self) -> Self::ByteArray;
    fn to_ne_bytes(self) -> Self::ByteArray;
    fn try_from_be_bytes(bytes: Self::ByteArray) -> Option<Self>;
    fn try_from_le_bytes(bytes: Self::ByteArray) -> Option<Self>;
    fn try_from_ne_bytes(bytes: Self::ByteArray) -> Option<Self>;
}

macro_rules! impl_number_like {
    ($ty:ty, underlying: $number:ty, min: $min:expr, max: $max:expr, try_from_underlying: $try_from_underlying:expr) => {
        impl Primitive for $ty {}
        impl NumberLike for $ty {
            const MIN: Self = $min;
            const MAX: Self = $max;

            type Underlying = $number;
            type ByteArray = [u8; size_of::<Self>()];

            fn to_underlying(self) -> Self::Underlying {
                #[allow(clippy::useless_transmute)]
                unsafe {
                    transmute::<Self, Self::Underlying>(self)
                }
            }

            fn try_from_underlying(underlying: Self::Underlying) -> Option<Self> {
                $try_from_underlying(underlying)
            }

            fn to_bytes(self) -> Self::ByteArray {
                #[allow(clippy::transmute_num_to_bytes)]
                unsafe {
                    transmute::<Self, Self::ByteArray>(self)
                }
            }

            fn try_from_bytes(bytes: Self::ByteArray) -> Option<Self> {
                Self::try_from_underlying(Self::Underlying::from_bytes(bytes))
            }

            fn to_be_bytes(self) -> Self::ByteArray {
                self.to_underlying().to_be_bytes()
            }

            fn to_le_bytes(self) -> Self::ByteArray {
                self.to_underlying().to_le_bytes()
            }

            fn to_ne_bytes(self) -> Self::ByteArray {
                self.to_underlying().to_ne_bytes()
            }

            fn try_from_be_bytes(bytes: Self::ByteArray) -> Option<Self> {
                Self::try_from_underlying(Self::Underlying::from_be_bytes(bytes))
            }

            fn try_from_le_bytes(bytes: Self::ByteArray) -> Option<Self> {
                Self::try_from_underlying(Self::Underlying::from_le_bytes(bytes))
            }

            fn try_from_ne_bytes(bytes: Self::ByteArray) -> Option<Self> {
                Self::try_from_underlying(Self::Underlying::from_ne_bytes(bytes))
            }
        }
    };
}

impl_number_like!(bool,
    underlying: u8,
    min: false,
    max: true,
    try_from_underlying: |v| match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
);
impl_number_like!(char,
    underlying: u32,
    min: '\0',
    max: '\u{10ffff}',
    try_from_underlying: |v| char::try_from(v).ok()
);

pub trait Number:
    NumberLike
    + LowerExp
    + UpperExp
    + Add<Self>
    + for<'a> Add<&'a Self>
    + AddAssign<Self>
    + for<'a> AddAssign<&'a Self>
    + Sub<Self>
    + for<'a> Sub<&'a Self>
    + SubAssign<Self>
    + for<'a> SubAssign<&'a Self>
    + Mul<Self>
    + for<'a> Mul<&'a Self>
    + MulAssign<Self>
    + for<'a> MulAssign<&'a Self>
    + Div<Self>
    + for<'a> Div<&'a Self>
    + DivAssign<Self>
    + for<'a> DivAssign<&'a Self>
    + Rem<Self>
    + for<'a> Rem<&'a Self>
    + RemAssign<Self>
    + for<'a> RemAssign<&'a Self>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<i8>
    + TryFrom<i16>
    + Sum
    + Product
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    fn from_bytes(bytes: Self::ByteArray) -> Self;
    fn as_mut_bytes(&mut self) -> &mut Self::ByteArray;

    fn from_be_bytes(bytes: Self::ByteArray) -> Self;
    fn from_le_bytes(bytes: Self::ByteArray) -> Self;
    fn from_ne_bytes(bytes: Self::ByteArray) -> Self;
}

macro_rules! impl_number {
    ($ty:ty, zero: $zero:expr, one: $one:expr, min: $min:expr, max: $max:expr) => {
        impl_number_like!($ty,
            underlying: Self,
            min: $min,
            max: $max,
            try_from_underlying: |v| Some(v)
        );
        impl Number for $ty {
            const ZERO: Self = $zero;
            const ONE: Self = $one;
            const TWO: Self = $one + $one;

            fn from_bytes(bytes: Self::ByteArray) -> Self {
                unsafe { transmute::<Self::ByteArray, Self>(bytes) }
            }

            fn as_mut_bytes(&mut self) -> &mut Self::ByteArray {
                unsafe { transmute::<&mut Self, &mut Self::ByteArray>(self) }
            }

            fn from_be_bytes(bytes: Self::ByteArray) -> Self {
                Self::from_be_bytes(bytes)
            }

            fn from_le_bytes(bytes: Self::ByteArray) -> Self {
                Self::from_le_bytes(bytes)
            }

            fn from_ne_bytes(bytes: Self::ByteArray) -> Self {
                Self::from_ne_bytes(bytes)
            }
        }
    };
}

pub trait Float: Number + From<f32> + From<bool> + Into<f64> {
    const RADIX: u32;
    const MANTISSA_DIGITS: u32;
    const DIGITS: u32;
    const EPSILON: Self;

    const MIN_FINITE: Self;
    const MIN_POSITIVE_SUBNORMAL: Self;
    const MIN_POSITIVE_NORMAL: Self;
    const MIN_EXP: i32;
    const MIN_10_EXP: i32;

    const MAX_FINITE: Self;
    const MAX_NEGATIVE_SUBNORMAL: Self;
    const MAX_NEGATIVE_NORMAL: Self;
    const MAX_EXP: i32;
    const MAX_10_EXP: i32;

    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    const NEG_ZERO: Self;

    type Bits: Unsigned;

    // @START@ DECL FLOAT
    // Generated by generate_delegates.py

    /// See [`f32::is_nan`].
    fn is_nan(self) -> bool;

    /// See [`f32::is_infinite`].
    fn is_infinite(self) -> bool;

    /// See [`f32::is_finite`].
    fn is_finite(self) -> bool;

    /// See [`f32::is_subnormal`].
    fn is_subnormal(self) -> bool;

    /// See [`f32::is_normal`].
    fn is_normal(self) -> bool;

    /// See [`f32::classify`].
    fn classify(self) -> FpCategory;

    /// See [`f32::is_sign_positive`].
    fn is_sign_positive(self) -> bool;

    /// See [`f32::is_sign_negative`].
    fn is_sign_negative(self) -> bool;

    /// See [`f32::recip`].
    fn recip(self) -> Self;

    /// See [`f32::to_degrees`].
    fn to_degrees(self) -> Self;

    /// See [`f32::to_radians`].
    fn to_radians(self) -> Self;

    /// See [`f32::max`].
    fn max(self, other: Self) -> Self;

    /// See [`f32::min`].
    fn min(self, other: Self) -> Self;

    /// See [`f32::to_bits`].
    fn to_bits(self) -> Self::Bits;

    /// See [`f32::from_bits`].
    fn from_bits(v: Self::Bits) -> Self;

    /// See [`f32::total_cmp`].
    fn total_cmp(&self, other: &Self) -> Ordering;

    /// See [`f32::clamp`].
    fn clamp(self, min: Self, max: Self) -> Self;

    // @END@ DECL FLOAT
}

macro_rules! impl_float {
    ($ty:ty, $bits:ty, $min_positive_subnormal:expr) => {
        impl_number!($ty, zero: 0.0, one: 1.0, min: Self::NEG_INFINITY, max: Self::INFINITY);
        impl Float for $ty {
            const RADIX: u32 = Self::RADIX;
            const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;
            const DIGITS: u32 = Self::DIGITS;
            const EPSILON: Self = Self::EPSILON;

            const MIN_FINITE: Self = Self::MIN;
            const MIN_POSITIVE_SUBNORMAL: Self = $min_positive_subnormal;
            const MIN_POSITIVE_NORMAL: Self = Self::MIN_POSITIVE;
            const MIN_EXP: i32 = Self::MIN_EXP;
            const MIN_10_EXP: i32 = Self::MIN_10_EXP;

            const MAX_FINITE: Self = Self::MAX;
            const MAX_NEGATIVE_SUBNORMAL: Self = -Self::MIN_POSITIVE_SUBNORMAL;
            const MAX_NEGATIVE_NORMAL: Self = -Self::MIN_POSITIVE_NORMAL;
            const MAX_EXP: i32 = Self::MAX_EXP;
            const MAX_10_EXP: i32 = Self::MAX_10_EXP;

            const NAN: Self = Self::NAN;
            const INFINITY: Self = Self::INFINITY;
            const NEG_INFINITY: Self = Self::NEG_INFINITY;

            const NEG_ZERO: Self = -0.0;

            type Bits = $bits;

            // @START@ IMPL FLOAT
            // Generated by generate_delegates.py

            fn is_nan(self) -> bool {
                Self::is_nan(self)
            }

            fn is_infinite(self) -> bool {
                Self::is_infinite(self)
            }

            fn is_finite(self) -> bool {
                Self::is_finite(self)
            }

            fn is_subnormal(self) -> bool {
                Self::is_subnormal(self)
            }

            fn is_normal(self) -> bool {
                Self::is_normal(self)
            }

            fn classify(self) -> FpCategory {
                Self::classify(self)
            }

            fn is_sign_positive(self) -> bool {
                Self::is_sign_positive(self)
            }

            fn is_sign_negative(self) -> bool {
                Self::is_sign_negative(self)
            }

            fn recip(self) -> Self {
                Self::recip(self)
            }

            fn to_degrees(self) -> Self {
                Self::to_degrees(self)
            }

            fn to_radians(self) -> Self {
                Self::to_radians(self)
            }

            fn max(self, other: Self) -> Self {
                Self::max(self, other)
            }

            fn min(self, other: Self) -> Self {
                Self::min(self, other)
            }

            fn to_bits(self) -> Self::Bits {
                Self::to_bits(self)
            }

            fn from_bits(v: Self::Bits) -> Self {
                Self::from_bits(v)
            }

            fn total_cmp(&self, other: &Self) -> Ordering {
                Self::total_cmp(&self, other)
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                Self::clamp(self, min, max)
            }

            // @END@ IMPL FLOAT
        }
    };
}

impl_float!(f32, u32, 1e-45);
impl_float!(f64, u64, 5e-324);

pub trait Integer:
    Number
    + Ord
    + Eq
    + Not
    + BitAnd<Self>
    + for<'a> BitAnd<&'a Self>
    + BitAndAssign<Self>
    + for<'a> BitAndAssign<&'a Self>
    + BitOr<Self>
    + for<'a> BitOr<&'a Self>
    + BitOrAssign<Self>
    + for<'a> BitOrAssign<&'a Self>
    + BitXor<Self>
    + for<'a> BitXor<&'a Self>
    + BitXorAssign<Self>
    + for<'a> BitXorAssign<&'a Self>
    + Shl<Self>
    + for<'a> Shl<&'a Self>
    + ShlAssign<Self>
    + for<'a> ShlAssign<&'a Self>
    + Shr<Self>
    + for<'a> Shr<&'a Self>
    + ShrAssign<Self>
    + for<'a> ShrAssign<&'a Self>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<i128>
    + TryFrom<isize>
    + TryInto<u8>
    + TryInto<u16>
    + TryInto<u32>
    + TryInto<u64>
    + TryInto<u128>
    + TryInto<usize>
    + TryInto<i8>
    + TryInto<i16>
    + TryInto<i32>
    + TryInto<i64>
    + TryInto<i128>
    + TryInto<isize>
    + Hash
    + Binary
    + Octal
    + LowerHex
    + UpperHex
{
    type Unsigned: Unsigned;
    type Signed: Signed;

    fn to_unsigned(self) -> Self::Unsigned;
    fn to_signed(self) -> Self::Signed;
}

macro_rules! impl_integer {
    ($ty:ty, $unsigned:ty, $signed:ty) => {
        impl_number!($ty, zero: 0, one: 1, min: Self::MIN, max: Self::MAX);
        impl Integer for $ty {
            type Unsigned = $unsigned;
            type Signed = $signed;

            fn to_unsigned(self) -> Self::Unsigned {
                #[allow(clippy::useless_transmute)]
                unsafe { transmute::<Self, Self::Unsigned>(self) }
            }

            fn to_signed(self) -> Self::Signed {
                #[allow(clippy::useless_transmute)]
                unsafe { transmute::<Self, Self::Signed>(self) }
            }
        }
    };
}

pub trait Unsigned: Integer + From<u8> {}

macro_rules! impl_unsigned {
    ($ty:ty, $signed:ty) => {
        impl_integer!($ty, Self, $signed);
        impl Unsigned for $ty {}
    };
}

impl_unsigned!(u8, i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);
impl_unsigned!(u128, i128);
impl_unsigned!(usize, isize);

pub trait Signed: Integer + Neg + From<i8> {}

macro_rules! impl_signed {
    ($ty:ty, $unsigned:ty) => {
        impl_integer!($ty, $unsigned, Self);
        impl Signed for $ty {}
    };
}

impl_signed!(i8, u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);
impl_signed!(i128, u128);
impl_signed!(isize, usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subnormal_consts() {
        assert_eq!(f32::MIN_POSITIVE_SUBNORMAL, f32::from_bits(1));
        assert_eq!(f32::MAX_NEGATIVE_SUBNORMAL, -f32::from_bits(1));
        assert_eq!(f64::MIN_POSITIVE_SUBNORMAL, f64::from_bits(1));
        assert_eq!(f64::MAX_NEGATIVE_SUBNORMAL, -f64::from_bits(1));
    }
}
