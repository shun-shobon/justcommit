#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod args;
mod config;
mod git;
mod openai;

use anyhow::Result;
use args::Args;
use clap::Parser;

use crate::config::Config;
use crate::git::get_diffs;
use crate::openai::generate_commit_message;

#[tokio::main]
async fn main() -> Result<()> {
    Args::parse();
    let config = Config::load()?;

    let diffs = get_diffs()?;
    let message = generate_commit_message(&config.openai_token, diffs).await?;

    println!("{message}");

    Ok(())
}
