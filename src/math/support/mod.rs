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
mod int_traits;

#[allow(unused_imports)] // Re-exported for upstream API completeness; consumer may not use 256-bit types
#[cfg(not(feature = "sonair_certified"))]
pub use big::{i256, u256};
#[allow(unused_imports, clippy::single_component_path_imports)] // cfg_if re-export is needed by macros despite clippy false positive
pub(crate) use cfg_if;
pub use env::{FpResult, Round, Status};
#[allow(unused_imports)] // Re-exported for upstream API completeness; not all traits used in sonair_certified
#[cfg(not(feature = "sonair_certified"))]
pub use float_traits::{DFloat, Float, HFloat, IntTy};
#[cfg(feature = "sonair_certified")]
pub use float_traits::{Float, IntTy};
pub use int_traits::{CastFrom, CastInto, DInt, HInt, Int, MinInt};

/// Hint to the compiler that the current path is cold.
pub fn cold_path() {
    #[cfg(intrinsics_enabled)]
    core::intrinsics::cold_path();
}
