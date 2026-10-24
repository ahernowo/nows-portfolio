# Approval: Re-skin the site with a new palette and Inter

## Verdict
Approved. Rework 1 cleared the one medium finding (#1, heading-anchor contrast) and the three low ones (#2-#4). I reproduced every check and found nothing new.

## Checks I ran
- `cargo fmt --all --check`: clean (rc 0)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- `zola --root site build`: 5 pages, 2 sections, no warnings
- Plan greps for checks 3-8 and 16: all as expected (details below)
- Contrast: I recomputed 22 pairs and all 12 tint mixes with the WCAG formula in my own script. Every value matches the plan, the build report and `docs/design-system.md` (e.g. coral on rose 4.69, coral on peach 4.90, crimson on page 7.03, link underline 3.35 / 3.04).
- A fresh `factory serve --port 4617` built from this tree: the font returns 200 at 72,920 bytes, and the served `app.css` holds the Inter `@font-face` and `.andon { color: var(--bg) }`. I stopped the server afterwards.
- `factory verify WO-0002` after my runs: every gate passes, and the seal is intact.

## Request against the change
The request asked for the five-colour palette with set roles per theme, the two "never as text" rules, AA contrast with a table in the reports, and self-hosted variable Inter with its OFL licence. It asked to remove the old fonts, update the OG script and CSP if needed, record everything in the design-system skill and docs, and leave the layout alone.
- Built: tokens in `_tokens.scss`, and type changes (size, weight, tracking, line height only) in `_base`, `_chrome`, `_home`, `_lists` and `_prose`. Also the favicon, theme-color metas, preload, colophon, OG script and image, dashboard and editor CSS, the design-system skill, the ui-ux-review skill and `docs/design-system.md`.
- Nows's three decisions are applied as recorded. Q1: dark secondary text is coral. Q2: only running-text links take the accent; the menu, wordmark, list titles, search results and chips use `--ink`. Q3: no margin, padding, gap or grid line changed (check 7 grep is empty).
- CSP: it already allows `font-src 'self'`, so no change was needed, and `_headers` is unchanged. Security matched the five font files to Fontsource 5.3.0 by SHA-256.
- Out of scope with a reason: `tokyo-night` code token colours (six stay below 4.5:1, all better than before; this needs its own order), the Decap admin (it follows `/main.css`), and the README.
- Beyond the plan's lines, each one disclosed with a reason in `build.rework-1.md`:
  - `tabular-nums` was removed from `.slip-lines dd` and narrowed on the dashboard, because Inter's tabular set widens the hyphen in "WO-0002".
  - `button, input { letter-spacing: inherit; }`
  - The editor backdrop is now maroon.
  - The pager title span (the fix for #3).
  All are type or colour changes and within the request.
- Truthfulness: the docs and OG image add no new claims about Nows. The OG text is unchanged, and the docs describe only colours, roles and the font.

## Acceptance checks
1. Pass. fmt, clippy and tests are clean (my run).
2. Pass. The zola build has no warnings, and newpost found no problems (my run).
3. Pass. `ls -l site/static/fonts` lists only `LICENSE-Inter.txt` (4,477 bytes) and the four Inter files at 72,920, 133,336, 79,716 and 146,460 bytes, as the plan specifies.
4. Pass. The archivo/newsreader grep prints nothing (rc 1).
5. Pass. The old-palette grep prints nothing (rc 1).
6. Pass. No `font-stretch` remains. Outside `_tokens.scss`, the only raw colours are the two `rgb(255 255 255 / ...)` lines on `.code-copy` (`_prose.scss:289-290`).
7. Pass. The layout-property grep over `git diff -U0 site/sass` prints nothing.
8. Pass. `git diff --stat site/static/_headers` is empty. The design shots run found 0 network or console problems (design.md).
9. Pass. The shots run found 0 axe, 0 layout and 0 console problems (design.md, rework 1).
10. Pass. I opened `target/shots/WO-0002-design-r1/home-1440-dark.png`. The page is maroon with a #570000 top slip, peach and rose copies, and a coral stamp inside the stack. The headline runs to 5 lines, and dates and descriptions are coral. Light mode was passed by design in round one.
11. Pass. In `home-1440-dark.png`, the menu and list titles are off-white, while "All work", "All articles", "More on what I'm doing now" and the footer links are coral and underlined. In `r-how-foot-focus-1440-dark.png`, the tags, the pager title and "Back to all writing" are coral and underlined. In `r-how-pager-1440-light.png`, the pager title is crimson and only the title is underlined.
12. Pass. Standfirsts are upright and the italic file loads (design.md, both rounds). Source: `font-style: italic` was removed from `.article-dek`.
13. Pass. The 390 crops and the shots run show no overflow (design.md).
14. Pass. `pre` has `box-shadow: inset 0 0 0 1px var(--code-edge)`, which is `#684c4c` in dark (diff). Design looked at it in round one.
15. Pass. `--focus` found 0 problems, and the manual Tab through the pager and footer was done in both themes (design.md). I opened `r-anchor-focus-light.png`: the "#" is deep red with an ink ring.
16. Pass. `site/public/favicon.svg` holds #FDFFFF, #280000, #B10F2E and #DE7C5A. `index.html` has theme-color metas `content=#FDFFFF` (light) and `content=#280000` (dark).
17. Pass. `og.png` is 1200x630 (PNG header). I opened it. "Nows" is in maroon Inter on off-white, and the role and tag lines stop before the slips. There is a crimson stamp. The slips overlap one another exactly as in the committed image (`git show HEAD:site/static/images/og.png`), so that is the design, not hidden text.
18. Pass. Font 200 and the new CSS confirmed on a fresh build (see above). The andon text is #F3F0F0 on #B10F2E at 6.23 in light and #280000 on #ff7b6b at 7.61 in dark (recomputed).
19. Pass. The skill and the docs both name the five colours, the roles in each theme, both "never as text" rules (skill lines 47-48), the Q2 accent rule (line 50), Q1, Inter, the type table and the contrast table. "indigo" and "ledger blue" appear 0 times in each. The ui-ux-review lines are updated. "Set in Inter." appears on `/`, `/articles/`, `/about/` and `/404.html` in `site/public`.

## For Nows
Read these first in `git diff`:
1. `site/sass/_tokens.scss` (the whole palette and the fonts)
2. `docs/design-system.md` (your own words on colour, type and contrast)
3. `site/sass/_base.scss` and `_chrome.scss` (who gets the accent)
4. `site/sass/_prose.scss`

The font swap shows up as 8 deleted and 5 new files in `site/static/fonts/`. `.claude/agent-memory/`, `.claude/skills/` and `.factory/` also changed. Those are the line's own notes and records, expected alongside the site change.

To try by hand:
- `zola --root site serve`, then switch the theme on `/` and `/articles/how-this-site-is-built/`. In dark mode, the links in the footer and crumb are the same coral as the text around them, and only the underline tells them apart.
- Your `factory serve` on port 4600 is an older build. Restart it to see the board in the new colours, because the CSS is compiled into the binary.
- The stage B editor's CSS ships inside the cms-api worker, so it only changes after you redeploy that worker.

Known leftovers, not blocking:
- Six code-highlight colours stay below 4.5:1 on the slab. This needs a follow-up order for a code theme.
- In dark mode, the editor's preview code block has no edge against its page.
- `_prose.scss` has a stray double blank line where `.tag` was removed (around line 48).

Suggested commit message: `Re-skin the site in five colours and Inter`
