//! Support for rounding directions and status flags as specified by IEEE 754.
//!
//! Rust does not support the floating point environment so rounding mode is passed as an argument
//! and status flags are returned as part of the result. There is currently not much support for
//! this; most existing ports from musl use a form of `force_eval!` to raise exceptions, but this
//! has no side effects in Rust. Further, correct behavior relies on elementary operations making
//! use of the correct rounding and raising relevant exceptions, which is not the case for Rust.
//!
//! This module exists so no functionality is lost when porting algorithms that respect floating
//! point environment, and so that some functionality may be tested (that which does not rely on
//! side effects from elementary operations). Full support would require wrappers around basic
//! operations, but there is no plan to add this at the current time.

/// A value combined with a floating point status.
pub struct FpResult<T> {
    pub val: T,
    #[cfg_attr(not(feature = "unstable-public-internals"), allow(dead_code))] // Field is read by tests and unstable-public-internals consumers; appears unused in normal builds
    pub status: Status,
}

#[cfg_attr(coverage_nightly, coverage(off))] // Infrastructure type: constructors are trivial field assignments
impl<T> FpResult<T> {
    pub fn new(val: T, status: Status) -> Self {
        Self { val, status }
    }

    /// Return `val` with `Status::OK`.
    pub fn ok(val: T) -> Self {
        Self {
            val,
            status: Status::OK,
        }
    }
}

/// IEEE 754 rounding mode, excluding the optional `roundTiesToAway` version of nearest.
///
/// Integer representation comes from what CORE-MATH uses for indexing.
#[cfg_attr(not(feature = "unstable-public-internals"), allow(dead_code))] // Variants beyond Nearest are used by non-certified rounding functions; appears unused in normal builds
#[cfg_attr(not(feature = "sonair_certified"), derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum Round {
    /// IEEE 754 nearest, `roundTiesToEven`.
    Nearest = 0,
    /// IEEE 754 `roundTowardNegative`.
    Negative = 1,
    /// IEEE 754 `roundTowardPositive`.
    Positive = 2,
    /// IEEE 754 `roundTowardZero`.
    Zero = 3,
}

/// IEEE 754 exception status flags.
#[cfg_attr(not(feature = "sonair_certified"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(u8);

#[cfg_attr(coverage_nightly, coverage(off))] // Infrastructure type: status flag constants and trivial bitwise query methods
impl Status {
    /// Default status indicating no errors.
    pub const OK: Self = Self(0);

    /// No definable result.
    ///
    /// Includes:
    /// - Any ops on sNaN, with a few exceptions.
    /// - `0 * inf`, `inf * 0`.
    /// - `fma(0, inf, c)` or `fma(inf, 0, c)`, possibly excluding `c = qNaN`.
    /// - `+inf + -inf` and similar (includes subtraction and fma).
    /// - `0.0 / 0.0`, `inf / inf`
    /// - `remainder(x, y)` if `y == 0.0` or `x == inf`, and neither is NaN.
    /// - `sqrt(x)` with `x < 0.0`.
    pub const INVALID: Self = Self(1);

    /// Division by zero.
    ///
    /// The default result for division is +/-inf based on operand sign. For `logB`, the default
    /// result is -inf.
    /// `x / y` when `x != 0.0` and `y == 0.0`,
    #[cfg_attr(not(feature = "unstable-public-internals"), allow(dead_code))] // Used by non-certified logB/division functions; not used by certified f32 functions
    pub const DIVIDE_BY_ZERO: Self = Self(1 << 2);

    /// The result exceeds the maximum finite value.
    ///
    /// The default result depends on rounding mode. `Nearest*` rounds to +/- infinity, sign based
    /// on the intermediate result. `Zero` rounds to the signed maximum finite. `Positive` and
    /// `Negative` round to signed maximum finite in one direction, signed infinity in the other.
    #[cfg_attr(not(feature = "unstable-public-internals"), allow(dead_code))] // Used by non-certified overflow-detecting functions; not used by certified f32 functions
    pub const OVERFLOW: Self = Self(1 << 3);

    /// The result is subnormal and lost precision.
    pub const UNDERFLOW: Self = Self(1 << 4);

    /// The finite-precision result does not match that of infinite precision, and the reason
    /// is not represented by one of the other flags.
    pub const INEXACT: Self = Self(1 << 5);

    /// True if `UNDERFLOW` is set.
    #[cfg_attr(not(feature = "unstable-public-internals"), allow(dead_code))] // Query method exposed for testing; not called in certified production code
    pub const fn underflow(self) -> bool {
        self.0 & Self::UNDERFLOW.0 != 0
    }

    /// True if `OVERFLOW` is set.
    #[cfg_attr(not(feature = "unstable-public-internals"), allow(dead_code))] // Query method exposed for testing; not called in certified production code
    pub const fn overflow(self) -> bool {
        self.0 & Self::OVERFLOW.0 != 0
    }

    #[cfg(not(feature = "sonair_certified"))]
    pub fn set_underflow(&mut self, val: bool) {
        self.set_flag(val, Self::UNDERFLOW);
    }

    /// True if `INEXACT` is set.
    #[allow(dead_code)] // Query method for INEXACT flag; used by non-certified functions and tests
    pub const fn inexact(self) -> bool {
        self.0 & Self::INEXACT.0 != 0
    }

    #[cfg(not(feature = "sonair_certified"))]
    pub fn set_inexact(&mut self, val: bool) {
        self.set_flag(val, Self::INEXACT);
    }

    #[cfg(not(feature = "sonair_certified"))]
    fn set_flag(&mut self, val: bool, mask: Self) {
        if val {
            self.0 |= mask.0;
        } else {
            self.0 &= !mask.0;
        }
    }

    #[allow(dead_code)] // Used internally by generic math functions to combine status flags
    pub(crate) const fn with(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
