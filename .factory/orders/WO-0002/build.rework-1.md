# Build: Re-skin the site with a new palette and Inter

## What changed
The site now uses Nows's five colours and self-hosted Inter. Colours are the tokens in `_tokens.scss` (renamed copy tokens `--copy-peach` / `--copy-rose`, new `--link`, `--link-line`, `--code-edge`, `--dialog-shadow`, `--backdrop`), with every hex value exactly as the plan gives it. Links in running text take crimson (light) or coral (dark). The menu, wordmark, list titles, search results and topic chips set themselves back to `--ink`. Type follows the plan's table: Inter only, every `font-stretch` removed, bigger and tighter headings, more line height, upright standfirsts. No margin, padding, gap, grid, width or position changed. The favicon, theme-color metas, OG image, factory dashboard CSS, editor CSS, design-system skill, `docs/design-system.md` and ui-ux-review skill follow.

## Files
```
 M .claude/agent-memory/builder/MEMORY.md             builder memory (station step 6)
 M .claude/agent-memory/builder/shots-on-windows.md   builder memory
 M .claude/agent-memory/planner/MEMORY.md             planner's, already modified before this build (not mine)
 M .claude/skills/design-system/SKILL.md      palette, tokens, accent/secondary rules, contrast, Inter type table, checklist wording
 M .claude/skills/ui-ux-review/SKILL.md       typography and accent lines; "peach and rose copies"
 M .factory/board.json                        CLI
 M docs/design-system.md                      Colour (roles, tints, contrast), Type, Fonts in Nows's voice
 M scripts/make-og-image.py                   Inter, new colours, axis order read from the font, fitted line sizes
 M site/sass/_base.scss                       body tracking/line height, --link on a and .link-button, pre/code tracking, button/input tracking
 M site/sass/_chrome.scss                     wordmark, menu and tools in --ink, nav hover rule, footer name, dialog tokens, search colours
 M site/sass/_home.scss                       hero/slip/stamp type, renamed copies, no tabular-nums on slip values
 M site/sass/_lists.scss                      titles and deks per table, list titles and chips in --ink, crumb/block-more take --link
 M site/sass/_prose.scss                      titles, dek upright, prose and heading type, links, code edge, copy-button ring, alerts, pager
 M site/sass/_tokens.scss                     four Inter @font-face rules, all tokens, header comment, scale
 M site/static/favicon.svg                    new colours, both themes
 D site/static/fonts/ (6 Archivo/Newsreader woff2 + 2 licences)
?? site/static/fonts/LICENSE-Inter.txt + 4 inter-*-opsz-*.woff2
 M site/static/images/og.png                  regenerated
 M site/templates/base.html                   Inter preload, theme-color #FDFFFF / #280000
 M site/templates/partials/footer.html        "Set in Inter."
 M tools/factory/dashboard/app.css            Inter, palette, andon text, backdrop, tabular-nums scoped
 M workers/cms-api/admin/app.css              palette, toast text, preview slab, dialog backdrop
?? .factory/orders/WO-0002/                   CLI
```

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, no warnings
- `cargo test --workspace`: 65 passed, 0 failed
- `zola --root site build`: ok, no warnings
- `zola --root site check --skip-external-links`: ok
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- wasm check not run (no Rust changed; the editor CSS is `include_str!`-ed, and native clippy compiled it).

