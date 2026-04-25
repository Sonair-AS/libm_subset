#![allow(unknown_lints)] // FIXME(msrv) we shouldn't need this

use core::{mem, ops};

use super::int_traits::{CastFrom, Int, MinInt};

/// Trait for some basic operations on floats
#[allow(dead_code)] // Some trait items and constants are only used in tests or by specific float configurations
pub trait Float:
    Copy
    // + fmt::Debug
    + PartialEq
    + PartialOrd
    + ops::AddAssign
    + ops::MulAssign
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::Neg<Output = Self>
    + 'static
{
    /// A uint of the same width as the float
    type Int: Int<OtherSign = Self::SignedInt, Unsigned = Self::Int>;

    /// A int of the same width as the float
    type SignedInt: Int
        + MinInt<OtherSign = Self::Int, Unsigned = Self::Int>
        + ops::Neg<Output = Self::SignedInt>;

    const ZERO: Self;
    const NEG_ZERO: Self;
    const ONE: Self;
    const NEG_ONE: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;
    const NEG_NAN: Self;
    const MAX: Self;
    const MIN: Self;
    const EPSILON: Self;
    const PI: Self;
    const NEG_PI: Self;
    const FRAC_PI_2: Self;

    const MIN_POSITIVE_NORMAL: Self;

    /// The bitwidth of the float type
    const BITS: u32;

    /// The bitwidth of the significand
    const SIG_BITS: u32;

    /// The bitwidth of the exponent
    const EXP_BITS: u32 = Self::BITS - Self::SIG_BITS - 1;

    /// The saturated (maximum bitpattern) value of the exponent, i.e. the infinite
    /// representation.
    ///
    /// This shifted fully right, use `EXP_MASK` for the shifted value.
    const EXP_SAT: u32 = (1 << Self::EXP_BITS) - 1;

    /// The exponent bias value
    const EXP_BIAS: u32 = Self::EXP_SAT >> 1;

    /// Maximum unbiased exponent value.
    const EXP_MAX: i32 = Self::EXP_BIAS as i32;

    /// Minimum *NORMAL* unbiased exponent value.
    const EXP_MIN: i32 = -(Self::EXP_MAX - 1);

    /// Minimum subnormal exponent value.
    const EXP_MIN_SUBNORM: i32 = Self::EXP_MIN - Self::SIG_BITS as i32;

    /// A mask for the sign bit
    const SIGN_MASK: Self::Int;

    /// A mask for the significand
    const SIG_MASK: Self::Int;

    /// A mask for the exponent
    const EXP_MASK: Self::Int;

    /// The implicit bit of the float format
    const IMPLICIT_BIT: Self::Int;

    /// Returns `self` transmuted to `Self::Int`
    fn to_bits(self) -> Self::Int;

    /// Returns `self` transmuted to `Self::SignedInt`
    #[allow(dead_code)] // Part of the Float API; used by fma in non-certified builds
    fn to_bits_signed(self) -> Self::SignedInt {
        self.to_bits().signed()
    }

    /// Check bitwise equality.
    #[allow(dead_code)] // Used in tests via assert_biteq! macro
    fn biteq(self, rhs: Self) -> bool {
        self.to_bits() == rhs.to_bits()
    }

    /// Checks if two floats have the same bit representation. *Except* for NaNs! NaN can be
    /// represented in multiple different ways.
    ///
    /// This method returns `true` if two NaNs are compared. Use [`biteq`](Self::biteq) instead
    /// if `NaN` should not be treated separately.
    #[allow(dead_code)] // Used in tests for NaN-tolerant float comparison
    fn eq_repr(self, rhs: Self) -> bool {
        if self.is_nan() && rhs.is_nan() {
            true
        } else {
            self.biteq(rhs)
        }
    }

    /// Returns true if the value is NaN.
    fn is_nan(self) -> bool;

    /// Returns true if the value is +inf or -inf.
    fn is_infinite(self) -> bool;

    /// Returns true if the sign is negative. Extracts the sign bit regardless of zero or NaN.
    fn is_sign_negative(self) -> bool;

    /// Returns true if the sign is positive. Extracts the sign bit regardless of zero or NaN.
    fn is_sign_positive(self) -> bool {
        !self.is_sign_negative()
    }

    /// Returns if `self` is subnormal.
    #[allow(dead_code)] // Used in tests for subnormal detection
    fn is_subnormal(self) -> bool {
        (self.to_bits() & Self::EXP_MASK) == Self::Int::ZERO
    }

    /// Returns the exponent, not adjusting for bias, not accounting for subnormals or zero.
    fn ex(self) -> u32 {
        u32::cast_from(self.to_bits() >> Self::SIG_BITS) & Self::EXP_SAT
    }

    /// Extract the exponent and adjust it for bias, not accounting for subnormals or zero.
    fn exp_unbiased(self) -> i32 {
        self.ex().signed() - (Self::EXP_BIAS as i32)
    }

    /// Returns the significand with no implicit bit (or the "fractional" part)
    #[allow(dead_code)] // Part of the Float API; used in fma and test helpers
    fn frac(self) -> Self::Int {
        self.to_bits() & Self::SIG_MASK
    }

    /// Returns a `Self::Int` transmuted back to `Self`
    fn from_bits(a: Self::Int) -> Self;

    /// Constructs a `Self` from its parts. Inputs are treated as bits and shifted into position.
    fn from_parts(negative: bool, exponent: u32, significand: Self::Int) -> Self {
        let sign = if negative {
            Self::Int::ONE
        } else {
            Self::Int::ZERO
        };
        Self::from_bits(
            (sign << (Self::BITS - 1))
                | (Self::Int::cast_from(exponent & Self::EXP_SAT) << Self::SIG_BITS)
                | (significand & Self::SIG_MASK),
        )
    }

    #[allow(dead_code)] // Part of the Float API; used in non-certified builds
    fn abs(self) -> Self;

    /// Returns a number composed of the magnitude of self and the sign of sign.
    fn copysign(self, other: Self) -> Self;

    /// Fused multiply add, rounding once.
    #[cfg(not(feature = "sonair_certified"))]
    fn fma(self, y: Self, z: Self) -> Self;

    /// Returns (normalized exponent, normalized significand)
    #[allow(dead_code)] // Part of the Float API; used by fma in non-certified builds
    #[cfg(not(feature = "sonair_certified"))]
    fn normalize(significand: Self::Int) -> (i32, Self::Int);

    /// Returns a number that represents the sign of self.
    #[allow(dead_code)] // Part of the Float API; available for downstream use
    fn signum(self) -> Self {
        if self.is_nan() {
            self
        } else {
            Self::ONE.copysign(self)
        }
    }

    /// Make a best-effort attempt to canonicalize the number. Note that this is allowed
    /// to be a nop and does not always quiet sNaNs.
    fn canonicalize(self) -> Self {
        // FIXME: LLVM often removes this. We should determine whether we can remove the operation,
        // or switch to something based on `llvm.canonicalize` (which has crashes,
        // <https://github.com/llvm/llvm-project/issues/32650>).
        self * Self::ONE
    }
}

