/// Ceil (f32)
///
/// Finds the nearest integer greater than or equal to `x`.
#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn ceilf(x: f32) -> f32 {
    select_implementation! {
        name: ceilf,
        args: x,
    }

    super::generic::ceil(x)
}
