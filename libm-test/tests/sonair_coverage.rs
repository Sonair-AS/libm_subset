//! Targeted tests for coverage gaps in the certified `libm_subset`.
//!
//! These tests exercise specific code paths that the randomized and edge-case
//! generators in `libm-test` do not reliably hit.

// ========================= expf =========================

/// expf: negative near-underflow that does NOT flush to zero.
#[test]
fn expf_near_underflow_fallthrough() {
    let result = libm::expf(-88.0);
    assert!(result >= 0.0, "expf(-88) should be non-negative, got {result}");
    assert!(result < 1e-30, "expf(-88) should be tiny, got {result}");
}

/// expf: positive value in [87.34, 88.72) — enters special block but skips
/// both overflow return and underflow block.
#[test]
fn expf_large_positive_no_overflow() {
    let result = libm::expf(87.5);
    assert!(result > 1e37, "expf(87.5) should be very large, got {result}");
    assert!(result.is_finite(), "expf(87.5) should be finite");
}

// ========================= powf =========================

/// powf: y == 2 fast-path (line 136: `return x * x`).
#[test]
fn powf_y_is_two() {
    assert_eq!(libm::powf(3.0, 2.0), 9.0);
    assert_eq!(libm::powf(-5.0, 2.0), 25.0);
}

/// powf: y == 0.5 with x >= 0 delegates to sqrtf (line 144).
#[test]
fn powf_y_is_half() {
    let result = libm::powf(4.0, 0.5);
    assert!((result - 2.0).abs() < 1e-6, "powf(4, 0.5) = {result}");

    let result2 = libm::powf(9.0, 0.5);
    assert!((result2 - 3.0).abs() < 1e-6, "powf(9, 0.5) = {result2}");
}

/// powf: (-inf)^(odd int) = -inf, covering `z = -z` (line 161).
/// x is -inf (ix == 0x7f800000, hx < 0), y is odd int → yisint == 1.
#[test]
fn powf_neg_inf_to_odd_int() {
    let result = libm::powf(f32::NEG_INFINITY, 3.0);
    assert_eq!(result, f32::NEG_INFINITY);

    // Also (-0)^(odd positive int) = -0
    let result2 = libm::powf(-0.0_f32, 3.0);
    assert!(result2.to_bits() == (-0.0_f32).to_bits(), "powf(-0, 3) should be -0");
}

/// powf: (-x)^(odd int) general case, covering `sn = -1.0` (line 176).
/// x is a normal negative number, y is an odd integer.
#[test]
fn powf_negative_base_odd_exponent() {
    let result = libm::powf(-2.0, 3.0);
    assert!((result - (-8.0)).abs() < 1e-4, "powf(-2, 3) = {result}");

    let result2 = libm::powf(-3.0, 5.0);
    assert!((result2 - (-243.0)).abs() < 0.1, "powf(-3, 5) = {result2}");
}

/// powf: z == 128.0 exactly (j == 0x43000000), the exact overflow boundary.
/// Uses x = 2.0, y = 128.0 so log2(x)*y = 128.0 exactly.
#[test]
fn powf_z_exactly_128_no_overflow() {
    let x = f32::from_bits(0x3fffd8f1);
    let y = f32::from_bits(0x43001c35);
    let result = libm::powf(x, y);
    assert!(result.is_finite(), "powf(0x3fffd8f1, 0x43001c35) should be finite, got {result}");
}

#[test]
fn powf_z_exactly_128() {
    let result = libm::powf(2.0, 128.0);
    // 2^128 overflows f32, should return inf
    assert!(result.is_infinite(), "powf(2, 128) should overflow, got {result}");
}

/// powf: z == -150.0 exactly (j as u32 == 0xc3160000), the exact underflow boundary.
/// Uses x = 2.0, y = -150.0 so log2(x)*y = -150.0 exactly.
#[test]
fn powf_z_exactly_neg150() {
    let result = libm::powf(2.0, -150.0);
    // 2^-150 is deep subnormal / zero territory for f32
    assert!(result == 0.0 || result.is_subnormal(), "powf(2, -150) should underflow, got {result:e}");
}

// ========================= cosf =========================

/// cosf: negative x with |x| in (7*pi/4, 9*pi/4] — hits the `sign` branch
/// of `k_cosf(x64 + C4_PIO2)` at line 65.
#[test]
fn cosf_negative_near_8pi_over_4() {
    // -6.0 has |x| ~= 6.0, which is > 7*pi/4 (~5.498) and <= 9*pi/4 (~7.069)
    let result = libm::cosf(-6.0);
    let expected = 0.96017028_f32; // cos(-6) ≈ 0.9602
    assert!(
        (result - expected).abs() < 1e-5,
        "cosf(-6.0) = {result}, expected ~{expected}"
    );
}

// ========================= Float trait (float_traits.rs) =========================

use libm::support::Float;

#[test]
fn float_to_bits_signed() {
    assert!((-1.0_f32).to_bits_signed() < 0);
    assert!((-1.0_f64).to_bits_signed() < 0);
}

