/* SPDX-License-Identifier: MIT */
/* origin: musl src/math/ceilf.c */

//! Generic `ceil` algorithm.
//!
//! Note that this uses the algorithm from musl's `ceilf` rather than `ceil` or `ceill` because
//! performance seems to be better (based on icount) and it does not seem to experience rounding
//! errors on i386.

use crate::support::{Float, FpResult, Int, IntTy, MinInt, Status};

#[inline]
pub fn ceil<F: Float>(x: F) -> F {
    ceil_status(x).val
}

#[inline]
pub fn ceil_status<F: Float>(x: F) -> FpResult<F> {
    let zero = IntTy::<F>::ZERO;

    let mut ix = x.to_bits();
    let e = x.exp_unbiased();

    // If the represented value has no fractional part, no truncation is needed.
    if e >= F::SIG_BITS as i32 {
        return FpResult::ok(x);
    }

    let status;
    let res = if e >= 0 {
        // |x| >= 1.0
        let m = F::SIG_MASK >> e.unsigned();
        if (ix & m) == zero {
            // Portion to be masked is already zero; no adjustment needed.
            return FpResult::ok(x);
        }

        // Otherwise, raise an inexact exception.
        status = Status::INEXACT;

        if x.is_sign_positive() {
            ix += m;
        }

        ix &= !m;
        F::from_bits(ix)
    } else {
        // |x| < 1.0, raise an inexact exception since truncation will happen (unless x == 0).
        if ix & F::SIG_MASK == F::Int::ZERO {
            status = Status::OK;
        } else {
            status = Status::INEXACT;
        }

        if x.is_sign_negative() {
            // -1.0 < x <= -0.0; rounding up goes toward -0.0.
            F::NEG_ZERO
        } else if ix << 1 != zero {
            // 0.0 < x < 1.0; rounding up goes toward +1.0.
            F::ONE
        } else {
            // +0.0 remains unchanged
            x
        }
    };

    FpResult::new(res, status)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test against https://en.cppreference.com/w/cpp/numeric/math/ceil
    fn spec_test<F: Float>(cases: &[(F, F, Status)]) {
        let roundtrip = [
            F::ZERO,
            F::ONE,
            F::NEG_ONE,
            F::NEG_ZERO,
            F::INFINITY,
            F::NEG_INFINITY,
        ];

        for x in roundtrip {
            let FpResult { val, status } = ceil_status(x);
            assert_biteq!(val, x);
            assert!(status == Status::OK);
        }

        for &(x, res, res_stat) in cases {
            let FpResult { val, status } = ceil_status(x);
            assert_biteq!(val, res);
            assert!(status == res_stat);
        }
    }

    /* Skipping f16 / f128 "sanity_check"s due to rejected literal lexing at MSRV */

    #[test]
    fn sanity_check_f32() {
        assert_eq!(ceil(1.1f32), 2.0);
        assert_eq!(ceil(2.9f32), 3.0);
    }

    #[test]
    fn spec_tests_f32() {
        let cases = [
            (0.1, 1.0, Status::INEXACT),
            (-0.1, -0.0, Status::INEXACT),
            (0.9, 1.0, Status::INEXACT),
            (-0.9, -0.0, Status::INEXACT),
            (1.1, 2.0, Status::INEXACT),
            (-1.1, -1.0, Status::INEXACT),
            (1.9, 2.0, Status::INEXACT),
            (-1.9, -1.0, Status::INEXACT),
        ];
        spec_test::<f32>(&cases);
    }

    #[test]
    fn sanity_check_f64() {
        assert_eq!(ceil(1.1f64), 2.0);
        assert_eq!(ceil(2.9f64), 3.0);
    }

    #[test]
    fn spec_tests_f64() {
        let cases = [
            (0.1, 1.0, Status::INEXACT),
            (-0.1, -0.0, Status::INEXACT),
            (0.9, 1.0, Status::INEXACT),
            (-0.9, -0.0, Status::INEXACT),
            (1.1, 2.0, Status::INEXACT),
            (-1.1, -1.0, Status::INEXACT),
            (1.9, 2.0, Status::INEXACT),
            (-1.9, -1.0, Status::INEXACT),
        ];
        spec_test::<f64>(&cases);
    }

    #[test]
    fn ceil_nan_preserved_f32() {
        assert!(ceil(f32::NAN).is_nan());
    }

    #[test]
    fn ceil_nan_preserved_f64() {
        assert!(ceil(f64::NAN).is_nan());
    }

    #[test]
    fn ceil_infinity_unchanged_f32() {
        assert_biteq!(ceil(f32::INFINITY), f32::INFINITY);
        assert_biteq!(ceil(f32::NEG_INFINITY), f32::NEG_INFINITY);
    }

    #[test]
    fn ceil_infinity_unchanged_f64() {
        assert_biteq!(ceil(f64::INFINITY), f64::INFINITY);
        assert_biteq!(ceil(f64::NEG_INFINITY), f64::NEG_INFINITY);
    }

    #[test]
    fn ceil_subnormal_fraction_f32_ceil_toward_one() {
        let x = f32::from_bits(0x000116c2);
        assert_biteq!(ceil(x), 1.0f32);
        let FpResult { val, status } = ceil_status(x);
        assert_biteq!(val, 1.0f32);
        assert!(status == Status::INEXACT);
    }

    /// Bit-exact samples (in/out IEEE-754 bits).
    #[test]
    fn conformance_bit_exact_f32() {
        let cases = [
            (0x3fb33333_u32, 0x40000000_u32), // ceil(1.4) = 2.0
            (0xbf99999a_u32, 0xbf800000_u32), // ceil(-1.2) = -1.0
            (0x3f8ccccd_u32, 0x40000000_u32), // ceil(1.1) = 2.0
        ];
        for &(xb, yb) in &cases {
            assert_biteq!(ceil(f32::from_bits(xb)), f32::from_bits(yb));
        }
    }

    #[test]
    fn conformance_bit_exact_f64() {
        let cases = [
            (0x3ff199999999999a_u64, 0x4000000000000000_u64), // ceil(1.1) = 2.0
            (0xbff3333333333333_u64, 0xbff0000000000000_u64), // ceil(-1.2) = -1.0
        ];
        for &(xb, yb) in &cases {
            assert_biteq!(ceil(f64::from_bits(xb)), f64::from_bits(yb));
        }
    }
}
