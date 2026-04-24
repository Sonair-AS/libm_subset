#[macro_use]
pub mod macros;
#[cfg(not(feature = "sonair_certified"))]
mod big;
mod env;
// Runtime feature detection requires atomics.
#[cfg(target_has_atomic = "ptr")]
#[cfg(not(feature = "sonair_certified"))]
pub(crate) mod feature_detect;
mod float_traits;
#[cfg(any(not(feature = "sonair_certified"), test))]
pub mod hex_float;
mod int_traits;

#[allow(unused_imports)]
#[cfg(not(feature = "sonair_certified"))]
pub use big::{i256, u256};
// Clippy seems to have a false positive
#[allow(unused_imports, clippy::single_component_path_imports)]
pub(crate) use cfg_if;
pub use env::{FpResult, Round, Status};
pub(crate) use float_traits::{f32_from_bits, f64_from_bits};
#[allow(unused_imports)]
#[cfg(not(feature = "sonair_certified"))]
pub use float_traits::{DFloat, Float, HFloat, IntTy};
#[cfg(feature = "sonair_certified")]
pub use float_traits::{Float, IntTy};
#[allow(unused_imports)]
#[cfg(any(not(feature = "sonair_certified"), test))]
pub use hex_float::{hf32, hf64};
pub use int_traits::{CastFrom, CastInto, DInt, HInt, Int, MinInt};

/// Hint to the compiler that the current path is cold.
pub fn cold_path() {
    #[cfg(intrinsics_enabled)]
    core::intrinsics::cold_path();
}
