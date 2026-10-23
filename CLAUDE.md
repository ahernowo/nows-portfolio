# Nows portfolio

Nows's portfolio: a static Zola site with articles and case studies, a GitHub sign-in worker for Decap CMS (stage A), and a Rust CMS on Cloudflare Workers + D1 (stage B). Everything runs on free tiers. Nows is learning Rust through this repo, so explain Rust choices in plain words when you make them.

Nows works on Windows 11. Commands here run in Git Bash (Claude Code's shell on Windows) and in the CI's Linux.

## Map

- `site/`: the Zola site (`zola.toml`, `content/`, `templates/`, `sass/`, `static/`; `static/admin/` is Decap CMS, `static/_headers` the security headers).
- `crates/cms-core/`: the rules shared by everything: validation, slugs, front matter, sessions, safe Markdown preview. Pure Rust, unit-tested.
- `workers/decap-oauth/`, `workers/cms-api/`: the two Rust Workers.
- `tools/newpost/`: starts articles and case studies, and checks content.
- `tools/factory/`: the production line's CLI and dashboard.
- `tools/shots/`: screenshots and accessibility scans of the built site.
- `docs/`: guides for Nows. Start at `docs/setup.md`.

Path-scoped notes in `.claude/rules/` load when you touch `site/`, content, or Rust; each names the skill to load.

## Checks

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
zola --root site build
cargo run -q -p newpost -- --check   # after content changes
```

CI runs the same set on every push and pull request.

## Ground rules

- Secrets live in `wrangler secret` and git-ignored `.dev.vars` files. Keep them out of code, logs, reports and messages.
- Write about Nows only from what Nows has said or what is in this repo; mark a missing fact with `<!-- Nows: ... -->`.
- Committing, pushing and deploying are Nows's steps. Suggest a commit message; Nows runs the command or approves it.

## The factory

Changes go through a line of agents. Nows starts an order with `/factory <what to change>`, resumes one with `/factory-resume <WO-id>`, ships with `/release`, and improves the line with `/kaizen`.

Stations, in order: `planner` → `builder` → `ui-ux-reviewer` → `security-checker` → `approver`. Each station records its work with the `factory` CLI (the `factory-protocol` skill); `.factory/` is written by the CLI alone. A critical finding or a fourth rework pulls the **andon** and stops the line until Nows clears it. An approved order is left uncommitted for Nows to review.

For a small question or a one-line fix, work directly; the line is for changes worth a review. Dashboard: `factory serve`, then http://127.0.0.1:4600.
