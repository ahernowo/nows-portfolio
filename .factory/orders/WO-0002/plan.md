# Plan: Re-skin the site with a new palette and Inter

## Goal
Visitors see the site in Nows's five colours (#FDFFFF, #B10F2E, #570000, #280000, #DE7C5A) and in Inter, with larger, tighter headings and more open line spacing. The page layout stays the same. The receipt-book idea stays: an off-white original, a peach copy and a rose copy, maroon carbon ink, deep-red ledger lines and a crimson "Reconciled" stamp. Dark mode is the maroon carbon sheet. The design-system skill and `docs/design-system.md` record the new rules, so later changes follow them.

## Scope
In:
- Colour tokens for both themes (`_tokens.scss`), plus the colours kept outside the tokens: favicon, theme-color metas, the raw shadow and backdrop values in `_chrome.scss`, and the OG image.
- Inter variable (Fontsource `@fontsource-variable/inter@5.3.0`, opsz files with weight 100-900 and optical size 14-32) self-hosted with its OFL licence. Archivo, Newsreader and their licences are deleted. A new preload, and new `@font-face` rules.
- Type: Inter for headings and body. Bigger headings, tighter letter-spacing, more line height (Q3). Standfirsts become upright. Every `font-stretch` goes, since Inter has no width axis.
- Links in running text take the accent (Q2). The menu, the wordmark, list titles, search results and topic chips stay in the text colour.
- Secondary text: #570000 in light, #DE7C5A in dark (Q1).
- The two tool pages that copy the palette (design-system skill, "Admin pages"): the factory dashboard, which loads the site's fonts from `/fonts/` and breaks when Archivo is deleted, and the stage B editor CSS.
- Docs and skills: the design-system skill, `docs/design-system.md`, the ui-ux-review skill (its typography and accent lines), the footer colophon.

Out:
- Margins, padding, gaps, grids, widths and positions (Q3). Space comes from type only.
- CSP and `_headers`. `font-src 'self'` already allows self-hosted fonts, and the new files have new names, so the immutable cache rule stays correct. No change is needed.
- Code token colours. The `tokyo-night` theme stays (a decision Nows did not object to). Only the slab colour changes. Six token colours stay below 4.5:1 (see Risks), though each one improves on today's slab. A theme change needs its own order.
- The og alt text in `writing-reference.md` ("the name Nows on carbon-copy paper colours"). The card still shows Nows on copy-paper colours, so the text stays true and the content stays untouched.
- Decap CMS (`site/static/admin/`). Its preview loads `/main.css`, so it follows the change automatically.
- README. It says only "Fonts under the SIL Open Font License", which stays true.

## Token values
Write these exact hex values. The mixes are sRGB, named so the docs can explain them.

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#FDFFFF` | `#280000` | Page background |
| `--slip-original` | `#FFFFFF` (off-white tint) | `#570000` | Raised surfaces, top slip |
| `--copy-peach` (renames `--copy-yellow`) | `#F5DED6` (coral 25% on off-white) | `#4C1912` (coral 20% on maroon) | Second copy, selection, search mark, inline code, note alerts |
| `--copy-rose` (renames `--copy-pink`) | `#F2DBE0` (crimson 15%) | `#5F0612` (crimson 40% on maroon) | Third copy, warning alerts |
| `--highlight` | `#F7E5DE` (coral 20%) | `#43130E` (coral 15% on maroon) | Row highlighter, hovers |
| `--ink` | `#280000` | `#FDFFFF` | Text, focus ring |
| `--ink-soft` | `#570000` | `#DE7C5A` | Dates, descriptions, captions, meta |
| `--link` (new) | `#B10F2E` | `#DE7C5A` | Links in running text |
| `--link-line` (new) | `#CF6F82` (crimson 60%) | `#954A36` (coral 60% on maroon) | Underline of those links (3:1 against the page) |
| `--rule` | `#E4D9D9` (deep red 15%) | `#482626` (off-white 15% on maroon) | Ledger lines |
| `--rule-strong` | `#BB9999` (deep red 40%) | `#684C4C` (off-white 30%) | Borders |
| `--stamp` | `#B10F2E` | `#DE7C5A` | Stamp, now-pulse, reading line, warning edge |
| `--stamp-blend` | multiply | screen | unchanged |
| `--code-bg` / `--code-ink` | `#280000` / `#FDFFFF` | same | Code slab |
| `--code-edge` (new) | `transparent` | `#684C4C` | 1px inset edge, so the slab shows on the maroon page |
| `--slip-shadow` | `0 1px 0 rgb(40 0 0 / 6%), 0 12px 26px -14px rgb(40 0 0 / 30%)` | keep today's dark value | |
| `--dialog-shadow` (new) | `0 24px 60px -24px rgb(40 0 0 / 45%)` | `0 24px 60px -24px rgb(0 0 0 / 70%)` | Search dialog |
| `--backdrop` (new) | `rgb(40 0 0 / 40%)` | `rgb(0 0 0 / 55%)` | Search backdrop |

