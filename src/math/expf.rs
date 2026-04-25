/* origin: FreeBSD /usr/src/lib/msun/src/e_expf.c */
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

use super::generic::scalbn;

const HALF: [f32; 2] = [0.5, -0.5];
const LN2_HI: f32 = 6.9314575195e-01; /* 0x3f317200 */
const LN2_LO: f32 = 1.4286067653e-06; /* 0x35bfbe8e */
const INV_LN2: f32 = 1.4426950216e+00; /* 0x3fb8aa3b */
/*
 * Domain [-0.34568, 0.34568], range ~[-4.278e-9, 4.447e-9]:
 * |x*(exp(x)+1)/(exp(x)-1) - p(x)| < 2**-27.74
 */
const P1: f32 = 1.6666625440e-1; /*  0xaaaa8f.0p-26 */
const P2: f32 = -2.7667332906e-3; /* -0xb55215.0p-32 */

/// Exponential, base *e* (f32)
///
/// Calculate the exponential of `x`, that is, *e* raised to the power `x`
/// (where *e* is the base of the natural system of logarithms, approximately 2.71828).
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn expf(mut x: f32) -> f32 {
    let x1p127 = f32::from_bits(0x7f000000); // 0x1p127f === 2 ^ 127
    let x1p_126 = f32::from_bits(0x800000); // 0x1p-126f === 2 ^ -126  /*original 0x1p-149f    ??????????? */
    let mut hx = x.to_bits();
    let sign = (hx >> 31) as i32; /* sign bit of x */
    let signb: bool = sign != 0;
    hx &= 0x7fffffff; /* high word of |x| */

    /* special cases */
    if hx >= 0x42aeac50 {
        /* if |x| >= -87.33655f or NaN */
        if hx > 0x7f800000 {
            /* NaN */
            return x;
        }
        if (hx >= 0x42b17218) && (!signb) {
            /* x >= 88.722839f */
            /* overflow */
            x *= x1p127;
            return x;
        }
        if signb {
            /* underflow */
            force_eval!(-x1p_126 / x);
            if hx >= 0x42cff1b5 {
                /* x <= -103.972084f */
                return 0.;
            }
        }
    }

    /* argument reduction */
    let k: i32;
    let hi: f32;
    let lo: f32;
    if hx > 0x3eb17218 {
        /* if |x| > 0.5 ln2 */
        if hx > 0x3f851592 {
            /* if |x| > 1.5 ln2 */
            k = (INV_LN2 * x + i!(HALF, sign as usize)) as i32;
        } else {
            k = 1 - sign - sign;
        }
        let kf = k as f32;
        hi = x - kf * LN2_HI; /* k*ln2hi is exact here */
        lo = kf * LN2_LO;
        x = hi - lo;
    } else if hx > 0x39000000 {
        /* |x| > 2**-14 */
        k = 0;
        hi = x;
        lo = 0.;
    } else {
        /* raise inexact */
        force_eval!(x1p127 + x);
        return 1. + x;
    }

    /* x is now in primary range */
    let xx = x * x;
    let c = x - xx * (P1 + xx * P2);
    let y = 1. + (x * c / (2. - c) - lo + hi);
    if k == 0 {
        y
    } else {
        scalbn(y, k)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`expf`].

    use super::*;

    #[test]
    fn expf_zero() {
        assert_biteq!(expf(0.0), 1.0);
    }

    #[test]
    fn expf_nan() {
        let r = expf(f32::NAN);
        assert!(r.is_nan());
    }

    #[test]
    fn expf_conformance_bit_exact() {
        let cases = [
            (0x3f800000_u32, 0x402df854_u32), // exp(1)
            (0xbf800000_u32, 0x3ebc5ab2_u32), // exp(-1)
            (0x42b00000_u32, 0x7ef882b7_u32), // exp(88) finite overflow edge
            (0xc2b00000_u32, 0x0041edc4_u32), // exp(-88) underflow-ish
            (0x1e3ce508_u32, 0x3f800000_u32), // tiny x -> 1+x
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(expf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }
}
