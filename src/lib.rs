#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod core;
pub mod engine;
pub mod error;
pub mod prelude;
mod utils;

#[cfg(test)]
mod test_utils {
    pub mod dotenv {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};
        use anyhow::Context;
        use once_cell::sync::OnceCell;

        static INITIALIZED: AtomicBool = AtomicBool::new(false);
        static LAST_ERROR: OnceCell<Arc<dotenv::Error>> = OnceCell::new();
        const ORDERING: Ordering = Ordering::SeqCst;

        pub fn initialize() {
            if let Some(error) = LAST_ERROR.get() {
                panic!("failed to initialize dotenv: {error}");
            } else if !INITIALIZED.load(ORDERING) {
                let result = match dotenv::dotenv().map_err(Arc::new) {
                    Ok(_) => {
                        INITIALIZED.store(true, ORDERING);
                        Ok(())
                    },
                    Err(error) => {
                        let _ = LAST_ERROR.set(Arc::clone(&error));
                        Err(error)
                    }
                };

                result.expect("failed to initialize dotenv")
            }
        }
    }
    pub mod text_synth {
        use once_cell::sync::{Lazy, OnceCell};
        use crate::core::TextSynth;
        use crate::engine::Engine;
        use crate::prelude::EngineDefinition;

        pub const ENGINE_DEFINITION: EngineDefinition = EngineDefinition::GptJ6B;
        static TEXT_SYNTH: OnceCell<TextSynth> = OnceCell::new();
        static ENGINE: Lazy<Engine> = Lazy::new(|| get().engine(ENGINE_DEFINITION));

        pub fn get() -> &'static TextSynth {
            TEXT_SYNTH.get_or_init(|| TextSynth::new(super::api_key().into()))
        }

        pub fn engine() -> &'static Engine<'static> {
            &ENGINE
        }
    }
    pub mod cache {
        use once_cell::sync::OnceCell;
        use crate::prelude::LogProbabilities;

        static LOG_PROBABILITIES: OnceCell<LogProbabilities> = OnceCell::new();

        pub fn initialize_log_probabilities(log_probabilities: LogProbabilities) {
            let _ = LOG_PROBABILITIES.set(log_probabilities);
        }

        pub fn get_log_probabilities() -> &'static LogProbabilities {
            LOG_PROBABILITIES.get().expect("log probabilities not initialized")
        }

        pub fn is_log_probabilities_initialized() -> bool {
            LOG_PROBABILITIES.get().is_some()
        }
    }

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
}

pub use crate::error::{Error, Result};
pub(crate) use error::UntaggedResult;
