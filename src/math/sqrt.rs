/// The square root of `x` (f16).

/// The square root of `x` (f32).
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn sqrtf(x: f32) -> f32 {
    select_implementation! {
        name: sqrtf,
        use_arch: any(
            all(target_arch = "aarch64", target_feature = "neon"),
            all(target_arch = "wasm32", intrinsics_enabled),
            target_feature = "sse2"
        ),
        args: x,
    }

    super::generic::sqrt(x)
}

/// The square root of `x` (f64).
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn sqrt(x: f64) -> f64 {
    select_implementation! {
        name: sqrt,
        use_arch: any(
            all(target_arch = "aarch64", target_feature = "neon"),
            all(target_arch = "wasm32", intrinsics_enabled),
            target_feature = "sse2"
        ),
        args: x,
    }

    super::generic::sqrt(x)
}
