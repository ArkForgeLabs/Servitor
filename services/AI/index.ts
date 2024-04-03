// import
import ollama from "ollama";
import { verify_json } from "../utils";

// input type
export interface Input {
  instruction: string;
  input: string;
}

// server
Bun.serve({
  port: 8080,
  hostname: "localhost",
  // server methods
  async fetch(req) {
    const url = new URL(req.url);
    // get the input structure
    if (url.pathname === "/input_structure") {
      return new Response(
        JSON.stringify({ instruction: "string", input: "string" })
      );
    }
    // post the input to ollama
    if (url.pathname === "/input" && req.method === "POST") {
      let text = await req.text();
      const input = JSON.parse(text) as Input;
      // verify the input to be of the correct type
      if (!verify_json(input, ["instruction", "input"])) {
        return new Response("Bad Request", { status: 400 });
      }

      // generate response
      const response = await ollama.generate({
        model: "hermes-mistral:instruct",
        prompt: `
<|im_start|>system
${input.instruction}<|im_end|>
<|im_start|>user
${input.input}<|im_end|>
<|im_start|>assistant`,
      });

      // return the response
      return new Response(JSON.stringify({ output: response }));
    }
    // 404
    return new Response("404!");
  },
});
