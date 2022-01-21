//! Core functionality of `textsynth`.
use crate::engine::definition::EngineDefinition;
use crate::engine::Engine;
use reqwest::{IntoUrl, RequestBuilder};

/// The main structure of `textsynth`.
#[derive(Debug, Clone)]
pub struct TextSynth {
    /// The client to make http requests to.
    pub client: reqwest::Client,

    /// The api key used to authenticate into the textsynth API.
    pub api_key: String,
}

impl TextSynth {
    /// Creates a new [`TextSynth`] instance.
    pub const fn new_with_client(client: reqwest::Client, api_key: String) -> TextSynth {
        TextSynth { client, api_key }
    }

    /// Try an create a new [`TextSynth`] instance with a default [`reqwest::Client`], returning an
    /// error if creating a default [`reqwest::Client`] fails.
    pub fn try_new(api_key: String) -> reqwest::Result<Self> {
        Ok(TextSynth::new_with_client(
            reqwest::Client::builder().build()?,
            api_key,
        ))
    }

    /// Create a new [`TextSynth`] instance with a default [`reqwest::Client`], panicking if
    /// creating a default [`reqwest::Client`] fails.
    pub fn new(api_key: String) -> TextSynth {
        Self::try_new(api_key).expect("failed to create a new `reqwest::Client`")
    }

    /// Create a new engine from the given definition.
    pub const fn engine(&self, definition: EngineDefinition) -> Engine {
        Engine::new(self, definition)
    }

    pub(crate) fn post(&self, url: impl IntoUrl) -> RequestBuilder {
        self.client.post(url).bearer_auth(&self.api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn test_new_with_client() {
        let _ = TextSynth::new_with_client(reqwest::Client::new(), test_utils::api_key().into());
    }

    #[test]
    fn test_try_new() {
        let _ = TextSynth::try_new(test_utils::api_key().into())
            .expect("failed to create new textsynth client");
    }

    #[test]
    fn test_new() {
        let _ = TextSynth::new(test_utils::api_key().into());
    }

    #[test]
    fn test_engine() {
        let textsynth = TextSynth::new(test_utils::api_key().into());
        let _ = textsynth.engine(EngineDefinition::GptJ6B);
    }
}
