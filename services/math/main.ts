import { req, res, Server } from "https://deno.land/x/faster/mod.ts";

const server = new Server();
// deno-lint-ignore no-explicit-any
server.get("/info", res("json"), async (ctx: any, next: any) => {
  ctx.res.body = {
    name: "Math",
    description: "test",
    version: "1.0.0",
    input_structure: {},
    output_structure: {},
  };
  await next();
});

// deno-lint-ignore no-explicit-any
server.get("/heartbeat", async (ctx: any, next: any) => {
  ctx.res.status = 200;
  ctx.res.body = { message: "Server is running" };
  await next();
});

await server.listen({ port: 3000 });
