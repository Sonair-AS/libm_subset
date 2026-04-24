#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn ldexpf(x: f32, n: i32) -> f32 {
    super::scalbnf(x, n)
}

#[cfg_attr(assert_no_panic, no_panic::no_panic)]
pub fn ldexp(x: f64, n: i32) -> f64 {
    super::scalbn(x, n)
}
