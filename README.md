# RustViz Web

Minimal instructions for building, changing, and running this website.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html): `cargo install mdbook --locked`
- [Node.js](https://nodejs.org/) 18+ (for local hosting)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and `wasm32-unknown-unknown` target:
  `rustup target add wasm32-unknown-unknown`

## Build after changes

From the repository root:

```bash
./rustwiz.sh build
```

This regenerates SVGs, rebuilds mdBook, and updates published WASM assets.

## Run website locally

```bash
./rustwiz.sh start
```

Open [http://127.0.0.1:8000](http://127.0.0.1:8000).
Use `PORT=9000 ./rustwiz.sh start` to change port.

## Clean generated outputs

From the repository root:

```bash
./rustwiz.sh clean
```
