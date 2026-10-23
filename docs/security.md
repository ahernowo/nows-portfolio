# Security

A portfolio is a small target, but this one holds GitHub credentials: anyone who steals a session or a token can publish on the site under my name. And AI agents run commands in the repository. The design keeps both risks small.

## What protects what

| Risk | Protection | Where |
|---|---|---|
| Script injection on the site | Static HTML; a strict Content Security Policy (same-origin scripts and styles only, no inline code); Tera escaping by default | `site/static/_headers`, templates |
| Clickjacking, sniffing, leaking referrers | `X-Frame-Options: DENY`, `frame-ancestors 'none'`, `nosniff`, `Referrer-Policy`, `Permissions-Policy`, HSTS | `site/static/_headers` |
| A tampered editor script | Decap pinned to one version with a Subresource Integrity hash; looser CSP only under `/admin/` | `site/static/admin/index.html` |
| Stolen sign-in (OAuth) | Random `state` in an HttpOnly cookie, compared in constant time; the token is posted only to the exact site origin | `workers/decap-oauth`, `crates/cms-core/src/oauth.rs` |
| Stolen CMS session | HMAC-signed session, 8 hours, `HttpOnly; Secure; SameSite=Strict`; the GitHub login is re-checked against an allowlist on every request | `crates/cms-core/src/session.rs`, `workers/cms-api/src/auth.rs` |
| Cross-site requests | Custom `X-CMS-Request` header, `Sec-Fetch-Site` and `Origin` checks on every write | `workers/cms-api/src/auth.rs` |
| SQL injection | Every query is a constant with bound parameters | `workers/cms-api/src/db.rs` |
| Malicious article content | Input limits, slug rules, TOML written by a serialiser; the CMS preview turns raw HTML into text and neutralises `javascript:` links | `crates/cms-core` |
| Over-powered tokens | GitHub token limited to Contents on one repository; OAuth scope `public_repo` | Setup steps 6 and 7 |
| Leaked secrets | Secrets only in `wrangler secret` and git-ignored `.dev.vars`; agents are denied reading them; the security checker scans every change | `.gitignore`, `.claude/settings.json` |
| An agent doing too much | Reviewers can't edit project files; the builder can't edit the board, agents or settings; commits, pushes and deploys ask you first; the build seal catches changes after review | `.claude/`, `tools/factory` |
| A vulnerable dependency | Workspace-pinned versions; the security checker looks up each new crate; CI builds from the lock file | `Cargo.toml`, `Cargo.lock` |

## Checking it yourself

- https://securityheaders.com and https://observatory.mozilla.org on your live address.
- `zola --root site build`, then `node tools/shots/shoot.mjs`: any CSP violation shows up as a console problem.
- `cargo install cargo-audit --locked`, then `cargo audit` for known vulnerabilities in dependencies.

## If a secret leaks

1. Rotate it first: a new OAuth client secret on GitHub, a new fine-grained token, or a new `SESSION_KEY`, then `npx wrangler secret put <NAME>` and `npx wrangler deploy`.
2. Check GitHub's security log and the CMS **Activity** list for anything you didn't do.
3. Then clean the history if it was committed. Once pushed, assume it was copied.

## Reporting a problem

On GitHub, enable **Settings → Code security → Private vulnerability reporting**, so people can report issues privately from the repository's Security tab. `SECURITY.md` says the same to visitors.
