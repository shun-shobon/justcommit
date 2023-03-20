export async function runCommand(
  command: string,
  ...args: string[]
): Promise<string> {
  const decoder = new TextDecoder("utf-8");

  const { code, stderr, stdout } = await new Deno
    .Command(command, { args })
    .output();

  if (code !== 0) throw new Error(decoder.decode(stderr));

  return decoder.decode(stdout);
}