/// Access the associated `Int` type from a float (helper to avoid ambiguous associated types).
pub type IntTy<F> = <F as Float>::Int;

macro_rules! float_impl {
    (
        $ty:ident,
        $ity:ident,
        $sity:ident,
        $bits:expr,
        $significand_bits:expr,
        $from_bits:path,
        $to_bits:path,
        $fma_intrinsic:ident,
        $fma_soft:ident
    ) => {
        #[allow(unstable_name_collisions)] // Our Float::BITS may collide with future f32::BITS/f64::BITS in std
        impl Float for $ty {
            type Int = $ity;
            type SignedInt = $sity;

            const ZERO: Self = 0.0;
            const NEG_ZERO: Self = -0.0;
            const ONE: Self = 1.0;
            const NEG_ONE: Self = -1.0;
            const INFINITY: Self = Self::INFINITY;
            const NEG_INFINITY: Self = Self::NEG_INFINITY;
            const NAN: Self = Self::NAN;
            // NAN isn't guaranteed to be positive but it usually is. We only use this for
            // tests.
            const NEG_NAN: Self = $from_bits($to_bits(Self::NAN) | Self::SIGN_MASK);
            const MAX: Self = -Self::MIN;
            // Sign bit set, saturated mantissa, saturated exponent with last bit zeroed
            const MIN: Self = $from_bits(Self::Int::MAX & !(1 << Self::SIG_BITS));
            const EPSILON: Self = <$ty>::EPSILON;

            // Exponent is a 1 in the LSB
            const MIN_POSITIVE_NORMAL: Self = $from_bits(1 << Self::SIG_BITS);

            const PI: Self = core::$ty::consts::PI;
            const NEG_PI: Self = -Self::PI;
            const FRAC_PI_2: Self = core::$ty::consts::FRAC_PI_2;

            const BITS: u32 = $bits;
            const SIG_BITS: u32 = $significand_bits;

            const SIGN_MASK: Self::Int = 1 << (Self::BITS - 1);
            const SIG_MASK: Self::Int = (1 << Self::SIG_BITS) - 1;
            const EXP_MASK: Self::Int = !(Self::SIGN_MASK | Self::SIG_MASK);
            const IMPLICIT_BIT: Self::Int = 1 << Self::SIG_BITS;

            fn to_bits(self) -> Self::Int {
                self.to_bits()
            }
            fn is_nan(self) -> bool {
                self.is_nan()
            }
            fn is_infinite(self) -> bool {
                self.is_infinite()
            }
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }
            fn from_bits(a: Self::Int) -> Self {
                Self::from_bits(a)
            }
            fn abs(self) -> Self {
                cfg_if! {
                    // FIXME(msrv): `abs` is available in `core` starting with 1.85.
                    if #[cfg(intrinsics_enabled)] {
                        self.abs()
                    } else {
                        super::super::generic::fabs(self)
                    }
                }
            }
            fn copysign(self, other: Self) -> Self {
                cfg_if! {
                    // FIXME(msrv): `copysign` is available in `core` starting with 1.85.
                    if #[cfg(intrinsics_enabled)] {
                        self.copysign(other)
                    } else {
                        super::super::generic::copysign(self, other)
                    }
                }
            }
            #[cfg(not(feature = "sonair_certified"))]
            fn fma(self, y: Self, z: Self) -> Self {
                cfg_if! {
                    // fma is not yet available in `core`
                    if #[cfg(intrinsics_enabled)] {
                        core::intrinsics::$fma_intrinsic(self, y, z)
                    } else {
                        super::super::generic::$fma_soft(self, y, z, super::Round::Nearest).val
                    }
                }
            }
            #[cfg(not(feature = "sonair_certified"))]
            fn normalize(significand: Self::Int) -> (i32, Self::Int) {
                let shift = significand.leading_zeros().wrapping_sub(Self::EXP_BITS);
                (
                    1i32.wrapping_sub(shift as i32),
                    significand << shift as Self::Int,
                )
            }
        }
    };
}

