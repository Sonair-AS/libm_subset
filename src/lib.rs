//! libm in pure Rust
#![no_std]
#![cfg_attr(intrinsics_enabled, allow(internal_features))] // Required to use core_intrinsics feature
#![cfg_attr(intrinsics_enabled, feature(core_intrinsics))]
// Crate-wide clippy allows: this crate is ported from C (musl libm) and the code style reflects
// the upstream mathematical conventions. Changing these patterns would diverge from upstream and
// make future syncs harder.
#![allow(clippy::assign_op_pattern)] // Upstream uses `x = x + y` style for clarity in math formulas
#![allow(clippy::deprecated_cfg_attr)] // Compatibility with older Rust versions
#![allow(clippy::eq_op)] // Intentional self-comparisons for NaN detection (`x != x`)
#![allow(clippy::excessive_precision)] // Mathematical constants require full precision
#![allow(clippy::float_cmp)] // Bitwise float comparison is intentional in math implementations
#![allow(clippy::int_plus_one)] // Upstream boundary checks use `x >= y + 1` style
#![allow(clippy::just_underscores_and_digits)] // Upstream variable names like `_0`, `_1` in formulas
#![allow(clippy::many_single_char_names)] // Mathematical convention: variables like x, y, z, t, u, v
#![allow(clippy::mixed_case_hex_literals)] // Upstream hex constants use mixed case
#![allow(clippy::needless_late_init)] // Upstream declares variables before conditional initialization
#![allow(clippy::needless_return)] // Upstream uses explicit returns for clarity in branching
#![allow(clippy::unreadable_literal)] // Hex float constants and bit patterns are clearer without separators
#![allow(clippy::zero_divided_by_zero)] // Intentional: used to generate NaN at compile time
#![forbid(unsafe_op_in_unsafe_fn)]

mod math;

pub use self::math::*;
