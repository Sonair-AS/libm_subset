//! libm in pure Rust
#![no_std]
#![cfg_attr(intrinsics_enabled, allow(internal_features))]
#![cfg_attr(intrinsics_enabled, feature(core_intrinsics))]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::deprecated_cfg_attr)]
#![allow(clippy::eq_op)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::float_cmp)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::mixed_case_hex_literals)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_return)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::zero_divided_by_zero)]
#![forbid(unsafe_op_in_unsafe_fn)]

mod math;

pub use self::math::*;
