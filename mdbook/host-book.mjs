#!/usr/bin/env node
/**
 * Serves the mdBook output directory (`./book/`) for local preview.
 * No dependencies — uses Node built-ins only (Node 18+).
 *
 * Usage (from mdbook/):  node host-book.mjs
 * Env: PORT (default 8000)
 */
import http from "node:http";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, "book");
const PORT = Number(process.env.PORT) || 8000;

const MIME = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".svg": "image/svg+xml",
  ".wasm": "application/wasm",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".gif": "image/gif",
  ".webp": "image/webp",
  ".ico": "image/x-icon",
  ".json": "application/json",
  ".woff2": "font/woff2",
  ".woff": "font/woff",
  ".ttf": "font/ttf",
  ".map": "application/json",
};

function safeResolve(root, urlPath) {
  const decoded = decodeURIComponent(urlPath.split("?")[0]);
  const joined = path.join(root, decoded);
  const resolved = path.resolve(joined);
  if (!resolved.startsWith(root)) {
    return null;
  }
  return resolved;
}

const server = http.createServer(async (req, res) => {
  if (req.method !== "GET" && req.method !== "HEAD") {
    res.writeHead(405);
    res.end();
    return;
  }

  try {
    let pathname = new URL(req.url || "/", "http://localhost").pathname;
    if (pathname === "/") {
      pathname = "/index.html";
    }

    let filePath = safeResolve(ROOT, pathname);
    if (!filePath) {
      res.writeHead(403);
      res.end("Forbidden");
      return;
    }

    let stat;
    try {
      stat = await fs.stat(filePath);
    } catch {
      res.writeHead(404);
      res.end("Not found");
      return;
    }

    if (stat.isDirectory()) {
      filePath = path.join(filePath, "index.html");
      try {
        stat = await fs.stat(filePath);
      } catch {
        res.writeHead(404);
        res.end("Not found");
        return;
      }
    }

    const ext = path.extname(filePath);
    const type = MIME[ext] || "application/octet-stream";
    const body = await fs.readFile(filePath);

    res.writeHead(200, {
      "Content-Type": type,
      "Content-Length": body.length,
    });
    if (req.method === "HEAD") {
      res.end();
    } else {
      res.end(body);
    }
  } catch (e) {
    console.error(e);
    res.writeHead(500);
    res.end("Internal error");
  }
});

server.listen(PORT, () => {
  console.log(`Serving ${ROOT}`);
  console.log(`Open http://127.0.0.1:${PORT}/`);
});
