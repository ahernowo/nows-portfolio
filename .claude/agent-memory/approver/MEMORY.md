# Approver memory

What good evidence looks like, and recurring reasons to send work back. Keep under 150 lines; merge and prune.

## Good evidence
- A check named with its result ("cargo test --workspace: 65 passed"), a screenshot path, or a file and line.
- Template changes: grep the built HTML in `site/public/` yourself (Zola minifies, so attributes are unquoted, e.g. `class=pager`, `content=#FDFFFF`, and attribute order changes; grep on text like `More writing` not `<nav class=pager`).
- Draft-dependent checks: `zola --root site build --drafts --output-dir <scratchpad>/drafts --force` keeps `site/public` and the seal untouched.
- Design crops live in `target/shots/<WO>-design*/` (rework rounds add `-r1`); open one or two (incl. a focus crop) rather than trusting the description.
- Builds into `site/public` and `target/` do not break the build seal; `factory verify` after your runs confirms it.
- Palette/contrast orders: recompute the contrast table and tint mixes with a small python -I script in the scratchpad (WCAG luminance); takes a minute and checks every number in plan, build and docs.
- Image changes (og.png): `git show HEAD:<path> > target/...` and Read both; tells intended overlap from a regression. Pillow is not in the shell's python; `file <png>` gives the size.
- Theme/JS behaviour: run the plan's inline Playwright script from `tools/shots` against the supervisor's :1111 server; also try storage blocked (`addInitScript` making `localStorage` throw) to catch toggles that stick (WO-0004).
- Deletions/renames of static files: `git grep -n <oldname> -- ':!site/public' ':!.factory'` catches stale refs in tools, workers and docs, not just templates (WO-0003 favicon).
- New untracked files Nows supplied (logos): compare `sha256sum` with the plan's hashes to confirm nobody edited them.
- Dashboard CSS is `include_str!`-compiled: a `factory serve` already on 4600 serves the OLD CSS. Run `cargo run -q -p factory -- serve --port 4617` in background, curl, then stop by PID (netstat + `taskkill //PID n //F`).

## Recurring reasons to send back
- None sent back by me yet (WO-0001 to WO-0004 approved). Reworks so far all came from design review and all were contrast misses axe cannot see: an element hidden at opacity 0 (WO-0002 #1), a `::placeholder` faded with opacity 0.8 once the muted token got lighter (WO-0004 #1). On palette orders, grep `opacity` in site/sass and recompute faded text yourself.

## Things to watch
- `git status` shows agent-memory, skills and `.factory/` changes alongside the site diff; they are expected, not scope creep. Tell Nows which files are the real change.
- shots `--focus` stops after 10 tab stops; footer focus claims need a manual check (lesson 2026-10-24).
- Builder "changes beyond the plan" section: accept when disclosed with a reason and within the request (WO-0002: tabular-nums scoping, form letter-spacing).
- Approved layout changes beyond the request's "keep layout unchanged" (WO-0003 40rem->43rem breakpoint) need Nows's decision in the board history; check it there, then say it plainly in the report.
- New or deleted static files show as `??`/` D` in git status: tell Nows to `git add` them explicitly.
- Plan grep checks on built HTML can assume the wrong attribute order (WO-0004 check 7); rerun with an order-free pattern like `<meta[^>]*theme-color[^>]*>` rather than failing the check.
- Tell Nows when a change needs a worker redeploy (cms-api admin CSS) or a dashboard restart to be visible.
