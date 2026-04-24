use core::marker::PhantomData;

use crate::*;

/// Generic helper for libm functions, abstracting over f32 and f64. <br/>
/// # Type Parameter:
/// - `T`: Either `f32` or `f64`
///
/// # Examples
///
/// `f32` helpers match the `libm::*f` functions and are available in all builds.
/// With the default features (without `sonair_certified`), the same pattern applies
/// for `f64` and the unprefixed `libm` exports where present.
///
/// ```rust
/// use libm::{self, Libm};
///
/// const PI_F32: f32 = 3.1415927410e+00;
///
/// assert!(Libm::<f32>::cos(0.0f32) == libm::cosf(0.0));
/// assert!(Libm::<f32>::sin(PI_F32) == libm::sinf(PI_F32));
/// ```
pub struct Libm<T>(PhantomData<T>);

macro_rules! libm_helper {
    ($t:ident, funcs: $funcs:tt) => {
        impl Libm<$t> {
            #![allow(unused_parens)]

            libm_helper! { $funcs }
        }
    };

    ({$($func:tt;)*}) => {
        $(
            libm_helper! { $func }
        )*
    };

    ((fn $func:ident($($arg:ident: $arg_typ:ty),*) -> ($($ret_typ:ty),*); => $libm_fn:ident)) => {
        #[inline(always)]
        pub fn $func($($arg: $arg_typ),*) -> ($($ret_typ),*) {
            $libm_fn($($arg),*)
        }
    };
}

// verify-apilist-start
#[cfg(not(feature = "sonair_certified"))]
libm_helper! {
    f32,
    funcs: {
        // verify-sorted-start
        (fn atan2(y: f32, x: f32) -> (f32);         => atan2f);
        (fn ceil(x: f32) -> (f32);                  => ceilf);
        (fn cos(x: f32) -> (f32);                   => cosf);
        (fn exp(x: f32) -> (f32);                   => expf);
        (fn fabs(x: f32) -> (f32);                  => fabsf);
        (fn floor(x: f32) -> (f32);                 => floorf);
        (fn pow(x: f32, y: f32) -> (f32);           => powf);
        (fn rint(x: f32) -> (f32);                  => rintf);
        (fn round(x: f32) -> (f32);                 => roundf);
        (fn roundeven(x: f32) -> (f32);             => roundevenf);
        (fn scalbn(x: f32, n: i32) -> (f32);        => scalbnf);
        (fn sin(x: f32) -> (f32);                   => sinf);
        (fn sqrt(x: f32) -> (f32);                  => sqrtf);
        (fn tan(x: f32) -> (f32);                   => tanf);
        // verify-sorted-end
    }
}

#[cfg(feature = "sonair_certified")]
libm_helper! {
    f32,
    funcs: {
        // verify-sorted-start
        (fn atan2(y: f32, x: f32) -> (f32);         => atan2f);
        (fn ceil(x: f32) -> (f32);                  => ceilf);
        (fn cos(x: f32) -> (f32);                   => cosf);
        (fn exp(x: f32) -> (f32);                   => expf);
        (fn fabs(x: f32) -> (f32);                  => fabsf);
        (fn floor(x: f32) -> (f32);                 => floorf);
        (fn pow(x: f32, y: f32) -> (f32);           => powf);
        (fn round(x: f32) -> (f32);                 => roundf);
        (fn sin(x: f32) -> (f32);                   => sinf);
        (fn sqrt(x: f32) -> (f32);                  => sqrtf);
        (fn tan(x: f32) -> (f32);                   => tanf);
        // verify-sorted-end
    }
}

#[cfg(not(feature = "sonair_certified"))]
libm_helper! {
    f64,
    funcs: {
        // verify-sorted-start
        (fn atan2(y: f64, x: f64) -> (f64);         => atan2);
        (fn ceil(x: f64) -> (f64);                  => ceil);
        (fn cos(x: f64) -> (f64);                   => cos);
        (fn exp(x: f64) -> (f64);                   => exp);
        (fn fabs(x: f64) -> (f64);                  => fabs);
        (fn floor(x: f64) -> (f64);                 => floor);
        (fn pow(x: f64, y: f64) -> (f64);           => pow);
        (fn rint(x: f64) -> (f64);                  => rint);
        (fn round(x: f64) -> (f64);                 => round);
        (fn roundevem(x: f64) -> (f64);             => roundeven);
        (fn scalbn(x: f64, n: i32) -> (f64);        => scalbn);
        (fn sin(x: f64) -> (f64);                   => sin);
        (fn sqrt(x: f64) -> (f64);                  => sqrt);
        (fn tan(x: f64) -> (f64);                   => tan);
        // verify-sorted-end
    }
}

// verify-apilist-end
