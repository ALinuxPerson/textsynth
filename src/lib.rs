#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod core;
pub mod engine;
pub mod error;
pub mod prelude;
mod utils;

#[cfg(test)]
mod test_utils {
    use std::env;
    use once_cell::sync::Lazy;

    static API_KEY: Lazy<String> = Lazy::new(|| env::var("API_KEY")
        .expect("pass an api key to run the tests"));

    pub fn api_key() -> &'static str {
        &API_KEY
    }
}

pub use crate::error::{Error, Result};
pub(crate) use error::UntaggedResult;
