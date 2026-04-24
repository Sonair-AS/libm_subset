/// Round `x` to the nearest integer, breaking ties away from zero.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn roundf(x: f32) -> f32 {
    super::generic::round(x)
}

#[cfg(test)]
mod tests {
    //! Unit tests for [`roundf`].

    use super::*;

    #[test]
    fn roundf_preserves_signed_zero() {
        assert_biteq!(roundf(0.0), 0.0);
        assert_biteq!(roundf(-0.0), -0.0);
    }

    #[test]
    fn roundf_conformance_bit_exact() {
        let cases = [
            (0x3f99999a_u32, 0x3f800000_u32), // 1.2 -> 1
            (0x3fc00000_u32, 0x40000000_u32), // 1.5 -> 2
            (0x3fcccccd_u32, 0x40000000_u32), // 1.6 -> 2
            (0xbfc00000_u32, 0xc0000000_u32), // -1.5 -> -2
            (0xc0200000_u32, 0xc0400000_u32), // -2.5 -> -3
            (0x40200000_u32, 0x40400000_u32), // 2.5 -> 3
        ];

        for (x_bits, y_bits) in cases {
            assert_biteq!(roundf(f32::from_bits(x_bits)), f32::from_bits(y_bits));
        }
    }
}
