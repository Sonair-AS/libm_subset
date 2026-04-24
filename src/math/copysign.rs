/// Sign of Y, magnitude of X (f32)
///
/// Constructs a number with the magnitude (absolute value) of its
/// first argument, `x`, and the sign of its second argument, `y`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn copysignf(x: f32, y: f32) -> f32 {
    super::generic::copysign(x, y)
}

/// Sign of Y, magnitude of X (f64)
///
/// Constructs a number with the magnitude (absolute value) of its
/// first argument, `x`, and the sign of its second argument, `y`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn copysign(x: f64, y: f64) -> f64 {
    super::generic::copysign(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::Float;

    fn spec_test<F: Float>(f: impl Fn(F, F) -> F) {
        assert_biteq!(f(F::ZERO, F::ZERO), F::ZERO);
        assert_biteq!(f(F::NEG_ZERO, F::ZERO), F::ZERO);
        assert_biteq!(f(F::ZERO, F::NEG_ZERO), F::NEG_ZERO);
        assert_biteq!(f(F::NEG_ZERO, F::NEG_ZERO), F::NEG_ZERO);

        assert_biteq!(f(F::ONE, F::ONE), F::ONE);
        assert_biteq!(f(F::NEG_ONE, F::ONE), F::ONE);
        assert_biteq!(f(F::ONE, F::NEG_ONE), F::NEG_ONE);
        assert_biteq!(f(F::NEG_ONE, F::NEG_ONE), F::NEG_ONE);

        assert_biteq!(f(F::INFINITY, F::INFINITY), F::INFINITY);
        assert_biteq!(f(F::NEG_INFINITY, F::INFINITY), F::INFINITY);
        assert_biteq!(f(F::INFINITY, F::NEG_INFINITY), F::NEG_INFINITY);
        assert_biteq!(f(F::NEG_INFINITY, F::NEG_INFINITY), F::NEG_INFINITY);

        // Not required but we expect it
        assert_biteq!(f(F::NAN, F::NAN), F::NAN);
        assert_biteq!(f(F::NAN, F::ONE), F::NAN);
        assert_biteq!(f(F::NAN, F::NEG_ONE), F::NEG_NAN);
        assert_biteq!(f(F::NAN, F::NEG_NAN), F::NEG_NAN);
        assert_biteq!(f(F::NEG_NAN, F::NAN), F::NAN);
        assert_biteq!(f(F::NEG_NAN, F::ONE), F::NAN);
        assert_biteq!(f(F::NEG_NAN, F::NEG_ONE), F::NEG_NAN);
        assert_biteq!(f(F::NEG_NAN, F::NEG_NAN), F::NEG_NAN);
        assert_biteq!(f(F::ONE, F::NAN), F::ONE);
        assert_biteq!(f(F::ONE, F::NEG_NAN), F::NEG_ONE);
        assert_biteq!(f(F::NEG_ONE, F::NAN), F::ONE);
        assert_biteq!(f(F::NEG_ONE, F::NEG_NAN), F::NEG_ONE);
    }

    #[test]
    fn spec_tests_f32() {
        spec_test::<f32>(copysignf);
    }

    #[test]
    fn spec_tests_f64() {
        spec_test::<f64>(copysign);
    }
}
