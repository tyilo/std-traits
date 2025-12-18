//! Traits for numbers and number-like types.
//!
//! Trait hierarchy:
//!
//! [`NumberLike`]:
//!   - [`bool`]
//!   - [`char`]
//!   - [`Number`]:
//!     - [`Float`]:
//!       - [`f32`], [`f64`]
//!     - [`Integer`]:
//!       - [`Signed`]:
//!         - [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`]
//!       - [`Unsigned`]:
//!         - [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`]

use core::{
    fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex},
    hash::Hash,
    iter::{Product, Sum},
    mem::{size_of, transmute},
    num::{FpCategory, ParseIntError},
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
    /// Same as the builtin `MIN` associated constant, except that this is
    /// [`NEG_INFINITY`](Float::NEG_INFINITY) for floats instead of the minimum
    /// finite value.
    const MIN: Self;
    /// Same as the builtin `MAX` associated constant, except that this is
    /// [`INFINITY`](Float::INFINITY) for floats instead of the maximum finite
    /// value.
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
    (
        $ty:ty,
        underlying: $number:ty,
        min: $min:expr,
        max: $max:expr,
        try_from_underlying: $try_from_underlying:expr
    ) => {
        impl Primitive for $ty {}
        impl NumberLike for $ty {
            const MIN: Self = $min;
            const MAX: Self = $max;

            type Underlying = $number;
            type ByteArray = [u8; size_of::<Self>()];

            fn to_underlying(self) -> Self::Underlying {
                #[allow(clippy::useless_transmute)]
                #[allow(unnecessary_transmutes)]
                unsafe {
                    transmute::<Self, Self::Underlying>(self)
                }
            }

            fn try_from_underlying(underlying: Self::Underlying) -> Option<Self> {
                $try_from_underlying(underlying)
            }

            fn to_bytes(self) -> Self::ByteArray {
                #[allow(unnecessary_transmutes)]
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
    + From<bool>
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

    /// See [`i32::abs`].
    fn abs(self) -> Self;

    /// See [`i32::signum`].
    fn signum(self) -> Self;

    #[cfg(feature = "std")]
    fn div_euclid(self, rhs: Self) -> Self;

    #[cfg(feature = "std")]
    fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! impl_number {
    (
        $ty:ty,
        zero: $zero:expr,
        one: $one:expr,
        min: $min:expr,
        max: $max:expr,
        abs: $abs:expr,
        signum: $signum:expr
    ) => {
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
                #[allow(unnecessary_transmutes)]
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

            fn abs(self) -> Self {
                $abs(self)
            }

            fn signum(self) -> Self {
                $signum(self)
            }

            #[cfg(feature = "std")]
            fn div_euclid(self, rhs: Self) -> Self {
                Self::div_euclid(self, rhs)
            }

            #[cfg(feature = "std")]
            fn rem_euclid(self, rhs: Self) -> Self {
                Self::rem_euclid(self, rhs)
            }
        }
    };
}

pub trait Float:
    Number + Neg + From<f32> + Into<f64> + From<i8> + From<i16> + From<u8> + From<u16>
{
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

    /// See [`f32::next_up`].
    fn next_up(self) -> Self;

    /// See [`f32::next_down`].
    fn next_down(self) -> Self;

    /// See [`f32::recip`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn recip(self) -> Self;

    /// See [`f32::to_degrees`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_degrees(self) -> Self;

    /// See [`f32::to_radians`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_radians(self) -> Self;

    /// See [`f32::max`].
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    fn max(self, other: Self) -> Self;

    /// See [`f32::min`].
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    fn min(self, other: Self) -> Self;

    /// See [`f32::midpoint`].
    fn midpoint(self, other: Self) -> Self;

    /// See [`f32::to_bits`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_bits(self) -> Self::Bits;

    /// See [`f32::from_bits`].
    fn from_bits(v: Self::Bits) -> Self;

    /// See [`f32::total_cmp`].
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering;

    /// See [`f32::clamp`].
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// See [`f32::copysign`].
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn copysign(self, sign: Self) -> Self;

    /// See [`f32::floor`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn floor(self) -> Self;

    /// See [`f32::ceil`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn ceil(self) -> Self;

    /// See [`f32::round`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn round(self) -> Self;

    /// See [`f32::round_ties_even`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn round_ties_even(self) -> Self;

    /// See [`f32::trunc`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn trunc(self) -> Self;

    /// See [`f32::fract`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn fract(self) -> Self;

    /// See [`f32::mul_add`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// See [`f32::powi`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn powi(self, n: i32) -> Self;

    /// See [`f32::powf`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn powf(self, n: Self) -> Self;

    /// See [`f32::sqrt`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn sqrt(self) -> Self;

    /// See [`f32::exp`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn exp(self) -> Self;

    /// See [`f32::exp2`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn exp2(self) -> Self;

    /// See [`f32::ln`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn ln(self) -> Self;

    /// See [`f32::log`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn log(self, base: Self) -> Self;

    /// See [`f32::log2`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn log2(self) -> Self;

    /// See [`f32::log10`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn log10(self) -> Self;

    /// See [`f32::cbrt`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn cbrt(self) -> Self;

    /// See [`f32::hypot`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn hypot(self, other: Self) -> Self;

    /// See [`f32::sin`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn sin(self) -> Self;

    /// See [`f32::cos`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn cos(self) -> Self;

    /// See [`f32::tan`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn tan(self) -> Self;

    /// See [`f32::asin`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn asin(self) -> Self;

    /// See [`f32::acos`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn acos(self) -> Self;

    /// See [`f32::atan`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn atan(self) -> Self;

    /// See [`f32::atan2`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn atan2(self, other: Self) -> Self;

    /// See [`f32::sin_cos`].
    #[cfg(feature = "std")]
    fn sin_cos(self) -> (Self, Self);

    /// See [`f32::exp_m1`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn exp_m1(self) -> Self;

    /// See [`f32::ln_1p`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn ln_1p(self) -> Self;

    /// See [`f32::sinh`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn sinh(self) -> Self;

    /// See [`f32::cosh`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn cosh(self) -> Self;

    /// See [`f32::tanh`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn tanh(self) -> Self;

    /// See [`f32::asinh`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn asinh(self) -> Self;

    /// See [`f32::acosh`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn acosh(self) -> Self;

    /// See [`f32::atanh`].
    #[cfg(feature = "std")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn atanh(self) -> Self;

    // @END@ DECL FLOAT
}

macro_rules! impl_float {
    ($ty:ty, $bits:ty, $min_positive_subnormal:expr) => {
        impl_number!(
            $ty,
            zero: 0.0,
            one: 1.0,
            min: Self::NEG_INFINITY,
            max: Self::INFINITY,
            abs: Self::abs,
            signum: Self::signum
        );
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

            fn next_up(self) -> Self {
                Self::next_up(self)
            }

            fn next_down(self) -> Self {
                Self::next_down(self)
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

            fn midpoint(self, other: Self) -> Self {
                Self::midpoint(self, other)
            }

            fn to_bits(self) -> Self::Bits {
                Self::to_bits(self)
            }

            fn from_bits(v: Self::Bits) -> Self {
                Self::from_bits(v)
            }

            fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
                Self::total_cmp(self, other)
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                Self::clamp(self, min, max)
            }

            fn copysign(self, sign: Self) -> Self {
                Self::copysign(self, sign)
            }

            #[cfg(feature = "std")]
            fn floor(self) -> Self {
                Self::floor(self)
            }

            #[cfg(feature = "std")]
            fn ceil(self) -> Self {
                Self::ceil(self)
            }

            #[cfg(feature = "std")]
            fn round(self) -> Self {
                Self::round(self)
            }

            #[cfg(feature = "std")]
            fn round_ties_even(self) -> Self {
                Self::round_ties_even(self)
            }

            #[cfg(feature = "std")]
            fn trunc(self) -> Self {
                Self::trunc(self)
            }

            #[cfg(feature = "std")]
            fn fract(self) -> Self {
                Self::fract(self)
            }

            #[cfg(feature = "std")]
            fn mul_add(self, a: Self, b: Self) -> Self {
                Self::mul_add(self, a, b)
            }

            #[cfg(feature = "std")]
            fn powi(self, n: i32) -> Self {
                Self::powi(self, n)
            }

            #[cfg(feature = "std")]
            fn powf(self, n: Self) -> Self {
                Self::powf(self, n)
            }

            #[cfg(feature = "std")]
            fn sqrt(self) -> Self {
                Self::sqrt(self)
            }

            #[cfg(feature = "std")]
            fn exp(self) -> Self {
                Self::exp(self)
            }

            #[cfg(feature = "std")]
            fn exp2(self) -> Self {
                Self::exp2(self)
            }

            #[cfg(feature = "std")]
            fn ln(self) -> Self {
                Self::ln(self)
            }

            #[cfg(feature = "std")]
            fn log(self, base: Self) -> Self {
                Self::log(self, base)
            }

            #[cfg(feature = "std")]
            fn log2(self) -> Self {
                Self::log2(self)
            }

            #[cfg(feature = "std")]
            fn log10(self) -> Self {
                Self::log10(self)
            }

            #[cfg(feature = "std")]
            fn cbrt(self) -> Self {
                Self::cbrt(self)
            }

            #[cfg(feature = "std")]
            fn hypot(self, other: Self) -> Self {
                Self::hypot(self, other)
            }

            #[cfg(feature = "std")]
            fn sin(self) -> Self {
                Self::sin(self)
            }

            #[cfg(feature = "std")]
            fn cos(self) -> Self {
                Self::cos(self)
            }

            #[cfg(feature = "std")]
            fn tan(self) -> Self {
                Self::tan(self)
            }

            #[cfg(feature = "std")]
            fn asin(self) -> Self {
                Self::asin(self)
            }

            #[cfg(feature = "std")]
            fn acos(self) -> Self {
                Self::acos(self)
            }

            #[cfg(feature = "std")]
            fn atan(self) -> Self {
                Self::atan(self)
            }

            #[cfg(feature = "std")]
            fn atan2(self, other: Self) -> Self {
                Self::atan2(self, other)
            }

            #[cfg(feature = "std")]
            fn sin_cos(self) -> (Self, Self) {
                Self::sin_cos(self)
            }

            #[cfg(feature = "std")]
            fn exp_m1(self) -> Self {
                Self::exp_m1(self)
            }

            #[cfg(feature = "std")]
            fn ln_1p(self) -> Self {
                Self::ln_1p(self)
            }

            #[cfg(feature = "std")]
            fn sinh(self) -> Self {
                Self::sinh(self)
            }

            #[cfg(feature = "std")]
            fn cosh(self) -> Self {
                Self::cosh(self)
            }

            #[cfg(feature = "std")]
            fn tanh(self) -> Self {
                Self::tanh(self)
            }

            #[cfg(feature = "std")]
            fn asinh(self) -> Self {
                Self::asinh(self)
            }

            #[cfg(feature = "std")]
            fn acosh(self) -> Self {
                Self::acosh(self)
            }

            #[cfg(feature = "std")]
            fn atanh(self) -> Self {
                Self::atanh(self)
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

    fn from_unsigned(v: Self::Unsigned) -> Self;
    fn from_signed(v: Self::Signed) -> Self;
    fn to_unsigned(self) -> Self::Unsigned;
    fn to_signed(self) -> Self::Signed;

    /// See [`i32::div_euclid`].
    #[cfg(not(feature = "std"))]
    fn div_euclid(self, rhs: Self) -> Self;

    /// See [`i32::rem_euclid`].
    #[cfg(not(feature = "std"))]
    fn rem_euclid(self, rhs: Self) -> Self;

    // @START@ DECL INTEGER
    // Generated by generate_delegates.py

    /// See [`i32::from_str_radix`].
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;

    /// See [`i32::count_ones`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_ones(self) -> u32;

    /// See [`i32::count_zeros`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_zeros(self) -> u32;

    /// See [`i32::leading_zeros`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_zeros(self) -> u32;

    /// See [`i32::trailing_zeros`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_zeros(self) -> u32;

    /// See [`i32::leading_ones`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_ones(self) -> u32;

    /// See [`i32::trailing_ones`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_ones(self) -> u32;

    /// See [`i32::rotate_left`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_left(self, n: u32) -> Self;

    /// See [`i32::rotate_right`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_right(self, n: u32) -> Self;

    /// See [`i32::swap_bytes`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn swap_bytes(self) -> Self;

    /// See [`i32::reverse_bits`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn reverse_bits(self) -> Self;

    /// See [`i32::from_be`].
    fn from_be(x: Self) -> Self;

    /// See [`i32::from_le`].
    fn from_le(x: Self) -> Self;

    /// See [`i32::to_be`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_be(self) -> Self;

    /// See [`i32::to_le`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_le(self) -> Self;

    /// See [`i32::checked_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_add(self, rhs: Self) -> Self;

    /// See [`i32::unchecked_add`].
    ///
    /// # Safety
    ///
    /// See [`i32::unchecked_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_add(self, rhs: Self) -> Self;

    /// See [`i32::checked_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_sub(self, rhs: Self) -> Self;

    /// See [`i32::unchecked_sub`].
    ///
    /// # Safety
    ///
    /// See [`i32::unchecked_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_sub(self, rhs: Self) -> Self;

    /// See [`i32::checked_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_mul(self, rhs: Self) -> Self;

    /// See [`i32::unchecked_mul`].
    ///
    /// # Safety
    ///
    /// See [`i32::unchecked_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_mul(self, rhs: Self) -> Self;

    /// See [`i32::checked_div`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_div`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_div(self, rhs: Self) -> Self;

    /// See [`i32::checked_div_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_div_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_div_euclid(self, rhs: Self) -> Self;

    /// See [`i32::checked_rem`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_rem`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_rem(self, rhs: Self) -> Self;

    /// See [`i32::checked_rem_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

    /// See [`i32::strict_rem_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_rem_euclid(self, rhs: Self) -> Self;

    /// See [`i32::checked_neg`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_neg(self) -> Option<Self>;

    /// See [`i32::strict_neg`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_neg(self) -> Self;

    /// See [`i32::checked_shl`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_shl(self, rhs: u32) -> Option<Self>;

    /// See [`i32::strict_shl`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_shl(self, rhs: u32) -> Self;

    /// See [`i32::unbounded_shl`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn unbounded_shl(self, rhs: u32) -> Self;

    /// See [`i32::checked_shr`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_shr(self, rhs: u32) -> Option<Self>;

    /// See [`i32::strict_shr`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_shr(self, rhs: u32) -> Self;

    /// See [`i32::unbounded_shr`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn unbounded_shr(self, rhs: u32) -> Self;

    /// See [`i32::checked_pow`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_pow(self, exp: u32) -> Option<Self>;

    /// See [`i32::strict_pow`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_pow(self, exp: u32) -> Self;

    /// See [`i32::saturating_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add(self, rhs: Self) -> Self;

    /// See [`i32::saturating_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub(self, rhs: Self) -> Self;

    /// See [`i32::saturating_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_mul(self, rhs: Self) -> Self;

    /// See [`i32::saturating_div`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_div(self, rhs: Self) -> Self;

    /// See [`i32::saturating_pow`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_pow(self, exp: u32) -> Self;

    /// See [`i32::wrapping_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_add(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_div`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_div_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div_euclid(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_rem`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_rem_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;

    /// See [`i32::wrapping_neg`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_neg(self) -> Self;

    /// See [`i32::wrapping_shl`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_shl(self, rhs: u32) -> Self;

    /// See [`i32::wrapping_shr`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_shr(self, rhs: u32) -> Self;

    /// See [`i32::wrapping_pow`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_pow(self, exp: u32) -> Self;

    /// See [`i32::overflowing_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_div`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_div(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_div_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_rem`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_rem_euclid`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);

    /// See [`i32::overflowing_neg`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_neg(self) -> (Self, bool);

    /// See [`i32::overflowing_shl`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);

    /// See [`i32::overflowing_shr`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);

    /// See [`i32::overflowing_pow`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    /// See [`i32::pow`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn pow(self, exp: u32) -> Self;

    /// See [`i32::isqrt`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn isqrt(self) -> Self;

    /// See [`i32::ilog`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog(self, base: Self) -> u32;

    /// See [`i32::ilog2`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog2(self) -> u32;

    /// See [`i32::ilog10`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog10(self) -> u32;

    /// See [`i32::checked_ilog`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_ilog(self, base: Self) -> Option<u32>;

    /// See [`i32::checked_ilog2`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_ilog2(self) -> Option<u32>;

    /// See [`i32::checked_ilog10`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_ilog10(self) -> Option<u32>;

    /// See [`i32::abs_diff`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs_diff(self, other: Self) -> Self::Unsigned;

    /// See [`i32::midpoint`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn midpoint(self, rhs: Self) -> Self;

    // @END@ DECL INTEGER
}

macro_rules! impl_integer {
    ($ty:ty, $unsigned:ty, $signed:ty, abs: $abs:expr, signum: $signum:expr) => {
        impl_number!(
            $ty,
            zero: 0,
            one: 1,
            min: Self::MIN,
            max: Self::MAX,
            abs: $abs,
            signum: $signum
        );
        impl Integer for $ty {
            type Unsigned = $unsigned;
            type Signed = $signed;

            fn from_unsigned(v: Self::Unsigned) -> Self {
                #[allow(clippy::useless_transmute)]
                #[allow(unnecessary_transmutes)]
                unsafe { transmute::<Self::Unsigned, Self>(v) }
            }

            fn from_signed(v: Self::Signed) -> Self {
                #[allow(clippy::useless_transmute)]
                #[allow(unnecessary_transmutes)]
                unsafe { transmute::<Self::Signed, Self>(v) }
            }

            fn to_unsigned(self) -> Self::Unsigned {
                #[allow(clippy::useless_transmute)]
                #[allow(unnecessary_transmutes)]
                unsafe { transmute::<Self, Self::Unsigned>(self) }
            }

            fn to_signed(self) -> Self::Signed {
                #[allow(clippy::useless_transmute)]
                #[allow(unnecessary_transmutes)]
                unsafe { transmute::<Self, Self::Signed>(self) }
            }

            #[cfg(not(feature = "std"))]
            fn div_euclid(self, rhs: Self) -> Self {
                Self::div_euclid(self, rhs)
            }

            #[cfg(not(feature = "std"))]
            fn rem_euclid(self, rhs: Self) -> Self {
                Self::rem_euclid(self, rhs)
            }

            // @START@ IMPL INTEGER
            // Generated by generate_delegates.py

            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                Self::from_str_radix(src, radix)
            }

            fn count_ones(self) -> u32 {
                Self::count_ones(self)
            }

            fn count_zeros(self) -> u32 {
                Self::count_zeros(self)
            }

            fn leading_zeros(self) -> u32 {
                Self::leading_zeros(self)
            }

            fn trailing_zeros(self) -> u32 {
                Self::trailing_zeros(self)
            }

            fn leading_ones(self) -> u32 {
                Self::leading_ones(self)
            }

            fn trailing_ones(self) -> u32 {
                Self::trailing_ones(self)
            }

            fn rotate_left(self, n: u32) -> Self {
                Self::rotate_left(self, n)
            }

            fn rotate_right(self, n: u32) -> Self {
                Self::rotate_right(self, n)
            }

            fn swap_bytes(self) -> Self {
                Self::swap_bytes(self)
            }

            fn reverse_bits(self) -> Self {
                Self::reverse_bits(self)
            }

            fn from_be(x: Self) -> Self {
                Self::from_be(x)
            }

            fn from_le(x: Self) -> Self {
                Self::from_le(x)
            }

            fn to_be(self) -> Self {
                Self::to_be(self)
            }

            fn to_le(self) -> Self {
                Self::to_le(self)
            }

            fn checked_add(self, rhs: Self) -> Option<Self> {
                Self::checked_add(self, rhs)
            }

            fn strict_add(self, rhs: Self) -> Self {
                Self::strict_add(self, rhs)
            }

            unsafe fn unchecked_add(self, rhs: Self) -> Self {
                Self::unchecked_add(self, rhs)
            }

            fn checked_sub(self, rhs: Self) -> Option<Self> {
                Self::checked_sub(self, rhs)
            }

            fn strict_sub(self, rhs: Self) -> Self {
                Self::strict_sub(self, rhs)
            }

            unsafe fn unchecked_sub(self, rhs: Self) -> Self {
                Self::unchecked_sub(self, rhs)
            }

            fn checked_mul(self, rhs: Self) -> Option<Self> {
                Self::checked_mul(self, rhs)
            }

            fn strict_mul(self, rhs: Self) -> Self {
                Self::strict_mul(self, rhs)
            }

            unsafe fn unchecked_mul(self, rhs: Self) -> Self {
                Self::unchecked_mul(self, rhs)
            }

            fn checked_div(self, rhs: Self) -> Option<Self> {
                Self::checked_div(self, rhs)
            }

            fn strict_div(self, rhs: Self) -> Self {
                Self::strict_div(self, rhs)
            }

            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                Self::checked_div_euclid(self, rhs)
            }

            fn strict_div_euclid(self, rhs: Self) -> Self {
                Self::strict_div_euclid(self, rhs)
            }

            fn checked_rem(self, rhs: Self) -> Option<Self> {
                Self::checked_rem(self, rhs)
            }

            fn strict_rem(self, rhs: Self) -> Self {
                Self::strict_rem(self, rhs)
            }

            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                Self::checked_rem_euclid(self, rhs)
            }

            fn strict_rem_euclid(self, rhs: Self) -> Self {
                Self::strict_rem_euclid(self, rhs)
            }

            fn checked_neg(self) -> Option<Self> {
                Self::checked_neg(self)
            }

            fn strict_neg(self) -> Self {
                Self::strict_neg(self)
            }

            fn checked_shl(self, rhs: u32) -> Option<Self> {
                Self::checked_shl(self, rhs)
            }

            fn strict_shl(self, rhs: u32) -> Self {
                Self::strict_shl(self, rhs)
            }

            fn unbounded_shl(self, rhs: u32) -> Self {
                Self::unbounded_shl(self, rhs)
            }

            fn checked_shr(self, rhs: u32) -> Option<Self> {
                Self::checked_shr(self, rhs)
            }

            fn strict_shr(self, rhs: u32) -> Self {
                Self::strict_shr(self, rhs)
            }

            fn unbounded_shr(self, rhs: u32) -> Self {
                Self::unbounded_shr(self, rhs)
            }

            fn checked_pow(self, exp: u32) -> Option<Self> {
                Self::checked_pow(self, exp)
            }

            fn strict_pow(self, exp: u32) -> Self {
                Self::strict_pow(self, exp)
            }

            fn saturating_add(self, rhs: Self) -> Self {
                Self::saturating_add(self, rhs)
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                Self::saturating_sub(self, rhs)
            }

            fn saturating_mul(self, rhs: Self) -> Self {
                Self::saturating_mul(self, rhs)
            }

            fn saturating_div(self, rhs: Self) -> Self {
                Self::saturating_div(self, rhs)
            }

            fn saturating_pow(self, exp: u32) -> Self {
                Self::saturating_pow(self, exp)
            }

            fn wrapping_add(self, rhs: Self) -> Self {
                Self::wrapping_add(self, rhs)
            }

            fn wrapping_sub(self, rhs: Self) -> Self {
                Self::wrapping_sub(self, rhs)
            }

            fn wrapping_mul(self, rhs: Self) -> Self {
                Self::wrapping_mul(self, rhs)
            }

            fn wrapping_div(self, rhs: Self) -> Self {
                Self::wrapping_div(self, rhs)
            }

            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                Self::wrapping_div_euclid(self, rhs)
            }

            fn wrapping_rem(self, rhs: Self) -> Self {
                Self::wrapping_rem(self, rhs)
            }

            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                Self::wrapping_rem_euclid(self, rhs)
            }

            fn wrapping_neg(self) -> Self {
                Self::wrapping_neg(self)
            }

            fn wrapping_shl(self, rhs: u32) -> Self {
                Self::wrapping_shl(self, rhs)
            }

            fn wrapping_shr(self, rhs: u32) -> Self {
                Self::wrapping_shr(self, rhs)
            }

            fn wrapping_pow(self, exp: u32) -> Self {
                Self::wrapping_pow(self, exp)
            }

            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_add(self, rhs)
            }

            fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_sub(self, rhs)
            }

            fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_mul(self, rhs)
            }

            fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_div(self, rhs)
            }

            fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_div_euclid(self, rhs)
            }

            fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_rem(self, rhs)
            }

            fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                Self::overflowing_rem_euclid(self, rhs)
            }

            fn overflowing_neg(self) -> (Self, bool) {
                Self::overflowing_neg(self)
            }

            fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                Self::overflowing_shl(self, rhs)
            }

            fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                Self::overflowing_shr(self, rhs)
            }

            fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                Self::overflowing_pow(self, exp)
            }

            fn pow(self, exp: u32) -> Self {
                Self::pow(self, exp)
            }

            fn isqrt(self) -> Self {
                Self::isqrt(self)
            }

            fn ilog(self, base: Self) -> u32 {
                Self::ilog(self, base)
            }

            fn ilog2(self) -> u32 {
                Self::ilog2(self)
            }

            fn ilog10(self) -> u32 {
                Self::ilog10(self)
            }

            fn checked_ilog(self, base: Self) -> Option<u32> {
                Self::checked_ilog(self, base)
            }

            fn checked_ilog2(self) -> Option<u32> {
                Self::checked_ilog2(self)
            }

            fn checked_ilog10(self) -> Option<u32> {
                Self::checked_ilog10(self)
            }

            fn abs_diff(self, other: Self) -> Self::Unsigned {
                Self::abs_diff(self, other)
            }

            fn midpoint(self, rhs: Self) -> Self {
                Self::midpoint(self, rhs)
            }

            // @END@ IMPL INTEGER
        }
    };
}

