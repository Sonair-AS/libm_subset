/* origin: FreeBSD /usr/src/lib/msun/src/s_tanf.c */
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

use super::{k_tanf, rem_pio2f};

/* Small multiples of pi/2 rounded to double precision. */
const T1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x3FF921FB, 0x54442D18 */
const T2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x400921FB, 0x54442D18 */
const T3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x4012D97C, 0x7F3321D2 */
const T4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x401921FB, 0x54442D18 */

/// The tangent of `x` (f32).
///
/// `x` is specified in radians.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn tanf(x: f32) -> f32 {
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
        return k_tanf(x64, false);
    }
    if ix <= 0x407b53d1 {
        /* |x| ~<= 5*pi/4 */
        if ix <= 0x4016cbe3 {
            /* |x| ~<= 3pi/4 */
            return k_tanf(if sign { x64 + T1_PIO2 } else { x64 - T1_PIO2 }, true);
        } else {
            return k_tanf(if sign { x64 + T2_PIO2 } else { x64 - T2_PIO2 }, false);
        }
    }
    if ix <= 0x40e231d5 {
        /* |x| ~<= 9*pi/4 */
        if ix <= 0x40afeddf {
            /* |x| ~<= 7*pi/4 */
            return k_tanf(if sign { x64 + T3_PIO2 } else { x64 - T3_PIO2 }, true);
        } else {
            return k_tanf(if sign { x64 + T4_PIO2 } else { x64 - T4_PIO2 }, false);
        }
    }

    /* tan(Inf or NaN) is NaN */
    if ix >= 0x7f800000 {
        return x - x;
    }

    /* argument reduction */
    let (n, y) = rem_pio2f(x);
    k_tanf(y, n & 1 != 0)
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`tanf`].

    use super::*;

    #[test]
    fn tanf_zero() {
        assert_biteq!(tanf(0.0), 0.0);
        assert_biteq!(tanf(-0.0), -0.0);
    }

    #[test]
    fn tanf_preserves_odd_symmetry() {
        let x = f32::from_bits(0x3f490fdb); // pi/4
        assert_biteq!(tanf(-x), -tanf(x));
    }

    #[test]
    fn tanf_tiny_returns_x() {
        let x = f32::from_bits(0x000116c2);
        assert_biteq!(tanf(x), x);
    }

    #[test]
    fn tanf_nan_and_infinity() {
        assert!(tanf(f32::NAN).is_nan());
        assert!(tanf(f32::INFINITY).is_nan());
        assert!(tanf(f32::NEG_INFINITY).is_nan());
    }

    /// Large |x| forces the `rem_pio2f` path in `tanf` (not the T1…T4 `k_tanf` segment ranges).
    #[test]
    fn rem_pio2f_path_variants() {
        let p1000 = f32::from_bits(0x4544597c);
        assert_biteq!(tanf(p1000), f32::from_bits(0x38fb56bf));
        let p1000_neg = f32::from_bits(0xc544597c);
        assert_biteq!(tanf(p1000_neg), f32::from_bits(0xb8fb56bf));
        let p500 = f32::from_bits(0x44c4597c);
        assert_biteq!(tanf(p500), f32::from_bits(0x387b56bf));
    }

    #[test]
    fn tanf_conformance_bit_exact() {
        let cases = [
            (0x3f800000_u32, 0x3fc75923_u32), // tan(1)
            (0x3f490fdb_u32, 0x3f800000_u32), // tan(pi/4)
            (0xbf490fdb_u32, 0xbf800000_u32), // tan(-pi/4)
            (0x4544597c_u32, 0x38fb56bf_u32), // large arg
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(tanf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }

    #[test]
    fn tanf_subnormal_returns_x() {
        let x = f32::from_bits(0x0000_0001);
        assert_biteq!(tanf(x), x);
    }

    #[test]
    fn tanf_pi4_direct() {
        let x = core::f32::consts::FRAC_PI_4;
        assert!((tanf(x) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn tanf_3pi4_to_5pi4_range() {
        let x = core::f32::consts::PI;
        assert!(tanf(x).abs() < 1e-4);
        assert_biteq!(tanf(-x), -tanf(x));
    }

    #[test]
    fn tanf_5pi4_to_7pi4_range() {
        let x = 5.0 * core::f32::consts::FRAC_PI_4 + 0.01;
        assert!(tanf(x).is_finite());
        assert_biteq!(tanf(-x), -tanf(x));
    }

    #[test]
    fn tanf_7pi4_to_9pi4_range() {
        let x = 7.0 * core::f32::consts::FRAC_PI_4;
        assert!(tanf(x).is_finite());
        assert_biteq!(tanf(-x), -tanf(x));
    }

    #[test]
    fn tanf_9pi4_range() {
        let x = 9.0 * core::f32::consts::FRAC_PI_4;
        assert!((tanf(x) - 1.0).abs() < 0.01);
        assert_biteq!(tanf(-x), -tanf(x));
    }

    #[test]
    fn tanf_subnormal_underflow_path() {
        let x = f32::from_bits(0x0000_0001);
        assert_biteq!(tanf(x), x);
    }

    #[test]
    fn tanf_tiny_nonsubnormal_inexact_path() {
        let x = f32::from_bits(0x30000000); // ~4.6e-10, non-subnormal, < 2^-12
        assert_biteq!(tanf(x), x);
    }

    #[test]
    fn tanf_pi4_direct_k_tanf() {
        let x = core::f32::consts::FRAC_PI_4 * 0.99;
        assert!((tanf(x) - 0.99).abs() < 0.05);
        assert_biteq!(tanf(-x), -tanf(x));
    }
}
