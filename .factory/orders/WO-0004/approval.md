# Approval: Soften the palette and make light the default theme

## Verdict
Approved. One rework (design #1, search placeholder at 4.00:1 in light) is fixed and I re-verified it by calculation.

## Checks I ran
- `cargo fmt --all --check`: ok (rc 0)
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, no warnings
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- `zola --root site build`: ok, 5 pages, 2 sections, no warnings
- Plan checks 3, 4, 5, 6, 7 and 18 (greps), rerun by me: all print the expected output (details below).
- Contrast and tint mixes recomputed with my own WCAG script (`python -I`, scratchpad): every tint hex reproduces from its stated mix, and every ratio in the plan, build report, skill and doc matches mine (for example light muted on page 6.34, dark link on peach 4.90, dark stamp on rose 3.74, placeholder at 0.8 = 4.00 light / 5.99 dark, editor #E8546D on field 4.23).
- Theme behaviour (plan check 8) in Playwright against :1111 with the system set to dark, at 1440 and 390: `first ['rgb(250, 245, 243)', null, 'Theme: light', null] 1`, `click ['rgb(31, 27, 28)', 'dark', 'Theme: dark', 'dark']`, reload and the next page (/articles/) stay dark, `back [... 'light', 'Theme: light', 'light']`. Extra: with localStorage throwing, the button still toggles light -> dark -> light with no page errors (the builder's reason for reading `data-theme` holds).
- `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0004-approve --drafts --pages /,/articles/writing-reference/,/work/`: 12 shots, 0 console/network/layout problems, 0 axe rule failures. Opened `home-1440-light.png` and the reviewer's `WO-0004-design/home-1440-dark.png`.
- `og.png`: 1200x630 RGB (`file`). Read it next to `git show HEAD:site/static/images/og.png`: same layout and slip overlap, only colours changed.
- `factory verify WO-0004` after all my runs: every gate passes, seal intact.

## Request against the change
Asked: soften the palette to Nows's exact hex values in both themes, light as the default whatever the system setting, dark only by the Theme button and remembered, label names the theme showing, logo unchanged, AA contrast with a table, theme-color meta and OG script on the light background, design-system skill and doc updated.
Built: all of it. `_tokens.scss` holds exactly Nows's values (light #FAF5F3 / #FFFFFF / #2B1618 / #6B5557 / #B10F2E / #B10F2E; dark #1F1B1C / #2C2527 / #F2EDEB / #C2B6B3 / #E8896A / #E8546D); `prefers-color-scheme` removed; head.js applies a saved choice only; site.js cycles light/dark; header label "Theme: light"; one theme-color meta #FAF5F3; OG script and image recoloured; skill and doc rewritten.
Within the plan, not literally asked: a new `--accent` token (so the dark stamp can differ from the dark accent), the shots tool's dark run now saves the choice (otherwise dark shots would be light), dashboard and editor CSS recoloured (design-system "Admin pages" rule). Out of scope with a reason: the code slab #280000 (new slab colours fail more token classes), the logo/favicon, layout, dashboard/editor theme behaviour.
Beyond the plan, disclosed by the builder: two wording changes in `docs/design-system.md` ("maroon carbon", "deep red" lines) that would otherwise be false; no new facts about Nows. Accepted.

## Acceptance checks
1. Pass: fmt, clippy, tests (my run).
2. Pass: zola build, newpost check (my run).
3. Pass: grep prints nothing; `grep -c` on built main.css prints 0 (my run).
4. Pass: old-palette grep prints nothing; `fdffff|280000` only at `_tokens.scss:68-69, 94-95` and `workers/cms-api/admin/app.css:170` (my run).
5. Pass: token diff read line by line against the table; `var(--stamp)` only `_home.scss:151`; `var(--accent)` at `_home.scss:43`, `_prose.scss:381, 387, 487` (my run).
6. Pass: layout-property diff grep prints nothing (my run).
7. Pass: built head has `<meta content=light name=color-scheme>` and exactly one `<meta content=#FAF5F3 name=theme-color>`, no media (my run; plan's pattern needed the minifier's attribute order).
8. Pass: my Playwright run, output above.
9. Pass: my shots run on three pages (0 problems, 0 axe failures); the full ten-page run is in the build and design reports (40 shots, 0/0).
10. Pass: home 1440 light (my shot) and dark (design shot) show the palette as specified; page heights match the before shots (build report).
11. Pass: build report computed styles (crimson/coral links with underline, ink wordmark and menu); visible in my home shots (footer and "All work" links crimson / coral, underlined).
12. Pass: design report (alerts and slab in both themes); my axe scan of writing-reference in both themes has 0 failures.
13. Pass: my run, accessible name "Theme: light" / "Theme: dark" counts 1 at 390 and 1440.
14. Pass: design report (manual Tab through both articles, 30 and 29 stops, heading anchor and Copy button by hand).
15. Pass: my look at og.png against HEAD; pixel colour per build report (Pillow not installed in my shell).
16. Pass: build report (dashboard served on 4601, computed colours); andon ratios 5.92 / 6.74 match my recomputation.
17. Pass: I read both diffs: palette per theme, both never-as-text rules, `--accent` vs `--stamp`, muted text at full opacity, theme behaviour, token and contrast tables with the placeholder row, all matching `_tokens.scss`; no "maroon" or system-driven wording left.
18. Pass: `git diff --stat` on _headers, favicon, marks and content prints nothing (my run).

## For Nows
Read first in `git diff`: `site/sass/_tokens.scss` (the whole palette), `site/static/js/site.js` and `head.js` (light default, dark by choice), `site/templates/base.html`, then `docs/design-system.md`.
Try by hand: open the site with your system in dark mode; it should be light with "Theme: light". Click Theme, reload, open another page: dark stays. Click again to go back.
Not part of the site change but in `git status`: agent memories, the two skills, `.factory/` (expected). The editor CSS only shows after you redeploy cms-api; the dashboard after `factory` is rebuilt/restarted.
Visitors who relied on their dark system setting now get light until they press Theme; that is what you asked for.
Suggested commit message: `Soften the palette and make light the default theme`