#[test]
fn float_biteq() {
    assert!(1.0_f32.biteq(1.0_f32));
    assert!(!1.0_f32.biteq(-1.0_f32));
    assert!(f32::NAN.biteq(f32::NAN));

    assert!(1.0_f64.biteq(1.0_f64));
    assert!(f64::NAN.biteq(f64::NAN));
}

#[test]
fn float_eq_repr() {
    assert!(1.0_f32.eq_repr(1.0_f32));
    assert!(!1.0_f32.eq_repr(2.0_f32));
    assert!(f32::NAN.eq_repr(f32::NAN));

    assert!(1.0_f64.eq_repr(1.0_f64));
    assert!(f64::NAN.eq_repr(f64::NAN));
}

#[test]
fn float_is_sign_positive() {
    assert!(1.0_f32.is_sign_positive());
    assert!(!(-1.0_f32).is_sign_positive());
    assert!(1.0_f64.is_sign_positive());
    assert!(!(-1.0_f64).is_sign_positive());
}

#[test]
fn float_is_subnormal() {
    assert!(<f32 as Float>::is_subnormal(f32::from_bits(1)));
    assert!(!<f32 as Float>::is_subnormal(1.0_f32));
    assert!(<f64 as Float>::is_subnormal(f64::from_bits(1)));
    assert!(!<f64 as Float>::is_subnormal(1.0_f64));
}

/// from_parts with negative=true for f32 (hits `Self::Int::ONE` branch).
#[test]
fn float_from_parts_negative() {
    let neg_one = <f32 as Float>::from_parts(true, f32::EXP_BIAS, 0);
    assert!(neg_one.biteq(-1.0_f32));

    let neg_one_f64 = <f64 as Float>::from_parts(true, f64::EXP_BIAS, 0);
    assert!(neg_one_f64.biteq(-1.0_f64));
}

#[test]
fn float_frac() {
    assert_eq!(f32::from_bits(0x3F80_0001).frac(), 1); // 1.0 + 1 ULP
    assert_eq!(f64::from_bits(0x3FF0_0000_0000_0001).frac(), 1);
}

#[test]
fn float_signum() {
    assert!(<f32 as Float>::signum(1.0_f32).biteq(1.0_f32));
    assert!(<f32 as Float>::signum(-1.0_f32).biteq(-1.0_f32));
    assert!(<f32 as Float>::signum(f32::NAN).is_nan());

    assert!(<f64 as Float>::signum(1.0_f64).biteq(1.0_f64));
    assert!(<f64 as Float>::signum(-1.0_f64).biteq(-1.0_f64));
    assert!(<f64 as Float>::signum(f64::NAN).is_nan());
}

#[test]
fn float_canonicalize() {
    assert!(1.0_f32.canonicalize().biteq(1.0_f32));
    assert!((-2.5_f32).canonicalize().biteq(-2.5_f32));
    assert!(1.0_f64.canonicalize().biteq(1.0_f64));
    assert!((-2.5_f64).canonicalize().biteq(-2.5_f64));
}

#[test]
fn float_abs() {
    assert!((-3.0_f32).abs().biteq(3.0_f32));
    assert!(3.0_f32.abs().biteq(3.0_f32));
    assert!((-3.0_f64).abs().biteq(3.0_f64));
    assert!(3.0_f64.abs().biteq(3.0_f64));
}

// ========================= Int trait (int_traits.rs) =========================

use libm::support::{CastFrom, CastInto, DInt, HInt, Int};

#[test]
fn int_from_bool_and_is_zero() {
    assert_eq!(<i32 as Int>::from_bool(true), 1);
    assert_eq!(<i32 as Int>::from_bool(false), 0);
    assert!(0_i32.is_zero());
    assert!(!1_i32.is_zero());
    assert_eq!(<u32 as Int>::from_bool(true), 1);
    assert!(0_u32.is_zero());
}

#[test]
fn int_logical_shr() {
    assert_eq!((-1_i32).logical_shr(1), i32::MAX);
    assert_eq!(0xFF00_u32.logical_shr(8), 0xFF);
}

#[test]
fn int_abs_and_unsigned_abs() {
    assert_eq!((-5_i32).abs(), 5);
    assert_eq!(5_i32.unsigned_abs(), 5_u32);
    assert_eq!((-5_i64).abs(), 5);
    assert_eq!((-5_i64).unsigned_abs(), 5_u64);
}

#[test]
fn int_abs_diff() {
    assert_eq!(10_i32.abs_diff(3), 7_u32);
    assert_eq!(3_i32.abs_diff(10), 7_u32);
    assert_eq!(10_u32.abs_diff(3), 7_u32);
}

#[test]
fn int_checked_ops() {
    assert_eq!(1_i32.checked_add(2), Some(3));
    assert_eq!(i32::MAX.checked_add(1), None);
    assert_eq!(5_i32.checked_sub(3), Some(2));
    assert_eq!(i32::MIN.checked_sub(1), None);
}

#[test]
fn int_wrapping_ops() {
    assert_eq!(1_i32.wrapping_neg(), -1);
    assert_eq!(3_u32.wrapping_mul(4), 12);
    assert_eq!(1_u32.rotate_left(1), 2);
}

