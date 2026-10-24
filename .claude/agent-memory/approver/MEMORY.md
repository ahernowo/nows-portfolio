# Approver memory

What good evidence looks like, and recurring reasons to send work back. Keep under 150 lines; merge and prune.

## Good evidence
- A check named with its result ("cargo test --workspace: 65 passed"), a screenshot path, or a file and line.
- Template changes: grep the built HTML in `site/public/` yourself (Zola minifies, so attributes are unquoted, e.g. `class=pager`, `content=#FDFFFF`, and attribute order changes; grep on text like `More writing` not `<nav class=pager`).
- Draft-dependent checks: `zola --root site build --drafts --output-dir <scratchpad>/drafts --force` keeps `site/public` and the seal untouched.
- Design crops live in `target/shots/<WO>-design*/` (rework rounds add `-r1`); open one or two (incl. a focus crop) rather than trusting the description.
- Builds into `site/public` and `target/` do not break the build seal; `factory verify` after your runs confirms it.
- Palette/contrast orders: recompute the contrast table and tint mixes with a small python -I script in the scratchpad (WCAG luminance); takes a minute and checks every number in plan, build and docs.
- Image changes (og.png): `git show HEAD:<path> > target/...` and Read both; tells intended overlap from a regression.
- Dashboard CSS is `include_str!`-compiled: a `factory serve` already on 4600 serves the OLD CSS. Run `cargo run -q -p factory -- serve --port 4617` in background, curl, then stop by PID (netstat + `taskkill //PID n //F`).

## Recurring reasons to send back
- None sent back by me yet (WO-0001, WO-0002 approved). Reworks so far came from design review: a contrast miss on an element hidden at opacity 0 (WO-0002 #1, token meant for borders used as text colour).

## Things to watch
- `git status` shows agent-memory, skills and `.factory/` changes alongside the site diff; they are expected, not scope creep. Tell Nows which files are the real change.
- shots `--focus` stops after 10 tab stops; footer focus claims need a manual check (lesson 2026-10-24).
- Builder "changes beyond the plan" section: accept when disclosed with a reason and within the request (WO-0002: tabular-nums scoping, form letter-spacing).
- Tell Nows when a change needs a worker redeploy (cms-api admin CSS) or a dashboard restart to be visible.
