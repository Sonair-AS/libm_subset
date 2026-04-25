/* origin: FreeBSD /usr/src/lib/msun/src/e_logf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
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

const LN2_HI: f32 = 6.9313812256e-01; /* 0x3f317180 */
const LN2_LO: f32 = 9.0580006145e-06; /* 0x3717f7d1 */
/* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95e-11, 4.97e-11]). */
const LG1: f32 = 0.66666662693; /*  0xaaaaaa.0p-24*/
const LG2: f32 = 0.40000972152; /*  0xccce13.0p-25 */
const LG3: f32 = 0.28498786688; /*  0x91e9ee.0p-25 */
const LG4: f32 = 0.24279078841; /*  0xf89e26.0p-26 */

/// The natural logarithm of `x` (f32).
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn logf(mut x: f32) -> f32 {
    let x1p25 = f32::from_bits(0x4c000000); // 0x1p25f === 2 ^ 25

    let mut ix = x.to_bits();
    let mut k = 0i32;

    if (ix < 0x00800000) || ((ix >> 31) != 0) {
        /* x < 2**-126  */
        if ix << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if (ix >> 31) != 0 {
            return (x - x) / 0.; /* log(-#) = NaN */
        }
        /* subnormal number, scale up x */
        k -= 25;
        x *= x1p25;
        ix = x.to_bits();
    } else if ix >= 0x7f800000 {
        return x;
    } else if ix == 0x3f800000 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    ix += 0x3f800000 - 0x3f3504f3;
    k += ((ix >> 23) as i32) - 0x7f;
    ix = (ix & 0x007fffff) + 0x3f3504f3;
    x = f32::from_bits(ix);

    let f = x - 1.;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * LG4);
    let t2 = z * (LG1 + w * LG3);
    let r = t2 + t1;
    let hfsq = 0.5 * f * f;
    let dk = k as f32;
    s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`logf`].

    use super::*;

    #[test]
    fn logf_one_is_zero() {
        assert_biteq!(logf(1.0), 0.0);
    }

    #[test]
    fn logf_zero_is_negative_infinity() {
        assert_biteq!(logf(0.0), f32::NEG_INFINITY);
        assert_biteq!(logf(-0.0), f32::NEG_INFINITY);
    }

    #[test]
    fn logf_negative_is_nan() {
        assert!(logf(-1.0).is_nan());
    }

    #[test]
    fn logf_infinity() {
        assert_biteq!(logf(f32::INFINITY), f32::INFINITY);
    }

    #[test]
    fn logf_nan() {
        assert!(logf(f32::NAN).is_nan());
    }

    #[test]
    fn logf_conformance_log2_and_half() {
        assert_biteq!(logf(f32::from_bits(0x40000000)), f32::from_bits(0x3f317218));
        assert_biteq!(logf(f32::from_bits(0x3f000000)), f32::from_bits(0xbf317218));
    }

    #[test]
    fn logf_conformance_log_e() {
        assert_biteq!(logf(f32::from_bits(0x402df854)), f32::from_bits(0x3f800000));
    }

    #[test]
    fn logf_conformance_tiny_normal() {
        assert_biteq!(logf(f32::from_bits(0x0da24260)), f32::from_bits(0xc28a27b5));
    }

    #[test]
    fn logf_subnormal_input() {
        let subnorm = f32::from_bits(0x0000_0001);
        let result = logf(subnorm);
        assert!(result < -100.0);
        assert!(result.is_finite());
    }

    #[test]
    fn logf_subnormal_various() {
        let subnorm2 = f32::from_bits(0x007F_FFFF);
        let result = logf(subnorm2);
        assert!(result < 0.0);
        assert!(result.is_finite());
    }
}
