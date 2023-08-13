mod args;

use anyhow::{ensure, Context as _, Result};
use args::Args;
use chatgpt::{prelude::*, types::Role};
use clap::Parser;
use git2::{DiffFormat, DiffLineType, DiffOptions, Repository, Tree};
use glob::Pattern;
use indoc::indoc;
use once_cell::sync::Lazy;
use std::{env, path::PathBuf};

static IGNORE_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new("*.lock").unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let pwd = env::current_dir()?;
    let repo = Repository::open(pwd)?;
    let head_tree = repo.head()?.peel_to_tree()?;

    let changed_files = get_changed_files(&repo, &head_tree)?;
    ensure!(!changed_files.is_empty(), "No files changed in this commit");

    let diffs = get_file_diffs(&repo, &changed_files, &head_tree)?;

    let openai_token = env::var("OPENAI_TOKEN").context("OPENAI_TOKEN is not set")?;

    let openai_config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Gpt35Turbo)
        .build()
        .unwrap();
    let chatgpt_client = ChatGPT::new_with_config(openai_token, openai_config).unwrap();

    // Based on https://github.com/di-sukharev/opencommit/blob/916ddf02d1fcbf6082362e153262e177e39b743b/src/generateCommitMessageFromGitDiff.ts
    let history = vec![
        ChatMessage {
            role: Role::System,
            content: concat!(
                "You are to act as the author of a commit message in git. ",
                "Your mission is to create clean and comprehensive commit messages in the conventional commit convention and explain WHAT were the changes and WHY the changes were done. ",
                "I'll send you an output of 'git diff --staged' command, and you convert it into a commit message. Do not preface the commit with anything. ",
                "Don't add any descriptions to the commit, only commit message. ",
                "Use the present tense. Lines must not be longer than 74 characters. Use english to answer.",
            ).to_string(),
        },
        ChatMessage {
            role: Role::User,
            content: indoc! {"
                diff --git a/src/server.ts b/src/server.ts
                index ad4db42..f3b18a9 100644
                --- a/src/server.ts
                +++ b/src/server.ts
                @@ -10,7 +10,7 @@
                import {
                initWinstonLogger();

                const app = express();
                -const port = 7799;
                +const PORT = 7799;

                app.use(express.json());

                @@ -34,6 +34,6 @@
                app.use((_, res, next) => {
                // ROUTES
                app.use(PROTECTED_ROUTER_URL, protectedRouter);

                -app.listen(port, () => {
                -  console.log(`Server listening on port ${port}`);
                +app.listen(process.env.PORT || PORT, () => {
                +  console.log(`Server listening on port ${PORT}`);
                });
            "}.to_string(),
        },
        ChatMessage {
            role: Role::Assistant,
            content: indoc! {"
                fix(server.ts): Change port variable case from lowercase port to uppercase PORT to improve semantics
                feat(server.ts): Add support for process.env.PORT environment variable to be able to run app on a configurable port
            "}.to_string(),
        },
        ChatMessage {
            role: Role::User,
            content: diffs,
        }
    ];

    let resp = chatgpt_client.send_history(&history).await?;

    println!("{}", resp.message().content);

    Ok(())
}

fn get_changed_files(repo: &Repository, head_tree: &Tree) -> Result<Vec<PathBuf>> {
    let files = repo
        .diff_tree_to_index(Some(head_tree), None, None)?
        .deltas()
        .filter_map(|delta| {
            let path = delta.new_file().path()?;

            if IGNORE_PATTERN.matches_path(path) {
                return None;
            }

            Some(path.to_owned())
        })
        .collect();

    Ok(files)
}

fn get_file_diffs<'a>(
    repo: &'a Repository,
    files: &[PathBuf],
    head_tree: &Tree<'a>,
) -> Result<String> {
    let mut diff_ops = DiffOptions::new();
    for file in files.iter() {
        diff_ops.pathspec(file);
    }

    let mut diff_string = String::new();

    let diff = repo.diff_tree_to_index(Some(head_tree), None, Some(&mut diff_ops))?;
    diff.print(DiffFormat::Patch, |_, _, line| {
        let prefix = match line.origin_value() {
            DiffLineType::Addition => "+",
            DiffLineType::Deletion => "-",
            DiffLineType::Context => " ",
            _ => "",
        };
        let content = String::from_utf8_lossy(line.content());

        diff_string.push_str(prefix);
        diff_string.push_str(&content);

        true
    })?;

    Ok(diff_string)
}
