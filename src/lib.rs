#![no_std]
// re-exporting whole modules
pub mod iterators;
pub mod wiretypes;
pub mod encoder;
pub mod decoder;
pub mod traits;

// re-exporting specific pieces of modules for convenient shorter-hand access
pub use crate::iterators::LimitedIterator;
pub use crate::wiretypes::wire_types;
pub use crate::decoder::DecodeError;
pub use crate::traits::*;