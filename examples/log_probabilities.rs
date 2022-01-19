mod common;

use anyhow::Context;
use std::env;
use textsynth::prelude::{EngineDefinition, NonEmptyString};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (textsynth, retrieval_method) = common::textsynth()?;
    let engine = textsynth.engine(EngineDefinition::GptJ6B);
    let mut args = env::args().skip(retrieval_method.skip_by());
    let (context, continuation) = (
        args.next().context("expected context")?,
        args.next().context("expected continuation")?,
    );
    let continuation =
        NonEmptyString::new(continuation).context("continuation must not be empty")?;
    let log_probabilities = engine
        .log_probabilities(context, continuation)
        .await
        .context("failed to connect to textsynth")?
        .context("failed to retrieve log probabilities")?;

    println!(
        "log probability = {}, is greedy = {}, total tokens = {}",
        log_probabilities.log_probability(),
        log_probabilities.is_greedy(),
        log_probabilities.total_tokens(),
    );

    Ok(())
}
