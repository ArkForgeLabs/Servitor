import { req, res, Server } from "https://deno.land/x/faster/mod.ts";

const server = new Server();
server.get("/", res("html"), async (ctx: any, next: any) => {
  ctx.res.body = `
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="utf-8">
          <title>title example</title>
        </head>
        </body>
          HTML body example
        <body>
      </html>
    `;
  await next();
});

await server.listen({ port: 3000 });
