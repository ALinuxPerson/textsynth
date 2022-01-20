pub mod dotenv;
pub mod text_synth;
pub mod cache;

use std::env;
use once_cell::sync::Lazy;

static API_KEY: Lazy<String> = Lazy::new(|| {
    dotenv::initialize();
    env::var("API_KEY")
        .expect("pass an api key to run the tests")
});

pub fn api_key() -> &'static str {
    &API_KEY
}
