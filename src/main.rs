mod args;

use anyhow::{Context, Result};
use args::Args;
// use chatgpt::prelude::*;
use clap::Parser;
use git2::{DiffFormat, DiffLineType, DiffOptions, Repository};
use glob::Pattern;
use once_cell::sync::Lazy;
use std::env;

static IGNORE_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new("*.lock").unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let pwd = env::current_dir()?;
    let repo = Repository::open(pwd).context("Failed to open repository")?;

    let head_tree = repo.head()?.peel_to_tree()?;

    let mut diff_ops = DiffOptions::new();

    repo.diff_tree_to_index(Some(&head_tree), None, None)?
        .deltas()
        .filter_map(|delta| {
            let path = delta.new_file().path()?;

            if IGNORE_PATTERN.matches_path(path) {
                return None;
            }

            Some(path.to_owned())
        })
        .for_each(|path| {
            dbg!(&path);
            diff_ops.pathspec(path);
        });

    let diff = repo.diff_tree_to_index(Some(&head_tree), None, Some(&mut diff_ops))?;
    diff.print(DiffFormat::Patch, |_, _, line| {
        let content = String::from_utf8_lossy(line.content());
        match line.origin_value() {
            DiffLineType::Addition => print!("+{}", content),
            DiffLineType::Deletion => print!("-{}", content),
            DiffLineType::Context => print!(" {}", content),
            _ => print!("{}", content),
        }

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
