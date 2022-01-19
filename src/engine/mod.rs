//! Common engine types and operations.

pub mod definition;
pub mod log_probabilities;
pub mod text_completion;

use crate::core::TextSynth;
use crate::engine::log_probabilities::{LogProbabilities, LogProbabilitiesRequest, NonEmptyString};
use crate::engine::text_completion::TextCompletionBuilder;
use definition::EngineDefinition;

/// An engine which will be used for synthesizing text.
#[derive(Debug, Clone)]
pub struct Engine<'ts> {
    /// A reference to [`TextSynth`] which will be used to make HTTP requests to the API.
    pub text_synth: &'ts TextSynth,

    /// A definition of the engine.
    pub definition: EngineDefinition,
}

impl<'ts> Engine<'ts> {
    /// Creates a new engine.
    pub const fn new(text_synth: &'ts TextSynth, definition: EngineDefinition) -> Self {
        Self {
            text_synth,
            definition,
        }
    }

    /// See [`LogProbabilities`] for information about this return value.
    ///
    /// # Arguments
    ///   - `context`: If empty, the context is set to the End-Of-Text token.
    ///   - `continuation`: Must be a non empty string.
    pub async fn log_probabilities(
        &self,
        context: String,
        continuation: NonEmptyString,
    ) -> reqwest::Result<crate::Result<LogProbabilities>> {
        let url = format!(
            "https://api.textsynth.com/v1/engines/{}/logprob",
            self.definition.id()
        );
        self.text_synth
            .post(url)
            .json(&LogProbabilitiesRequest {
                context,
                continuation,
            })
            .send()
            .await?
            .json::<crate::UntaggedResult<_>>()
            .await
            .map(Into::into)
    }

    /// Create a builder for text completion.
    pub fn text_completion(&self, prompt: String) -> TextCompletionBuilder {
        TextCompletionBuilder::new(self, prompt)
    }
}
