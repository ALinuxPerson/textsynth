#[macro_use]
pub mod cache;

pub mod dotenv;
pub mod text_synth;

use std::env;
use std::time::Duration;
use once_cell::sync::Lazy;


static API_KEY: Lazy<String> = Lazy::new(|| {
    dotenv::initialize();
    env::var("API_KEY")
        .expect("pass an api key to run the tests")
});
static TIMEOUT: Lazy<Option<Duration>> = Lazy::new(|| {
    dotenv::initialize();
    env::var("TIMEOUT").ok()?.parse::<f64>().ok().map(Duration::from_secs_f64)
});

pub fn api_key() -> &'static str {
    &API_KEY
}

pub fn timeout() -> Option<Duration> {
    *TIMEOUT
}
