#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod args;
mod commit;
mod config;
mod env;
mod git;

use anyhow::Result;
use args::Args;
use clap::Parser;

use crate::config::Config;
use crate::env::Env;
use crate::git::get_diffs;

static APP_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load();
    let env = Env::create(&args, config)?;

    let diffs = get_diffs()?;
    let message = commit::generate_message(&env.openai_token, diffs).await?;

    println!("{message}");

    Ok(())
}