## Acceptance checks
1. Pass: fmt, clippy, test as above.
2. Pass: zola build and newpost check as above.
3. Pass: `ls -l site/static/fonts` shows exactly LICENSE-Inter.txt 4477, inter-latin-opsz-normal 72920, inter-latin-ext-opsz-normal 133336, inter-latin-opsz-italic 79716, inter-latin-ext-opsz-italic 146460. Fetched from jsdelivr `@fontsource-variable/inter@5.3.0/files/`; unicode ranges copied from that version's `opsz.css`.
4. Pass: the archivo/newsreader grep prints nothing.
5. Pass: the old-colour grep prints nothing.
6. Pass: no `font-stretch`; the only raw colours outside `_tokens.scss` are the two `rgb(255 255 255 / ...)` lines on `.code-copy`.
7. Pass: the layout-property diff grep prints nothing.
8. Pass: `git diff --stat site/static/_headers` is empty. The shots run shows 0 console/network/CSP problems, and every font request returns 200.
9. Pass: `shoot.mjs --drafts` on the 10 pages, both themes, 390 and 1440: 40 shots, "Console, network and layout problems (0)", "Accessibility scan, WCAG 2.2 AA (0 rule failures)". There are no colour-contrast failures at all, so no code token class sits on a scanned page.
10. Pass, but needs a design reviewer's eye: `home-1440-light/dark.png`. Scratch Playwright measured the hero at 5 lines at 1280 and at 1440 (84 px, `--step-5` max set to 5.25rem) and 5 lines at 390. Stamp span [866,1081] inside the stack [864,1296] at 1440, and [786,995] in [784,1216] at 1280. Dark: #280000 page, #570000 top slip, coral stamp, coral dates and descriptions.
11. Pass (computed styles, both themes): crumb, prose links, `.tag`, `.pager-link`, `.back-link a`, footer links, page-aside, hero-now, block-more, 404 links and the link-button are rgb(177,15,46) underlined in #CF6F82 in light, and rgb(222,124,90) underlined in #954A36 in dark. The wordmark, `.site-nav a`, `.tool`, `.index-link` and `.topic-list a` are #280000 / #FDFFFF. `.index-desc`, `.index-when`, `.index-meta` and `.year-title` are #570000 / coral.
12. Pass: `.article-dek` and `.case-dek` compute `font-style: normal`. On writing-reference, `em` is italic and the network shows `200 inter-latin-opsz-italic.woff2` in both themes.
13. Pass: at 390, no h1/h2/h3, pager link, form value or wordmark has scrollWidth > clientWidth on any of the 10 pages; no sideways scroll.
14. Pass: dark `pre` box-shadow is `rgb(104,76,76) 0 0 0 1px inset`. Crop: `target/shots/WO-0002-extra/pre-dark.png`. Text #FDFFFF on #280000.
15. Pass: `--focus` on `/` and writing-reference, both themes: "every tab stop shows a focus ring". Manual Tab from the last prose control of how-this-site-is-built, 12 stops (heading anchors, 4 tags, pager, "Back to all writing", Email, GitHub): every one has a 2px solid ring, #280000 in light and #FDFFFF in dark. Crops are `foot-tab5-*.png` and `footer-focus-*.png`. Code Copy button ring is #FDFFFF on the slab in light (`copy-focus2-light.png`).
16. Pass: `site/public/favicon.svg` holds #FDFFFF #280000 #B10F2E / #280000 #FDFFFF #DE7C5A. `index.html` has theme-color #FDFFFF (light) and #280000 (dark).
17. Pass: og.png 1200x630, opened with Read. "Nows" in maroon Inter 700 on off-white. The role line was fitted to end by x=660, clear of the slips at 690. The stamp text was fitted inside its frame and is crimson.
18. Pass, with a note: Nows's dashboard already running on 4600 is an older binary with the old `app.css` compiled in (`include_str!`), so it still requests Archivo (404). A fresh `cargo run -p factory -- serve --port 4601` served `/fonts/inter-latin-opsz-normal.woff2` with 200, and the body renders in Inter (`document.fonts.check` true). Andon text is rgb(40,0,0) on #ff7b6b in dark (7.61) and #F3F0F0 on #B10F2E in light (6.23). Shots are in `target/shots/WO-0002-extra/dashboard-*.png` and `andon-*.png`. Nows restarts `factory serve` to see it.
19. Pass: the skill and the doc name the five colours with roles per theme, the two "never as text" rules, the accent rule, the secondary rule, Inter, the type table and the contrast table. A grep for archivo/newsreader/indigo/yellow/pink in both files finds nothing. ui-ux-review lines 44 and 50 are updated. "Set in Inter." is in the footer partial, so it is on every page.

