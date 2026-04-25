/// Absolute value (magnitude) (f32)
///
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn fabsf(x: f32) -> f32 {
    select_implementation! {
        name: fabsf,
        args: x,
    }

    super::generic::fabs(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::Float;

    /// Based on https://en.cppreference.com/w/cpp/numeric/math/fabs
    fn spec_test<F: Float>(f: impl Fn(F) -> F) {
        assert_biteq!(f(F::ZERO), F::ZERO);
        assert_biteq!(f(F::NEG_ZERO), F::ZERO);
        assert_biteq!(f(F::INFINITY), F::INFINITY);
        assert_biteq!(f(F::NEG_INFINITY), F::INFINITY);
        assert!(f(F::NAN).is_nan());

        // Not spec rewquired but we expect it
        assert!(f(F::NAN).is_sign_positive());
        assert!(f(F::from_bits(F::NAN.to_bits() | F::SIGN_MASK)).is_sign_positive());
    }

    #[test]
    fn sanity_check_f32() {
        assert_eq!(fabsf(-1.0f32), 1.0);
        assert_eq!(fabsf(2.8f32), 2.8);
    }

    #[test]
    fn spec_tests_f32() {
        spec_test::<f32>(fabsf);
    }

    #[test]
    fn fabsf_conformance_bit_exact() {
        let cases = [
            (0xbf800000_u32, 0x3f800000_u32), // |-1| = 1
            (0xc0200000_u32, 0x40200000_u32), // |-2.5| = 2.5
            (0x7f800000_u32, 0x7f800000_u32), // |+inf|
            (0xff800000_u32, 0x7f800000_u32), // |-inf|
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(fabsf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }
}
