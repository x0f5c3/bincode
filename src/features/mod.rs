#[cfg(feature = "alloc")]
mod impl_alloc;
#[cfg(feature = "alloc")]
pub use self::impl_alloc::*;

#[cfg(feature = "std")]
mod impl_std;
#[cfg(feature = "std")]
pub use self::impl_std::*;

#[cfg(feature = "derive")]
mod derive;
#[cfg(feature = "derive")]
pub use self::derive::*;
#[cfg(feature = "generic-array")]
mod impl_genreric_array;

#[cfg(feature = "generic-array")]
pub use impl_genreric_array::*;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;
