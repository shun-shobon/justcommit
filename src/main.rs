mod args;

use anyhow::{Context, Result};
use args::Args;
// use chatgpt::prelude::*;
use clap::Parser;
use git2::{DiffFormat, Repository};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let pwd = env::current_dir()?;
    let repo = Repository::open(pwd).context("Failed to open repository")?;

    let head_tree = repo.head()?.peel_to_tree()?;

    let diff = repo.diff_tree_to_index(Some(&head_tree), None, None)?;
    diff.print(DiffFormat::Patch, |_, _, line| {
        let content = String::from_utf8_lossy(line.content());
        println!("{}", content);

        true
    })?;

    Ok(())

    // let openai_token = env::var("OPENAI_TOKEN").context("OPENAI_TOKEN is not set")?;

    // let openai_config = ModelConfigurationBuilder::default()
    //     .engine(ChatGPTEngine::Gpt35Turbo)
    //     .build()
    //     .unwrap();
    // let chatgpt_client = ChatGPT::new_with_config(openai_token, openai_config).unwrap();

    // let resp = chatgpt_client.send_message("こんにちは").await?;

    // println!("{}", resp.message().content);

    // Ok(())
}
