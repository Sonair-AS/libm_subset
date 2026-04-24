// Note: generic functions are marked `#[inline]` because, even though generic functions are
// typically inlined, this does not seem to always be the case.

mod ceil;
mod copysign;
mod fabs;
mod floor;
#[cfg(not(feature = "sonair_certified"))]
mod fma;
#[cfg(not(feature = "sonair_certified"))]
mod fma_wide;
#[cfg(not(feature = "sonair_certified"))]
mod fmod;
#[cfg(not(feature = "sonair_certified"))]
mod rint;
mod round;
mod scalbn;
mod sqrt;
mod trunc;

pub use ceil::ceil;
pub use copysign::copysign;
pub use fabs::fabs;
pub use floor::floor;
#[cfg(not(feature = "sonair_certified"))]
pub use fma::fma_round;
#[cfg(not(feature = "sonair_certified"))]
pub use fma_wide::fma_wide_round;
#[cfg(not(feature = "sonair_certified"))]
pub use fmod::fmod;
#[cfg(not(feature = "sonair_certified"))]
pub use rint::rint_round;
pub use round::round;
pub use scalbn::scalbn;
pub use sqrt::sqrt;
pub(crate) use trunc::trunc;
