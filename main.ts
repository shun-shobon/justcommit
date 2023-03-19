import { Command } from "./deps.ts";
import { readDiffStagedFiles } from "./git.ts";
import { generateCommitMessage } from "./generator.ts";

await new Command()
  .name("justcommit")
  .description("AI powered commit message generator")
  .action(async () => {
    const token = Deno.env.get("OPENAI_TOKEN");
    if (!token) {
      throw new Error("OPENAI_TOKEN is not set");
    }

    const diff = await readDiffStagedFiles();
    const commitMessage = await generateCommitMessage(token, diff);

    console.log(commitMessage);
  })
  .parse(Deno.args);
