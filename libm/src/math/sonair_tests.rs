//! Targeted coverage tests for pub(crate) functions and methods
//! that are not externally visible.

#[cfg(test)]
mod tests {
    use crate::math::rem_pio2_large::rem_pio2_large;
    use crate::math::rem_pio2f::rem_pio2f;
    use crate::support::Status;

    // ========================= rem_pio2f =========================

    /// rem_pio2f: inf input returns (0, NaN).
    /// Unreachable via public API (sinf/cosf/tanf guard against inf/NaN),
    /// but tested here for completeness.
    #[test]
    fn rem_pio2f_inf() {
        let (n, y) = rem_pio2f(f32::INFINITY);
        assert_eq!(n, 0);
        assert!(y.is_nan());
    }

    #[test]
    fn rem_pio2f_nan() {
        let (n, y) = rem_pio2f(f32::NAN);
        assert_eq!(n, 0);
        assert!(y.is_nan());
    }

    #[test]
    fn rem_pio2f_neg_inf() {
        let (n, y) = rem_pio2f(f32::NEG_INFINITY);
        assert_eq!(n, 0);
        assert!(y.is_nan());
    }

    // ========================= rem_pio2_large =========================

    /// rem_pio2_large with e0 sufficiently negative to force jv < 0,
    /// covering the `jv = 0` branch (line 260).
    /// div!(-28, 24) = -1 in Rust integer division, triggering jv < 0.
    #[test]
    fn rem_pio2_large_negative_jv() {
        let tx = [1.0_f64];
        let mut ty = [0.0_f64];
        let _n = rem_pio2_large(&tx, &mut ty, -25, 0);
    }

    // ========================= rem_pio2_large consecutive zero iq terms =========================

    /// Covers lines 362-364 (while iq[jk-k]==0) and 387-390 (while iq[jz]==0).
    /// Found via exhaustive f32 search: sinf(9450752.0) triggers both paths.
    #[test]
    fn rem_pio2_large_consecutive_zero_iq() {
        let x = [9450752.0_f64];
        let mut y = [0.0_f64];
        let _ = rem_pio2_large(&x, &mut y, 25, 0);
    }

    // ========================= Status::with (pub(crate)) =========================

    #[test]
    fn status_with() {
        let combined = Status::UNDERFLOW.with(Status::INEXACT);
        assert!(combined.underflow());
        assert!(combined.inexact());
        assert!(!combined.overflow());
    }
}
