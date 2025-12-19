#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod key;
pub use key::Type;

#[cfg(feature = "set")]
mod set;
#[cfg(feature = "set")]
pub use set::*;

#[cfg(feature = "map")]
mod map;
#[cfg(feature = "map")]
pub use map::*;
