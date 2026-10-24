# Build: Re-skin the site with a new palette and Inter (rework 1)

## What changed
This round clears the four design findings. The other rework-0 changes are the same as before (see `build.rework-1.md`). The "#" heading anchors now use `--ink-soft` instead of `--rule-strong`, so they pass contrast in both themes. Standfirsts get `text-wrap: pretty`. The pager now underlines only the page title, not the "Older/Newer" hint. The editor preview now uses the system sans stack instead of Georgia. Margins, padding and grid are unchanged.

## Files
Changed this round (the full list from `git status --short` is the same as rework 0):
- `site/sass/_prose.scss`: anchor colour (#1), `text-wrap: pretty` on `.article-dek, .case-dek` (#2), `.pager-link` sets `text-decoration-line: none` and a new `.pager-title` rule carries the underline and its hover (#3).
- `site/templates/article.html`, `site/templates/work-page.html`: the pager title is wrapped in `<span class="pager-title">` (#3).
- `workers/cms-api/admin/app.css`: `.preview` font changed to `1.0625rem/1.65 var(--sans)` (#4).
- `docs/design-system.md`, `.claude/skills/design-system/SKILL.md`: added the heading-anchor pair to the contrast tables and to the `--ink-soft` uses. The skill also gains a `text-wrap: pretty` note and a line on the pager underline.

## Checks
- `cargo fmt --all`: no changes
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo check -p cms-api --target wasm32-unknown-unknown`: ok (`app.css` is pulled in with `include_str!`)
- `zola --root site build`: ok, 5 pages, 0 warnings
- `zola --root site check --skip-external-links`: ok
- `cargo run -q -p newpost -- --check`: checked 6 files, no problems

## Acceptance checks
All checks from rework 0 still stand; the layout and tokens are the same apart from the lines above. Evidence for this round comes from `node target/wo2-rework.mjs` (Playwright on a `--drafts` build, run at 1440 and 390 in both light and dark). Its screenshots are in `target/shots/WO-0002-rework/`.
- #1: Anchor focused. Computed colour is rgb(87,0,0) #570000 in light (14.89:1) and rgb(222,124,90) #DE7C5A in dark (6.55:1), with a 2px solid ring in ink. Screenshots: `anchor-*-*.png`.
- #2: Computed `text-wrap: pretty`. At 1440 the standfirst ends "...All of it free / to run." with two words on the last line (`dek-1440-*.png`). At 390 it wraps to 4 lines (`dek-390-*.png`).
- #3: Computed styles. On the link, `text-decoration-line: none`. On the hint, none, coloured `--ink-soft`. On the title, underline in `--link-line` (#CF6F82 light, #954A36 dark). On hover the title's underline becomes currentColor. Screenshots: `pager-*.png`, `pager-hover-*.png`. Only the title is underlined.
- #4: CSS diff only. `--sans` is defined at the top of `workers/cms-api/admin/app.css`. I did not screenshot the editor (that needs `wrangler dev` with D1); the design reviewer can judge this from the diff.

## Findings fixed (rework)
- #1: `.prose .heading-anchor` now uses `color: var(--ink-soft)`. The pair is in both contrast tables.
- #2: `text-wrap: pretty` on `.article-dek, .case-dek`. This is a type-only property, so it stays within Nows's white-space decision.
- #3: The title is in `span.pager-title`. The underline moves from the link to the title, because a child cannot remove an underline set on its parent. The hover darkening is the same as on `a`.
- #4: `.preview` now uses `var(--sans)`.

## Notes for reviewers
- Look at the pager at the foot of `/articles/how-this-site-is-built/` and `/articles/writing-reference/` in both themes, at rest and on hover. Also Tab onto the pager: the focus ring still wraps the whole link (hint and title).
- Look at the heading anchors on article h2s, on hover and on focus.
- The only work page has no neighbours, so there is no case-study pager to see. Its template change is the same as the article's.