pub trait Unsigned: Integer<Unsigned = Self> + From<u8> {
    // @START@ DECL UNSIGNED
    // Generated by generate_delegates.py

    /// See [`u32::cast_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_signed(self) -> Self::Signed;

    /// See [`u32::checked_add_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self>;

    /// See [`u32::strict_add_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_add_signed(self, rhs: Self::Signed) -> Self;

    /// See [`u32::checked_sub_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_sub_signed(self, rhs: Self::Signed) -> Option<Self>;

    /// See [`u32::strict_sub_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_sub_signed(self, rhs: Self::Signed) -> Self;

    /// See [`u32::checked_signed_diff`].
    fn checked_signed_diff(self, rhs: Self) -> Option<Self::Signed>;

    /// See [`u32::saturating_add_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add_signed(self, rhs: Self::Signed) -> Self;

    /// See [`u32::saturating_sub_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub_signed(self, rhs: Self::Signed) -> Self;

    /// See [`u32::wrapping_add_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_add_signed(self, rhs: Self::Signed) -> Self;

    /// See [`u32::wrapping_sub_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_sub_signed(self, rhs: Self::Signed) -> Self;

    /// See [`u32::carrying_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool);

    /// See [`u32::overflowing_add_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_add_signed(self, rhs: Self::Signed) -> (Self, bool);

