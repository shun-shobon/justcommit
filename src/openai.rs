use anyhow::Result;
use chatgpt::{prelude::*, types::Role};
use indoc::indoc;

pub async fn generate_commit_message(token: &str, diffs: impl Into<String>) -> Result<String> {
    let openai_config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Gpt35Turbo)
        .build()
        .unwrap();
    let chatgpt_client = ChatGPT::new_with_config(token, openai_config).unwrap();

    let history = create_history(diffs);

    let resp = chatgpt_client.send_history(&history).await?;

    Ok(resp.message().content.to_owned())
}

fn create_history(diffs: impl Into<String>) -> Vec<ChatMessage> {
    // Based on https://github.com/di-sukharev/opencommit/blob/916ddf02d1fcbf6082362e153262e177e39b743b/src/generateCommitMessageFromGitDiff.ts
    vec![
        ChatMessage {
            role: Role::System,
            content: concat!(
                "You are to act as the author of a commit message in git. ",
                "Your mission is to create clean and comprehensive commit messages in the conventional commit convention and explain WHAT were the changes and WHY the changes were done. ",
                "I'll send you an output of 'git diff --staged' command, and you convert it into a commit message. ",
                "Do not preface the commit with anything. ",
                "Don't add any descriptions to the commit, only commit message. ",
                "Use the present tense. ",
                "Lines must not be longer than 74 characters. ",
                "Use english to answer.",
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
            content: diffs.into(),
        }
    ]
}
