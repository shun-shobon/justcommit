mod args;
mod config;
mod git;
mod openai;

use anyhow::Result;
use args::Args;
use clap::Parser;

use crate::config::ConfigData;
use crate::git::get_diffs;
use crate::openai::generate_commit_message;

#[tokio::main]
async fn main() -> Result<()> {
    Args::parse();
    let config = ConfigData::load()?;

    let diffs = get_diffs()?;
    let message = generate_commit_message(&config.token, diffs).await?;

    println!("{}", message);

    Ok(())
}
