import { req, res, Server } from "https://deno.land/x/faster@v8.7/mod.ts";

async function discord_webhook(
  url: string,
  content: string,
  username: string = "webhook node notification",
  avatar_url: string = "https://arkforge.net/logo.png"
) {
  await fetch(url, {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      // the username to be displayed
      username,
      // the avatar to be displayed
      avatar_url,
      // contents of the message to be sent
      content,
    }),
  });
}

interface Input {
  url: string;
  method: string;
  headers: object;
  body: string;
}

const server = new Server();

// deno-lint-ignore no-explicit-any
server.get("/heartbeat", async (ctx: any, next: any) => {
  ctx.res.status = 200;
  ctx.res.body = { message: "Server is running" };
  await next();
});

// deno-lint-ignore no-explicit-any
server.get("/info", res("json"), async (ctx: any, next: any) => {
  ctx.res.body = {
    name: "Webhook",
    description: "A simple webhook service that can send messages to webhooks",
    version: "1.0.0",
    input_structure: {
      url: "string",
      method: "string",
      headers: "object",
      body: "string",
    },
    output_structure: {},
  };
  await next();
});

// deno-lint-ignore no-explicit-any
server.post("/input", req("json"), async (ctx: any, next: any) => {
  try {
    // verify the input data is correct and safe before processing it.
    const input: Input = ctx.body;
    if (
      !input ||
      !input.url ||
      !input.body ||
      !input.method ||
      !input.headers
    ) {
      ctx.res.status = 400;
      throw new Error("Invalid input");
    }

    console.log(input);

    if (input.url.includes("discord")) {
      discord_webhook(
        "https://discord.com/api/webhooks/1248661489678684210/MEF5C4ZfU-_kzzSvMc7VZC6S1Yr8b6uWkxKZq6e2Cgh7xhwTFCVh64Oj5049rUFENGK_",
        input.body
      );
      ctx.res.status = 200;
    } else {
      console.log("Not a discord webhook");
      ctx.res.status = 401;
    }
  } catch (err) {
    console.error(err);
    ctx.res.body = { error: err.message || "Internal Server Error" };
  }

  await next();
});

await server.listen({ port: 3000 });
