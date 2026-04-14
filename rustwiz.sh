#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

usage() {
  cat <<'EOF'
Usage: ./rustwiz.sh <command>

Commands:
  clean   Remove generated build outputs for a fresh rebuild
  build   Build wasm package and regenerate SVG/book outputs
  start   Start local website server (mdbook/host-book.mjs)
EOF
}

clean_builds() {
  echo "== cargo clean (workspace) =="
  cargo clean

  if [[ -f parse/Cargo.toml ]]; then
    echo "== cargo clean (parse) =="
    (cd parse && cargo clean)
  fi

  echo "== remove generated example SVGs =="
  shopt -s nullglob globstar
  for f in src/examples/**/vis_code.svg src/examples/**/vis_timeline.svg; do
    rm -f "$f"
    echo "removed $f"
  done
  shopt -u globstar nullglob

  echo "== remove mdBook output and generated sources =="
  rm -rf mdbook/book
  rm -rf mdbook/src

  echo "== remove copied theme/js wasm assets =="
  rm -f mdbook/theme/book.js
  rm -rf mdbook/theme/pkg

  echo ""
  echo "Clean finished."
}

build_all() {
  echo "== build wasm =="
  bash wasm/build.sh
  echo "== regenerate examples and mdBook =="
  bash mdbook/generate_svg.sh
  echo ""
  echo "Build finished."
}

start_site() {
  echo "== start local server =="
  cd mdbook
  node host-book.mjs
}

cmd="${1:-}"
case "$cmd" in
  clean)
    clean_builds
    ;;
  build)
    build_all
    ;;
  start)
    start_site
    ;;
  *)
    usage
    exit 1
    ;;
esac
