/// The square root of `x` (f16).

/// The square root of `x` (f32).
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn sqrtf(x: f32) -> f32 {
    select_implementation! {
        name: sqrtf,
        use_arch: any(
            all(target_arch = "aarch64", target_feature = "neon"),
            target_feature = "sse2"
        ),
        args: x,
    }

    super::generic::sqrt(x)
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`sqrtf`].

    use super::*;

    #[test]
    fn sqrtf_negative_is_nan() {
        assert!(sqrtf(-1.0).is_nan());
    }

    #[test]
    fn sqrtf_zero() {
        assert_biteq!(sqrtf(0.0), 0.0);
        assert_biteq!(sqrtf(-0.0), -0.0);
    }

    #[test]
    fn sqrtf_infinity() {
        assert_biteq!(sqrtf(f32::INFINITY), f32::INFINITY);
    }

    #[test]
    fn sqrtf_conformance_bit_exact() {
        let cases = [
            (0x3f800000_u32, 0x3f800000_u32), // sqrt(1)
            (0x40000000_u32, 0x3fb504f3_u32), // sqrt(2)
            (0x42c80000_u32, 0x41200000_u32), // sqrt(100)
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(sqrtf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }
}
