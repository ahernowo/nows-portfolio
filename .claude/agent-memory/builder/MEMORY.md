# Builder memory

Gotchas that cost time once. Keep under 150 lines; merge and prune.

## Zola and Tera 2
- `insert_anchor_links = "left"` keeps heading text plain; "heading" mode wraps the text in a link with an aria-label that hides it from screen readers.
- GitHub alerts render as `blockquote.markdown-alert-note` and friends, with no shared base class; style them with `blockquote[class^="markdown-alert-"]`.
- `zola build --output-dir` needs an absolute path, or `get_url(..., cachebust=true)` cannot find `main.css`.

- Tera 2: `{%- if page.higher or page.lower %}` is safe for guarding the pager (WO-0001).

## Type and colour (Inter, since WO-0002)
- Inter's `tabular-nums` also widens the hyphen to figure width ("WO - 0002"). Put it only on dates, counts and amounts, never on `body` or text with ids.
- Chrome's UA stylesheet resets `letter-spacing` on `button`/`input`; `_base.scss` sets `letter-spacing: inherit` on them.
- A child cannot remove an underline set on its parent `a`. To underline only part of a link (the pager title), set `text-decoration-line: none` on the `a` and put the underline and hover on a child span (WO-0002 #3).
- Check every text colour in the Sass against 4.5:1, including hidden-until-focus ones like `.heading-anchor` (opacity 0, so axe misses it). Grep for `color: var(--rule` first.
- Deks wrap differently when the font changes; `text-wrap: pretty` on standfirsts stops a lone last word.
- Contrast: a 20-line Python WCAG script (relative luminance, sRGB mix) re-checks a palette in seconds; run it on every final hex value.

## Screenshots and checks
- [Shots on Windows](shots-on-windows.md): MSYS_NO_PATHCONV for --pages, the 10-stop focus cap, cropping, scratch scripts in target/, waiting for and stopping servers.

## Tools and scripts
- The dashboard's `app.css` is compiled in with `include_str!`. An already-running `factory serve` on 4600 shows the old CSS; test with `cargo run -q -p factory -- serve --port 4601`, then stop it by PID.
- `scripts/make-og-image.py` needs Pillow, which the system Python lacks: `python -m venv <scratchpad>/ogvenv`, then `ogvenv/Scripts/python -m pip install pillow fonttools brotli`.
- Long Markdown edits through a Bash heredoc failed once ("unexpected EOF"). For big multi-line replacements, Read the file and use Edit; for many small ones, a Python exact-replace script with a count check per block.

- `.claude/agent-memory/` is outside the build seal (`tools/factory/src/seal.rs`), so memory can be updated after `factory pass`.
- A font or type change on the site also changes `workers/cms-api/admin/app.css` (the `.preview` font) and the dashboard CSS; check both.

## Workers
- `Response::redirect` headers are immutable; build a 302 by hand when it must set cookies.
- The wasm target may be missing locally; `cargo clippy` natively still catches most errors, and CI builds the wasm.