    /// See [`u32::borrowing_sub`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool);

    /// See [`u32::overflowing_sub_signed`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_sub_signed(self, rhs: Self::Signed) -> (Self, bool);

    /// See [`u32::carrying_mul`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self);

    /// See [`u32::carrying_mul_add`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Self, Self);

    /// See [`u32::div_ceil`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn div_ceil(self, rhs: Self) -> Self;

    /// See [`u32::next_multiple_of`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn next_multiple_of(self, rhs: Self) -> Self;

    /// See [`u32::checked_next_multiple_of`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_next_multiple_of(self, rhs: Self) -> Option<Self>;

    /// See [`u32::is_multiple_of`].
    fn is_multiple_of(self, rhs: Self) -> bool;

    /// See [`u32::is_power_of_two`].
    fn is_power_of_two(self) -> bool;

    /// See [`u32::next_power_of_two`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn next_power_of_two(self) -> Self;

    /// See [`u32::checked_next_power_of_two`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_next_power_of_two(self) -> Option<Self>;

    // @END@ IMPL UNSIGNED
}

macro_rules! impl_unsigned {
    ($ty:ty, $signed:ty) => {
        impl_integer!(
            $ty,
            Self,
            $signed,
            abs: |v| v,
            signum: |v| (v > 0) as Self
        );
        impl Unsigned for $ty {
            // @START@ IMPL UNSIGNED
            // Generated by generate_delegates.py

            fn cast_signed(self) -> Self::Signed {
                Self::cast_signed(self)
            }

            fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self> {
                Self::checked_add_signed(self, rhs)
            }

            fn strict_add_signed(self, rhs: Self::Signed) -> Self {
                Self::strict_add_signed(self, rhs)
            }

            fn checked_sub_signed(self, rhs: Self::Signed) -> Option<Self> {
                Self::checked_sub_signed(self, rhs)
            }

            fn strict_sub_signed(self, rhs: Self::Signed) -> Self {
                Self::strict_sub_signed(self, rhs)
            }

            fn checked_signed_diff(self, rhs: Self) -> Option<Self::Signed> {
                Self::checked_signed_diff(self, rhs)
            }

            fn saturating_add_signed(self, rhs: Self::Signed) -> Self {
                Self::saturating_add_signed(self, rhs)
            }

            fn saturating_sub_signed(self, rhs: Self::Signed) -> Self {
                Self::saturating_sub_signed(self, rhs)
            }

            fn wrapping_add_signed(self, rhs: Self::Signed) -> Self {
                Self::wrapping_add_signed(self, rhs)
            }

            fn wrapping_sub_signed(self, rhs: Self::Signed) -> Self {
                Self::wrapping_sub_signed(self, rhs)
            }

            fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
                Self::carrying_add(self, rhs, carry)
            }

            fn overflowing_add_signed(self, rhs: Self::Signed) -> (Self, bool) {
                Self::overflowing_add_signed(self, rhs)
            }

            fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
                Self::borrowing_sub(self, rhs, borrow)
            }

            fn overflowing_sub_signed(self, rhs: Self::Signed) -> (Self, bool) {
                Self::overflowing_sub_signed(self, rhs)
            }

            fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
                Self::carrying_mul(self, rhs, carry)
            }

            fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Self, Self) {
                Self::carrying_mul_add(self, rhs, carry, add)
            }

            fn div_ceil(self, rhs: Self) -> Self {
                Self::div_ceil(self, rhs)
            }

            fn next_multiple_of(self, rhs: Self) -> Self {
                Self::next_multiple_of(self, rhs)
            }

            fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
                Self::checked_next_multiple_of(self, rhs)
            }

            fn is_multiple_of(self, rhs: Self) -> bool {
                Self::is_multiple_of(self, rhs)
            }

            fn is_power_of_two(self) -> bool {
                Self::is_power_of_two(self)
            }

            fn next_power_of_two(self) -> Self {
                Self::next_power_of_two(self)
            }

            fn checked_next_power_of_two(self) -> Option<Self> {
                Self::checked_next_power_of_two(self)
            }

            // @END@ IMPL UNSIGNED
        }
    };
}

