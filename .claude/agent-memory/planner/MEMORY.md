# Planner memory

Notes that make future plans better. Keep under 150 lines; merge and prune.

## The repo
- Drafts on purpose: `site/content/articles/writing-reference.md` (shows every prose element; use it to check prose styling) and the case studies `bank-reconciliation.md` and `donation-dashboard.md` (waiting for Nows's facts).
- A change to `base.html`, a partial, `_tokens.scss`, `_base.scss` or `_chrome.scss` affects every page; plan design review across all pages.
- `.pager` (in `_prose.scss`) is shared by `article.html` and `work-page.html`; editing its rules changes case studies too. Its links are `max-width: 48%` + 1rem gap, so adding `flex-wrap` wraps them below ~400 px. Prefer a new class.
- Few published pieces: an article can have no neighbours, so plan checks for both "pager empty" and "pager present" (use `--drafts` to get neighbours).
- Content rules are defined once in `crates/cms-core/src/article.rs`; the Decap fields in `site/static/admin/config.yml` mirror them. Plan both when a rule changes.
- Colours and fonts live outside `_tokens.scss` too: the favicon (from WO-0003: `static/images/nows-flow-favicon.svg`, fixed crimson; also linked from `static/admin/index.html`), `base.html` (theme-color metas, font preload), footer colophon (names the fonts), `scripts/make-og-image.py` + `og.png`, `tools/factory/dashboard/app.css` (loads site fonts via `/fonts/`, so deleting a font breaks it), `workers/cms-api/admin/app.css`, the `ui-ux-review` skill's typography line, README, the og alt text in `writing-reference.md`, raw `rgb(20 25 56 ...)` shadows in `_chrome.scss`.
- Code blocks use the `tokyo-night` Giallo theme (`zola.toml`); slab colour comes from `--code-bg`, token colours from `site/public/giallo.css` (classes `z-N`, numbered by colour, so never override by number). Six classes (z-4,5,6,9,16,23) stay under 4.5:1 on #280000; plan the axe check to expect them. Any lighter or greyer slab fails more (z-7, z-12, z-27 on #2B1618 or #1F1B1C), so WO-0004 kept the slab at #280000: compute all 34 from `giallo.css` before changing it.
- Inter from Fontsource: the `opsz` files carry weight and optical size; `wght` files carry weight only. Inter has no width axis, so every `font-stretch` becomes dead code.
- Link colour lives on bare `a` in `_base.scss`; elements that must stay text colour (wordmark, `.site-nav a`, `.index-link`, `.search-results a`, `.topic-list a`) inherit today and need an explicit `color` once `a` gets one.
- The one-row header (wordmark, nav, tools) is nearly full at its 40rem breakpoint (~29 px free at 641 px before WO-0003). Anything added to the header row needs the breakpoint moved or it wraps; take baseline shots at 641-705 px first.
- SVGs that use `currentColor` must be inline (an `<img>` draws them black). Inline from the file with `load_data(path="static/...", format="plain") | safe` inside an `aria-hidden` wrapper; `minify_html` lowercases `viewBox` but browsers fix it.
- Theme (from WO-0004): light for everyone, dark only from the Theme button (`localStorage.theme`, set in `head.js` before paint); no `prefers-color-scheme` on the site. `tools/shots/shoot.mjs` emulates `colorScheme`, so its dark run needs the saved choice too (init script). The dashboard and editor CSS keep their own theme behaviour.
- Palette work: compute every pair with a small WCAG script in the scratchpad (sRGB mixes, `cr(a,b)`). Planner can derive tints from Nows's colours without asking; ask only for colours with a role Nows did not give.
- Fonts come from Fontsource (file names like `<family>-latin-<axis>-normal.woff2`, same unicode ranges as `_tokens.scss`). Pin a version on jsdelivr.

## Requests that needed Nows
- Re-skin with a fixed five-colour palette (WO-0002): secondary text colour in dark (palette had none), which links take the accent, and "generous white space" vs "colours and type only, not layout".

## Working the line
- Long `factory report` heredocs fail on Windows ("unexpected EOF while looking for matching `''"): the command is too long. Build the report in the scratchpad with several `cat >>` heredocs of ~6 KB, then `factory report <id> plan < file`. The Write tool is blocked outside this folder.
- In Git Bash, run `tools/shots/shoot.mjs` with `MSYS_NO_PATHCONV=1`, or `--pages /` becomes `C:/Program Files/Git/` and the run crashes. Baseline shots before a build go in `target/shots/<id>-before/` (git-ignored, seal-safe).
- Untracked files a request supplies (logos, images) can't be diffed by reviewers; put their sha256 in the scope check.