Fonts: `--font-display` and `--font-text` both become `"Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif`. Both names stay, which keeps the diff small. The tokens comment says they are the same family.

## Type
Q3: change only sizes, weight, tracking and line height.

| Element | Size | Weight | Letter-spacing | Line height |
|---|---|---|---|---|
| `.hero-title` | `--step-5`: keep the min and slope; set the max between 4.75 and 5.75rem so the headline wraps to 5 lines or fewer at 1280 and 1440 px | 700 | -0.03em | 1.04 |
| `.page-title` | `--step-4` becomes `clamp(2.5rem, 3.6vw + 1.4rem, 3.75rem)` | 700 | -0.025em | 1.08 |
| `.article-title`, `.case-title` | `clamp(2.6rem, 5vw + 0.6rem, 4.5rem)` | 700 | -0.028em | 1.06 |
| `.block-title` | `--step-3` becomes 2.625rem | 650 | -0.022em | 1.1 |
| `.index-title`, `.prose h2` | `--step-2` becomes 1.625rem | 600 / 650 | -0.017em | 1.2 / 1.25 |
| `.prose h3` | 1.25rem | 600 | -0.012em | 1.35 |
| `.year-title` | `--step-1` | 650 | -0.012em | inherit |
| `.wordmark` | 1.5rem | 750 | -0.025em | 1 |
| `.stamp span` | unchanged | 800 | -0.01em | 1 |
| `.slip-system`, `.footer-name`, `.pager-link`, `.back-link` | unchanged | 650 / 700 / 600 / 600 | -0.01em | inherit |
| body | 16px | 400 | -0.011em | 1.6 (was 1.55) |
| `.prose` | 1.1875rem (1.125rem below 40rem) | 400 | -0.014em | 1.7 (was 1.62) |
| `.article-dek`, `.case-dek` | 1.375rem, upright | 400 | -0.017em | 1.5 (was 1.4) |
| `.hero-sub`, `.page-dek` | `--step-1` | 400 | -0.017em | 1.5 (was 1.45) |
| Tables, alerts | unchanged | | | 1.5 / 1.6 |
| `pre`, `code` | unchanged | | `letter-spacing: normal` (the body tracking must not reach the monospace) | unchanged |

Delete every `font-stretch`. Blockquotes and `<em>` keep italic, which uses the real Inter italic files.

