// Based on: https://platform.openai.com/docs/api-reference/chat/create

import { z } from "./deps.ts";

const ORIGIN = "https://api.openai.com";
const CHAT_COMPLETION_PATH = "/v1/chat/completions";

export async function chatCompletion(
  token: string,
  request: ChatCompletionRequest,
): Promise<ChatCompletionResponse> {
  const url = new URL(CHAT_COMPLETION_PATH, ORIGIN);

  return await sendRequest(url, token, chatCompletionResponseSchema, request);
}

async function sendRequest<T extends z.ZodSchema>(
  url: URL,
  token: string,
  responseSchema: T,
  body: unknown,
): Promise<z.infer<T>> {
  const res = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(body),
  });
  const data = await res.json();

  if (!res.ok) {
    throw new OpenAiError(
      `Request failed: ${res.status} ${res.statusText}`,
      {
        cause: data,
      },
    );
  }

  try {
    return responseSchema.parse(data);
  } catch (err) {
    throw new OpenAiError(
      `Failed to parse response`,
      {
        cause: err,
      },
    );
  }
}

export type MessageRole = z.infer<typeof messageRoleSchema>;
export type Message = z.infer<typeof messageSchema>;
export type ChatCompletionRequest = z.infer<typeof chatCompletionRequestSchema>;
export type ChatCompletionResponse = z.infer<
  typeof chatCompletionResponseSchema
>;

const messageRoleSchema = z.union([
  z.literal("system"),
  z.literal("user"),
  z.literal("assistant"),
]);

const messageSchema = z.object({
  role: messageRoleSchema,
  content: z.string(),
});

const chatCompletionRequestSchema = z.object({
  model: z.literal("gpt-3.5-turbo"),
  messages: z.array(messageSchema),
  temperature: z.number().optional(),
  top_p: z.number().optional(),
  n: z.number().optional(),
  steram: z.boolean().optional(),
  stop: z.union([z.array(z.string()), z.string()]).optional(),
  max_tokens: z.number().optional(),
  presence_penalty: z.number().optional(),
  frequency_penalty: z.number().optional(),
  logit_bias: z.unknown().optional(),
  user: z.string().optional(),
});

const chatCompletionResponseSchema = z.object({
  id: z.string(),
  object: z.literal("chat.completion"),
  created: z.number(),
  model: z.string(),
  choices: z.array(
    z.object({
      message: messageSchema,
      finish_reason: z.union([
        z.literal("stop"),
        z.literal("length"),
        z.literal("content_filter"),
        z.literal("null"),
      ]),
      index: z.number(),
    }),
  ).nonempty(),
});

export class OpenAiError extends Error {
  constructor(message: string, option: ErrorOptions) {
    super(message, option);

    this.name = "OpenAiError";
  }
}
