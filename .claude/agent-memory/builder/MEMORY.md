# Builder memory

Gotchas that cost time once. Keep under 150 lines; merge and prune.

## Zola and Tera 2
- `insert_anchor_links = "left"` keeps heading text plain; "heading" mode wraps the text in a link with an aria-label that hides it from screen readers.
- GitHub alerts render as `blockquote.markdown-alert-note` and friends, with no shared base class; style them with `blockquote[class^="markdown-alert-"]`.
- `zola build --output-dir` needs an absolute path, or `get_url(..., cachebust=true)` cannot find `main.css`. In Git Bash: `--output-dir "$(pwd -W)/target/x"`.
- `minify_html` reorders attributes: the built meta is `<meta content=#FAF5F3 name=theme-color>`. Grep `<meta[^>]*theme-color[^>]*>`, not `<meta name=...`.

- Tera 2: `{%- if page.higher or page.lower %}` is safe for guarding the pager (WO-0001).
- Inline SVG from a file: `load_data(path="static/...svg", format="plain")` + `| replace(...) | safe` works; put `aria-hidden` on a wrapping span (WO-0003).
- Tera 2 that worked first time (WO-0005): `{% if page.extra?.journey?.steps %}` in `head_extra` and `content` (a top-level `set` outside blocks is not seen inside them), `{% for row in [10, 9, 7] %}`, `range(end=n)`, list literals indexed `targets[i]`. `minify_html` strips `<!-- Nows -->` comments from the output.
- Grid with a hidden button: named `grid-template-areas` keep an empty row (and its gap) when the item is `display: none`. For a row that must vanish, drop the areas at that breakpoint and move the item with `order` (WO-0006 hero Pause).
- `ignored_static = [...]` in zola.toml drops static files from the build; the supervisor's running `zola serve` needs a restart to see it.
- Any `sass/x.scss` without `_` compiles to `/x.css`: a per-page stylesheet needs no `main.scss` change.

## Type and colour (Inter, since WO-0002)
- Inter's `tabular-nums` also widens the hyphen to figure width ("WO - 0002"). Put it only on dates, counts and amounts, never on `body` or text with ids.
- Chrome's UA stylesheet resets `letter-spacing` on `button`/`input`; `_base.scss` sets `letter-spacing: inherit` on them.
- A child cannot remove an underline set on its parent `a`. To underline only part of a link (the pager title), set `text-decoration-line: none` on the `a` and put the underline and hover on a child span (WO-0002 #3).
- Check every text colour in the Sass against 4.5:1, including hidden-until-focus ones like `.heading-anchor` (opacity 0, so axe misses it). Grep for `color: var(--rule` first.
- On a palette change also grep `opacity` in `site/sass/`: faded text (the search `::placeholder` at 0.8) loses contrast and axe never sees placeholders (WO-0004 #1). Placeholders need `opacity: 1` (Firefox UA fades to 0.54).
- Deks wrap differently when the font changes; `text-wrap: pretty` on standfirsts stops a lone last word.
- Contrast: a 20-line Python WCAG script (relative luminance, sRGB mix) re-checks a palette in seconds; run it on every final hex value.
- Since WO-0004 the site ignores `prefers-color-scheme`: dark only via `localStorage.theme = "dark"`. In scratch Playwright scripts use `context.addInitScript(() => localStorage.setItem("theme","dark"))`; `colorScheme: "dark"` alone shows light.

## Screenshots and checks
- [Shots on Windows](shots-on-windows.md): MSYS_NO_PATHCONV for --pages, the 10-stop focus cap, cropping, scratch scripts in target/, waiting for and stopping servers.
- The supervisor's `zola serve` on 1111 has no drafts (no writing-reference, no pager). For drafts, build to `target/<x>` with `--drafts --base-url http://127.0.0.1:4733` and serve with `startServer`.
- Full-page shots sometimes miss a lazy image (WO-0004: og.png figure blank in one dark shot). Re-shoot that page before calling it a regression.
- A Pillow venv (the ogvenv recipe below, recreated per session in the scratchpad) reads pixels, compares shot sizes and crops faster than the Playwright data-URL trick.

## Tools and scripts
- The dashboard's `app.css` is compiled in with `include_str!`. An already-running `factory serve` on 4600 shows the old CSS; test with `cargo run -q -p factory -- serve --port 4601`, then stop it by PID.
- `scripts/make-og-image.py` needs Pillow, which the system Python lacks: `python -m venv <scratchpad>/ogvenv`, then `ogvenv/Scripts/python -m pip install pillow fonttools brotli`.
- Long heredocs fail in the Bash tool ("unexpected EOF while looking for matching `''"), even quoted ones, e.g. `factory report <<'EOF'` with a long report. Write the report to `target/<wo>-build-report.md` with Write, then `factory report <id> build < target/...md`. For big edits: Read + Edit, or a Python exact-replace script (in target/) with a count check per block.

- Python edit scripts on Windows: `read_text`/`write_text` turn LF files into CRLF (git then warns "CRLF will be replaced"). Use `read_bytes().decode()` / `write_bytes(s.encode())` (WO-0006).
- In a slice-out script, search the end marker *after* the start: `s.index(end, a)`. config.yml has one `Body` field per collection; `s.index` found the first and duplicated the work collection (WO-0006).
- After edits, compare against HEAD: a short script printing CRLF and line counts from `git show HEAD:<f>` vs the file catches both of the above.
- `.claude/agent-memory/` is outside the build seal (`tools/factory/src/seal.rs`), so memory can be updated after `factory pass`.
- A font or type change on the site also changes `workers/cms-api/admin/app.css` (the `.preview` font) and the dashboard CSS; check both.

## Motion and video
- A class rule `.btn { display: inline-flex }` beats the UA `[hidden]`; add `.btn[hidden] { display: none }` for JS-revealed buttons.
- A pause control for many animations: `.paused .scope * { animation-play-state: paused !important }`; each `animation:` shorthand resets play-state, so it needs `!important`.
- Playwright's Chromium can't decode H.264: shots log `journey.mp4 ERR_ABORTED`. For real playback, `chromium.launch({ executablePath: "C:/Program Files/Google/Chrome/Application/chrome.exe" })` works here.
- Evidence that motion stopped: `el.getAnimations()` play states, plus two `locator.screenshot()` buffers compared with `Buffer.compare`.

## Workers
- `Response::redirect` headers are immutable; build a 302 by hand when it must set cookies.
- The wasm target may be missing locally; `cargo clippy` natively still catches most errors, and CI builds the wasm.
