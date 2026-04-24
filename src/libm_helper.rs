use core::marker::PhantomData;

use crate::*;

/// Helper for `f32` libm-style functions (the `*f` suffixed APIs).
///
/// # Examples
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
        (fn logf(x: f32) -> (f32);           => logf);
        (fn pow(x: f32, y: f32) -> (f32);           => powf);
        (fn round(x: f32) -> (f32);                 => roundf);
        (fn sin(x: f32) -> (f32);                   => sinf);
        (fn sqrt(x: f32) -> (f32);                  => sqrtf);
        (fn tan(x: f32) -> (f32);                   => tanf);
        // verify-sorted-end
    }
}

// verify-apilist-end
