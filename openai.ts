// Based on: https://platform.openai.com/docs/api-reference/chat/create

import { z } from "./deps.ts";

const messageRoleSchema = z.union([
  z.literal("system"),
  z.literal("user"),
  z.literal("assistant"),
]);

export type MessageRole = z.infer<typeof messageRoleSchema>;

const messageSchema = z.object({
  role: messageRoleSchema,
  content: z.string(),
});

export type Message = z.infer<typeof messageSchema>;

const chatCompletionRequestSchema = z.object({
  model: z.literal("gpt-3.5-turbo"),
  messages: z.array(messageSchema),
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

export type ChatCompletionRequest = z.infer<typeof chatCompletionRequestSchema>;

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

export type ChatCompletionResponse = z.infer<
  typeof chatCompletionResponseSchema
>;

export class OpenAiError extends Error {
  constructor(message: string, option: ErrorOptions) {
    super(message, option);

    this.name = "OpenAiError";
  }
}

export class OpenAiClient {
  static readonly ORIGIN = "https://api.openai.com";
  static readonly CHAT_COMPLETION_PATH = "/v1/chat/completions";

  readonly #token: string;

  constructor(token: string) {
    this.#token = token;
  }

  async chatCompletion(
    request: ChatCompletionRequest,
  ): Promise<ChatCompletionResponse> {
    const url = new URL(OpenAiClient.CHAT_COMPLETION_PATH, OpenAiClient.ORIGIN);

    return await this.#sendRequest(url, chatCompletionResponseSchema, request);
  }

  async #sendRequest<T extends z.ZodSchema>(
    url: URL,
    responseSchema: T,
    body: unknown,
  ): Promise<z.infer<T>> {
    const res = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.#token}`,
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
}
