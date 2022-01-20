#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod core;
pub mod engine;
pub mod error;
pub mod prelude;
mod utils;

#[cfg(test)]
mod test_utils;

pub use crate::error::{Error, Result};
pub(crate) use error::UntaggedResult;