float_impl!(
    f32,
    u32,
    i32,
    32,
    23,
    f32_from_bits,
    f32_to_bits,
    fmaf32,
    fma_wide_round
);
float_impl!(
    f64,
    u64,
    i64,
    64,
    52,
    f64_from_bits,
    f64_to_bits,
    fmaf64,
    fma_round
);

/* FIXME(msrv): vendor some things that are not const stable at our MSRV */

/// `f32::from_bits`
#[allow(dead_code)] // Called by float_impl! macro in const exprs; compiler const-evaluates away the call
#[allow(unnecessary_transmutes)] // Transmute is needed for const fn; from_bits() is not const-stable on all supported MSRVs
pub const fn f32_from_bits(bits: u32) -> f32 {
    // SAFETY: u32 and f32 have the same size and alignment; any bit pattern is valid.
    unsafe { mem::transmute::<u32, f32>(bits) }
}

/// `f32::to_bits`
#[allow(dead_code)] // Counterpart to f32_from_bits; kept for API symmetry, used by float_impl! macro
#[allow(unnecessary_transmutes)] // Transmute is needed for const fn; to_bits() is not const-stable on all supported MSRVs
pub const fn f32_to_bits(x: f32) -> u32 {
    // SAFETY: u32 and f32 have the same size and alignment; any bit pattern is valid.
    unsafe { mem::transmute::<f32, u32>(x) }
}

/// `f64::from_bits`
#[allow(dead_code)] // Counterpart to f32_from_bits; kept for API symmetry, used by float_impl! macro
#[allow(unnecessary_transmutes)] // Transmute is needed for const fn; from_bits() is not const-stable on all supported MSRVs
pub const fn f64_from_bits(bits: u64) -> f64 {
    // SAFETY: u64 and f64 have the same size and alignment; any bit pattern is valid.
    unsafe { mem::transmute::<u64, f64>(bits) }
}

