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
        (fn fmod(x: f32, y: f32) -> (f32);          => fmodf);
        (fn modf(x: f32) -> (f32, f32);             => modff);
        (fn nextafter(x: f32, y: f32) -> (f32);     => nextafterf);
        (fn pow(x: f32, y: f32) -> (f32);           => powf);
        (fn remainder(x: f32, y: f32) -> (f32);     => remainderf);
        (fn remquo(x: f32, y: f32) -> (f32, i32);   => remquof);
        (fn rint(x: f32) -> (f32);                  => rintf);
        (fn round(x: f32) -> (f32);                 => roundf);
        (fn roundeven(x: f32) -> (f32);             => roundevenf);
        (fn scalbn(x: f32, n: i32) -> (f32);        => scalbnf);
        (fn sin(x: f32) -> (f32);                   => sinf);
        (fn sincos(x: f32) -> (f32, f32);           => sincosf);
        (fn sqrt(x: f32) -> (f32);                  => sqrtf);
        (fn tan(x: f32) -> (f32);                   => tanf);
        (fn tgamma(x: f32) -> (f32);                => tgammaf);
        (fn trunc(x: f32) -> (f32);                 => truncf);
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
        (fn fmaximum(x: f64, y: f64) -> (f64);      => fmaximum);
        (fn fmaximum_num(x: f64, y: f64) -> (f64);  => fmaximum_num);
        (fn fmaximum_numf(x: f32, y: f32) -> (f32); => fmaximum_numf);
        (fn fmaximumf(x: f32, y: f32) -> (f32);     => fmaximumf);
        (fn fminimum(x: f64, y: f64) -> (f64);      => fminimum);
        (fn fminimum_num(x: f64, y: f64) -> (f64);  => fminimum_num);
        (fn fminimum_numf(x: f32, y: f32) -> (f32); => fminimum_numf);
        (fn fminimumf(x: f32, y: f32) -> (f32);     => fminimumf);
        (fn fmod(x: f64, y: f64) -> (f64);          => fmod);
        (fn modf(x: f64) -> (f64, f64);             => modf);
        (fn nextafter(x: f64, y: f64) -> (f64);     => nextafter);
        (fn pow(x: f64, y: f64) -> (f64);           => pow);
        (fn remainder(x: f64, y: f64) -> (f64);     => remainder);
        (fn remquo(x: f64, y: f64) -> (f64, i32);   => remquo);
        (fn rint(x: f64) -> (f64);                  => rint);
        (fn round(x: f64) -> (f64);                 => round);
        (fn roundevem(x: f64) -> (f64);             => roundeven);
        (fn scalbn(x: f64, n: i32) -> (f64);        => scalbn);
        (fn sin(x: f64) -> (f64);                   => sin);
        (fn sincos(x: f64) -> (f64, f64);           => sincos);
        (fn sqrt(x: f64) -> (f64);                  => sqrt);
        (fn tan(x: f64) -> (f64);                   => tan);
        (fn tgamma(x: f64) -> (f64);                => tgamma);
        (fn trunc(x: f64) -> (f64);                 => trunc);
        // verify-sorted-end
    }
}

// verify-apilist-end
