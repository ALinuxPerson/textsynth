use anyhow::Context;
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
    let output = engine
        .text_completion(prompt)
        .now()
        .await
        .context("failed to connect to the textsynth api")?
        .context("failed to complete text")?;
    println!("{}", output.text());

    assert!(output.reached_end());

    println!(
        "reached end = {}, total tokens = {}, truncated prompt = {}",
        output.reached_end(),
        output.total_tokens().expect("total tokens should exist"),
        output.truncated_prompt()
    );

    Ok(())
}
