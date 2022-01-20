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