/// `f64::to_bits`
#[allow(dead_code)] // Counterpart to f64_from_bits; kept for API symmetry, used by float_impl! macro
#[allow(unnecessary_transmutes)] // Transmute is needed for const fn; to_bits() is not const-stable on all supported MSRVs
pub const fn f64_to_bits(x: f64) -> u64 {
    // SAFETY: u64 and f64 have the same size and alignment; any bit pattern is valid.
    unsafe { mem::transmute::<f64, u64>(x) }
}

/// Trait for floats twice the bit width of another integer.
#[cfg(not(feature = "sonair_certified"))]
#[allow(dead_code)] // Part of the upstream libm API; used by fma_wide in non-certified builds
pub trait DFloat: Float {
    /// Float that is half the bit width of the floatthis trait is implemented for.
    type H: HFloat<D = Self>;

    /// Narrow the float type.
    fn narrow(self) -> Self::H;
}

/// Trait for floats half the bit width of another float.
#[cfg(not(feature = "sonair_certified"))]
#[allow(dead_code)] // Part of the upstream libm API; used by fma_wide in non-certified builds
pub trait HFloat: Float {
    /// Float that is double the bit width of the float this trait is implemented for.
    type D: DFloat<H = Self>;

    /// Widen the float type.
    fn widen(self) -> Self::D;
}

#[cfg(not(feature = "sonair_certified"))]
macro_rules! impl_d_float {
    ($($X:ident $D:ident),*) => {
        $(
            impl DFloat for $D {
                type H = $X;

                fn narrow(self) -> Self::H {
                    self as $X
                }
            }
        )*
    };
}

#[cfg(not(feature = "sonair_certified"))]
macro_rules! impl_h_float {
    ($($H:ident $X:ident),*) => {
        $(
            impl HFloat for $H {
                type D = $X;

                fn widen(self) -> Self::D {
                    self as $X
                }
            }
        )*
    };
}

#[cfg(not(feature = "sonair_certified"))]
impl_d_float!(f32 f64);

