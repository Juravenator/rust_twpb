#![no_std]
// re-exporting whole modules
pub mod decoder;
pub mod encoder;
pub mod iterators;
pub mod traits;
pub mod wiretypes;

// re-exporting specific pieces of modules for convenient shorter-hand access
pub use crate::decoder::DecodeError;
pub use crate::iterators::LimitedIterator;
pub use crate::traits::*;
pub use crate::wiretypes::wire_types;
