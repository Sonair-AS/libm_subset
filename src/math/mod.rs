#![allow(clippy::approx_constant)] // Mathematical constants (pi, e, etc.) appear as intermediate values in formulas, not as replaceable std constants

macro_rules! force_eval {
    ($e:expr) => {
        // SAFETY: read_volatile on a reference to a stack temporary. The reference is valid for
        // the duration of the read. Used to force the compiler to evaluate the expression for
        // its floating-point side effects (e.g. raising underflow).
        unsafe { ::core::ptr::read_volatile(&$e) }
    };
}

/// Unchecked array indexing for release builds. In debug builds this falls through to normal
/// bounds-checked indexing (see below).
///
/// SAFETY for all arms: callers must ensure the index is within bounds. This invariant is
/// verified by the debug-mode fallback which uses normal indexing and will panic on OOB.
#[cfg(not(debug_assertions))]
macro_rules! i {
    ($array:expr, $index:expr) => {
        unsafe { *$array.get_unchecked($index) }
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        unsafe { *$array.get_unchecked_mut($index) == $rhs }
    };
}

/// Unchecked mutating array indexing for release builds (`=`, `+=`, `-=`, `&=`).
///
/// SAFETY for all arms: callers must ensure the index is within bounds. This invariant is
/// verified by the debug-mode fallback which uses normal indexing and will panic on OOB.
#[cfg(not(debug_assertions))]
macro_rules! i_mut {
    ($array:expr, $index:expr, = , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) = $rhs;
        }
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) += $rhs;
        }
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) -= $rhs;
        }
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) &= $rhs;
        }
    };
}

#[cfg(debug_assertions)]
macro_rules! i {
    ($array:expr, $index:expr) => {
        *$array.get($index).unwrap()
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        *$array.get_mut($index).unwrap() == $rhs
    };
}

/// Mutating indexed access (`=`, `+=`, `-=`, `&=`). For reads and `==`
/// comparisons, use `i!` in the same module.
#[cfg(debug_assertions)]
macro_rules! i_mut {
    ($array:expr, $index:expr, = , $rhs:expr) => {
        *$array.get_mut($index).unwrap() = $rhs;
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        *$array.get_mut($index).unwrap() -= $rhs;
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        *$array.get_mut($index).unwrap() += $rhs;
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        *$array.get_mut($index).unwrap() &= $rhs;
    };
}

// `support` may be public for testing
#[macro_use]
#[cfg(feature = "unstable-public-internals")]
pub mod support;

#[macro_use]
#[cfg(not(feature = "unstable-public-internals"))]
pub(crate) mod support;

cfg_if! {
    if #[cfg(feature = "unstable-public-internals")] {
        pub mod generic;
    } else {
        mod generic;
    }
}

// Private modules
mod arch;
mod k_cosf;
mod k_sinf;
mod k_tanf;
mod rem_pio2_large;
mod rem_pio2f;

// Private re-imports
use self::k_cosf::k_cosf;
use self::k_sinf::k_sinf;
use self::k_tanf::k_tanf;
use self::rem_pio2_large::rem_pio2_large;
use self::rem_pio2f::rem_pio2f;

// Public modules
mod atan2f;
mod ceil;
mod cosf;
mod expf;
mod fabs;
mod floor;
mod logf;
mod powf;
mod round;
mod sinf;
mod sqrt;
mod tanf;
// Use separated imports instead of {}-grouped imports for easier merging.
pub use self::atan2f::atan2f;
pub use self::ceil::ceilf;
pub use self::cosf::cosf;
pub use self::expf::expf;
pub use self::fabs::fabsf;
pub use self::floor::{floor, floorf};
pub use self::logf::logf;
pub use self::powf::powf;
pub use self::round::roundf;
pub use self::sinf::sinf;
pub use self::sqrt::sqrtf;
pub use self::tanf::tanf;
