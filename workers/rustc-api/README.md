# rustviz-compile-proxy

Small Cloudflare Worker that forwards `POST` bodies to `https://play.rust-lang.org/execute` and adds CORS headers so the mdBook playground can call it from a static origin when the browser cannot reach the Playground directly.

## Usage

1. `npm install` then `npx wrangler deploy`
2. Copy the deployed URL (e.g. `https://rustviz-compile-proxy.<subdomain>.workers.dev`)
3. In [`mdbook/playground.md`](../mdbook/playground.md), set on the root element:

   `data-compile-endpoint="https://…/execute"` — **note:** the Worker forwards to `/execute` on the playground; your Worker URL is the fetch target (no extra path).

4. Rebuild the book (`mdbook build` or `./mdbook/generate_svg.sh`).

If `data-compile-endpoint` is empty, the playground calls `https://play.rust-lang.org/execute` directly (works when CORS allows it).
