/// Floor (f64)
///
/// Finds the nearest integer less than or equal to `x`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn floor(x: f64) -> f64 {
    select_implementation! {
        name: floor,
        use_arch_required: all(target_arch = "x86", not(target_feature = "sse2")),
        args: x,
    }

    return super::generic::floor(x);
}

/// Floor (f32)
///
/// Finds the nearest integer less than or equal to `x`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn floorf(x: f32) -> f32 {
    select_implementation! {
        name: floorf,
        args: x,
    }

    return super::generic::floor(x);
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`floor`] and [`floorf`].

    use super::*;

    #[test]
    fn floor_preserves_negative_zero() {
        assert_biteq!(floor(-0.0), -0.0);
        assert_biteq!(floorf(-0.0), -0.0);
    }

    #[test]
    fn floor_conformance_bit_exact_f64() {
        let cases = [
            (0x3ffb333333333333_u64, 0x3ff0000000000000_u64), // floor(1.7)
            (0xbffb333333333333_u64, 0xc000000000000000_u64), // floor(-1.7)
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(floor(f64::from_bits(x_bits)), f64::from_bits(y_bits));
        }
    }

    #[test]
    fn floorf_conformance_bit_exact() {
        let cases = [
            (0x3fd9999a_u32, 0x3f800000_u32), // floor(1.7)
            (0xbfd9999a_u32, 0xc0000000_u32), // floor(-1.7)
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(floorf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }
}
