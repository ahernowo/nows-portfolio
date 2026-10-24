# Approval: Add the Nows logo to the header and favicon

## Verdict
Approved.

## Checks I ran
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- `zola --root site build`: 5 pages, 2 sections, no warnings
- `factory verify WO-0003` after my runs: every gate holds, build seal intact
- `sha256sum site/static/images/nows-flow-*.svg`: all four match the plan (logo files unedited)
- `git grep -n favicon` over tracked files (outside `site/public`, `.factory`): no reference left except the two new hrefs

## Request against the change
Asked: the flow mark before "Nows" in the header, linking home, `aria-hidden`, visible focus ring; `nows-flow-favicon.svg` as the favicon; layout, colours and fonts unchanged.
Built: the mark is inlined through `load_data` in `partials/header.html` inside the existing home link, in a `<span aria-hidden="true">`, painting in `--ink` via `currentColor`; the favicon points at the new file on the site (`base.html:26`) and in `/admin/` (`admin/index.html:8`); the old `site/static/favicon.svg` is deleted.
One layout change: the header's phone query moves from 40rem to 43rem so the row does not wrap between 641 and 688 px. Nows approved this explicitly (supervisor note and board decision 03:39). The footer query is untouched.
Out of scope, with reasons in the plan: `nows-flow-mark-dark.svg` and `-light.svg` (not asked for; published but unused), og.png, tokens, fonts.
Nothing in the diff goes beyond the plan. Agent-memory and `.factory/` changes are line bookkeeping.

## Acceptance checks
1. Pass: my `zola --root site build`, no warnings.
2. Pass: my runs above.
3. Pass: my loop over every built `index.html` plus `404.html` printed nothing.
4. Pass: I saw `<span aria-hidden=true class=wordmark-mark>`, `grep -c '<title>Nows</title>'` = 0, and the link ends `</svg></span>Nows</a>`.
5. Pass: build report (`getByRole('link', {name:'Nows', exact:true})` = 1) and design report (`getByRole('img', {name:'Nows'})` = 0).
6. Pass: design report (computed fill = name colour in both themes, header height unchanged); I viewed `target/shots/WO-0003-design/hdr-689-light.png`.
7. Pass: build and design reports (fill follows theme at every step).
8. Pass: build report (pointer cursor, click on mark goes to `/`).
9. Pass: build and design reports; I viewed `hdr-689-light.png`: mark, name, nav, Search and Theme on one row.
10. Pass: I viewed `hdr-641-dark.png`: mark, name and icon tools on row one, nav below, as the phone header.
11. Pass: I viewed `wm-focus-dark.png`: one off-white ring around mark and name, unclipped; shots tool reported 0 focus problems (design report).
12. Pass: design report, axe 0 failures on six pages, both themes.
13. Pass: build and design reports (`wm-forced.png`, both take the system link colour).
14. Pass: design report, 0 console, network or CSP problems.
15. Pass: I saw `<link href=.../images/nows-flow-favicon.svg rel=icon type=image/svg+xml>`, `site/public/favicon.svg` absent, the new file present; I viewed `favicon-tabs-2x.png`: crimson square reads on light and dark tab greys.
16. Pass: `git diff site/static/admin/index.html` is the one line 8 change (I read it).
17. Pass: my grep shows only the two new hrefs.
18. Pass: `git status --short` matches the planned list plus agent-memory files; hashes match.

## For Nows
Read first in `git diff`: `site/templates/partials/header.html` (the `load_data` line and the span), then `site/sass/_chrome.scss` (new `.wordmark-mark` rules and the 43rem query). `base.html` and `admin/index.html` are one-line href changes.
Stage the deletion and the new files too: `git add site/static/favicon.svg site/static/images/nows-flow-*.svg` (the dark and light marks go along unused unless you leave them out).
Try by hand: `zola --root site serve`, look at the browser tab icon, and drag the window between about 640 and 700 px to see the header switch to the phone layout at 688 px.
Design note, not a defect: the mark sits about 1 px low optically; `translate: 0 -0.06em` on `.wordmark-mark svg` would lift it if you want.
Every page now carries about 4.5 KB of inline path data (before compression).
Suggested commit message: `Add the flow mark to the header and a new favicon`
