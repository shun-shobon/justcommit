import { z } from "./deps.ts";
import { runCommand } from "./utils.ts";

export async function readToken(credential: Credential): Promise<string> {
  switch (credential.mode) {
    case "plain":
      return credential.token;
    case "1password":
      return await runCommand("op", "read", credential.token);
  }
}

export type Credential = z.infer<typeof credentialSchema>;

export const credentialSchema = z.object({
  mode: z.enum(["plain", "1password"]),
  token: z.string(),
});
