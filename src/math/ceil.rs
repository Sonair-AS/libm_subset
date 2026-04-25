/// Ceil (f32)
///
/// Finds the nearest integer greater than or equal to `x`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn ceilf(x: f32) -> f32 {
    select_implementation! {
        name: ceilf,
        args: x,
    }

    super::generic::ceil(x)
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`ceilf`] (wrapper around `math::generic::ceil`).

    use super::*;

    #[test]
    fn ceilf_sanity() {
        assert_biteq!(ceilf(1.1f32), 2.0f32);
        assert_biteq!(ceilf(2.9f32), 3.0f32);
        assert_biteq!(ceilf(-1.1f32), -1.0f32);
    }

    #[test]
    fn ceilf_preserves_signed_zero() {
        assert_biteq!(ceilf(0.0f32), 0.0f32);
        assert_biteq!(ceilf(-0.0f32), -0.0f32);
    }

    #[test]
    fn ceilf_negative_fraction_ceil_toward_zero() {
        assert_biteq!(ceilf(-0.1f32), -0.0f32);
        assert_biteq!(ceilf(-0.9f32), -0.0f32);
    }

    #[test]
    fn ceilf_nan_and_infinity() {
        assert!(ceilf(f32::NAN).is_nan());
        assert_biteq!(ceilf(f32::INFINITY), f32::INFINITY);
        assert_biteq!(ceilf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    }

    #[test]
    fn ceilf_conformance_bit_exact() {
        let cases = [
            (0x3fb33333_u32, 0x40000000_u32), // ceil(1.4) = 2.0
            (0xbf99999a_u32, 0xbf800000_u32), // ceil(-1.2) = -1.0
            (0x3f8ccccd_u32, 0x40000000_u32), // ceil(1.1) = 2.0
        ];
        for &(xb, yb) in &cases {
            assert_biteq!(ceilf(f32::from_bits(xb)), f32::from_bits(yb));
        }
    }
}
