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
mod expo2;
#[cfg(not(feature = "sonair_certified"))]
mod k_cos;
mod k_cosf;
#[cfg(not(feature = "sonair_certified"))]
mod k_expo2;
#[cfg(not(feature = "sonair_certified"))]
mod k_expo2f;
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
use self::expo2::expo2;
#[cfg(not(feature = "sonair_certified"))]
use self::k_cos::k_cos;
use self::k_cosf::k_cosf;
#[cfg(not(feature = "sonair_certified"))]
use self::k_expo2::k_expo2;
#[cfg(not(feature = "sonair_certified"))]
use self::k_expo2f::k_expo2f;
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
mod acos;
#[cfg(not(feature = "sonair_certified"))]
mod acosf;
#[cfg(not(feature = "sonair_certified"))]
mod asin;
#[cfg(not(feature = "sonair_certified"))]
mod asinf;
#[cfg(not(feature = "sonair_certified"))]
mod asinh;
#[cfg(not(feature = "sonair_certified"))]
mod asinhf;
#[cfg(not(feature = "sonair_certified"))]
mod atan;
#[cfg(not(feature = "sonair_certified"))]
mod atan2;
mod atan2f;
mod atanf;
#[cfg(not(feature = "sonair_certified"))]
mod atanh;
#[cfg(not(feature = "sonair_certified"))]
mod atanhf;
#[cfg(not(feature = "sonair_certified"))]
mod cbrt;
#[cfg(not(feature = "sonair_certified"))]
mod cbrtf;
mod ceil;
mod copysign;
#[cfg(not(feature = "sonair_certified"))]
mod cos;
mod cosf;
#[cfg(not(feature = "sonair_certified"))]
mod cosh;
#[cfg(not(feature = "sonair_certified"))]
mod coshf;
#[cfg(not(feature = "sonair_certified"))]
mod erf;
#[cfg(not(feature = "sonair_certified"))]
mod erff;
#[cfg(not(feature = "sonair_certified"))]
mod exp10;
#[cfg(not(feature = "sonair_certified"))]
mod exp10f;
#[cfg(not(feature = "sonair_certified"))]
mod exp2;
#[cfg(not(feature = "sonair_certified"))]
mod exp2f;
mod expf;
#[cfg(not(feature = "sonair_certified"))]
mod expm1;
#[cfg(not(feature = "sonair_certified"))]
mod expm1f;
mod fabs;
#[cfg(not(feature = "sonair_certified"))]
mod fdim;
mod floor;
#[cfg(not(feature = "sonair_certified"))]
mod fma;
#[cfg(not(feature = "sonair_certified"))]
mod fmin_fmax;
#[cfg(not(feature = "sonair_certified"))]
mod fminimum_fmaximum;
#[cfg(not(feature = "sonair_certified"))]
mod fminimum_fmaximum_num;
#[cfg(not(feature = "sonair_certified"))]
mod fmod;
#[cfg(not(feature = "sonair_certified"))]
mod frexp;
#[cfg(not(feature = "sonair_certified"))]
mod frexpf;
#[cfg(not(feature = "sonair_certified"))]
mod hypot;
#[cfg(not(feature = "sonair_certified"))]
mod hypotf;
#[cfg(not(feature = "sonair_certified"))]
mod ilogb;
#[cfg(not(feature = "sonair_certified"))]
mod ilogbf;
#[cfg(not(feature = "sonair_certified"))]
mod j0;
#[cfg(not(feature = "sonair_certified"))]
mod j0f;
#[cfg(not(feature = "sonair_certified"))]
mod j1;
#[cfg(not(feature = "sonair_certified"))]
mod j1f;
#[cfg(not(feature = "sonair_certified"))]
mod jn;
#[cfg(not(feature = "sonair_certified"))]
mod jnf;
#[cfg(not(feature = "sonair_certified"))]
mod ldexp;
#[cfg(not(feature = "sonair_certified"))]
mod lgamma;
#[cfg(not(feature = "sonair_certified"))]
mod lgamma_r;
#[cfg(not(feature = "sonair_certified"))]
mod lgammaf;
#[cfg(not(feature = "sonair_certified"))]
mod lgammaf_r;
#[cfg(not(feature = "sonair_certified"))]
mod log;
#[cfg(not(feature = "sonair_certified"))]
mod log10;
#[cfg(not(feature = "sonair_certified"))]
mod log10f;
#[cfg(not(feature = "sonair_certified"))]
mod log1p;
#[cfg(not(feature = "sonair_certified"))]
mod log1pf;
#[cfg(not(feature = "sonair_certified"))]
mod log2;
#[cfg(not(feature = "sonair_certified"))]
mod log2f;
mod logf;
#[cfg(not(feature = "sonair_certified"))]
mod modf;
#[cfg(not(feature = "sonair_certified"))]
mod modff;
#[cfg(not(feature = "sonair_certified"))]
mod nextafter;
#[cfg(not(feature = "sonair_certified"))]
mod nextafterf;
#[cfg(not(feature = "sonair_certified"))]
mod pow;
mod powf;
#[cfg(not(feature = "sonair_certified"))]
mod remainder;
#[cfg(not(feature = "sonair_certified"))]
mod remainderf;
#[cfg(not(feature = "sonair_certified"))]
mod remquo;
#[cfg(not(feature = "sonair_certified"))]
mod remquof;
#[cfg(not(feature = "sonair_certified"))]
mod rint;
mod round;
#[cfg(not(feature = "sonair_certified"))]
mod roundeven;
mod scalbn;
#[cfg(not(feature = "sonair_certified"))]
mod sin;
#[cfg(not(feature = "sonair_certified"))]
mod sincos;
#[cfg(not(feature = "sonair_certified"))]
mod sincosf;
mod sinf;
#[cfg(not(feature = "sonair_certified"))]
mod sinh;
#[cfg(not(feature = "sonair_certified"))]
mod sinhf;
mod sqrt;
#[cfg(not(feature = "sonair_certified"))]
mod tan;
mod tanf;
#[cfg(not(feature = "sonair_certified"))]
mod tanh;
#[cfg(not(feature = "sonair_certified"))]
mod tanhf;
#[cfg(not(feature = "sonair_certified"))]
mod tgamma;
#[cfg(not(feature = "sonair_certified"))]
mod tgammaf;
#[cfg(not(feature = "sonair_certified"))]
mod trunc;

