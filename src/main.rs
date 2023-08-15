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

static APP_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load(&args)?;

    let diffs = get_diffs()?;
    let message = generate_commit_message(&config.openai_token, diffs).await?;

    println!("{message}");

    Ok(())
}
