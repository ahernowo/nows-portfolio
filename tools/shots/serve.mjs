// A small static file server for the built site that applies the same
// headers Cloudflare Pages would, read from site/public/_headers. That way
// screenshots run under the real Content Security Policy, and a blocked
// inline script shows up here before it shows up in production.
//
// HSTS and upgrade-insecure-requests are dropped, because this server is
// plain http on localhost.

import http from "node:http";
import fs from "node:fs";
import path from "node:path";

const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".avif": "image/avif",
  ".mp4": "video/mp4",
  ".woff2": "font/woff2",
  ".xml": "application/xml",
  ".txt": "text/plain; charset=utf-8",
  ".yml": "text/yaml; charset=utf-8",
};

/** Parse a Cloudflare Pages `_headers` file into rules. */
export function parseHeaders(text) {
  const rules = [];
  let current = null;
  for (const line of text.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    if (!/^\s/.test(line)) {
      current = { pattern: trimmed, headers: [] };
      rules.push(current);
      continue;
    }
    if (!current) continue;
    if (trimmed.startsWith("! ")) {
      current.headers.push([trimmed.slice(2).trim().toLowerCase(), null]);
    } else {
      const i = trimmed.indexOf(":");
      if (i > 0) current.headers.push([trimmed.slice(0, i).trim().toLowerCase(), trimmed.slice(i + 1).trim()]);
    }
  }
  return rules;
}

function patternToRegExp(pattern) {
  const escaped = pattern.replace(/[.+?^${}()|[\]\\]/g, "\\$&").replace(/\*/g, ".*");
  return new RegExp("^" + escaped + "$");
}

/** The headers for one path: rules apply in file order, `! Name` removes. */
export function headersFor(rules, urlPath) {
  const out = {};
  for (const rule of rules) {
    if (!patternToRegExp(rule.pattern).test(urlPath)) continue;
    for (const [name, value] of rule.headers) {
      if (value === null) delete out[name];
      else out[name] = out[name] ? `${out[name]}, ${value}` : value;
    }
  }
  delete out["strict-transport-security"];
  if (out["content-security-policy"]) {
    out["content-security-policy"] = out["content-security-policy"].replace(/;\s*upgrade-insecure-requests/, "");
  }
  return out;
}

/** Serve `root` on 127.0.0.1. Resolves to { server, url }. Port 0 picks a free one. */
export function startServer(root, port = 0) {
  const headersFile = path.join(root, "_headers");
  const rules = fs.existsSync(headersFile) ? parseHeaders(fs.readFileSync(headersFile, "utf8")) : [];
  const realRoot = fs.realpathSync(root);

  const server = http.createServer((req, res) => {
    let urlPath;
    try {
      urlPath = decodeURIComponent(new URL(req.url, "http://localhost").pathname);
    } catch {
      res.writeHead(400).end();
      return;
    }
    let file = path.join(realRoot, urlPath);
    if (file !== realRoot && !file.startsWith(realRoot + path.sep)) {
      res.writeHead(403).end();
      return;
    }
    if (fs.existsSync(file) && fs.statSync(file).isDirectory()) file = path.join(file, "index.html");
    let status = 200;
    if (!fs.existsSync(file)) {
      file = path.join(realRoot, "404.html");
      status = 404;
      if (!fs.existsSync(file)) {
        res.writeHead(404, { "content-type": "text/plain; charset=utf-8" }).end("Not found");
        return;
      }
    }
    const type = TYPES[path.extname(file).toLowerCase()] || "application/octet-stream";
    res.writeHead(status, { "content-type": type, ...headersFor(rules, urlPath) });
    fs.createReadStream(file)
      .on("error", () => res.destroy())
      .pipe(res);
  });

  return new Promise((resolve) => {
    server.listen(port, "127.0.0.1", () => {
      const { port: actual } = server.address();
      resolve({ server, url: `http://127.0.0.1:${actual}` });
    });
  });
}

// Run directly: node tools/shots/serve.mjs [site/public] [port]
if (import.meta.url === `file://${process.argv[1]}` || process.argv[1]?.endsWith("serve.mjs")) {
  const root = process.argv[2] || "site/public";
  const port = Number(process.argv[3] || 1112);
  const { url } = await startServer(root, port);
  console.log(`Serving ${root} with its _headers at ${url}`);
}
