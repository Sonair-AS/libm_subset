use crate::support::Float;

/// Copy the sign of `y` to `x`.
#[inline]
pub fn copysign<F: Float>(x: F, y: F) -> F {
    let mut ux = x.to_bits();
    let uy = y.to_bits();
    ux &= !F::SIGN_MASK;
    ux |= uy & F::SIGN_MASK;
    F::from_bits(ux)
}

#[cfg(test)]
mod tests {
    use super::copysign;
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
        spec_test::<f32>(|x, y| copysign(x, y));
    }

    #[test]
    fn spec_tests_f64() {
        spec_test::<f64>(|x, y| copysign(x, y));
    }
}
