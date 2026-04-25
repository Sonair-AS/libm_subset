use core::{cmp, ops};

/// Minimal integer implementations needed on all integer types, including wide integers.
#[allow(dead_code)] // Some trait items are only used in tests or by specific float configurations
pub trait MinInt:
    Copy
    // + fmt::Debug
    + ops::BitOr<Output = Self>
    + ops::Not<Output = Self>
    + ops::Shl<u32, Output = Self>
{
    /// Type with the same width but other signedness
    type OtherSign: MinInt;
    /// Unsigned version of Self
    type Unsigned: MinInt;

    /// If `Self` is a signed integer
    const SIGNED: bool;

    /// The bitwidth of the int type
    const BITS: u32;

    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

/// Access the associated `OtherSign` type from an int (helper to avoid ambiguous associated
/// types).
pub type OtherSign<I> = <I as MinInt>::OtherSign;

/// Trait for some basic operations on integers
#[allow(dead_code)] // Some trait items are only used in tests or by specific float configurations
pub trait Int:
    MinInt
    // + fmt::Display
    // + fmt::Binary
    // + fmt::LowerHex
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
    + ops::DivAssign
    + ops::RemAssign
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign<i32>
    + ops::ShlAssign<u32>
    + ops::ShrAssign<u32>
    + ops::ShrAssign<i32>
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::Shl<i32, Output = Self>
    + ops::Shl<u32, Output = Self>
    + ops::Shr<i32, Output = Self>
    + ops::Shr<u32, Output = Self>
    + ops::BitXor<Output = Self>
    + ops::BitAnd<Output = Self>
    + cmp::Ord
    + From<bool>
    + CastFrom<i32>
    + CastFrom<u16>
    + CastFrom<u32>
    + CastFrom<u8>
    + CastFrom<usize>
    + CastInto<i32>
    + CastInto<u16>
    + CastInto<u32>
    + CastInto<u8>
    + CastInto<usize>
{
    fn signed(self) -> OtherSign<Self::Unsigned>;
    fn unsigned(self) -> Self::Unsigned;
    fn from_unsigned(unsigned: Self::Unsigned) -> Self;
    fn abs(self) -> Self;
    fn unsigned_abs(self) -> Self::Unsigned;

    fn from_bool(b: bool) -> Self;

    /// Prevents the need for excessive conversions between signed and unsigned
    fn logical_shr(self, other: u32) -> Self;

    /// Absolute difference between two integers.
    fn abs_diff(self, other: Self) -> Self::Unsigned;

    // copied from primitive integers, but put in a trait
    fn is_zero(self) -> bool;
    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;
    fn wrapping_neg(self) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_mul(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn wrapping_shl(self, other: u32) -> Self;
    fn wrapping_shr(self, other: u32) -> Self;
    fn rotate_left(self, other: u32) -> Self;
    fn overflowing_add(self, other: Self) -> (Self, bool);
    fn overflowing_sub(self, other: Self) -> (Self, bool);
    fn carrying_add(self, other: Self, carry: bool) -> (Self, bool);
    fn borrowing_sub(self, other: Self, borrow: bool) -> (Self, bool);
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn ilog2(self) -> u32;
}

macro_rules! int_impl_common {
    ($ty:ty) => {
        fn from_bool(b: bool) -> Self {
            b as $ty
        }

        fn logical_shr(self, other: u32) -> Self {
            Self::from_unsigned(self.unsigned().wrapping_shr(other))
        }

        fn is_zero(self) -> bool {
            self == Self::ZERO
        }

        fn checked_add(self, other: Self) -> Option<Self> {
            self.checked_add(other)
        }

        fn checked_sub(self, other: Self) -> Option<Self> {
            self.checked_sub(other)
        }

        fn wrapping_neg(self) -> Self {
            <Self>::wrapping_neg(self)
        }

        fn wrapping_add(self, other: Self) -> Self {
            <Self>::wrapping_add(self, other)
        }

        fn wrapping_mul(self, other: Self) -> Self {
            <Self>::wrapping_mul(self, other)
        }

        fn wrapping_sub(self, other: Self) -> Self {
            <Self>::wrapping_sub(self, other)
        }

        fn wrapping_shl(self, other: u32) -> Self {
            <Self>::wrapping_shl(self, other)
        }

        fn wrapping_shr(self, other: u32) -> Self {
            <Self>::wrapping_shr(self, other)
        }

        fn rotate_left(self, other: u32) -> Self {
            <Self>::rotate_left(self, other)
        }

        fn overflowing_add(self, other: Self) -> (Self, bool) {
            <Self>::overflowing_add(self, other)
        }

        fn overflowing_sub(self, other: Self) -> (Self, bool) {
            <Self>::overflowing_sub(self, other)
        }

        fn leading_zeros(self) -> u32 {
            <Self>::leading_zeros(self)
        }

        fn trailing_zeros(self) -> u32 {
            <Self>::trailing_zeros(self)
        }

        fn ilog2(self) -> u32 {
            // On older MSRV, this resolves to the trait method which won't work,
            // but this is only called behind gates that ensure a new enough version.
            #[allow(clippy::incompatible_msrv)] // Guarded by cfg gates at call sites
            <Self>::ilog2(self)
        }

        fn carrying_add(self, other: Self, carry: bool) -> (Self, bool) {
            let (ab, of1) = self.overflowing_add(other);
            let (abc, of2) = ab.overflowing_add(Self::from_bool(carry));
            // `of1 && of2` is possible with signed integers if a negative sum
            // overflows to `MAX` and adding the carry overflows again back to `MIN`
            (abc, of1 ^ of2)
        }

        fn borrowing_sub(self, other: Self, borrow: bool) -> (Self, bool) {
            let (ab, of1) = self.overflowing_sub(other);
            let (abc, of2) = ab.overflowing_sub(Self::from_bool(borrow));
            (abc, of1 ^ of2)
        }
    };
}

macro_rules! int_impl {
    ($ity:ty, $uty:ty) => {
        impl MinInt for $uty {
            type OtherSign = $ity;
            type Unsigned = $uty;

            const BITS: u32 = <Self as MinInt>::ZERO.count_zeros();
            const SIGNED: bool = Self::MIN != Self::ZERO;

            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <Self>::MIN;
            const MAX: Self = <Self>::MAX;
        }

        impl Int for $uty {
            fn signed(self) -> $ity {
                self as $ity
            }

            fn unsigned(self) -> Self {
                self
            }

            fn abs(self) -> Self {
                panic!("abs is not implemented");
            }

            fn unsigned_abs(self) -> Self {
                panic!("unsigned_abs is not implemented");
            }

            // It makes writing macros easier if this is implemented for both signed and unsigned
            #[allow(clippy::wrong_self_convention)] // Intentional: constructor-style name for macro uniformity
            fn from_unsigned(me: $uty) -> Self {
                me
            }

            fn abs_diff(self, other: Self) -> Self {
                self.abs_diff(other)
            }

            int_impl_common!($uty);
        }

        impl MinInt for $ity {
            type OtherSign = $uty;
            type Unsigned = $uty;

            const BITS: u32 = <Self as MinInt>::ZERO.count_zeros();
            const SIGNED: bool = Self::MIN != Self::ZERO;

            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <Self>::MIN;
            const MAX: Self = <Self>::MAX;
        }

        impl Int for $ity {
            fn signed(self) -> Self {
                self
            }

            fn unsigned(self) -> $uty {
                self as $uty
            }

            fn abs(self) -> Self {
                self.abs()
            }

            fn unsigned_abs(self) -> Self::Unsigned {
                self.unsigned_abs()
            }

            fn from_unsigned(me: $uty) -> Self {
                me as $ity
            }

            fn abs_diff(self, other: Self) -> $uty {
                self.abs_diff(other)
            }

            int_impl_common!($ity);
        }
    };
}

int_impl!(i32, u32);
int_impl!(i64, u64);

#[cfg(any(not(feature = "sonair_certified"), test))]
int_impl!(i128, u128);

#[cfg(any(not(feature = "sonair_certified"), test))]
int_impl!(isize, usize);
#[cfg(any(not(feature = "sonair_certified"), test))]
int_impl!(i8, u8);
#[cfg(any(not(feature = "sonair_certified"), test))]
int_impl!(i16, u16);

/// Trait for integers twice the bit width of another integer. This is implemented for all
/// primitives except for `u8`, because there is not a smaller primitive.
#[allow(dead_code)] // Some trait items are only used by specific float configurations (e.g. fma)
pub trait DInt: MinInt {
    /// Integer that is half the bit width of the integer this trait is implemented for
    type H: HInt<D = Self>;

    /// Returns the low half of `self`
    fn lo(self) -> Self::H;
    /// Returns the high half of `self`
    fn hi(self) -> Self::H;
    /// Returns the low and high halves of `self` as a tuple
    fn lo_hi(self) -> (Self::H, Self::H) {
        (self.lo(), self.hi())
    }
    /// Constructs an integer using lower and higher half parts
    #[allow(unused)] // Part of the DInt API; used by fma in non-certified builds
    fn from_lo_hi(lo: Self::H, hi: Self::H) -> Self {
        lo.zero_widen() | hi.widen_hi()
    }
}

/// Trait for integers half the bit width of another integer. This is implemented for all
/// primitives except for `u128`, because it there is not a larger primitive.
pub trait HInt: Int {
    /// Integer that is double the bit width of the integer this trait is implemented for
    type D: DInt<H = Self> + MinInt;

    // NB: some of the below methods could have default implementations (e.g. `widen_hi`), but for
    // unknown reasons this can cause infinite recursion when optimizations are disabled. See
    // <https://github.com/rust-lang/compiler-builtins/pull/707> for context.

    /// Widens (using default extension) the integer to have double bit width
    fn widen(self) -> Self::D;
    /// Widens (zero extension only) the integer to have double bit width. This is needed to get
    /// around problems with associated type bounds (such as `Int<Othersign: DInt>`) being unstable
    fn zero_widen(self) -> Self::D;
    /// Widens the integer to have double bit width and shifts the integer into the higher bits
    #[allow(unused)] // Part of the HInt API; used by fma in non-certified builds
    fn widen_hi(self) -> Self::D;
    /// Widening multiplication with zero widening. This cannot overflow.
    #[allow(dead_code)] // Part of the HInt API; used by big integer types in non-certified builds
    fn zero_widen_mul(self, rhs: Self) -> Self::D;
    /// Widening multiplication. This cannot overflow.
    fn widen_mul(self, rhs: Self) -> Self::D;
}

macro_rules! impl_d_int {
    ($($X:ident $D:ident),*) => {
        $(
            impl DInt for $D {
                type H = $X;

                fn lo(self) -> Self::H {
                    self as $X
                }
                fn hi(self) -> Self::H {
                    (self >> <$X as MinInt>::BITS) as $X
                }
            }
        )*
    };
}

macro_rules! impl_h_int {
    ($($H:ident $uH:ident $X:ident),*) => {
        $(
            impl HInt for $H {
                type D = $X;

                fn widen(self) -> Self::D {
                    self as $X
                }
                fn zero_widen(self) -> Self::D {
                    (self as $uH) as $X
                }
                fn zero_widen_mul(self, rhs: Self) -> Self::D {
                    self.zero_widen().wrapping_mul(rhs.zero_widen())
                }
                fn widen_mul(self, rhs: Self) -> Self::D {
                    self.widen().wrapping_mul(rhs.widen())
                }
                fn widen_hi(self) -> Self::D {
                    (self as $X) << <Self as MinInt>::BITS
                }
            }
        )*
    };
}

impl_d_int!(u32 u64, i32 i64);
impl_h_int!(
    u32 u32 u64,
    i32 u32 i64
);

#[cfg(any(not(feature = "sonair_certified"), test))]
impl_d_int!(u64 u128, i64 i128);
#[cfg(any(not(feature = "sonair_certified"), test))]
impl_h_int!(
    u64 u64 u128,
    i64 u64 i128
);

#[cfg(any(not(feature = "sonair_certified"), test))]
impl_d_int!(u8 u16, u16 u32, i8 i16, i16 i32);
#[cfg(any(not(feature = "sonair_certified"), test))]
impl_h_int!(
    u8 u8 u16,
    u16 u16 u32,
    i8 u8 i16,
    i16 u16 i32
);

/// Trait to express (possibly lossy) casting of integers
pub trait CastInto<T: Copy>: Copy {
    /// By default, casts should be exact.
    #[track_caller]
    fn cast(self) -> T;

    /// Call for casts that are expected to truncate.
    ///
    /// In practice, this is exactly the same as `cast`; the main difference is to document intent
    /// in code. `cast` may panic in debug mode.
    #[allow(dead_code)] // Part of the upstream CastInto API; not all cast paths are used
    fn cast_lossy(self) -> T;
}

#[allow(dead_code)] // Blanket-implemented from CastInto; cast_from_lossy not used in all configurations
pub trait CastFrom<T: Copy>: Copy {
    /// By default, casts should be exact.
    #[track_caller]
    fn cast_from(value: T) -> Self;

    /// Call for casts that are expected to truncate.
    fn cast_from_lossy(value: T) -> Self;
}

impl<T: Copy, U: CastInto<T> + Copy> CastFrom<U> for T {
    fn cast_from(value: U) -> Self {
        value.cast()
    }

    fn cast_from_lossy(value: U) -> Self {
        value.cast_lossy()
    }
}

macro_rules! cast_into {
    ($ty:ty) => {
        cast_into!($ty; usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
    };
    ($ty:ty; $($into:ty),*) => {$(
        impl CastInto<$into> for $ty {
            fn cast(self) -> $into {
                // All we can really do to enforce casting rules is check the rules when in
                // debug mode.
                #[cfg(not(feature = "compiler-builtins"))]
                #[cfg(any(not(feature = "sonair_certified"), test))]
                debug_assert!(<$into>::try_from(self).is_ok(), "failed cast from {self}");
                self as $into
            }

            fn cast_lossy(self) -> $into {
                self as $into
            }
        }
    )*};
}

macro_rules! cast_into_float {
    ($ty:ty) => {
        cast_into_float!($ty; f32, f64);
    };
    ($ty:ty; $($into:ty),*) => {$(
        impl CastInto<$into> for $ty {
            fn cast(self) -> $into {
                #[cfg(not(feature = "compiler-builtins"))]
                #[cfg(any(not(feature = "sonair_certified"), test))]
                debug_assert_eq!(self as $into as $ty, self, "inexact float cast");
                self as $into
            }

            fn cast_lossy(self) -> $into {
                self as $into
            }
        }
    )*};
}

cfg_if! {
    if #[cfg(all(feature = "sonair_certified", not(test)))] {
        // Only the source/target combinations actually used by the math routines.
        // The `Int` trait requires CastFrom/CastInto for {u8, u16, u32, i32, usize},
        // and `Float` additionally needs CastInto for u64/i64.
        cast_into!(usize; u8, u16, usize, u32, i32, u64, i64);
        cast_into!(u8;    u8, u16, usize, u32, i32, u64, i64);
        cast_into!(u16;   u8, u16, usize, u32, i32, u64, i64);
        cast_into!(u32;   u8, u16, usize, u32, i32, u64, i64);
        cast_into!(i32;   u8, u16, usize, u32, i32, u64, i64);
        cast_into!(u64;   u8, u16, usize, u32, i32, u64, i64);
        cast_into!(i64;   u8, u16, usize, u32, i32, u64, i64);
        cast_into_float!(i32);
        cast_into_float!(i64);
    } else {
        cast_into!(usize);
        cast_into!(isize);
        cast_into!(u8);
        cast_into!(i8);
        cast_into!(u16);
        cast_into!(i16);
        cast_into!(u32);
        cast_into!(i32);
        cast_into!(u64);
        cast_into!(i64);
        cast_into!(u128);
        cast_into!(i128);
        cast_into_float!(i8);
        cast_into_float!(i16);
        cast_into_float!(i32);
        cast_into_float!(i64);
        cast_into_float!(i128);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exercise every `Int` trait method through explicit trait-qualified calls.
    /// This ensures llvm-cov counts the macro-expanded function bodies.
    #[inline(never)]
    fn exercise_int_common<T: Int>(zero: T, one: T, two: T) {
        assert!(Int::is_zero(zero));
        assert!(!Int::is_zero(one));
        assert!(T::from_bool(true) == one);
        assert!(T::from_bool(false) == zero);

        assert!(Int::checked_add(one, one) == Some(two));
        assert!(Int::checked_sub(two, one) == Some(one));

        assert!(Int::wrapping_neg(zero) == zero);
        assert!(Int::wrapping_add(one, one) == two);
        assert!(Int::wrapping_sub(two, one) == one);
        assert!(Int::wrapping_mul(one, two) == two);
        assert!(Int::wrapping_shl(one, 0) == one);
        assert!(Int::wrapping_shr(two, 1) == one);
        assert!(!(Int::rotate_left(one, 1) == zero));

        let (oa_val, oa_of) = Int::overflowing_add(one, one);
        assert!(oa_val == two && !oa_of);
        let (os_val, os_uf) = Int::overflowing_sub(two, one);
        assert!(os_val == one && !os_uf);

        let (ca, ca_of) = Int::carrying_add(one, one, false);
        assert!(ca == two && !ca_of);
        let (bs, bs_uf) = Int::borrowing_sub(two, one, false);
        assert!(bs == one && !bs_uf);

        assert!(Int::leading_zeros(one) == T::BITS - 1);
        assert!(Int::trailing_zeros(one) == 0);
        assert!(Int::ilog2(one) == 0);

        assert!(Int::logical_shr(two, 1) == one);
        core::hint::black_box(Int::abs_diff(two, one));
    }

    #[test]
    fn int_trait_dispatch_u32() {
        exercise_int_common::<u32>(0, 1, 2);
        assert!(Int::checked_add(u32::MAX, 1u32).is_none());
        assert!(Int::checked_sub(0u32, 1u32).is_none());
        assert!(Int::wrapping_add(u32::MAX, 1u32) == 0u32);
        assert!(Int::wrapping_sub(0u32, 1u32) == u32::MAX);
        let (s, o) = Int::overflowing_add(u32::MAX, 1u32);
        assert!(s == 0u32 && o);
        let (d, u) = Int::overflowing_sub(0u32, 1u32);
        assert!(d == u32::MAX && u);
    }

    #[test]
    fn int_trait_dispatch_i32() {
        exercise_int_common::<i32>(0, 1, 2);
        assert!(Int::checked_add(i32::MAX, 1i32).is_none());
        assert!(Int::wrapping_neg(-1i32) == 1i32);
        assert!(Int::wrapping_neg(i32::MIN) == i32::MIN);
    }

    #[test]
    fn int_trait_dispatch_u64() {
        exercise_int_common::<u64>(0, 1, 2);
        assert!(Int::checked_add(u64::MAX, 1u64).is_none());
        assert!(Int::checked_sub(0u64, 1u64).is_none());
        assert!(Int::wrapping_add(u64::MAX, 1u64) == 0u64);
        assert!(Int::wrapping_sub(0u64, 1u64) == u64::MAX);
        let (s, o) = Int::overflowing_add(u64::MAX, 1u64);
        assert!(s == 0u64 && o);
        let (d, u) = Int::overflowing_sub(0u64, 1u64);
        assert!(d == u64::MAX && u);
    }

    #[test]
    fn int_trait_dispatch_i64() {
        exercise_int_common::<i64>(0, 1, 2);
        assert!(Int::checked_add(i64::MAX, 1i64).is_none());
        assert!(Int::wrapping_neg(-1i64) == 1i64);
        assert!(Int::wrapping_neg(i64::MIN) == i64::MIN);
    }

    #[test]
    fn signed_unsigned_conversions_u32() {
        assert_eq!(Int::signed(42u32), 42i32);
        assert_eq!(Int::unsigned(42u32), 42u32);
        assert_eq!(u32::from_unsigned(42u32), 42u32);
    }

    #[test]
    fn signed_unsigned_conversions_i32() {
        assert_eq!(Int::signed(-1i32), -1i32);
        assert_eq!(Int::unsigned(-1i32), u32::MAX);
        assert_eq!(i32::from_unsigned(0u32), 0i32);
    }

    #[test]
    fn signed_unsigned_conversions_u64() {
        assert_eq!(Int::signed(42u64), 42i64);
        assert_eq!(Int::unsigned(42u64), 42u64);
    }

    #[test]
    fn signed_unsigned_conversions_i64() {
        assert_eq!(Int::signed(-1i64), -1i64);
        assert_eq!(Int::unsigned(-1i64), u64::MAX);
    }

    #[test]
    fn i32_abs_ops() {
        assert_eq!(Int::abs(-5i32), 5);
        assert_eq!(Int::unsigned_abs(-5i32), 5u32);
    }

    #[test]
    fn i64_abs_ops() {
        assert_eq!(Int::abs(-5i64), 5);
        assert_eq!(Int::unsigned_abs(-5i64), 5u64);
    }

    #[test]
    #[should_panic]
    fn unsigned_abs_panics() {
        let _ = Int::abs(1u32);
    }

    #[test]
    #[should_panic]
    fn unsigned_unsigned_abs_panics() {
        let _ = Int::unsigned_abs(1u32);
    }

    #[test]
    fn dint_lo_hi() {
        let v: u64 = 0x0000_0002_0000_0001;
        assert_eq!(DInt::lo(v), 1u32);
        assert_eq!(DInt::hi(v), 2u32);
        let (lo, hi) = DInt::lo_hi(v);
        assert_eq!(lo, 1u32);
        assert_eq!(hi, 2u32);
    }

    #[test]
    fn dint_from_lo_hi() {
        let v = u64::from_lo_hi(0xAAAA_BBBBu32, 0xCCCC_DDDDu32);
        assert_eq!(v, 0xCCCC_DDDD_AAAA_BBBBu64);
    }

    #[test]
    fn dint_i64() {
        let v: i64 = 0x0000_0002_0000_0001;
        assert_eq!(DInt::lo(v), 1i32);
        assert_eq!(DInt::hi(v), 2i32);
    }

    #[inline(never)]
    fn exercise_hint<H: HInt>(a: H, b: H, zero: H)
    where
        H::D: PartialEq,
    {
        assert!(!(HInt::widen(a) == HInt::widen(zero)));
        assert!(!(HInt::zero_widen(a) == HInt::zero_widen(zero)));
        assert!(!(HInt::widen_hi(a) == HInt::widen_hi(zero)));
        assert!(!(HInt::widen_mul(a, b) == HInt::widen_mul(zero, b)));
        assert!(HInt::zero_widen_mul(zero, b) == HInt::zero_widen_mul(b, zero));
    }

    #[test]
    fn hint_u32() {
        exercise_hint::<u32>(3, 7, 0);
        assert_eq!(HInt::widen(0xFFFF_FFFFu32), 0x0000_0000_FFFF_FFFFu64);
        assert_eq!(HInt::zero_widen(0xFFFF_FFFFu32), 0x0000_0000_FFFF_FFFFu64);
        assert_eq!(HInt::widen_hi(1u32), 0x0000_0001_0000_0000u64);
    }

    #[test]
    fn hint_i32() {
        exercise_hint::<i32>(3, 7, 0);
        assert_eq!(HInt::widen(-1i32), -1i64);
        assert_eq!(HInt::zero_widen(-1i32), 0x0000_0000_FFFF_FFFFi64);
    }

    #[test]
    fn cast_into_u32() {
        let v: u32 = 42;
        assert_eq!(<u32 as CastInto<u32>>::cast(v), 42u32);
        assert_eq!(<u32 as CastInto<i32>>::cast(v), 42i32);
        assert_eq!(<u32 as CastInto<u64>>::cast(v), 42u64);
        assert_eq!(<u32 as CastInto<i64>>::cast(v), 42i64);
        assert_eq!(<u32 as CastInto<u8>>::cast(v), 42u8);
        assert_eq!(<u32 as CastInto<u16>>::cast(v), 42u16);
        assert_eq!(<u32 as CastInto<usize>>::cast(v), 42usize);
    }

    #[test]
    fn cast_into_i32() {
        let v: i32 = 42;
        assert_eq!(<i32 as CastInto<u32>>::cast(v), 42u32);
        assert_eq!(<i32 as CastInto<i32>>::cast(v), 42i32);
        assert_eq!(<i32 as CastInto<u64>>::cast(v), 42u64);
        assert_eq!(<i32 as CastInto<i64>>::cast(v), 42i64);
        assert_eq!(<i32 as CastInto<u8>>::cast(v), 42u8);
        assert_eq!(<i32 as CastInto<u16>>::cast(v), 42u16);
        assert_eq!(<i32 as CastInto<usize>>::cast(v), 42usize);
    }

    #[test]
    fn cast_into_u64() {
        let v: u64 = 42;
        assert_eq!(<u64 as CastInto<u32>>::cast(v), 42u32);
        assert_eq!(<u64 as CastInto<i32>>::cast(v), 42i32);
        assert_eq!(<u64 as CastInto<u64>>::cast(v), 42u64);
        assert_eq!(<u64 as CastInto<i64>>::cast(v), 42i64);
        assert_eq!(<u64 as CastInto<u8>>::cast(v), 42u8);
        assert_eq!(<u64 as CastInto<u16>>::cast(v), 42u16);
        assert_eq!(<u64 as CastInto<usize>>::cast(v), 42usize);
    }

    #[test]
    fn cast_into_i64() {
        let v: i64 = 42;
        assert_eq!(<i64 as CastInto<u32>>::cast(v), 42u32);
        assert_eq!(<i64 as CastInto<i32>>::cast(v), 42i32);
        assert_eq!(<i64 as CastInto<u64>>::cast(v), 42u64);
        assert_eq!(<i64 as CastInto<i64>>::cast(v), 42i64);
        assert_eq!(<i64 as CastInto<u8>>::cast(v), 42u8);
        assert_eq!(<i64 as CastInto<u16>>::cast(v), 42u16);
        assert_eq!(<i64 as CastInto<usize>>::cast(v), 42usize);
    }

    #[test]
    fn cast_into_usize() {
        let v: usize = 42;
        assert_eq!(<usize as CastInto<u32>>::cast(v), 42u32);
        assert_eq!(<usize as CastInto<i32>>::cast(v), 42i32);
        assert_eq!(<usize as CastInto<u64>>::cast(v), 42u64);
        assert_eq!(<usize as CastInto<i64>>::cast(v), 42i64);
        assert_eq!(<usize as CastInto<u8>>::cast(v), 42u8);
        assert_eq!(<usize as CastInto<u16>>::cast(v), 42u16);
        assert_eq!(<usize as CastInto<usize>>::cast(v), 42usize);
    }

    #[test]
    fn cast_into_float_i32() {
        let v: i32 = 42;
        assert_eq!(<i32 as CastInto<f32>>::cast(v), 42.0f32);
        assert_eq!(<i32 as CastInto<f64>>::cast(v), 42.0f64);
    }

    #[test]
    fn cast_into_float_i64() {
        let v: i64 = 42;
        assert_eq!(<i64 as CastInto<f32>>::cast(v), 42.0f32);
        assert_eq!(<i64 as CastInto<f64>>::cast(v), 42.0f64);
    }

    #[test]
    fn cast_lossy_int() {
        let v: u64 = 0x1_0000_0000;
        let w: u32 = <u64 as CastInto<u32>>::cast_lossy(v);
        assert_eq!(w, 0u32);
        let x: u32 = <u32 as CastFrom<u64>>::cast_from_lossy(v);
        assert_eq!(x, 0u32);
    }

    #[test]
    fn cast_lossy_float() {
        let large: i64 = (1i64 << 55) + 1;
        let f: f32 = <i64 as CastInto<f32>>::cast_lossy(large);
        assert!(f > 0.0);
        let d: f64 = <i64 as CastInto<f64>>::cast_lossy(large);
        assert!(d > 0.0);
        let f2: f32 = <f32 as CastFrom<i64>>::cast_from_lossy(large);
        assert!(f2 > 0.0);
        let d2: f64 = <f64 as CastFrom<i64>>::cast_from_lossy(large);
        assert!(d2 > 0.0);
    }
}