impl_unsigned!(u8, i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);
impl_unsigned!(u128, i128);
impl_unsigned!(usize, isize);

pub trait Signed: Integer<Signed = Self> + Neg + From<i8> {
    // @START@ DECL SIGNED
    // Generated by generate_delegates.py

    /// See [`i32::cast_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_unsigned(self) -> Self::Unsigned;

    /// See [`i32::checked_add_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add_unsigned(self, rhs: Self::Unsigned) -> Option<Self>;

    /// See [`i32::strict_add_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_add_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// See [`i32::checked_sub_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_sub_unsigned(self, rhs: Self::Unsigned) -> Option<Self>;

    /// See [`i32::strict_sub_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_sub_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// See [`i32::checked_abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_abs(self) -> Option<Self>;

    /// See [`i32::strict_abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_abs(self) -> Self;

    /// See [`i32::checked_isqrt`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_isqrt(self) -> Option<Self>;

    /// See [`i32::saturating_add_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// See [`i32::saturating_sub_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// See [`i32::saturating_neg`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_neg(self) -> Self;

    /// See [`i32::saturating_abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_abs(self) -> Self;

    /// See [`i32::wrapping_add_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_add_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// See [`i32::wrapping_sub_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_sub_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// See [`i32::wrapping_abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_abs(self) -> Self;

