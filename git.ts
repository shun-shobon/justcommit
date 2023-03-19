export async function readDiffStagedFiles(): Promise<string> {
  const output = await runGit("diff", "--staged");

  return output;
}

async function runGit(...args: string[]): Promise<string> {
  const decoder = new TextDecoder("utf-8");

  const { code, stderr, stdout } = await new Deno.Command("git", { args })
    .output();

  if (code !== 0) {
    throw new GitError(
      `\`git ${args.join(" ")}\` failed with exit code ${code}`,
      {
        cause: decoder.decode(stderr),
      },
    );
  }

  return decoder.decode(stdout);
}

class GitError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message, options);

    this.name = "GitError";
  }
}
