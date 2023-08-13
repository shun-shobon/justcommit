mod args;

use anyhow::{Context, Result};
use args::Args;
use chatgpt::prelude::*;
use clap::Parser;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let openai_token = env::var("OPENAI_TOKEN").context("OPENAI_TOKEN is not set")?;

    let openai_config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Gpt35Turbo)
        .build()
        .unwrap();
    let chatgpt_client = ChatGPT::new_with_config(openai_token, openai_config).unwrap();

    let resp = chatgpt_client.send_message("こんにちは").await?;

    println!("{}", resp.message().content);

    Ok(())
}
