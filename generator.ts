import { chatCompletion, ChatCompletionRequest, Message } from "./openai.ts";

export async function generateCommitMessage(
  token: string,
  diff: string,
): Promise<string> {
  const messages: Message[] = [
    systemMessage,
    userMessage,
    assistantMessage,
    {
      "role": "user",
      "content": diff,
    },
  ];

  const request: ChatCompletionRequest = {
    model: "gpt-3.5-turbo",
    messages,
    max_tokens: 25,
    temperature: 0.2,
  };

  const response = await chatCompletion(token, request);

  const commitMessage = response.choices[0].message.content;
  return commitMessage;
}

const systemMessage: Message = {
  role: "system",
  content: [
    "# Instraction",
    [
      "You are a pro author of a commit message in git.",
      "Output the best commit message based on the following constraints.",
    ].join(" "),
    "",
    "# Constraint",
    "- The commit message must be less than 50 characters.",
    "- The commit message must be clean.",
    "- Follow the conventional commit message format.",
    "- Use uppercase for the first letter of the commit message.",
    "- Don't add period at the end of the commit message.",
    "- Don't add any description to the commit message.",
    "",
    "# Input/Output",
    "Input is an output of 'git diff --staged' command.",
    "Output is a commit message.",
  ].join("\n"),
};

const userMessage: Message = {
  role: "user",
  content: [
    "diff --git a/git.ts b/git.ts",
    "new file mode 100644",
    "index 0000000..d87232a",
    "--- /dev/null",
    "+++ b/git.ts",
    "@@ -0,0 +1,31 @@",
    "+export async function readDiffStagedFiles(): Promise<string> {",
    '+  const output = await runGit("diff", "--staged");',
    "+",
    "+  return output;",
    "+}",
    "+",
    "+async function runGit(...args: string[]): Promise<string> {",
    '+  const decoder = new TextDecoder("utf-8");',
    "+",
    '+  const { code, stderr, stdout } = await new Deno.Command("git", { args })',
    "+    .output();",
    "+",
    "+  if (code !== 0) {",
    "+    throw new GitError(",
    '+      `\`git ${args.join(" ")}\` failed with exit code ${code}`,',
    "+      {",
    "+        cause: decoder.decode(stderr),",
    "+      },",
    "+    );",
    "+  }",
    "+",
    "+  return decoder.decode(stdout);",
    "+}",
    "+",
    "+class GitError extends Error {",
    "+  constructor(message: string, options?: ErrorOptions) {",
    "+    super(message, options);",
    "+",
    '+    this.name = "GitError";',
    "+  }",
    "+}",
  ].join("\n"),
};

const assistantMessage: Message = {
  role: "assistant",
  content: "feat: Add `readDiffStagedFiles()`",
};
