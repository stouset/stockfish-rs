#[cfg(not(feature = "unaccelerated"))]
#[doc(hidden)]
pub mod cached;

#[doc(hidden)]
pub mod computed;

#[cfg(not(feature = "unaccelerated"))]
#[doc(inline)]
pub use cached::*;

#[cfg(feature = "unaccelerated")]
#[doc(inline)]
pub use computed::*;
