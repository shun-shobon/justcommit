use std::process::Command;

use anyhow::Context;

use crate::args::Args;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Env {
    pub openai_token: String,
}

impl Env {
    pub fn create(args: &Args, config: Option<Config>) -> anyhow::Result<Self> {
        let mut openai_token = args
            .openai_token
            .clone()
            .or_else(|| config.map(|c| c.openai_token))
            .context("OpenAI token is not set. Please set it in the config file or pass it as an argument.")?;

        if openai_token.starts_with("op://") {
            openai_token = resolve_1password_ref(&openai_token)?;
        }

        Ok(Self { openai_token })
    }
}

fn resolve_1password_ref(str: &str) -> anyhow::Result<String> {
    let output = Command::new("op")
        .args(["read", "--no-newline", str])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("failed to run op");
    }

    let stdout = String::from_utf8(output.stdout)?;

    Ok(stdout)
}