#[cfg(not(feature = "sonair_certified"))]
impl_h_float!(f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_f32() {
        // Constants
        assert_eq!(f32::EXP_SAT, 0b11111111);
        assert_eq!(f32::EXP_BIAS, 127);
        assert_eq!(f32::EXP_MAX, 127);
        assert_eq!(f32::EXP_MIN, -126);
        assert_eq!(f32::EXP_MIN_SUBNORM, -149);

        // `exp_unbiased`
        assert_eq!(f32::FRAC_PI_2.exp_unbiased(), 0);
        assert_eq!((1.0f32 / 2.0).exp_unbiased(), -1);
        assert_eq!(f32::MAX.exp_unbiased(), 127);
        assert_eq!(f32::MIN.exp_unbiased(), 127);
        assert_eq!(f32::MIN_POSITIVE.exp_unbiased(), -126);
        // This is a convenience method and not ldexp, `exp_unbiased` does not return correct
        // results for zero and subnormals.
        assert_eq!(f32::ZERO.exp_unbiased(), -127);
        assert_eq!(f32::from_bits(0x1).exp_unbiased(), -127);
        assert_eq!(f32::MIN_POSITIVE, f32::MIN_POSITIVE_NORMAL);

        // `from_parts`
        assert_biteq!(f32::from_parts(true, f32::EXP_BIAS, 0), -1.0f32);
        assert_biteq!(
            f32::from_parts(false, 10 + f32::EXP_BIAS, 0),
            1024.0f32
        );
        assert_biteq!(f32::from_parts(false, 0, 1), f32::from_bits(0x1));
    }

    #[test]
    fn check_f64() {
        // Constants
        assert_eq!(f64::EXP_SAT, 0b11111111111);
        assert_eq!(f64::EXP_BIAS, 1023);
        assert_eq!(f64::EXP_MAX, 1023);
        assert_eq!(f64::EXP_MIN, -1022);
        assert_eq!(f64::EXP_MIN_SUBNORM, -1074);

        // `exp_unbiased`
        assert_eq!(f64::FRAC_PI_2.exp_unbiased(), 0);
        assert_eq!((1.0f64 / 2.0).exp_unbiased(), -1);
        assert_eq!(f64::MAX.exp_unbiased(), 1023);
        assert_eq!(f64::MIN.exp_unbiased(), 1023);
        assert_eq!(f64::MIN_POSITIVE.exp_unbiased(), -1022);
        // This is a convenience method and not ldexp, `exp_unbiased` does not return correct
        // results for zero and subnormals.
        assert_eq!(f64::ZERO.exp_unbiased(), -1023);
        assert_eq!(f64::from_bits(0x1).exp_unbiased(), -1023);
        assert_eq!(f64::MIN_POSITIVE, f64::MIN_POSITIVE_NORMAL);

        // `from_parts`
        assert_biteq!(f64::from_parts(true, f64::EXP_BIAS, 0), -1.0f64);
        assert_biteq!(
            f64::from_parts(false, 10 + f64::EXP_BIAS, 0),
            1024.0f64
        );
        assert_biteq!(f64::from_parts(false, 0, 1), f64::from_bits(0x1));
    }

    #[test]
    fn to_bits_signed_f32() {
        assert_eq!(1.0f32.to_bits_signed(), 1.0f32.to_bits() as i32);
        assert_eq!((-1.0f32).to_bits_signed(), (-1.0f32).to_bits() as i32);
    }

    #[test]
    fn biteq_f32() {
        assert!(1.0f32.biteq(1.0f32));
        assert!(!1.0f32.biteq(-1.0f32));
        assert!(0.0f32.biteq(0.0f32));
        assert!(!0.0f32.biteq(-0.0f32));
        assert!(f32::NAN.biteq(f32::NAN));
    }

    #[test]
    fn eq_repr_f32() {
        assert!(1.0f32.eq_repr(1.0f32));
        assert!(!1.0f32.eq_repr(-1.0f32));
        assert!(f32::NAN.eq_repr(f32::NAN));
        assert!(!0.0f32.eq_repr(-0.0f32));
    }

    /// Exercise Float trait methods through fully-qualified trait calls
    /// so llvm-cov counts the macro-expanded bodies, not inherent methods.
    #[inline(never)]
    fn exercise_float<F: Float>(one: F, neg_one: F, zero: F, neg_zero: F, subnorm: F, nan: F) {
        assert!(Float::is_subnormal(subnorm));
        assert!(!Float::is_subnormal(one));

        assert!(Float::frac(one) == F::Int::ZERO);

        assert_biteq!(Float::signum(one), one);
        assert_biteq!(Float::signum(neg_one), neg_one);
        assert!(Float::signum(nan).is_nan());

        assert_biteq!(Float::canonicalize(one), one);
        assert_biteq!(Float::canonicalize(neg_one), neg_one);

        assert!(Float::is_sign_positive(one));
        assert!(!Float::is_sign_positive(neg_one));
        assert!(Float::is_sign_positive(zero));
        assert!(!Float::is_sign_positive(neg_zero));

        assert!(Float::is_infinite(F::INFINITY));
        assert!(Float::is_infinite(F::NEG_INFINITY));
        assert!(!Float::is_infinite(one));

        assert_biteq!(Float::abs(neg_one), one);
        assert_biteq!(Float::abs(one), one);

        assert_biteq!(Float::copysign(one, neg_one), neg_one);
        assert_biteq!(Float::copysign(neg_one, one), one);
    }

    #[test]
    fn float_trait_dispatch_f32() {
        exercise_float::<f32>(1.0, -1.0, 0.0, -0.0, f32::from_bits(1), f32::NAN);
    }

    #[test]
    fn float_trait_dispatch_f64() {
        exercise_float::<f64>(1.0, -1.0, 0.0, -0.0, f64::from_bits(1), f64::NAN);
    }

    #[test]
    fn f32_from_bits_fn() {
        assert_eq!(f32_from_bits(0x3f800000), 1.0f32);
        assert_eq!(f32_from_bits(0), 0.0f32);
        assert!(f32_from_bits(0x7fc00000).is_nan());
    }

    #[test]
    fn f32_to_bits_fn() {
        assert_eq!(f32_to_bits(1.0f32), 0x3f800000);
        assert_eq!(f32_to_bits(0.0f32), 0);
    }

    #[test]
    fn f64_from_bits_fn() {
        assert_eq!(f64_from_bits(0x3ff0000000000000), 1.0f64);
        assert_eq!(f64_from_bits(0), 0.0f64);
        assert!(f64_from_bits(0x7ff8000000000000).is_nan());
    }

    #[test]
    fn f64_to_bits_fn() {
        assert_eq!(f64_to_bits(1.0f64), 0x3ff0000000000000);
        assert_eq!(f64_to_bits(0.0f64), 0);
    }
}
