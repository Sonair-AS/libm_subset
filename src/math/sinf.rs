/* origin: FreeBSD /usr/src/lib/msun/src/s_sinf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Optimized by Bruce D. Evans.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

use core::f64::consts::FRAC_PI_2;

use super::{k_cosf, k_sinf, rem_pio2f};

/* Small multiples of pi/2 rounded to double precision. */
const S1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x3FF921FB, 0x54442D18 */
const S2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x400921FB, 0x54442D18 */
const S3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x4012D97C, 0x7F3321D2 */
const S4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x401921FB, 0x54442D18 */

/// The sine of `x` (f32).
///
/// `x` is specified in radians.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn sinf(x: f32) -> f32 {
    let x64 = x as f64;

    let x1p120 = f32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    if ix <= 0x3f490fda {
        /* |x| ~<= pi/4 */
        if ix < 0x39800000 {
            /* |x| < 2**-12 */
            /* raise inexact if x!=0 and underflow if subnormal */
            force_eval!(if ix < 0x00800000 {
                x / x1p120
            } else {
                x + x1p120
            });
            return x;
        }
        return k_sinf(x64);
    }
    if ix <= 0x407b53d1 {
        /* |x| ~<= 5*pi/4 */
        if ix <= 0x4016cbe3 {
            /* |x| ~<= 3pi/4 */
            if sign {
                return -k_cosf(x64 + S1_PIO2);
            } else {
                return k_cosf(x64 - S1_PIO2);
            }
        }
        return k_sinf(if sign {
            -(x64 + S2_PIO2)
        } else {
            -(x64 - S2_PIO2)
        });
    }
    if ix <= 0x40e231d5 {
        /* |x| ~<= 9*pi/4 */
        if ix <= 0x40afeddf {
            /* |x| ~<= 7*pi/4 */
            if sign {
                return k_cosf(x64 + S3_PIO2);
            } else {
                return -k_cosf(x64 - S3_PIO2);
            }
        }
        return k_sinf(if sign { x64 + S4_PIO2 } else { x64 - S4_PIO2 });
    }

    /* sin(Inf or NaN) is NaN */
    if ix >= 0x7f800000 {
        return x - x;
    }

    /* general argument reduction needed */
    let (n, y) = rem_pio2f(x);
    match n & 3 {
        0 => k_sinf(y),
        1 => k_cosf(y),
        2 => k_sinf(-y),
        _ => -k_cosf(y),
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`sinf`] (sine in radians, `f32`).

    use super::*;

    #[test]
    fn sinf_preserves_signed_zero() {
        assert_biteq!(sinf(0.0), 0.0);
        assert_biteq!(sinf(-0.0), -0.0);
    }

    #[test]
    fn sinf_tiny_returns_x() {
        // |x| < 2**-12: implementation returns `x` (raises inexact; see `sinf` source).
        let x = f32::from_bits(0x000116c2);
        assert_biteq!(sinf(x), x);
    }

    #[test]
    fn sinf_nan_and_infinity() {
        assert!(sinf(f32::NAN).is_nan());
        assert!(sinf(f32::INFINITY).is_nan());
        assert!(sinf(f32::NEG_INFINITY).is_nan());
    }

    #[test]
    fn sinf_odd_symmetry_known_quadrant() {
        let x = f32::from_bits(0x3f490fdb); // pi/4
        assert_biteq!(sinf(-x), -sinf(x));
    }

    /// |x| so large that `rem_pio2f` does **not** use the medium-argument branch
    /// (`ix < 0x4dc90fdb`); it calls `rem_pio2_large` (see `rem_pio2f.rs`).
    /// `1000·π` and similar tests above still use the medium path; this input does not.
    #[test]
    fn rem_pio2_large_path_bit_exact() {
        // ~1e9, just above the medium/large split in `rem_pio2f`
        let x = f32::from_bits(0x4e6e6b28);
        assert_biteq!(sinf(x), f32::from_bits(0x3f0bbc65));
    }

    /// Arguments large enough that `|x|` is **not** handled by the π/4 … 9π/4
    /// `k_sinf` / `k_cosf` branches; `sinf` uses `rem_pio2f` and the
    /// `match n & 3` path (see `sinf` source). Values are bit-exact for this
    /// implementation.
    #[test]
    fn rem_pio2f_path_variants() {
        let p1000 = f32::from_bits(0x4544597c); // ~1000·π
        assert_biteq!(sinf(p1000), f32::from_bits(0x38fb56bf));
        let p1000_neg = f32::from_bits(0xc544597c);
        assert_biteq!(sinf(p1000_neg), f32::from_bits(0xb8fb56bf));
        let p500 = f32::from_bits(0x44c4597c); // ~500·π, different `n` mod 4
        assert_biteq!(sinf(p500), f32::from_bits(0x387b56bf));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn sinf_conformance_bit_exact() {
        let cases = [
            (0x3f800000_u32, 0x3f576aa4_u32),   // sin(1.0)
            (0x3f060a92_u32, 0x3f000000_u32),   // sin(pi/6) == 0.5
            (0x3f490fdb_u32, 0x3f3504f3_u32),   // sin(pi/4)
            (0x3fc90fdb_u32, 0x3f800000_u32),   // sin(pi/2) == 1.0
            (0xbf490fdb_u32, 0xbf3504f3_u32),   // sin(-pi/4)
            (0x4544597c_u32, 0x38fb56bf_u32),   // large arg -> `rem_pio2f` path
            (0x40490fdb_u32, 0xb3bbbd2e_u32),   // sin(pi as f32) (cancellation)
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(sinf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }
}