## Contrast table (re-computed from the final hex values, WCAG formula)
| Pair | Light | Dark |
|---|---|---|
| Text on page | 19.19 | 19.19 |
| Text on top slip | 19.26 | 14.89 |
| Text on peach / rose / highlight | 14.95 / 14.66 / 15.79 | 14.38 / 13.75 / 15.64 |
| Secondary on page | 14.89 | 6.55 |
| Secondary on top slip | 14.94 | 5.08 |
| Secondary on peach / rose / highlight | 11.60 / 11.37 / 12.25 | 4.90 / 4.69 / 5.33 |
| Link on page | 7.03 | 6.55 |
| Link on peach / rose / highlight | 5.48 / 5.37 / 5.78 | 4.90 / 4.69 / 5.33 |
| Link underline vs page | 3.35 | 3.04 |
| Stamp on rose copy | 5.37 | 4.69 before screen blend |
| Focus ring vs page | 19.19 | 19.19 |
| Code text on slab | 19.19 | 19.19 |
| Dashboard lamps | run 5.65, wait 5.35, stop 6.23 on #F3F0F0 | run 10.40, wait 12.74, stop 7.61 on #280000; 5.70 on #4C1912 |
| Dashboard ink-soft on surfaces | 13.19 (#570000 on #F3F0F0) | 5.77 on #3A0C09, 4.90 on #4C1912 |
| Andon / editor error toast | 6.23 / 7.03 | 7.61 / 6.55 |
| Editor ok / error | 6.40 / 7.03 on #FDFFFF | 8.81 / 5.08 on #570000 |

The plan's tint mixes check out against the light values. In dark, "on maroon" means on #280000. No pair puts #DE7C5A text on a light background or #B10F2E text on a dark one.

## Changes beyond the plan's lines (small, all type or colour)
- **Pager links are underlined at rest.** Check 11 asks for every running-text link to be underlined. `.pager-link` had `text-decoration: none` with underline on hover only, so I dropped both rules and the base `a` underline now applies. The link is a flex column, so the underline runs under the "Older/Newer" hint as well as the title (one link, so it cannot be split without a template change). See `foot-tab5-light.png`.
- **Tabular figures scoped.** Inter's `tnum` widens the hyphen to figure width (measured 6.88 px to 9.73 px). On the dashboard, `body` had `tabular-nums`, which spaced every id ("WO - 0002", "nows - portfolio"). It now applies only to `.updated`, `.shift-list dd`, `.station-count` and `time`. On the site I removed it from `.slip-lines dd` ("DN-2041"); dates, years, meta and counts keep it.
- `button, input { letter-spacing: inherit; }` in `_base.scss`, because Chrome resets tracking on form controls.
- Nav hover draws a 1px `--rule-strong` underline at 0.4em offset (the plan's rule). `.tool:hover` keeps only its border change, since its colour no longer changes.
- The editor's `dialog::backdrop` was indigo `rgb(15 18 50 / .45)`; it is now `rgb(40 0 0 / .45)`.
- ui-ux-review line 49 said "yellow and pink copies"; it now says "peach and rose".

## Findings fixed (rework)
None (first build).

## Notes for reviewers
- Look first at: `home-1440-light.png` and `home-1440-dark.png` (stack, stamp and headline), `articles-how-this-site-is-built-1440-dark.png` (coral links against coral meta, told apart only by the underline), and `articles-writing-reference-1440-dark.png` (alerts on #4C1912 and #5F0612, slab edge). All are in `target/shots/WO-0002/`. Extras are in `target/shots/WO-0002-extra/`.
- The stamp at 390 and 1440 overlaps the CRM slip's "Donor/Gift" rows, as it did before. Its width is now set by Inter 800 (no width axis), and it stays inside the stack.
- The pager underline under the hint is a judgement call (see above). The alternative is a `<span>` around the title in `article.html` and `work-page.html`.
- In dark, the stage B editor's `.preview pre` (#280000) sits on the #280000 page with no edge. The plan fixed those values; it could take a `--rule` edge like the site's slab. This is not visible on the site.
- The code token classes z-4, z-5, z-6, z-9, z-16 and z-23 stay below 4.5:1 on the slab, as the plan accepted. No scanned page triggered them.
- The editor CSS reaches production only when Nows redeploys cms-api. The dashboard needs a `factory serve` restart.