// Use separated imports instead of {}-grouped imports for easier merging.
#[cfg(not(feature = "sonair_certified"))]
pub use self::acos::acos;
#[cfg(not(feature = "sonair_certified"))]
pub use self::acosf::acosf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::asin::asin;
#[cfg(not(feature = "sonair_certified"))]
pub use self::asinf::asinf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::asinh::asinh;
#[cfg(not(feature = "sonair_certified"))]
pub use self::asinhf::asinhf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::atan::atan;
#[cfg(not(feature = "sonair_certified"))]
pub use self::atan2::atan2;
pub use self::atan2f::atan2f;
pub use self::atanf::atanf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::atanh::atanh;
#[cfg(not(feature = "sonair_certified"))]
pub use self::atanhf::atanhf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::cbrt::cbrt;
#[cfg(not(feature = "sonair_certified"))]
pub use self::cbrtf::cbrtf;
pub use self::ceil::ceilf;
pub use self::copysign::{copysign, copysignf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::cos::cos;
pub use self::cosf::cosf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::cosh::cosh;
#[cfg(not(feature = "sonair_certified"))]
pub use self::coshf::coshf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::erf::{erf, erfc};
#[cfg(not(feature = "sonair_certified"))]
pub use self::erff::{erfcf, erff};
#[cfg(not(feature = "sonair_certified"))]
pub use self::exp10::exp10;
#[cfg(not(feature = "sonair_certified"))]
pub use self::exp10f::exp10f;
#[cfg(not(feature = "sonair_certified"))]
pub use self::exp2::exp2;
#[cfg(not(feature = "sonair_certified"))]
pub use self::exp2f::exp2f;
pub use self::expf::expf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::expm1::expm1;
#[cfg(not(feature = "sonair_certified"))]
pub use self::expm1f::expm1f;
pub use self::fabs::fabsf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::fdim::{fdim, fdimf};
pub use self::floor::{floor, floorf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::fma::{fma, fmaf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::fmin_fmax::{fmax, fmaxf, fmin, fminf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::fminimum_fmaximum::{fmaximum, fmaximumf, fminimum, fminimumf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::fminimum_fmaximum_num::{fmaximum_num, fmaximum_numf, fminimum_num, fminimum_numf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::fmod::{fmod, fmodf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::frexp::frexp;
#[cfg(not(feature = "sonair_certified"))]
pub use self::frexpf::frexpf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::hypot::hypot;
#[cfg(not(feature = "sonair_certified"))]
pub use self::hypotf::hypotf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::ilogb::ilogb;
#[cfg(not(feature = "sonair_certified"))]
pub use self::ilogbf::ilogbf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::j0::{j0, y0};
#[cfg(not(feature = "sonair_certified"))]
pub use self::j0f::{j0f, y0f};
#[cfg(not(feature = "sonair_certified"))]
pub use self::j1::{j1, y1};
#[cfg(not(feature = "sonair_certified"))]
pub use self::j1f::{j1f, y1f};
#[cfg(not(feature = "sonair_certified"))]
pub use self::jn::{jn, yn};
#[cfg(not(feature = "sonair_certified"))]
pub use self::jnf::{jnf, ynf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::ldexp::{ldexp, ldexpf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::lgamma::lgamma;
#[cfg(not(feature = "sonair_certified"))]
pub use self::lgamma_r::lgamma_r;
#[cfg(not(feature = "sonair_certified"))]
pub use self::lgammaf::lgammaf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::lgammaf_r::lgammaf_r;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log::log;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log10::log10;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log10f::log10f;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log1p::log1p;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log1pf::log1pf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log2::log2;
#[cfg(not(feature = "sonair_certified"))]
pub use self::log2f::log2f;
pub use self::logf::logf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::modf::modf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::modff::modff;
#[cfg(not(feature = "sonair_certified"))]
pub use self::nextafter::nextafter;
#[cfg(not(feature = "sonair_certified"))]
pub use self::nextafterf::nextafterf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::pow::pow;
pub use self::powf::powf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::remainder::remainder;
#[cfg(not(feature = "sonair_certified"))]
pub use self::remainderf::remainderf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::remquo::remquo;
#[cfg(not(feature = "sonair_certified"))]
pub use self::remquof::remquof;
#[cfg(not(feature = "sonair_certified"))]
pub use self::rint::{rint, rintf};
pub use self::round::roundf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::roundeven::{roundeven, roundevenf};
pub use self::scalbn::{scalbn, scalbnf};
#[cfg(not(feature = "sonair_certified"))]
pub use self::sin::sin;
#[cfg(not(feature = "sonair_certified"))]
pub use self::sincos::sincos;
#[cfg(not(feature = "sonair_certified"))]
pub use self::sincosf::sincosf;
pub use self::sinf::sinf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::sinh::sinh;
#[cfg(not(feature = "sonair_certified"))]
pub use self::sinhf::sinhf;
pub use self::sqrt::sqrtf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::tan::tan;
pub use self::tanf::tanf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::tanh::tanh;
#[cfg(not(feature = "sonair_certified"))]
pub use self::tanhf::tanhf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::tgamma::tgamma;
#[cfg(not(feature = "sonair_certified"))]
pub use self::tgammaf::tgammaf;
#[cfg(not(feature = "sonair_certified"))]
pub use self::trunc::{trunc, truncf};

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

#[cfg(not(feature = "sonair_certified"))]
#[inline]
fn combine_words(hi: u32, lo: u32) -> f64 {
    f64::from_bits(((hi as u64) << 32) | lo as u64)
}
