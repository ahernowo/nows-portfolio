---
name: release
description: Check everything and walk through deploying the site and, when they changed, the workers. Nows approves each push and deploy.
disable-model-invocation: true
---

# Release

Deploying is Nows's decision at every step. You check, explain what will go live, and run each deploying command only when Nows approves it (the project settings make those commands ask).

## 1. Line and tree

1. `factory status`. Open orders or a stopped line are worth a sentence to Nows before shipping; a stopped line means the release waits.
2. `git status --short` and `git branch --show-current`. Uncommitted changes: show `git diff --stat`, and ask Nows whether to commit them first (with the approver's suggested message when an order produced them) or release without them.

Done when the tree is clean on `main`, or Nows has said what to do with the changes.

## 2. Checks

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -q -p newpost -- --check
zola --root site build
zola --root site check --skip-external-links
```

Any red check stops the release: report it, and offer `/factory` to fix it.

## 3. What will go live

Find the last release tag with `git describe --tags --abbrev=0 --match "release-*"` (none means this is the first release). Then `git diff --stat <tag>..HEAD` and group the changes:

- `site/`: goes live when `main` is pushed (Cloudflare Pages rebuilds).
- `workers/decap-oauth/`, `workers/cms-api/`, `crates/cms-core/`: need a worker deploy.
- `workers/cms-api/migrations/`: new files need a remote migration, applied before the worker deploy.

Tell Nows the list in plain words.

## 4. Deploy, in order, each with Nows's approval

1. Push: `git push origin main`. Cloudflare Pages builds the site; the build log is in the Cloudflare dashboard under Workers & Pages.
2. New migrations: `npx wrangler d1 migrations apply portfolio-cms --remote`, run inside `workers/cms-api`.
3. Changed workers: `npx wrangler deploy` inside each changed worker's folder.
4. Tag the release: `git tag -a release-YYYY-MM-DD -m "<one line>"`, then `git push origin --tags`.

## 5. Smoke test

When the Pages build has finished (a few minutes), with the live address from `site/zola.toml` `base_url`:

- `curl -sI <base_url>` shows `200` and a `content-security-policy` header.
- `curl -s <base_url>/atom.xml` starts with `<?xml`.
- For a deployed worker: `curl -s -o /dev/null -w "%{http_code}" <worker-url>/admin` returns `200` (cms-api), or `<worker-url>/auth` returns `302` (decap-oauth).

Report the results and anything Nows should click through by hand: the home page, one article, `/admin/` sign-in.
