# Approver memory

What good evidence looks like, and recurring reasons to send work back. Keep under 150 lines; merge and prune.

## Good evidence
- A check named with its result ("cargo test --workspace: 63 passed"), a screenshot path, or a file and line.
- Template changes: grep the built HTML in `site/public/` yourself (Zola minifies, so attributes are unquoted, e.g. `class=pager`, and attribute order changes; grep on text like `More writing` not `<nav class=pager`).
- Draft-dependent checks: `zola --root site build --drafts --output-dir <scratchpad>/drafts --force` keeps `site/public` and the seal untouched.
- Design crops live in `target/shots/<WO>-design/`; open one or two (incl. a focus crop) rather than trusting the description.
- Builds into `site/public` and `target/` do not break the build seal; `factory verify` after your runs confirms it.

## Recurring reasons to send back
- (none yet; WO-0001 approved clean, first order)

## Things to watch
- `git status` shows agent-memory and `.factory/` changes alongside the site diff; they are expected, not scope creep. Tell Nows which files are the real change.
- shots `--focus` stops after 10 tab stops; footer focus claims need a manual check (lesson saved 2026-10-24).
