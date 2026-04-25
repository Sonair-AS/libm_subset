/* origin: FreeBSD /usr/src/lib/msun/src/e_atan2f.c */
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

/* atanf implementation (from s_atanf.c), kept private for atan2f only */

use super::fabsf;

const PI: f32 = 3.1415927410e+00; /* 0x40490fdb */
const PI_LO: f32 = -8.7422776573e-08; /* 0xb3bbbd2e */

const ATAN_HI: [f32; 4] = [
    4.6364760399e-01, /* atan(0.5)hi 0x3eed6338 */
    7.8539812565e-01, /* atan(1.0)hi 0x3f490fda */
    9.8279368877e-01, /* atan(1.5)hi 0x3f7b985e */
    1.5707962513e+00, /* atan(inf)hi 0x3fc90fda */
];

const ATAN_LO: [f32; 4] = [
    5.0121582440e-09, /* atan(0.5)lo 0x31ac3769 */
    3.7748947079e-08, /* atan(1.0)lo 0x33222168 */
    3.4473217170e-08, /* atan(1.5)lo 0x33140fb4 */
    7.5497894159e-08, /* atan(inf)lo 0x33a22168 */
];

const A_T: [f32; 5] = [
    3.3333328366e-01,
    -1.9999158382e-01,
    1.4253635705e-01,
    -1.0648017377e-01,
    6.1687607318e-02,
];

#[cfg_attr(assert_no_panic, no_panic::no_panic)]
fn atan_for_atan2f(mut x: f32) -> f32 {
    let x1p_120 = f32::from_bits(0x03800000); // 0x1p-120 === 2 ^ (-120)

    let z: f32;

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    if ix >= 0x4c800000 {
        /* if |x| >= 2**26 */
        if x.is_nan() {
            return x;
        }
        z = i!(ATAN_HI, 3) + x1p_120;
        return if sign { -z } else { z };
    }
    let id = if ix < 0x3ee00000 {
        /* |x| < 0.4375 */
        if ix < 0x39800000 {
            /* |x| < 2**-12 */
            if ix < 0x00800000 {
                /* raise underflow for subnormal x */
                force_eval!(x * x);
            }
            return x;
        }
        -1
    } else {
        x = fabsf(x);
        if ix < 0x3f980000 {
            /* |x| < 1.1875 */
            if ix < 0x3f300000 {
                /*  7/16 <= |x| < 11/16 */
                x = (2. * x - 1.) / (2. + x);
                0
            } else {
                /* 11/16 <= |x| < 19/16 */
                x = (x - 1.) / (x + 1.);
                1
            }
        } else if ix < 0x401c0000 {
            /* |x| < 2.4375 */
            x = (x - 1.5) / (1. + 1.5 * x);
            2
        } else {
            /* 2.4375 <= |x| < 2**26 */
            x = -1. / x;
            3
        }
    };
    /* end of argument reduction */
    z = x * x;
    let w = z * z;
    /* break sum from i=0 to 10 aT[i]z**(i+1) into odd and even poly */
    let s1 = z * (i!(A_T, 0) + w * (i!(A_T, 2) + w * i!(A_T, 4)));
    let s2 = w * (i!(A_T, 1) + w * i!(A_T, 3));
    if id < 0 {
        return x - x * (s1 + s2);
    }
    let id = id as usize;
    let z = i!(ATAN_HI, id) - ((x * (s1 + s2) - i!(ATAN_LO, id)) - x);
    if sign {
        -z
    } else {
        z
    }
}

