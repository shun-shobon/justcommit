mod openai;

use std::fmt::Display;

use self::openai::{Function, FunctionCallRequest, Message, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub async fn generate_message(
    openai_token: &str,
    diffs: impl Into<String> + Send,
) -> anyhow::Result<String> {
    let mut commit_func_params = schemars::schema_for!(CommitMessage);
    commit_func_params.meta_schema = None;

    let commit_func = Function {
        name: "set_commit_message",
        description: Some("Sets the commit message"),
        parameters: serde_json::to_value(commit_func_params)?,
    };

    let messages = create_history(diffs);
    let message = openai::send_request(
        openai_token,
        Some(&[commit_func]),
        FunctionCallRequest::Call("set_commit_message"),
        &messages,
    )
    .await?;
    let Some(function_call) = message.function_call else {
        anyhow::bail!("No function call returned from OpenAI API")
    };

    let commit_args = serde_json::from_str::<CommitMessage>(&function_call.arguments)?;
    Ok(commit_args.to_string())
}

fn create_history(diffs: impl Into<String>) -> Vec<Message> {
    // Based on https://github.com/di-sukharev/opencommit/blob/916ddf02d1fcbf6082362e153262e177e39b743b/src/generateCommitMessageFromGitDiff.ts
    vec![
        Message {
            role: Role::System,
            content: Some(concat!(
                "You are to act as the author of a commit message in git. ",
                "Your mission is to create clean and comprehensive commit messages in the conventional commit convention and explain WHAT were the changes and WHY the changes were done. ",
                "I'll send you an output of 'git diff --staged' command, and you convert it into a commit message. ",
                "Do not preface the commit with anything. ",
                "Don't add any descriptions to the commit, only commit message. ",
                "Use the present tense. ",
                "Lines must not be longer than 74 characters. ",
                "Use english to answer.",
            ).to_string()),
            function_call: None,
        },
        Message {
            role: Role::User,
            content: Some(diffs.into()),
            function_call: None,
        }
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct CommitMessage {
    /// The type of the commit
    #[serde(rename = "type")]
    commit_type: CommitType,

    /// The subject of the commit
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum CommitType {
    /// A new feature
    Feat,

    /// A bug fix
    Fix,

    /// Changes that affect the build system or external dependencies
    Build,

    /// Changes to our CI configuration files and scripts
    Ci,

    /// Documentation only changes
    Docs,

    /// A code change that improves performance
    Perf,

    /// A code change that neither fixes a bug nor adds a feature
    Refactor,

    /// Changes that do not affect the meaning of the code
    Style,

    /// Adding missing tests or correcting existing tests
    Test,

    /// Other changes that don't modify src or test files
    Chore,
}

impl Display for CommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.commit_type, self.message)
    }
}

impl Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Feat => "feat",
            Self::Fix => "fix",
            Self::Build => "build",
            Self::Ci => "ci",
            Self::Docs => "docs",
            Self::Perf => "perf",
            Self::Refactor => "refactor",
            Self::Style => "style",
            Self::Test => "test",
            Self::Chore => "chore",
        };

        write!(f, "{s}")
    }
}
