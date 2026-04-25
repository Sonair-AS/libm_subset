/* origin: FreeBSD /usr/src/lib/msun/src/s_cosf.c */
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
const C1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x3FF921FB, 0x54442D18 */
const C2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x400921FB, 0x54442D18 */
const C3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x4012D97C, 0x7F3321D2 */
const C4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x401921FB, 0x54442D18 */

/// The cosine of `x` (f32).
///
/// `x` is specified in radians.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn cosf(x: f32) -> f32 {
    let x64 = x as f64;

    let x1p120 = f32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    if ix <= 0x3f490fda {
        /* |x| ~<= pi/4 */
        if ix < 0x39800000 {
            /* |x| < 2**-12 */
            /* raise inexact if x != 0 */
            force_eval!(x + x1p120);
            return 1.;
        }
        return k_cosf(x64);
    }
    if ix <= 0x407b53d1 {
        /* |x| ~<= 5*pi/4 */
        if ix > 0x4016cbe3 {
            /* |x|  ~> 3*pi/4 */
            return -k_cosf(if sign { x64 + C2_PIO2 } else { x64 - C2_PIO2 });
        } else if sign {
            return k_sinf(x64 + C1_PIO2);
        } else {
            return k_sinf(C1_PIO2 - x64);
        }
    }
    if ix <= 0x40e231d5 {
        /* |x| ~<= 9*pi/4 */
        if ix > 0x40afeddf {
            /* |x| ~> 7*pi/4 */
            return k_cosf(if sign { x64 + C4_PIO2 } else { x64 - C4_PIO2 });
        } else if sign {
            return k_sinf(-x64 - C3_PIO2);
        } else {
            return k_sinf(x64 - C3_PIO2);
        }
    }

    /* cos(Inf or NaN) is NaN */
    if ix >= 0x7f800000 {
        return x - x;
    }

    /* general argument reduction needed */
    let (n, y) = rem_pio2f(x);
    match n & 3 {
        0 => k_cosf(y),
        1 => k_sinf(-y),
        2 => -k_cosf(y),
        _ => k_sinf(y),
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`cosf`].

    use super::*;

    #[test]
    fn cosf_zero_returns_one() {
        assert_biteq!(cosf(0.0), 1.0);
        assert_biteq!(cosf(-0.0), 1.0);
    }

    #[test]
    fn cosf_preserves_even_symmetry() {
        let x = f32::from_bits(0x3f490fdb); // pi/4
        assert_biteq!(cosf(-x), cosf(x));
    }

    #[test]
    fn cosf_tiny_returns_one() {
        let x = f32::from_bits(0x000116c2);
        assert_biteq!(cosf(x), 1.0);
    }

    #[test]
    fn cosf_nan_and_infinity() {
        assert!(cosf(f32::NAN).is_nan());
        assert!(cosf(f32::INFINITY).is_nan());
        assert!(cosf(f32::NEG_INFINITY).is_nan());
    }

    /// Large |x| forces the `rem_pio2f` path in `cosf` (not the early `k_cosf` / `k_sinf` ranges).
    #[test]
    fn rem_pio2f_path_variants() {
        let p1000 = f32::from_bits(0x4544597c);
        assert_biteq!(cosf(p1000), f32::from_bits(0x3f800000));
        let p1000_neg = f32::from_bits(0xc544597c);
        assert_biteq!(cosf(p1000_neg), f32::from_bits(0x3f800000));
        let p500 = f32::from_bits(0x44c4597c);
        assert_biteq!(cosf(p500), f32::from_bits(0x3f800000));
    }

    #[test]
    fn cosf_conformance_bit_exact() {
        let cases = [
            (0x3f800000_u32, 0x3f0a5140_u32), // cos(1)
            (0x3f490fdb_u32, 0x3f3504f3_u32), // cos(pi/4)
            (0x3fc90fdb_u32, 0xb33bbd2e_u32), // cos(pi/2) (cancellation)
            (0x4544597c_u32, 0x3f800000_u32), // large arg
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(cosf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }

    #[test]
    fn cosf_subnormal_returns_one() {
        let x = f32::from_bits(0x0000_0001);
        assert_biteq!(cosf(x), 1.0);
    }

    #[test]
    fn cosf_3pi4_to_5pi4_range() {
        let x = core::f32::consts::PI;
        assert!((cosf(x) - (-1.0)).abs() < 1e-5);
        assert!((cosf(-x) - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn cosf_5pi4_sign_paths() {
        let x = 5.0 * core::f32::consts::FRAC_PI_4;
        assert_biteq!(cosf(x), cosf(-x));
        assert!(cosf(x).abs() <= 1.0);
    }

    #[test]
    fn cosf_7pi4_to_9pi4_range() {
        let x = 7.0 * core::f32::consts::FRAC_PI_4;
        let neg_x = -x;
        assert!((cosf(x) - cosf(neg_x)).abs() < 1e-6);
    }

    #[test]
    fn cosf_near_9pi4() {
        let x = 9.0 * core::f32::consts::FRAC_PI_4;
        let neg_x = -x;
        assert!((cosf(x) - cosf(neg_x)).abs() < 1e-6);
    }

    #[test]
    fn cosf_3pi4_region_signed() {
        let x = 3.0 * core::f32::consts::FRAC_PI_4;
        assert_biteq!(cosf(x), cosf(-x));
        assert!(cosf(x) < 0.0);
    }

    #[test]
    fn cosf_rem_pio2f_all_arms() {
        use super::rem_pio2f;
        let cases: &[(u32, i32)] = &[
            (0x413c7edd, 0),
            (0x40e231d6, 1),
            (0x410a3ae7, 2),
            (0x41235ce2, 3),
        ];
        for &(xb, want_mod) in cases {
            let x = f32::from_bits(xb);
            let (n, _y) = rem_pio2f(x);
            assert_eq!(n & 3, want_mod, "for x=0x{xb:08x}");
            let r = cosf(x);
            assert!(r.abs() <= 1.0, "cosf(0x{xb:08x}) = {r}, out of [-1, 1]");
            assert_biteq!(cosf(-x), cosf(x));
        }
    }

    #[test]
    fn cosf_subnormal_path() {
        let x = f32::from_bits(0x0000_0001);
        assert_biteq!(cosf(x), 1.0);
        assert_biteq!(cosf(-x), 1.0);
    }

    #[test]
    fn cosf_tiny_nonsubnormal_returns_one() {
        let x = f32::from_bits(0x30000000); // ~4.6e-10, non-subnormal, < 2^-12
        assert_biteq!(cosf(x), 1.0);
    }

    #[test]
    fn cosf_inside_pi4() {
        let x = 0.5f32;
        let r = cosf(x);
        assert!((r - 0.87758).abs() < 0.001);
    }
}
