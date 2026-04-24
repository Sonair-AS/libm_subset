#![allow(clippy::approx_constant)] // many false positives

macro_rules! force_eval {
    ($e:expr) => {
        unsafe { ::core::ptr::read_volatile(&$e) }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! i {
    ($array:expr, $index:expr) => {
        unsafe { *$array.get_unchecked($index) }
    };
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
    ($array:expr, $index:expr, == , $rhs:expr) => {
        unsafe { *$array.get_unchecked_mut($index) == $rhs }
    };
}

#[cfg(debug_assertions)]
macro_rules! i {
    ($array:expr, $index:expr) => {
        *$array.get($index).unwrap()
    };
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
    ($array:expr, $index:expr, == , $rhs:expr) => {
        *$array.get_mut($index).unwrap() == $rhs
    };
}

// Temporary macro to avoid panic codegen for division (in debug mode too). At
// the time of this writing this is only used in a few places, and once
// rust-lang/rust#72751 is fixed then this macro will no longer be necessary and
// the native `/` operator can be used and panics won't be codegen'd.
#[cfg(any(debug_assertions, not(intrinsics_enabled)))]
macro_rules! div {
    ($a:expr, $b:expr) => {
        $a / $b
    };
}

#[cfg(all(not(debug_assertions), intrinsics_enabled))]
macro_rules! div {
    ($a:expr, $b:expr) => {
        unsafe { core::intrinsics::unchecked_div($a, $b) }
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
#[cfg(not(feature = "sonair_certified"))]
mod k_cos;
mod k_cosf;
#[cfg(not(feature = "sonair_certified"))]
mod k_sin;
mod k_sinf;
#[cfg(not(feature = "sonair_certified"))]
mod k_tan;
mod k_tanf;
#[cfg(not(feature = "sonair_certified"))]
mod rem_pio2;
mod rem_pio2_large;
mod rem_pio2f;

// Private re-imports
#[cfg(not(feature = "sonair_certified"))]
use self::k_cos::k_cos;
use self::k_cosf::k_cosf;
#[cfg(not(feature = "sonair_certified"))]
use self::k_sin::k_sin;
use self::k_sinf::k_sinf;
#[cfg(not(feature = "sonair_certified"))]
use self::k_tan::k_tan;
use self::k_tanf::k_tanf;
#[cfg(not(feature = "sonair_certified"))]
use self::rem_pio2::rem_pio2;
use self::rem_pio2_large::rem_pio2_large;
use self::rem_pio2f::rem_pio2f;
#[allow(unused_imports)]
#[cfg(not(feature = "sonair_certified"))]
use self::support::{CastFrom, CastInto, DFloat, DInt, Float, HFloat, HInt, Int, IntTy, MinInt};

// Public modules
#[cfg(not(feature = "sonair_certified"))]
mod atan2;
mod atan2f;
mod ceil;
#[cfg(not(feature = "sonair_certified"))]
mod cos;
mod cosf;
mod expf;
mod fabs;
mod floor;
#[cfg(not(feature = "sonair_certified"))]
mod pow;
mod powf;
mod round;
#[cfg(not(feature = "sonair_certified"))]
mod sin;
mod sinf;
mod sqrt;
#[cfg(not(feature = "sonair_certified"))]
mod tan;
mod tanf;
// Use separated imports instead of {}-grouped imports for easier merging.
pub use self::atan2f::atan2f;
pub use self::ceil::ceilf;
pub use self::cosf::cosf;
pub use self::expf::expf;
pub use self::fabs::fabsf;
pub use self::floor::floorf;
pub use self::powf::powf;
pub use self::round::roundf;
pub use self::sinf::sinf;
pub use self::sqrt::sqrtf;
pub use self::tanf::tanf;
#[cfg(not(feature = "sonair_certified"))]
#[inline]
fn get_high_word(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

#[cfg(not(feature = "sonair_certified"))]
#[inline]
fn get_low_word(x: f64) -> u32 {
    x.to_bits() as u32
}

#[cfg(not(feature = "sonair_certified"))]
#[inline]
fn with_set_high_word(f: f64, hi: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0x00000000_ffffffff;
    tmp |= (hi as u64) << 32;
    f64::from_bits(tmp)
}

#[cfg(not(feature = "sonair_certified"))]
#[inline]
fn with_set_low_word(f: f64, lo: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0xffffffff_00000000;
    tmp |= lo as u64;
    f64::from_bits(tmp)
}