## Changes
- `site/static/fonts/`: delete the six Archivo/Newsreader woff2 files and `LICENSE-Archivo.txt` and `LICENSE-Newsreader.txt`. Add new `inter-latin-opsz-normal.woff2` (72,920 B), `inter-latin-ext-opsz-normal.woff2` (133,336 B), `inter-latin-opsz-italic.woff2` (79,716 B), `inter-latin-ext-opsz-italic.woff2` (146,460 B) from `https://cdn.jsdelivr.net/npm/@fontsource-variable/inter@5.3.0/files/`, and new `LICENSE-Inter.txt` from the package's `LICENSE`.
- `site/sass/_tokens.scss`: four `@font-face` rules for "Inter" (normal and italic, latin and latin-ext; `font-weight: 100 900`; unicode ranges as in Fontsource's `opsz.css`, the same as today's). Token values from the table, in `:root` and `dark-tokens`. Rename the copy tokens. Rewrite the header comment for the new palette. Update the scale comment.
- `site/sass/_base.scss`: the body font-size stays; line height and tracking per the type table, and drop `font-stretch`. On `a`, use `color: var(--link)` and `text-decoration-color: var(--link-line)`. `.link-button` matches. Add `pre, code { letter-spacing: normal; }`. `::selection` uses `--copy-peach`.
- `site/sass/_chrome.scss`: the wordmark per the type table, with `color: var(--ink)`. `.site-nav a` and `.tool` use `--ink` (Q2: the menu stays text colour). Nav hover shows a 1px `--rule-strong` underline. The current item keeps its 2px ink underline. `.footer-name` per the type table. Search dialog: use `--dialog-shadow` and `--backdrop` in place of the raw `rgb(20 25 56 ...)`. `.search-results a` uses `color: var(--ink)`. `mark` uses `--copy-peach`.
- `site/sass/_home.scss`: hero, slip and stamp type per the table. `.slip-2` and `.slip-3` use the renamed tokens. No position or size changes.
- `site/sass/_lists.scss`: titles and deks per the table. `.index-link` and `.topic-list a` use `color: var(--ink)`. `.crumb a` and `.block-more` drop their `--ink-soft` colour and take `--link` from `a`.
- `site/sass/_prose.scss`: title, dek (remove `font-style: italic`) and heading type per the table. `.tag` drops `color: var(--ink)` and takes `--link`. `.pager-link` and `.back-link` take `--link` (the hint stays `--ink-soft`). `pre` gets `box-shadow: inset 0 0 0 1px var(--code-edge)`. Add `.code-copy:focus-visible { outline-color: var(--code-ink); }`, because an ink ring on the slab is invisible in light. Inline code and alerts use the renamed tokens. Drop the `font-stretch` lines.
- `site/templates/base.html`: preload `fonts/inter-latin-opsz-normal.woff2`. Theme-color metas `#FDFFFF` (light) and `#280000` (dark).
- `site/templates/partials/footer.html`: the colophon says "Set in Inter."
- `site/static/favicon.svg`: light `.p #FDFFFF`, `.i #280000`, `.s #B10F2E`. Dark `.p #280000`, `.i #FDFFFF`, `.s #DE7C5A`.
- `scripts/make-og-image.py`: Inter in place of Archivo and Newsreader (the tag line becomes upright Inter 400 in SOFT). Colours PAPER `#FDFFFF`, INK `#280000`, SOFT `#570000`, RULE `#E4D9D9`, copies `#F5DED6` and `#F2DBE0`, the top slip `#FFFFFF`, STAMP `#B10F2E`, shadow `(40, 0, 0, 70)`. Inter has two axes, so take their order from `get_variation_axes()` before calling `set_variation_by_axes`. Keep the role line clear of the slips (they start at x=690): shrink its size if Inter runs wider.
- `site/static/images/og.png`: regenerate with the script (`pip install pillow fonttools brotli`, then `python scripts/make-og-image.py`).
- `tools/factory/dashboard/app.css`: `@font-face` "Inter" pointing at `/fonts/inter-latin-opsz-normal.woff2`. `--font` uses Inter. Drop the `font-stretch` lines (lines 7, 61, 171). Dark: `--bg #280000`, `--surface #3A0C09`, `--surface-2 #4C1912`, `--ink #FDFFFF`, `--ink-soft #DE7C5A`, `--rule #482626`, `--rule-strong #684C4C`, `--yellow #4C1912`, `--pink #5F0612`. The lamps `--run`, `--wait` and `--stop` keep their values. Light: `--bg #F3F0F0`, `--surface #FDFFFF`, `--surface-2 #FFFFFF`, `--ink #280000`, `--ink-soft #570000`, `--rule #E4D9D9`, `--rule-strong #BB9999`, `--run #146C4C`, `--wait #835B00`, `--stop #B10F2E`, `--yellow #F5DED6`, `--pink #F2DBE0`. `.andon` text becomes `color: var(--bg)` (today's white on `#ff7b6b` is about 2.4:1). The drawer backdrop becomes `rgb(0 0 0 / 55%)`.
- `workers/cms-api/admin/app.css`: light `--paper #FDFFFF`, `--ink #280000`, `--ink-soft #570000`, `--canary #F5DED6`, `--rose #F2DBE0`, `--rule #E4D9D9`, `--stamp #B10F2E`, `--field #FFFFFF`. Dark `--paper #280000`, `--ink #FDFFFF`, `--ink-soft #DE7C5A`, `--canary #4C1912`, `--rose #5F0612`, `--rule #482626`, `--stamp #DE7C5A`, `--field #570000`. `--ok` keeps its values. `.toast.bad` text becomes `var(--paper)`. The `.preview pre` colours become `#280000` / `#FDFFFF`. The fonts stay system fonts.
- `.claude/skills/design-system/SKILL.md`: rewrite the token table, the receipt-book description (colours), Type (Inter only; the type table above; the browsing-to-reading switch is now size and line height, not a family), and the accent rule. The accent goes to links in running text plus the stamp, now-pulse, reading line and warning alerts, never to headings, the menu or list titles. Add "#DE7C5A never as text on a light background; #B10F2E never as text on a dark one". Focus rings use `--ink`, which matches the code. Fonts section: Inter from Fontsource. In the generated-look checklist, reword "near-black background with one neon accent" so it plainly means a neutral black with a neon accent. The maroon and coral pairing is the brand, not that pattern. The "Admin pages" note stays.
- `docs/design-system.md`: rewrite the Colour, Type and Fonts sections in Nows's voice: the five colours and their roles in each theme, Inter, and a contrast table (the one below).
- `.claude/skills/ui-ux-review/SKILL.md`: line 44 becomes "Inter for everything; sizes, weights, tracking and line heights from the type table". The "Stamp red stays rare" line becomes "The accent appears only on running-text links, the stamp, now-pulse, reading line and warning alerts."

## Contrast table
Computed with the WCAG formula. The builder re-checks the final values and pastes the table into the build report and the docs.

| Pair | Light | Dark |
|---|---|---|
| Text on page | #280000 on #FDFFFF 19.19 | #FDFFFF on #280000 19.19 |
| Text on raised slip | on #FFFFFF 19.2 | on #570000 14.89 |
| Text on peach / rose / highlight | 14.95 / 14.66 / 15.79 | 14.38 / 13.75 / 15.64 |
| Secondary text on page | #570000 on #FDFFFF 14.89 | #DE7C5A on #280000 6.55 |
| Secondary text on slip | 14.9 | #DE7C5A on #570000 5.08 |
| Secondary on peach / rose / highlight | 11.60 / 11.37 / 12.25 | 4.90 / 4.69 / 5.33 |
| Link on page | #B10F2E on #FDFFFF 7.03 | #DE7C5A on #280000 6.55 |
| Link on peach / rose / highlight | 5.48 / 5.37 / 5.78 | 4.90 / 4.69 / 5.33 |
| Link underline vs page (non-text) | #CF6F82 3.35 | #954A36 3.04 |
| Stamp on rose copy | 5.37 | coral with screen blend, lighter than 4.69 |
| Focus ring vs page | 19.19 | 19.19 |
| Code text on slab | #FDFFFF on #280000 19.19 | same |
| Code tokens on slab | 28 of 34 are 4.70-16.37; z-4 2.66, z-5 2.82, z-6 3.30, z-9 3.90, z-16 3.09, z-23 3.84 (today on #161b45: 2.27-3.33) | same |
| Dashboard lamps | run #146C4C 5.65, wait #835B00 5.35, stop #B10F2E 6.23 on #F3F0F0 | run 10.40, wait 12.74, stop 7.61 on #280000; stop 5.70 on #4C1912 |
| Andon / editor error toast | #F3F0F0 on #B10F2E 6.23; #FDFFFF on #B10F2E 7.03 | #280000 on #ff7b6b 7.61; #280000 on #DE7C5A 6.55 |
| Editor ok / error | #1d6b4f on #FDFFFF 6.40; #B10F2E 7.03 | #7fd8b4 on #570000 8.81; #DE7C5A on #570000 5.08 |

No pair puts #DE7C5A text on a light background or #B10F2E text on a dark one.

## Acceptance checks
1. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass (the editor CSS is compiled in with `include_str!`).
2. `zola --root site build` succeeds with no warnings. `cargo run -q -p newpost -- --check` passes.
3. `ls site/static/fonts` lists exactly `LICENSE-Inter.txt` and the four `inter-*-opsz-*.woff2` files, at the byte sizes given above.
4. `grep -rniE "archivo|newsreader" site scripts tools/factory docs .claude/skills README.md --exclude-dir=public --exclude-dir=node_modules` prints nothing.
5. `grep -rniE "1f2463|b4302a|fcfcfa|141938|4a4f86|fbf0b4|f9dce2|ff8273|ff8a7d|b9c9e6|8fa6d3|aeb3da|eceef9|161b45|0e1230|1d2350|copy-yellow|copy-pink|rgb\(20 25 56|rgb\(31 36 99" site scripts tools/factory/dashboard workers/cms-api/admin --exclude-dir=public` prints nothing.
6. `grep -rn "font-stretch" site/sass tools/factory/dashboard/app.css` prints nothing. `grep -rnE "#[0-9a-fA-F]{3,6}\b|rgb\(" site/sass | grep -v _tokens.scss` prints only the two `rgb(255 255 255 / ...)` lines on `.code-copy`.
7. Layout untouched: `git diff -U0 site/sass | grep -E "^[-+]\s*(margin|padding|gap|column-gap|row-gap|grid-template|width|max-width|min-width|height|min-height|top|left|right|bottom|inset|translate|rotate)\b"` prints nothing.
8. `git diff --stat site/static/_headers` is empty. The shots run (check 9) reports no CSP blocks and no failed font requests.
9. `node tools/shots/shoot.mjs --out target/shots/WO-0002 --drafts --pages /,/work/,/articles/,/about/,/now/,/tags/,/404.html,/articles/how-this-site-is-built/,/articles/writing-reference/,/work/this-website/` (both themes, 390 and 1440) reports no sideways scrolling, no console errors, and no axe colour-contrast failures, except spans inside `pre` with the six token classes listed in the contrast table.
10. Home at 1440, light: off-white background, a maroon headline in Inter at 5 lines or fewer (also at 1280), a white top slip, peach and rose copies, and a crimson "Reconciled" stamp that stays inside the stack. Dark: a #280000 page, a #570000 top slip, a coral stamp, and coral dates and descriptions in the work and writing lists.
11. On `/articles/how-this-site-is-built/` at 1440: the "Writing" crumb, prose links, tag links, Older/Newer titles, "Back to all writing" and footer links are crimson in light and coral in dark, each underlined. The wordmark and the menu items are maroon in light and off-white in dark. On `/work/` and `/articles/`, list titles are text colour in both themes, and dates and descriptions are #570000 in light and coral in dark.
12. Standfirsts on an article and a case study render upright. `<em>` and blockquotes on `/articles/writing-reference/` render in true italic (the shots network list shows `inter-latin-opsz-italic.woff2` loaded with 200).
13. At 390 px: the hero, article, case and page titles wrap with no clipped or overflowing word, and nothing overlaps the receipt stack.
14. In dark, the code slab on `/articles/writing-reference/` shows a 1px edge against the page. Code reads as off-white on maroon.
15. Focus: `--focus` on `/` and `/articles/writing-reference/` shows a visible ring at every stop in both themes, including the code Copy button in light. Per the WO-0001 lesson, also Tab manually from the last prose link of `/articles/how-this-site-is-built/` through the pager, "Back to all writing" and the footer links, checking the ring at each one in light and dark.
16. `site/public/favicon.svg` holds `#FDFFFF`, `#280000` and `#B10F2E` (light) and `#DE7C5A` (dark). `site/public/index.html` has theme-color metas with `#FDFFFF` and `#280000`.
17. `site/static/images/og.png` is 1200x630 and shows "Nows" in maroon Inter on off-white, with no text hidden behind a slip and a crimson stamp (open it with Read).
18. With `factory serve` running, `curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:4600/fonts/inter-latin-opsz-normal.woff2` prints `200`. The board renders in Inter, and the andon bar text is readable (values per the contrast table).
19. The design-system skill and `docs/design-system.md` name the five colours, their roles in each theme, the two "never as text" rules, the accent rule (Q2), the secondary-text rule (Q1), Inter, the type table and the contrast table. Neither mentions Archivo, Newsreader, indigo or ledger blue. The ui-ux-review skill lines are updated. The footer colophon on every page reads "Set in Inter."

## Risks
- **Design: Inter runs much wider than narrow Archivo.** At the same size, headings take about 1.4 times the width. Look at the hero line count (check 10), the stamp width inside the stack, long words like "reconciliation" in titles at 390 px, the 48%-wide pager links, the case form fields, and the wordmark next to the menu at 390 px.
- **Design: in dark, coral serves both secondary text and links** (Q1 plus Q2). Links stand apart from the coral text around them (footer, crumb, meta) only by their underline. Check that `--link-line` is visible on the dark footer and crumb. The generated-look checklist: maroon with coral must not read as "dark background with neon accent"; the receipt stack keeps it specific.
- **Accessibility:** the coral margins are thin in dark: 4.69 on the rose copy (slip-3 labels, alerts) and 4.90 on peach. Any further tint change breaks AA, so re-check the final hex values. Six code token colours stay below 4.5:1. No published article uses them today (`grep` of `site/public`), but the draft `writing-reference` may. They are better than today. A follow-up order should pick a code theme that clears 4.5:1 on #280000.
- **Truthfulness:** the docs describe the design in Nows's voice. Write only what this order sets (colours, roles, font). Add no new story about why.
- **Security:** fonts come from one pinned npm version over jsdelivr. Check the byte sizes (check 3). No CSP or header change. The editor CSS ships inside the cms-api worker, so Nows redeploys it to see the change there.
- **Breakage outside the site:** the dashboard reads fonts from `site/static/fonts` (`tools/factory/src/server.rs:186`). Check 18 covers it.

## Skills the builder needs
zola-site, design-system, ui-ux-review (to run the shots tool), factory-protocol.

## Decisions from Nows
- Q1, dark-mode secondary text (dates, descriptions, captions): #DE7C5A coral. Light-mode secondary text stays #570000.
- Q2, which links take #B10F2E / #DE7C5A: links in running text only (article and page text, footer, breadcrumbs, previous/next, "back to writing"). The main menu and the titles in the work and writing lists stay text colour. Planner's reading, for the reviewers: inline links in sentences (hero "now" line, page aside, tag line, 404 list, case-form links) and the "All work" / "All articles" links count as running text. Topic chips, search results and the wordmark stay text colour, like the menu and list titles.
- Q3, white space: from type only (bigger headings, tighter letter-spacing, more line height). Margins, padding and the grid stay as they are.
- Accepted without objection: the receipt-book idea stays; Fontsource `@fontsource-variable/inter@5.3.0`; upright standfirsts; code slab #280000 with the existing token colours.
