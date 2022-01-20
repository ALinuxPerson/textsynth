//! Operations involving text completion.

use crate::engine::definition::EngineDefinition;
use crate::engine::Engine;
use arrayvec::ArrayVec;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};
use tap::Pipe;

/// Maximum number of tokens to generate. A token represents typically 4 or 5 characters for latin
/// scripts. The total number of tokens (prompt + generated text) cannot exceed the model's maximum
/// context length.
///
/// This depends on a [`EngineDefinition`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct MaxTokens(usize);

impl MaxTokens {
    /// Creates a new maximum number of tokens. Ensured to be valid for the given engine definition.
    pub fn new(max_tokens: usize, engine_definition: &EngineDefinition) -> Option<Self> {
        if max_tokens < engine_definition.max_tokens() {
            Some(Self(max_tokens))
        } else {
            None
        }
    }

    /// Returns the maximum number of tokens.
    pub fn inner(&self) -> usize {
        self.0
    }
}

/// Select the next output token among the most probable ones so that their cumulative probability
/// is larger than `top_p`. A higher `top_p` gives more diversity but a potentially less relevant
/// output.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Serialize)]
pub struct TopP(f64);

impl TopP {
    /// Create a new [`TopP`] value. It must be between 0.0 and 1.0 inclusive.
    pub fn new(top_p: f64) -> Option<Self> {
        if (0.0..=1.0).contains(&top_p) {
            Some(Self(top_p))
        } else {
            None
        }
    }
}

/// Select the next output token among the `top_k` most likely ones. A higher `top_k` gives more
/// diversity but a potentially less relevant output.
pub type TopK = bounded_integer::BoundedU16<1, 1000>;

/// Stop the generation when the string(s) are encountered. The generated text does not contain the
/// string.
pub type Stop = ArrayVec<String, 5>;

#[derive(Serialize, Default)]
struct TextCompletionRequest {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<TopK>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
}

/// A text completion response from the API.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub struct TextCompletion {
    text: String,
    reached_end: bool,
    truncated_prompt: Option<bool>,
    total_tokens: Option<usize>,
}

impl TextCompletion {
    /// Returns the generated text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// If true, indicates that this is the last answer. It is only useful if the text completion
    /// request was streamed.
    pub fn reached_end(&self) -> bool {
        self.reached_end
    }

    /// If true, indicates that the prompt was truncated because it was too large compared to the
    /// model's maximum context length. Only the end of the prompt is used to generate the completion.
    pub fn truncated_prompt(&self) -> bool {
        self.truncated_prompt.unwrap_or(false)
    }

    /// Indicates the total number of tokens including the prompt and generated text. It is useful
    /// to estimate the number of compute resources used by the request.
    ///
    /// Returns [`None`] if the text completion request was streamed and isn't the final completion
    /// yet.
    pub fn total_tokens(&self) -> Option<usize> {
        self.total_tokens
    }
}

/// A type returned from [`TextCompletionStream`].
///
/// The order and justification are as follows:
///   * [`reqwest::Error`] is returned if connecting to the API failed on the network level,
///   * [`serde_json::Error`] is returned if the API returned invalid JSON
///     (although this shouldn't happen),
///   * [`crate::Error`] is returned if the API returned an error.
pub type TextCompletionStreamResult =
    reqwest::Result<serde_json::Result<crate::Result<TextCompletion>>>;

/// A stream of text completion responses from the API.
pub struct TextCompletionStream<S: Stream<Item = reqwest::Result<Bytes>>>(S);

impl<S: Stream<Item = reqwest::Result<Bytes>> + Unpin> Stream for TextCompletionStream<S> {
    type Item = TextCompletionStreamResult;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        async fn next<S: Stream<Item = reqwest::Result<Bytes>> + Unpin>(
            this: &mut TextCompletionStream<S>,
        ) -> Option<TextCompletionStreamResult> {
            this.0.next().await.map(|result| {
                result
                    .map(|bytes| bytes.slice(..bytes.len() - 2))
                    .map(|bytes| {
                        serde_json::from_slice::<crate::UntaggedResult<TextCompletion>>(&bytes)
                    })
                    .map(|result| result.map(Into::into))
            })
        }

