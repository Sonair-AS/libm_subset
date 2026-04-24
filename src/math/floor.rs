/// Floor (f64)
///
/// Finds the nearest integer less than or equal to `x`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn floor(x: f64) -> f64 {
    select_implementation! {
        name: floor,
        use_arch: all(target_arch = "wasm32", intrinsics_enabled),
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
        use_arch: all(target_arch = "wasm32", intrinsics_enabled),
        args: x,
    }

    return super::generic::floor(x);
}
