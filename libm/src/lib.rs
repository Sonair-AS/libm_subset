//! libm in pure Rust
#![no_std]
#![cfg_attr(intrinsics_enabled, allow(internal_features))]
#![cfg_attr(intrinsics_enabled, feature(core_intrinsics))]
#![cfg_attr(
    all(intrinsics_enabled, target_family = "wasm"),
    feature(wasm_numeric_instr)
)]
#![cfg_attr(f128_enabled, feature(f128))]
#![cfg_attr(f16_enabled, feature(f16))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))] // Enable #[coverage(off)] attribute when running under cargo-llvm-cov nightly
// Upstream libm coding style differs from default clippy preferences.
// These lints are suppressed crate-wide to preserve fidelity with the upstream musl/FreeBSD ports.
#![allow(clippy::assign_op_pattern)] // Upstream uses `x = x + y` instead of `x += y` for clarity in math formulas
#![allow(clippy::deprecated_cfg_attr)] // MSRV compatibility: some cfg_attr forms are deprecated in newer Rust
#![allow(clippy::eq_op)] // Intentional self-comparisons used for NaN detection (x != x)
#![allow(clippy::excessive_precision)] // Math constants require full f64 precision literals
#![allow(clippy::float_cmp)] // Exact float comparisons are intentional in IEEE 754 special-value handling
#![allow(clippy::int_plus_one)] // Upstream style: `x >= y + 1` reads naturally in boundary conditions
#![allow(clippy::just_underscores_and_digits)] // Upstream variable names like `_0` match C originals
#![allow(clippy::many_single_char_names)] // Math code uses short variable names (x, y, z, t, w) per convention
#![allow(clippy::mixed_case_hex_literals)] // Upstream hex constants use mixed case from C originals
#![allow(clippy::needless_late_init)] // Upstream declares variables before conditional assignment (C style)
#![allow(clippy::needless_return)] // Upstream uses explicit return in some functions for clarity
#![allow(clippy::unreadable_literal)] // Hex constants for IEEE 754 bit patterns are intentionally unseparated
#![allow(clippy::zero_divided_by_zero)] // Intentional 0.0/0.0 used to produce NaN in const contexts
#![forbid(unsafe_op_in_unsafe_fn)]

mod libm_helper;
mod math;

#[allow(unused_imports)] // MSRV compatibility: older Rust versions require explicit `use core::f32` for associated constants
use core::{f32, f64};

pub use libm_helper::*;

pub use self::math::*;