        let output = next(self.deref_mut());
        futures::pin_mut!(output);
        output.poll(cx)
    }
}

/// A text completion builder.
pub struct TextCompletionBuilder<'ts, 'e> {
    /// The engine used to create this text completion request.
    pub engine: &'e Engine<'ts>,

    /// See [`Self::prompt`].
    pub prompt: String,

    /// See [`Self::max_tokens`].
    pub max_tokens: Option<MaxTokens>,

    /// See [`Self::temperature`].
    pub temperature: Option<f64>,

    /// See [`Self::top_k`].
    pub top_k: Option<TopK>,

    /// See [`Self::top_p`].
    pub top_p: Option<TopP>,
}

impl<'ts, 'e> TextCompletionBuilder<'ts, 'e> {
    /// Create a new text completion builder.
    pub const fn new(engine: &'e Engine<'ts>, prompt: String) -> Self {
        Self {
            engine,
            prompt,
            max_tokens: None,
            temperature: None,
            top_k: None,
            top_p: None,
        }
    }

    /// Set the maximum number of tokens to generate. See [`MaxTokens`] for more information.
    pub fn max_tokens(mut self, max_tokens: MaxTokens) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sampling temperature. A higher temperature means the model will select less common tokens
    /// leading to a larger diversity but potentially less relevant output. It is usually better to
    /// tune `top_p` or `top_k`.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set `top_k`. See [`TopK`] for more information.
    pub fn top_k(mut self, top_k: TopK) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set `top_p`. See [`TopP`] for more information.
    pub fn top_p(mut self, top_p: TopP) -> Self {
        self.top_p = Some(top_p);
        self
    }

    fn url(&self) -> String {
        let engine_id = self.engine.definition.id();
        format!("https://api.textsynth.com/v1/engines/{engine_id}/completions")
    }

    async fn now_impl(self, stop: Option<Stop>) -> reqwest::Result<crate::Result<TextCompletion>> {
        let url = self.url();
        let request = TextCompletionRequest {
            prompt: self.prompt,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            stream: None,
            stop,
        };

        self.engine
            .text_synth
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<crate::UntaggedResult<_>>()
            .await
            .map(Into::into)
    }

    /// Generate a text completion now.
    pub async fn now(self) -> reqwest::Result<crate::Result<TextCompletion>> {
        self.now_impl(None).await
    }

    /// Generate a text completion now, stopping when the specified list of strings are found.
    pub async fn now_until(self, stop: Stop) -> reqwest::Result<crate::Result<TextCompletion>> {
        self.now_impl(Some(stop)).await
    }

    async fn stream_impl(
        self,
        stop: Option<Stop>,
    ) -> reqwest::Result<TextCompletionStream<impl Stream<Item = reqwest::Result<Bytes>>>> {
        let url = self.url();
        let request = TextCompletionRequest {
            prompt: self.prompt,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            stream: Some(true),
            stop,
        };

        self.engine
            .text_synth
            .post(url)
            .json(&request)
            .send()
            .await?
            .bytes_stream()
            .pipe(TextCompletionStream)
            .pipe(Ok)
    }

    /// Create a text completion stream.
    pub async fn stream(
        self,
    ) -> reqwest::Result<TextCompletionStream<impl Stream<Item = reqwest::Result<Bytes>>>> {
        self.stream_impl(None).await
    }

    /// Create a text completion stream which will stop when the given strings at `stop` are
    /// encountered.
    pub async fn stream_until(
        self,
        stop: Stop,
    ) -> reqwest::Result<TextCompletionStream<impl Stream<Item = reqwest::Result<Bytes>>>> {
        self.stream_impl(Some(stop)).await
    }
}
