# RustViz Web

[RustViz Website](https://master.rustviz-web.pages.dev/)
Minimal instructions for building, changing, and running this website.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [mdBook v0.4.40](https://rust-lang.github.io/mdBook/guide/installation.html): `cargo install mdbook --version 0.4.40`
- [Node.js](https://nodejs.org/) 18+ (for local hosting)
- [wasm-pack](https://rustwasm.github.io/docs/book/game-of-life/setup.html) and `wasm32-unknown-unknown` target:
  `rustup target add wasm32-unknown-unknown`

## Build after changes

From the repository root:

```bash
./rustviz.sh build
```

This regenerates SVGs, rebuilds mdBook, and updates published WASM assets.

## Run website locally

```bash
./rustviz.sh start
```

Open [http://127.0.0.1:8000](http://127.0.0.1:8000).
Use `PORT=9000 ./rustviz.sh start` to change port.

## Deploy website (latest changes)

### Requirements

- [Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [Cloudflare account](https://dash.cloudflare.com/)
- Login to Cloudflare CLI: `npx wrangler login`

```bash
npx wrangler pages deploy --project-name rustviz-website --branch main
# Note: --branch main is optional, default branch is used by Cloudflare Pages
```

## Update Cloudflare Worker (Rust Playground API for Compile)

```bash
cd workers/rustc-api
npx wrangler deploy
```

## Clean generated outputs

From the repository root:

```bash
./rustviz.sh clean
```
