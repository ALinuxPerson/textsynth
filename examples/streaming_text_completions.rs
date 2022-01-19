use anyhow::Context;
use futures::StreamExt;
use std::io::Write;
use std::{env, io};
use textsynth::prelude::EngineDefinition;

mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (textsynth, retrieval_method) = common::textsynth()?;
    let prompt = env::args()
        .skip(retrieval_method.skip_by())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{}", prompt);
    io::stdout().flush().context("failed to flush stdout")?;

    let engine = textsynth.engine(EngineDefinition::GptJ6B);
    let mut text_completion_stream = engine
        .text_completion(prompt)
        .stream()
        .await
        .context("failed to get streaming text completion")?;
    let mut text_completions = Vec::new();

    while let Some(text_completion) = text_completion_stream.next().await {
        let text_completion = text_completion
            .context("failed to connect to textsynth api for next text completion")?
            .context("got invalid json from textsynth api")?
            .context("failed to get text completion")?;
        print!("{}", text_completion.text());
        io::stdout().flush().context("failed to flush stdout")?;
        text_completions.push(text_completion)
    }

    println!();

    for (index, text_completion) in text_completions.into_iter().enumerate() {
        println!(
            "{}. reached end = {}, total_tokens = {:?}, truncated prompt = {}",
            index + 1,
            text_completion.reached_end(),
            text_completion.total_tokens(),
            text_completion.truncated_prompt()
        );
    }
    Ok(())
}
