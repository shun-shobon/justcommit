use std::path::PathBuf;

use anyhow::{ensure, Result};
use git2::{DiffFormat, DiffLineType, DiffOptions, Repository, Tree};
use glob::Pattern;
use once_cell::sync::Lazy;

static IGNORE_PATTERN: Lazy<Pattern> = Lazy::new(|| Pattern::new("*.lock").unwrap());

pub fn get_diffs() -> Result<String> {
    let pwd = std::env::current_dir()?;
    let repo = Repository::open(pwd)?;
    let head_tree = repo.head()?.peel_to_tree()?;

    let changed_files = get_changed_files(&repo, &head_tree)?;
    ensure!(!changed_files.is_empty(), "No files changed in this commit");

    let diffs = get_diffs_from_files(&repo, &changed_files, &head_tree)?;

    Ok(diffs)
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

fn get_diffs_from_files<'a>(
    repo: &'a Repository,
    files: &[PathBuf],
    head_tree: &Tree<'a>,
) -> Result<String> {
    let mut diff_ops = DiffOptions::new();
    for file in files {
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
