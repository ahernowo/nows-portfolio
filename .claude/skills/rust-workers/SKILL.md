---
name: rust-workers
description: Conventions for the Rust workspace (cms-core, the two Cloudflare Workers, the newpost and factory tools), wasm builds, wrangler, D1 and secrets. Use when writing or reviewing Rust, Cargo.toml, wrangler.toml or D1 migrations.
---

# Rust in this repo

Nows is learning Rust here. Write code a learner can read top to bottom: small functions, named types over tuples, `?` for errors, and a one-line comment where a Rust idea carries the logic.

## The workspace

| Crate | Kind | Role |
|---|---|---|
| `crates/cms-core` | library | All rules: slugs, dates, article validation, TOML front matter, signed sessions, OAuth helpers, safe Markdown preview. Pure: no network, no clock, no randomness. |
| `workers/decap-oauth` | Worker (wasm) | GitHub sign-in for Decap CMS: `/auth`, `/callback`. |
| `workers/cms-api` | Worker (wasm) + D1 | The stage B CMS: admin page, sessions, drafts in D1, publish to git through the GitHub API. |
| `tools/newpost` | CLI | Starts articles and case studies; `--check` lints all content. |
| `tools/factory` | CLI + local web server | The factory board, gates, hooks and dashboard. |

Shared versions live in the root `Cargo.toml` under `[workspace.dependencies]`; a crate says `serde.workspace = true`. Workspace lints forbid `unsafe` and warn on `unwrap`, `dbg!` and `todo!`.

**Where code goes**: a rule (what counts as a valid slug, how long a session lasts, what front matter looks like) goes in `cms-core` with a unit test. A worker reads the request, calls `cms-core`, talks to D1 or GitHub, and shapes the response. That split is what makes the workers testable: `cargo test` runs natively, and workers have no test runner.

## Errors

- Libraries return `Result<T, E>` with a small error type or a `FieldError { field, message }`; messages say what to do ("Use 3 to 120 characters").
- Workers convert everything to `ApiError` (`workers/cms-api/src/http.rs`), which maps to a status code and a JSON body. Internal details go to `console_error!`, never into the response.
- Production code uses `?`, `ok_or`, `map_err`; `unwrap` and `expect` appear only in tests.

## Workers (worker 0.8)

- Entry point: `#[event(fetch)] async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response>`; routing is a `match` on method and path segments.
- Config: `env.var("NAME")` for `[vars]` in `wrangler.toml`; `env.secret("NAME")` for secrets.
- D1: `env.d1("DB")`, then `db.prepare(sql).bind(&[...])?` with `?1, ?2` placeholders. Every value goes through `bind`; SQL strings are constants. Queries live in `db.rs`.
- Redirects that set cookies: build the 302 by hand with `Headers` (the headers of `Response::redirect` are immutable).
- Randomness: `getrandom` with the `js` feature (state values, ids).
- HTML served by a worker is `include_str!`-ed from `admin/` and sent with the strict `ADMIN_CSP` (same-origin scripts and styles only), so scripts and styles stay in their own files. The one inline script, the Decap sign-in popup in `decap-oauth`, carries a per-response nonce.
- Every response passes through `http::finish`, which adds the security headers.

## Building and running

| Task | Command |
|---|---|
| Check natively (fast) | `cargo clippy --workspace --all-targets -- -D warnings` |
| Check for wasm | `cargo check -p cms-api --target wasm32-unknown-unknown` (needs `rustup target add wasm32-unknown-unknown`) |
| Run a worker locally | `npx wrangler dev` inside the worker folder; local values in `.dev.vars` (copy `.dev.vars.example`) |
| Create the database | `npx wrangler d1 create portfolio-cms`, then paste the id into `wrangler.toml` |
| Apply migrations | `npx wrangler d1 migrations apply portfolio-cms --local` (or `--remote`) |
| Set a secret | `npx wrangler secret put NAME` |
| Deploy | `npx wrangler deploy` (Nows runs this; see the `release` skill) |

`wrangler` runs the `[build] command`, which installs `worker-build` 0.8.7 and produces `build/index.js` plus the wasm file. Keep `worker` and `worker-build` on matching versions.

## D1

- Schema changes are new files in `workers/cms-api/migrations/` (`0002_add_x.sql`), never edits to an applied one.
- Optimistic locking: updates check `version` and bump it; a mismatch is a 409 with a message the editor shows as is.
- Free plan limits (enforced since September 2026): 5 million rows read and 100,000 rows written per day, 5 GB storage. Queries use the indexes in `0001_create_articles.sql` (`articles_updated`, `articles_published`, the unique `slug`) and every list has a `LIMIT`.

## Secrets

Secrets: `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, `SESSION_KEY` (32+ random bytes, base64), `GITHUB_CONTENT_TOKEN` (fine-grained, Contents read and write on the one repo). They live in `wrangler secret` and in `.dev.vars` for local runs; `.dev.vars` is git-ignored and the project settings stop agents from reading it. Code reads them with `env.secret`, compares tokens with `constant_time_eq`, and never logs them.

## Before you finish

- `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` are green.
- New rules in `cms-core` have tests for the edge cases (empty, too long, unicode, hostile input).
- A new dependency is added to `[workspace.dependencies]` with default features off when it has heavy defaults, and the build report says why it is needed.
