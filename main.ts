import { serve } from "./deps.ts";

const PORT = 1993;

const s = serve(`0.0.0.0:${PORT}`);
const greeting = new TextEncoder().encode("v0.0.4");

console.log(`Server started on port ${PORT}`);

for await (const req of s) {
  req.respond({ body: greeting });
}
