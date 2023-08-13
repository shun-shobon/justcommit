mod args;
mod git;
mod openai;

use anyhow::Result;
use args::Args;
use clap::Parser;

use crate::git::get_diffs;
use crate::openai::generate_commit_message;

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let diffs = get_diffs()?;
    let message = generate_commit_message(diffs).await?;

    println!("{}", message);

    Ok(())
}