    /// See [`i32::unsigned_abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn unsigned_abs(self) -> Self::Unsigned;

    /// See [`i32::overflowing_add_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_add_unsigned(self, rhs: Self::Unsigned) -> (Self, bool);

    /// See [`i32::overflowing_sub_unsigned`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_sub_unsigned(self, rhs: Self::Unsigned) -> (Self, bool);

    /// See [`i32::overflowing_abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_abs(self) -> (Self, bool);

    /// See [`i32::abs`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs(self) -> Self;

    /// See [`i32::signum`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn signum(self) -> Self;

    /// See [`i32::is_positive`].
    fn is_positive(self) -> bool;

    /// See [`i32::is_negative`].
    fn is_negative(self) -> bool;

    // @END@ DECL SIGNED
}

macro_rules! impl_signed {
    ($ty:ty, $unsigned:ty) => {
        impl_integer!($ty, $unsigned, Self, abs: Self::abs, signum: Self::signum);
        impl Signed for $ty {
            // @START@ IMPL SIGNED
            // Generated by generate_delegates.py

            fn cast_unsigned(self) -> Self::Unsigned {
                Self::cast_unsigned(self)
            }

            fn checked_add_unsigned(self, rhs: Self::Unsigned) -> Option<Self> {
                Self::checked_add_unsigned(self, rhs)
            }

            fn strict_add_unsigned(self, rhs: Self::Unsigned) -> Self {
                Self::strict_add_unsigned(self, rhs)
            }

            fn checked_sub_unsigned(self, rhs: Self::Unsigned) -> Option<Self> {
                Self::checked_sub_unsigned(self, rhs)
            }

            fn strict_sub_unsigned(self, rhs: Self::Unsigned) -> Self {
                Self::strict_sub_unsigned(self, rhs)
            }

            fn checked_abs(self) -> Option<Self> {
                Self::checked_abs(self)
            }

            fn strict_abs(self) -> Self {
                Self::strict_abs(self)
            }

            fn checked_isqrt(self) -> Option<Self> {
                Self::checked_isqrt(self)
            }

            fn saturating_add_unsigned(self, rhs: Self::Unsigned) -> Self {
                Self::saturating_add_unsigned(self, rhs)
            }

            fn saturating_sub_unsigned(self, rhs: Self::Unsigned) -> Self {
                Self::saturating_sub_unsigned(self, rhs)
            }

            fn saturating_neg(self) -> Self {
                Self::saturating_neg(self)
            }

            fn saturating_abs(self) -> Self {
                Self::saturating_abs(self)
            }

            fn wrapping_add_unsigned(self, rhs: Self::Unsigned) -> Self {
                Self::wrapping_add_unsigned(self, rhs)
            }

            fn wrapping_sub_unsigned(self, rhs: Self::Unsigned) -> Self {
                Self::wrapping_sub_unsigned(self, rhs)
            }

            fn wrapping_abs(self) -> Self {
                Self::wrapping_abs(self)
            }

            fn unsigned_abs(self) -> Self::Unsigned {
                Self::unsigned_abs(self)
            }

            fn overflowing_add_unsigned(self, rhs: Self::Unsigned) -> (Self, bool) {
                Self::overflowing_add_unsigned(self, rhs)
            }

            fn overflowing_sub_unsigned(self, rhs: Self::Unsigned) -> (Self, bool) {
                Self::overflowing_sub_unsigned(self, rhs)
            }

            fn overflowing_abs(self) -> (Self, bool) {
                Self::overflowing_abs(self)
            }

            fn abs(self) -> Self {
                Self::abs(self)
            }

            fn signum(self) -> Self {
                Self::signum(self)
            }

            fn is_positive(self) -> bool {
                Self::is_positive(self)
            }

            fn is_negative(self) -> bool {
                Self::is_negative(self)
            }

            // @END@ IMPL SIGNED
        }
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

    #[cfg(feature = "std")]
    #[test]
    fn test_float_floor() {
        assert_eq!(<f64 as Float>::floor(1.5), 1.0);
    }

    #[test]
    fn test_euclid_core() {
        fn test_int<T: Integer>(a: T, b: T) -> (T, T) {
            (a.div_euclid(b), a.rem_euclid(b))
        }

        assert_eq!(test_int(-7, 4), (-2, 1));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_euclid_std() {
        fn test_num<T: Number>(a: T, b: T) -> (T, T) {
            (a.div_euclid(b), a.rem_euclid(b))
        }

        assert_eq!(test_num(-7, 4), (-2, 1));
        assert_eq!(test_num(-7.0, 4.0), (-2.0, 1.0));
    }

    #[test]
    fn test_abs() {
        fn test_abs<T: Number>(a: T) -> T {
            a.abs()
        }

        assert_eq!(test_abs(1i32), 1);
        assert_eq!(test_abs(1u32), 1);
        assert_eq!(test_abs(1.0), 1.0);

        assert_eq!(test_abs(-1i32), 1);
        assert_eq!(test_abs(-1.0), 1.0);

        assert!(test_abs(f64::NAN).is_nan());
    }

    #[test]
    fn test_signum() {
        fn test_signum<T: Number>(a: T) -> T {
            a.signum()
        }

        assert_eq!(test_signum(123i32), 1);
        assert_eq!(test_signum(123u32), 1);
        assert_eq!(test_signum(123.0), 1.0);

        assert_eq!(test_signum(0i32), 0);
        assert_eq!(test_signum(0u32), 0);
        assert_eq!(test_signum(0.0), 1.0);
        assert_eq!(test_signum(-0.0), -1.0);

        assert_eq!(test_signum(-123i32), -1);
        assert_eq!(test_signum(-123.0), -1.0);

        assert!(test_signum(f64::NAN).is_nan());
    }

    #[test]
    fn test_int_conversions() {
        fn inner<T: Integer>(v: T) {
            let bytes = v.to_bytes();
            let v2: T = T::from_bytes(bytes);
            assert_eq!(v2, v);

            let signed = v.to_signed();
            let v2 = T::from_signed(signed);
            assert_eq!(v2, v);

            let unsigned = v.to_unsigned();
            let v2 = T::from_unsigned(unsigned);
            assert_eq!(v2, v);
        }

        inner(123u64);
        inner(-123i64);
    }
}