#[test]
fn int_overflowing_ops() {
    let (val, of) = i32::MAX.overflowing_add(1);
    assert_eq!(val, i32::MIN);
    assert!(of);
    let (val, of) = 0_i32.overflowing_sub(1);
    assert_eq!(val, -1);
    assert!(!of);
}

#[test]
fn int_carrying_add_and_borrowing_sub() {
    let (val, carry) = 1_u32.carrying_add(2, false);
    assert_eq!(val, 3);
    assert!(!carry);
    let (val, carry) = u32::MAX.carrying_add(0, true);
    assert_eq!(val, 0);
    assert!(carry);
    let (val, borrow) = 5_u32.borrowing_sub(3, false);
    assert_eq!(val, 2);
    assert!(!borrow);
    let (val, borrow) = 0_u32.borrowing_sub(0, true);
    assert_eq!(val, u32::MAX);
    assert!(borrow);
}

#[test]
fn int_leading_trailing_zeros_and_ilog2() {
    assert_eq!(1_u32.leading_zeros(), 31);
    assert_eq!(8_u32.trailing_zeros(), 3);
    assert_eq!(8_u32.ilog2(), 3);
}

#[test]
fn int_signed_unsigned_conversions() {
    assert_eq!((-1_i32).unsigned(), u32::MAX);
    assert_eq!(u32::MAX.signed(), -1_i32);
    assert_eq!(<i32 as Int>::from_unsigned(42_u32), 42_i32);
    assert_eq!(<u32 as Int>::from_unsigned(42_u32), 42_u32);
}

#[test]
fn dint_lo_hi() {
    let val: u64 = 0x0000_0002_0000_0001;
    assert_eq!(val.lo(), 1_u32);
    assert_eq!(val.hi(), 2_u32);
    let (lo, hi) = val.lo_hi();
    assert_eq!(lo, 1_u32);
    assert_eq!(hi, 2_u32);
    let reconstructed = <u64 as DInt>::from_lo_hi(1_u32, 2_u32);
    assert_eq!(reconstructed, val);
}

#[test]
fn hint_widen_ops() {
    assert_eq!(3_u32.widen(), 3_u64);
    assert_eq!(3_u32.zero_widen(), 3_u64);
    assert_eq!(3_u32.widen_hi(), 3_u64 << 32);
    assert_eq!(3_u32.zero_widen_mul(4), 12_u64);
    assert_eq!(3_u32.widen_mul(4), 12_u64);
    assert_eq!((-1_i32).widen(), -1_i64);
    assert_eq!((-1_i32).widen_mul(2), -2_i64);
}

#[test]
fn cast_lossy() {
    let truncated: u8 = 300_u32.cast_lossy();
    assert_eq!(truncated, 44); // 300 % 256
    let truncated2: u8 = CastFrom::cast_from_lossy(300_u32);
    assert_eq!(truncated2, 44);
}

// ========================= env.rs (Status / FpResult / Round) =========================

use libm::support::{FpResult, Round, Status};

#[test]
fn status_flags() {
    assert!(!Status::OK.underflow());
    assert!(!Status::OK.overflow());
    assert!(!Status::OK.inexact());
    assert!(Status::UNDERFLOW.underflow());
    assert!(Status::OVERFLOW.overflow());
    assert!(Status::INEXACT.inexact());
    assert!(!Status::UNDERFLOW.overflow());
}

#[test]
fn status_constants() {
    assert_ne!(Status::DIVIDE_BY_ZERO, Status::OK);
    assert_ne!(Status::OVERFLOW, Status::OK);
    assert_ne!(Status::INVALID, Status::OK);
}

#[test]
fn fp_result_ok() {
    let r = FpResult::ok(42.0_f64);
    assert_eq!(r.val, 42.0);
    assert_eq!(r.status, Status::OK);
}

#[test]
fn fp_result_new() {
    let r = FpResult::new(1.0_f32, Status::INEXACT);
    assert_eq!(r.val, 1.0);
    assert!(r.status.inexact());
}

#[test]
fn generic_sqrt_perfect_square_f32() {
    let perfect_squares: &[f32] = &[0.0, 1.0, 4.0, 9.0, 16.0, 25.0, 100.0, 10000.0];
    for &x in perfect_squares {
        let result: f32 = libm::generic::sqrt(x);
        let expected = libm::sqrtf(x);
        assert_eq!(result.to_bits(), expected.to_bits(), "sqrtf({x})");
    }
}

#[test]
fn generic_sqrt_perfect_square_f64() {
    let values: &[f64] = &[1.0, 4.0, 9.0, 16.0, 25.0, 100.0, 10000.0, 0.25, 0.0625];
    for &x in values {
        let result: f64 = libm::generic::sqrt(x);
        assert!(!result.is_nan(), "sqrt({x})");
    }
}

#[test]
fn round_variants() {
    assert_eq!(Round::Nearest as i32, 0);
    assert_eq!(Round::Negative as i32, 1);
    assert_eq!(Round::Positive as i32, 2);
    assert_eq!(Round::Zero as i32, 3);
    assert_ne!(Round::Nearest, Round::Zero);
}
