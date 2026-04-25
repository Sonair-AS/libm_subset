//! Architecture-specific support for x86-32 and x86-64 with SSE2

pub fn sqrtf(mut x: f32) -> f32 {
    // SAFETY: `sqrtss` is part of `sse2`, which this module is gated behind. It has no memory
    // access or side effects.
    unsafe {
        core::arch::asm!(
            "sqrtss {x}, {x}",
            x = inout(xmm_reg) x,
            options(nostack, nomem, pure),
        )
    };
    x
}

#[allow(dead_code)] // Part of the upstream libm API; gated behind sonair_certified in arch/mod.rs
pub fn sqrt(mut x: f64) -> f64 {
    // SAFETY: `sqrtsd` is part of `sse2`, which this module is gated behind. It has no memory
    // access or side effects.
    unsafe {
        core::arch::asm!(
            "sqrtsd {x}, {x}",
            x = inout(xmm_reg) x,
            options(nostack, nomem, pure),
        )
    };
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrtf_basic() {
        assert_eq!(sqrtf(4.0), 2.0);
        assert_eq!(sqrtf(9.0), 3.0);
        assert_eq!(sqrtf(0.0), 0.0);
        assert_eq!(sqrtf(1.0), 1.0);
        assert!(sqrtf(f32::NAN).is_nan());
        assert_eq!(sqrtf(f32::INFINITY), f32::INFINITY);
    }

    #[test]
    fn sqrt_basic() {
        assert_eq!(sqrt(4.0), 2.0);
        assert_eq!(sqrt(9.0), 3.0);
        assert_eq!(sqrt(0.0), 0.0);
        assert_eq!(sqrt(1.0), 1.0);
        assert!(sqrt(f64::NAN).is_nan());
        assert_eq!(sqrt(f64::INFINITY), f64::INFINITY);
    }
}
