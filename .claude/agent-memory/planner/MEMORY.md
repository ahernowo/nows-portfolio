# Planner memory

Notes that make future plans better. Keep under 150 lines; merge and prune.

## The repo
- Drafts on purpose: `site/content/articles/writing-reference.md` (shows every prose element; use it to check prose styling) and the case studies `bank-reconciliation.md` and `donation-dashboard.md` (waiting for Nows's facts).
- A change to `base.html`, a partial, `_tokens.scss`, `_base.scss` or `_chrome.scss` affects every page; plan design review across all pages.
- `.pager` (in `_prose.scss`) is shared by `article.html` and `work-page.html`; editing its rules changes case studies too. Its links are `max-width: 48%` + 1rem gap, so adding `flex-wrap` wraps them below ~400 px. Prefer a new class.
- Few published pieces: an article can have no neighbours, so plan checks for both "pager empty" and "pager present" (use `--drafts` to get neighbours).
- Content rules are defined once in `crates/cms-core/src/article.rs`; the Decap fields in `site/static/admin/config.yml` mirror them. Plan both when a rule changes.
- Colours and fonts live outside `_tokens.scss` too: `favicon.svg` (own light/dark colours), `base.html` (theme-color metas, font preload), footer colophon (names the fonts), `scripts/make-og-image.py` + `og.png`, `tools/factory/dashboard/app.css` (loads site fonts via `/fonts/`, so deleting a font breaks it), `workers/cms-api/admin/app.css`, the `ui-ux-review` skill's typography line, README, the og alt text in `writing-reference.md`, raw `rgb(20 25 56 ...)` shadows in `_chrome.scss`.
- Code blocks use the `tokyo-night` Giallo theme (`zola.toml`); slab colour comes from `--code-bg`, token colours from `site/public/giallo.css` (classes `z-N`, numbered by colour, so never override by number). Six classes (z-4,5,6,9,16,23) stay under 4.5:1 on any dark slab; plan the axe check to expect them.
- Inter from Fontsource: the `opsz` files carry weight and optical size; `wght` files carry weight only. Inter has no width axis, so every `font-stretch` becomes dead code.
- Link colour lives on bare `a` in `_base.scss`; elements that must stay text colour (wordmark, `.site-nav a`, `.index-link`, `.search-results a`, `.topic-list a`) inherit today and need an explicit `color` once `a` gets one.
- Fonts come from Fontsource (file names like `<family>-latin-<axis>-normal.woff2`, same unicode ranges as `_tokens.scss`). Pin a version on jsdelivr.

## Requests that needed Nows
- Re-skin with a fixed five-colour palette (WO-0002): secondary text colour in dark (palette had none), which links take the accent, and "generous white space" vs "colours and type only, not layout".

## Working the line
- Long `factory report` heredocs fail on Windows ("unexpected EOF while looking for matching `''"): the command is too long. Build the report in the scratchpad with several `cat >>` heredocs of ~6 KB, then `factory report <id> plan < file`. The Write tool is blocked outside this folder.
