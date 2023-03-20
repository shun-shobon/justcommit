import { path, TOML, z } from "./deps.ts";
import { credentialSchema } from "./credential.ts";

const CONFIG_DIR_NAME = "justcommit";
const CONFIG_FILE_NAME = "config.toml";

export async function readConfig(): Promise<Config | null> {
  const rawConfig = await Deno.readTextFile(getConfigPath());
  const tomlConfig = TOML.parse(rawConfig);
  const config = configSchema.parse(tomlConfig);
  return config;
}

export function getConfigDir(): string {
  const configHome = Deno.env.get("XDG_CONFIG_HOME") ??
    path.join(Deno.env.get("HOME") ?? "/", ".local", "share");

  return path.join(configHome, CONFIG_DIR_NAME);
}

export function getConfigPath(): string {
  return path.join(getConfigDir(), CONFIG_FILE_NAME);
}

export type Config = z.infer<typeof configSchema>;

const configSchema = z.object({
  credential: credentialSchema,
});