/// Arctangent of y/x (f32)
///
/// Computes the inverse tangent (arc tangent) of `y/x`.
/// Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
/// Returns a value in radians, in the range of -pi to pi.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn atan2f(y: f32, x: f32) -> f32 {
    if x.is_nan() || y.is_nan() {
        return x + y;
    }
    let mut ix = x.to_bits();
    let mut iy = y.to_bits();

    if ix == 0x3f800000 {
        /* x=1.0 */
        return atan_for_atan2f(y);
    }
    let m = ((iy >> 31) & 1) | ((ix >> 30) & 2); /* 2*sign(x)+sign(y) */
    ix &= 0x7fffffff;
    iy &= 0x7fffffff;

    /* when y = 0 */
    if iy == 0 {
        return match m {
            0 | 1 => y, /* atan(+-0,+anything)=+-0 */
            2 => PI,    /* atan(+0,-anything) = pi */
            _ => -PI,   /* atan(-0,-anything) =-pi */
        };
    }
    /* when x = 0 */
    if ix == 0 {
        return if m & 1 != 0 { -PI / 2. } else { PI / 2. };
    }
    /* when x is INF */
    if ix == 0x7f800000 {
        return if iy == 0x7f800000 {
            match m {
                0 => PI / 4.,       /* atan(+INF,+INF) */
                1 => -PI / 4.,      /* atan(-INF,+INF) */
                2 => 3. * PI / 4.,  /* atan(+INF,-INF)*/
                _ => -3. * PI / 4., /* atan(-INF,-INF)*/
            }
        } else {
            match m {
                0 => 0.,  /* atan(+...,+INF) */
                1 => -0., /* atan(-...,+INF) */
                2 => PI,  /* atan(+...,-INF) */
                _ => -PI, /* atan(-...,-INF) */
            }
        };
    }
    /* |y/x| > 0x1p26 */
    if (ix + (26 << 23) < iy) || (iy == 0x7f800000) {
        return if m & 1 != 0 { -PI / 2. } else { PI / 2. };
    }

    /* z = atan(|y/x|) with correct underflow */
    let z = if (m & 2 != 0) && (iy + (26 << 23) < ix) {
        /*|y/x| < 0x1p-26, x < 0 */
        0.
    } else {
        atan_for_atan2f(fabsf(y / x))
    };
    match m {
        0 => z,                /* atan(+,+) */
        1 => -z,               /* atan(-,+) */
        2 => PI - (z - PI_LO), /* atan(+,-) */
        _ => (z - PI_LO) - PI, /* case 3 */ /* atan(-,-) */
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`atan2f`].

    use super::*;

    #[test]
    fn atan2f_nan_inputs() {
        let nan = f32::NAN;
        assert!(atan2f(nan, 1.0).is_nan());
        assert!(atan2f(1.0, nan).is_nan());
        assert!(atan2f(nan, nan).is_nan());
    }

    /// `atan2(±1, ±∞)`, `atan2(±∞, 1)`, and `atan2(±∞, ±∞)`; π/2 matches the internal atan(inf) hi constant.
    #[test]
    fn atan2f_infinity_and_mixed() {
        let cases = [
            (0x3f800000_u32, 0x7f800000_u32, 0x0_u32), // atan2(1,+inf)->+0
            (0xbf800000_u32, 0x7f800000_u32, 0x80000000_u32), // atan2(-1,+inf)->-0
            (0x3f800000_u32, 0xff800000_u32, 0x40490fdb_u32), // atan2(1,-inf)->π
            (0xbf800000_u32, 0xff800000_u32, 0xc0490fdb_u32), // atan2(-1,-inf)->-π
            (0x7f800000_u32, 0x3f800000_u32, 0x3fc90fda_u32), // atan2(+inf,1)->π/2
            (0xff800000_u32, 0x3f800000_u32, 0xbfc90fda_u32), // atan2(-inf,1)->-π/2
            (0x7f800000_u32, 0xff800000_u32, 0x4016cbe4_u32), // atan2(+inf,-inf)->3π/4
            (0xff800000_u32, 0xff800000_u32, 0xc016cbe4_u32), // atan2(-inf,-inf)->-3π/4
        ];
        for (yb, xb, rb) in cases {
            assert_biteq!(
                atan2f(f32::from_bits(yb), f32::from_bits(xb)),
                f32::from_bits(rb)
            );
        }
    }

    #[test]
    fn atan2f_signed_zero_quadrants() {
        assert_biteq!(atan2f(0.0, 1.0), 0.0);
        assert_biteq!(atan2f(-0.0, 1.0), -0.0);
        assert_biteq!(atan2f(0.0, -1.0), f32::from_bits(0x40490fdb));
        assert_biteq!(atan2f(-0.0, -1.0), f32::from_bits(0xc0490fdb));
    }

    #[test]
    fn atan2f_conformance_bit_exact() {
        let cases = [
            (0x3f800000_u32, 0x3f800000_u32, 0x3f490fdb_u32), // atan2(1,1)
            (0x3f800000_u32, 0x0_u32, 0x3fc90fdb_u32),        // atan2(1,0) = pi/2
            (0xbf800000_u32, 0x0_u32, 0xbfc90fdb_u32),        // atan2(-1,0)
            (0x40400000_u32, 0x40800000_u32, 0x3f24bc7d_u32), // atan2(3,4)
            (0x7f800000_u32, 0x7f800000_u32, 0x3f490fdb_u32), // atan2(+inf,+inf)
            (0xff800000_u32, 0x7f800000_u32, 0xbf490fdb_u32), // atan2(-inf,+inf)
        ];

        for (yb, xb, rb) in cases {
            assert_biteq!(
                atan2f(f32::from_bits(yb), f32::from_bits(xb)),
                f32::from_bits(rb)
            );
        }
    }
}
