import { OpenAiClient } from "./openai.ts";

const token = Deno.env.get("OPENAI_TOKEN");
if (!token) {
  throw new Error("OPENAI_TOKEN is not set");
}

const openAi = new OpenAiClient(token);

const data = await openAi.chatCompletion({
  model: "gpt-3.5-turbo",
  messages: [
    {
      role: "user",
      content: "Say this is a test!",
    },
  ],
});

console.log(data);
