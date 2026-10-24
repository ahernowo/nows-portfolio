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
- `work-page.html` renders every case study (4 today). A feature meant for one case study must be switched on from that page's front matter (e.g. `[extra.journey]`), or it lands on all of them.
- `FrontMatter` in cms-core has no `deny_unknown_fields`, so new `extra` keys pass `newpost --check`; but Decap only knows the fields in `config.yml`, so plan a matching Decap field (or a warning) for each new key.
- `header.html` and `base.html` load `static/images/nows-flow-mark.svg` / `-favicon.svg`; if those files are missing the whole build fails. Before planning, run `zola --root site build --output-dir <scratchpad>/out --force` to catch a broken tree; it doesn't touch the dev server's output.
- A supplied mock-up (`design/<name>/`): check that `index.html`'s inline `<style>`/`<script>` match the `src/` files (WO-0005: they matched once whitespace was ignored) and scan for fetch/eval/outside URLs before planning a port.
- Porting mock-up CSS (WO-0005): grep its class names against `site/sass` + templates (global `.stamp` in `_home.scss` animates; `.hero`, `.crumb` exist), and remember `_base.scss` `svg { max-width: 100% }` clamps oversized SVGs. Zola's Sass passes `@container`, `color-mix`, `:has`, `cqw`, `svh`, `animation-timeline` and `160/300` through untouched (tested).
- Page-only styles: a non-underscore file in `site/sass/` compiles to its own CSS (`journey.scss` → `/journey.css`); link it from `head_extra` instead of growing `main.css`.
- Prefer server-rendered markup (Tera component) over JS `innerHTML` templates: no CSP/XSS question, and the page is complete without JS.
- Front matter strings are escaped, so `<!-- Nows -->` markers can't live there: use `# Nows:` TOML comments plus one marker block in the body. Decap may write empty lists, so switch a feature on a non-empty list, not on a table's presence.
- WCAG 2.2.2: motion that starts by itself and runs over 5 s needs a pause control. Check mock-up idle loops and load sequences against it.
- Playwright's Chromium can't play H.264; video checks need Chrome (`C:/Program Files/Google/Chrome/Application/chrome.exe`, via `CHROMIUM_PATH` or by hand). `shoot.mjs` has `--root` (shoot a scratch copy of `site/`) and `--scroll`; `--site` can't reuse a build with another base URL.
- Mock a content change before planning: copy `site/` to the scratchpad, edit, `shoot.mjs --root <copy> --viewport-only --no-axe`. WO-0006: filling the case-form in the journey hero pushed the 390 scene (and its Pause button) to ~810 px.
- Anything in `site/static/` deploys even if no page links it, and drafts don't hide it. `ignored_static = [...]` in `zola.toml` keeps a file in the repo but out of the build (tested, Zola 0.23.6). Check `git status -sb` for what's already pushed (= live).
- "Only inside the illustration" checks: a node one-liner that strips `<figure ... jn-room>...</figure>` from the built HTML and greps the rest (WO-0006 plan, check 5). A `<!-- Nows -->` marker that quotes a removed claim trips the plan's own grep; paraphrase it.
- Supervisor notes can be stale (WO-0006 said WO-0005 was uncommitted; it was HEAD). Trust `git log`/`git status`.

## Requests that needed Nows
- Re-skin with a fixed five-colour palette (WO-0002): secondary text colour in dark (palette had none), which links take the accent, and "generous white space" vs "colours and type only, not layout".
- Porting a mock-up (WO-0005): which content page gets it, real copy and figures, whether the invented names baked into the scene and video can stay, and how far the mock-up may go against the design system (load or idle motion, colours outside the palette, rounded corners). Ask all of these in one round.
- Honesty reframes (WO-0006) needed no questions: the request listed the facts. Derive only small things (year from a start date), keep Nows's wording in lists, and remove a feature whose purpose is results (the bars) from the code, docs and Decap, not just the page.

## Working the line
- Long `factory report` heredocs fail on Windows ("unexpected EOF while looking for matching `''"): the command is too long. Build the report in the scratchpad with several `cat >>` heredocs of ~3 KB (a 9 KB one failed in WO-0005), then `factory report <id> plan < file`. The Write tool is blocked outside this folder.
- In Git Bash, run `tools/shots/shoot.mjs` with `MSYS_NO_PATHCONV=1`, or `--pages /` becomes `C:/Program Files/Git/` and the run crashes. Baseline shots before a build go in `target/shots/<id>-before/` (git-ignored, seal-safe).
- Untracked files a request supplies (logos, images) can't be diffed by reviewers; put their sha256 in the scope check.
