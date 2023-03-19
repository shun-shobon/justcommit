const token = Deno.env.get("OPENAI_TOKEN");
const url = new URL("https://api.openai.com/v1/chat/completions");
const res = await fetch(url, {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
    Authorization: `Bearer ${token}`,
  },
  body: JSON.stringify({
    "model": "gpt-3.5-turbo",
    "messages": [{ "role": "user", "content": "Say this is a test!" }],
    "temperature": 0.7,
  }),
});
const data = await res.json();
console.log(data);
