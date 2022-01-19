use anyhow::Context;
use std::env;
use textsynth::prelude::TextSynth;

pub enum ApiKeyRetrievalMethod {
    EnvironmentVariable,
    CommandLineArgument,
}

impl ApiKeyRetrievalMethod {
    pub const fn skip_by(&self) -> usize {
        match self {
            Self::EnvironmentVariable => 1,
            Self::CommandLineArgument => 2,
        }
    }
}

pub fn textsynth() -> anyhow::Result<(TextSynth, ApiKeyRetrievalMethod)> {
    let (api_key, retrieval_method) = match env::var("API_KEY") {
        Ok(api_key) => (api_key, ApiKeyRetrievalMethod::EnvironmentVariable),
        Err(_) => (
            env::args().nth(1).context("pass the api key to either an `API_KEY` environment variable or a command line argument")?,
            ApiKeyRetrievalMethod::CommandLineArgument,
        ),
    };
    let textsynth = TextSynth::try_new(api_key).context("failed to create a TextSynth instance")?;

    Ok((textsynth, retrieval_method))
}

#[allow(dead_code)]
fn main() {}
