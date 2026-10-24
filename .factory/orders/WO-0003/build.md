# Build: Add the Nows logo to the header and favicon

## What changed
The header's home link now starts with Nows's flow mark. The header partial reads `nows-flow-mark.svg` at build time (`load_data(..., format="plain")`), drops its `<title>` so no "Nows" tooltip appears, and inlines it in a `<span class="wordmark-mark" aria-hidden="true">`, so it paints in the wordmark's `--ink` through `currentColor`. `.wordmark` became an inline-flex row with a 0.375rem gap, and the mark is sized 1.49em by 1em (about 36 by 24 px). The header's phone layout now starts at 43rem instead of 40rem, as Nows approved. The site and `/admin/` favicons point at `images/nows-flow-favicon.svg`, and the old `site/static/favicon.svg` is deleted.

## Files
- ` M site/templates/partials/header.html`: loads the mark once (with a Tera comment on why it is inline) and puts the hidden span before the name.
- ` M site/sass/_chrome.scss`: `.wordmark` inline-flex and gap; new `.wordmark-mark` rules; header query moved from 40rem to 43rem, with a comment. The footer query is unchanged.
- ` M site/templates/base.html`: favicon href changed (line 26).
- ` M site/static/admin/index.html`: favicon href changed (line 8, the only change).
- ` D site/static/favicon.svg`: the old favicon, no longer referenced. The deletion is unstaged; Nows stages it with the commit.
- `?? site/static/images/nows-flow-{mark,favicon,mark-dark,mark-light}.svg`: Nows's files, unedited. The hashes match the plan.
- ` M .factory/board.json`, `?? .factory/orders/WO-0003/`, ` M .claude/agent-memory/planner/MEMORY.md`: line bookkeeping.

## Checks
- `zola --root site build`: ok, 5 pages, 2 sections, no warnings
- `zola --root site check --skip-external-links`: ok
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok
- `cargo test --workspace`: 65 passed, 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- No Rust or worker code changed, so no wasm check.

## Acceptance checks
1. Pass: the build is clean.
2. Pass: all checks above are green.
3. Pass: the loop over every `index.html` plus `404.html` printed nothing.
4. Pass: `<span aria-hidden=true class=wordmark-mark>`; `grep -c '<title>Nows</title>'` gives 0; the link markup ends `</svg></span>Nows</a>`.
5. Pass: `getByRole('link', { name: 'Nows', exact: true })` matches 1 element on `/about/`. Axe reports 0 failures.
6. Pass, needs design judgement: `target/shots/WO-0003/{home,articles,about}-{390,1440}-{light,dark}.png`, with zoomed crops in `target/shots/WO-0003/zoom-wordmark-{light,dark}.png`. The mark is 35.8 x 24.0 px, centred on the name (both centres at y=34). Computed colours: the mark's fill equals the name's colour, rgb(40,0,0) in light and rgb(253,255,255) in dark. Header height compared with a HEAD build (`git archive` into `target/wo3-base`): 69 px at 689/705/1440 and 93.77 px at 320/390, the same as before.
7. Pass: clicking the Theme button through light, dark and auto changed the mark's fill together with the name's colour at every step. A visible flash cannot be measured; design review can glance at it.
8. Pass: the cursor over the mark is `pointer`; clicking the mark's centre on `/about/` went to `/`.
9. Pass: at 689, 705, 768 and 1024 px, with each label "Theme: auto/light/dark", the name, nav and tools share one centre line (within 2 px), the header is 69 px, and there is no sideways scrolling. No need to go to 44rem. Shot: `target/shots/WO-0003/header-689-light.png`.
10. Pass: at 688, 641, 390 and 320 px, the nav sits below the name, the icon tools sit on the name's row, and there is no sideways scrolling. Shots: `header-688-light.png`, `header-641-light.png`.
11. Pass: the `--focus` run on `/` and `/about/` reports "every tab stop shows a focus ring". `target/shots/WO-0003-focus/home-focus-2.png` shows the ring around mark and name in light. The tool shoots focus in light only, so I took dark by hand: `target/shots/WO-0003/focus-wordmark-dark.png` shows the off-white ring (`outline: rgb(253,255,255) solid 2px`), unclipped.
12. Pass: axe (WCAG 2.2 AA, both themes) shows 0 rule failures on `/`, `/about/` and `/articles/`.
13. Pass (Playwright `forcedColors: "active"` rather than DevTools): the name and mark both take rgb(0,0,159), the system link colour. Crop: `target/shots/WO-0003/zoom-wordmark-forced.png`.
14. Pass: both shot runs report 0 console, network or CSP problems.
15. Pass: `<link href=.../images/nows-flow-favicon.svg rel=icon type=image/svg+xml>`; `site/public/favicon.svg` is gone; `site/public/images/nows-flow-favicon.svg` exists. I did not view the browser tab icon (headless); design review can check it with `zola serve`.
16. Pass: `git diff --stat site/static/admin/index.html` shows 1 insertion and 1 deletion, line 8.
17. Pass: the grep shows only the two new `nows-flow-favicon.svg` hrefs.
18. Pass: `git status` matches the planned list, plus the planner's memory file. All four sha256 hashes match the plan.

## Notes for reviewers
- The breakpoint move: between 641 and 688 px the header is now the phone header. Compare `target/shots/WO-0003/header-641-light.png` with `target/shots/WO-0003-before/home-641-light.png`. Nows approved this move.
- Optical alignment: the bounding boxes are exactly centred, but the logo's viewBox has more empty space at the top. Judge it on `zoom-wordmark-light.png`. I left it as it is, since the plan calls any nudge a low, design-review call.
- Security: the inline SVG goes out through `| safe` from a repo file. The file holds only `svg`, `title`, `g`, `circle` and `path`, with no `style`, script or `href`. No CSP change.
- The scratch scripts and sites are in `target/` (`wo3-*.mjs`, `wo3-site`, `wo3-base*`), git-ignored and outside the seal.
