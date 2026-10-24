# Plan: Soften the palette and make light the default theme

## Goal
Every visitor sees the light theme first, whatever their system setting: a soft #FAF5F3 page, white surfaces, #2B1618 text, #6B5557 muted text and crimson #B10F2E links and stamp. Dark (#1F1B1C page, #2C2527 surfaces, #F2EDEB text, #C2B6B3 muted text, #E8896A accent, #E8546D stamp) appears only when the visitor picks it with the Theme button, and is remembered after that. The button's label says which theme is showing ("Theme: light" / "Theme: dark"). The design-system skill and `docs/design-system.md` record the new palette and theme rules.

## Scope
In:
- Colour tokens for both themes in `_tokens.scss`, with the tints worked out from Nows's colours (table below). The receipt-book idea stays: a white top slip and peach and rose copies.
- Theme behaviour: no more `prefers-color-scheme` on the site. Light is the default. The button cycles light and dark only (no "auto"). The choice is saved in `localStorage` (`theme`), as today.
- Colours kept outside the tokens: the theme-color meta and color-scheme meta (`base.html`), the OG image script and `og.png`, and the two tool stylesheets that copy the palette (the design-system skill's "Admin pages" rule: factory dashboard, stage B editor).
- `tools/shots/shoot.mjs`: its dark run must set the saved choice, because emulating a dark system no longer turns the site dark. Without this, every "dark" screenshot and axe scan would really show light.
- Docs: the design-system skill, `docs/design-system.md`, and the ui-ux-review skill's `--themes` row.

Out:
- Layout: margins, padding, gaps, grids, sizes and positions. The request says colours and theme behaviour only.
- The logo. The header mark paints with `currentColor`, so it follows `--ink` with no change (Nows: "needs no change"). The favicon (`nows-flow-favicon.svg`, a crimson tile with an off-white mark) is a self-contained image, and crimson is still the light accent, so it stays. The unused `nows-flow-mark-light.svg` / `-dark.svg` stay untouched.
- The code slab and its token colours (`--code-bg #280000`, `--code-ink #FDFFFF`, the `tokyo-night` classes). Every slab colour in the new palette makes things worse. On #2B1618 or #1F1B1C, nine token classes fall below 4.5:1 (z-7, z-12 and z-27 join today's six). Keeping #280000 holds the exception at today's six. The design-system skill says a code-theme change is its own order. In light, #280000 next to #2B1618 text is close to indistinguishable. In dark, the slab sits 1.13:1 off the page, with its 1 px edge.
- Theme behaviour of the dashboard and the stage B editor. They have no Theme button: the dashboard stays dark by default, the editor follows the system. Only their colour values change.
- Typography, motion, copy (apart from the button label), CSP/`_headers`, Decap (`site/static/admin/`; its preview loads `/main.css` and follows automatically).
- The og alt text in `writing-reference.md` ("the name Nows on carbon-copy paper colours"). It stays true.

## Token values
Write these exact hex values. The mixes are sRGB, named so the docs can explain them. "Coral" below means the dark accent #E8896A and "crimson" #B10F2E.

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#FAF5F3` | `#1F1B1C` | Page background (Nows) |
| `--slip-original` | `#FFFFFF` | `#2C2527` | Surfaces, the top slip (Nows) |
| `--copy-peach` | `#F9E3DB` (coral 24% on white) | `#432F2A` (coral 18% on #1F1B1C) | Second copy, selection, search marks, inline code, note alerts |
| `--copy-rose` | `#F6E2E6` (crimson 12% on white) | `#47262C` (#E8546D 20% on #1F1B1C) | Third copy, warning alerts |
| `--highlight` | `#FCEDE9` (coral 15% on white) | `#332624` (coral 10% on #1F1B1C) | Row highlighter, hovers |
| `--ink` | `#2B1618` | `#F2EDEB` | Text, focus rings (Nows) |
| `--ink-soft` | `#6B5557` | `#C2B6B3` | Muted text: dates, descriptions, captions, meta, heading anchors (Nows) |
| `--link` | `#B10F2E` | `#E8896A` | Links in running text (Nows) |
| `--link-line` | `#CD6679` (crimson 62% on #FAF5F3) | `#9C5F4C` (coral 62% on #1F1B1C) | Their underline, at least 3:1 against the page |
| `--accent` (new) | `#B10F2E` | `#E8896A` | Now-pulse, reading line, warning and caution alert edge |
| `--rule` | `#E1DAD9` (text 12% on page) | `#3D3839` (text 14% on page) | Ledger lines |
| `--rule-strong` | `#BCB2B1` (text 30%) | `#5A5656` (text 28%) | Borders |
| `--stamp` | `#B10F2E` | `#E8546D` | The "Reconciled" stamp only (Nows) |
| `--stamp-blend` | multiply | screen | unchanged |
| `--code-bg`, `--code-ink` | `#280000`, `#FDFFFF` | the same | unchanged (see Out) |
| `--code-edge` | `transparent` | `#5A5656` | 1 px edge of the slab on the dark page |
| `--slip-shadow` | `0 1px 0 rgb(43 22 24 / 6%), 0 12px 26px -14px rgb(43 22 24 / 30%)` | unchanged (black) | |
| `--dialog-shadow` | `0 24px 60px -24px rgb(43 22 24 / 45%)` | unchanged | |
| `--backdrop` | `rgb(43 22 24 / 40%)` | unchanged | |

Why `--accent` is new: today `--stamp` colours the stamp, now-pulse, reading line and warning edge. Nows gave the dark stamp its own colour (#E8546D) and called #E8896A "the accent". So the stamp keeps `--stamp`, and the three other marks move to `--accent`. In light, all three are #B10F2E, as today. Planner's reading for the reviewers: Nows's dark "accent" includes links. In light, Nows wrote "accent and links" for the same colour, and the design system defines the accent as running-text links.

## Changes
First, before editing: take baseline shots for the reviewers: `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0004-before --drafts --pages /,/articles/writing-reference/ --no-axe` (git-ignored, seal-safe).

- `site/sass/_tokens.scss`: token values from the table, in `:root` and in the `dark-tokens` mixin. Add `--accent` to both. Delete the `@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) ... }` block. Keep `:root[data-theme="dark"] { @include dark-tokens; }`. Rewrite the header comment for the new palette and the rules (light by default; dark only by choice; #E8896A and #E8546D never text on light, #B10F2E never text on dark).
- `site/sass/_home.scss`: `.now-pulse` background `var(--stamp)` becomes `var(--accent)` (line 43). The stamp (line 151) keeps `--stamp`.
- `site/sass/_prose.scss`: `.reading-progress` background becomes `var(--accent)` (line 487). `.markdown-alert-warning` and `-caution` `--alert-edge` become `var(--accent)` (lines 381, 387). Line 245's comment ("maroon slab ... maroon dark page") is reworded to "so the slab still shows on the dark page".
- `site/sass/_base.scss`: the comment above `a` (line 42) says "crimson in light, coral #E8896A in dark". No rule changes.
- `site/static/js/head.js`: apply a saved theme only (`dark` or `light`); with nothing saved, set nothing, so the light `:root` tokens apply. Comment: "Storage can be blocked; the page then stays light."
- `site/static/js/site.js`: `THEMES = ["light", "dark"]`. `readTheme()` returns `"dark"` only when storage says `"dark"`, otherwise `"light"`. On load, only set the label text from `readTheme()`, and write nothing to storage (a first visit stores nothing). On click: next theme, set `data-theme` on `<html>`, save it with `localStorage.setItem("theme", next)` inside `try`, and set every `[data-theme-label]` to `"Theme: " + theme`. Drop the `auto` branches. The section comment becomes "Theme: light by default, dark by choice".
- `site/templates/partials/header.html`: the label's no-JS text becomes `Theme: light` (line 22). The button keeps `aria-live="polite"`.
- `site/templates/base.html`: `<meta name="color-scheme" content="light">`. The two theme-color metas become one, `<meta name="theme-color" content="#FAF5F3">`, without `media`. The comment on line 31 stays true.
- `tools/shots/shoot.mjs`: in `openPage`, after creating the context and before `newPage`, when `theme === "dark"`, add `await context.addInitScript(() => { try { localStorage.setItem("theme", "dark"); } catch (e) {} });`. Keep `colorScheme: theme`. Update the header comment's `--themes` line: "light,dark (dark is set as the visitor's saved choice)".
- `scripts/make-og-image.py`: `PAPER #FAF5F3`, `INK #2B1618`, `SOFT #6B5557`, `RULE #E1DAD9`, `PEACH #F9E3DB`, `ROSE #F6E2E6`, `STAMP #B10F2E`, `SLIP #FFFFFF`. The slip shadow fill becomes `(43, 22, 24, 70)`. No layout change.
- `site/static/images/og.png`: regenerate (`pip install pillow fonttools brotli`, then `python scripts/make-og-image.py`).
- `tools/factory/dashboard/app.css`: header comment updated. Dark (default): `--bg #1F1B1C`, `--surface #2C2527`, `--surface-2 #432F2A`, `--ink #F2EDEB`, `--ink-soft #C2B6B3`, `--rule #3D3839`, `--rule-strong #5A5656`, `--yellow #432F2A`, `--pink #47262C`. Light: `--bg #F0EAE8` (text 5% on #FAF5F3), `--surface #FAF5F3`, `--surface-2 #FFFFFF`, `--ink #2B1618`, `--ink-soft #6B5557`, `--rule #E1DAD9`, `--rule-strong #BCB2B1`, `--yellow #F9E3DB`, `--pink #F6E2E6`. The lamps (`--run`, `--wait`, `--stop`) keep their values. Update the andon comment's ratios to 6.7:1 dark and 5.9:1 light.
- `workers/cms-api/admin/app.css`: header comment updated. Light: `--paper #FAF5F3`, `--ink #2B1618`, `--ink-soft #6B5557`, `--canary #F9E3DB`, `--rose #F6E2E6`, `--rule #E1DAD9`, `--stamp #B10F2E`, `--field #FFFFFF`. Dark: `--paper #1F1B1C`, `--ink #F2EDEB`, `--ink-soft #C2B6B3`, `--canary #432F2A`, `--rose #47262C`, `--rule #3D3839`, `--stamp #E8896A` (not #E8546D: the editor's `--stamp` colours error text, and #E8546D on the #2C2527 field is 4.23:1), `--field #2C2527`. `--ok` keeps its values. The backdrop (line 205) becomes `rgb(43 22 24 / .45)`. `.preview pre` keeps `#280000` / `#fdffff`, matching the site's slab.
- `.claude/skills/design-system/SKILL.md`: rewrite the palette paragraph (top), the palette table (light: #FAF5F3, #FFFFFF, #2B1618, #6B5557, #B10F2E; dark: #1F1B1C, #2C2527, #F2EDEB, #C2B6B3, #E8896A, #E8546D; the code slab #280000/#FDFFFF kept), the token table (from this plan), the "Themes:" line (light for every visitor whatever the system setting; dark only from the Theme button, saved as `theme` in localStorage and set as `data-theme="dark"` by `head.js` before first paint; the button reads "Theme: light" / "Theme: dark"; no `prefers-color-scheme`), the two never-as-text rules (#E8896A and #E8546D never text on light; #B10F2E never text on dark), the accent rule (`--link` and `--accent`; the stamp has its own `--stamp`), secondary text (#6B5557 / #C2B6B3), the contrast table (below), and the checklist's parenthetical (warm carbon #1F1B1C with coral #E8896A is the brand's dark theme; the receipt stack keeps it specific). Dark is no longer "the maroon carbon sheet": call it the carbon sheet. Type, layout, motion and copy sections stay.
- `docs/design-system.md`: the same palette, theme behaviour and contrast table, in Nows's voice: "The pieces" colour bullet, the Colour section (colours, rules, accent, tints table, contrast table), the code paragraph (slab kept at #280000; six token classes below 4.5:1, as before). Add a short "Themes" paragraph: light for everyone; dark when you choose it with the Theme button, and remembered on that browser.
- `.claude/skills/ui-ux-review/SKILL.md`: the `--themes` row reads "Themes (default both). Dark is set as the visitor's saved choice, the way the Theme button does; the system setting no longer changes the site."

## Contrast table
Computed with the WCAG formula from the hex values above (`scratch` script, sRGB). The builder re-computes the final values and puts this table in the build report and both docs.

| Pair | Light | Dark |
|---|---|---|
| Text on page | #2B1618 on #FAF5F3 15.78 | #F2EDEB on #1F1B1C 14.68 |
| Text on surface / top slip | on #FFFFFF 17.06 | on #2C2527 12.91 |
| Text on peach / rose / highlight | 13.84 / 13.76 / 14.98 | 10.78 / 11.44 / 12.53 |
| Muted text on page (also heading anchors #, tool kbd) | #6B5557 6.34 | #C2B6B3 8.63 |
| Muted text on surface / top slip | 6.86 | 7.59 |
| Muted text on peach / rose / highlight | 5.56 / 5.53 / 6.02 | 6.34 / 6.73 / 7.37 |
| Link on page | #B10F2E 6.52 | #E8896A 6.67 |
| Link on surface | 7.05 | 5.87 |
| Link on peach / rose / highlight (alerts, hovers) | 5.72 / 5.69 / 6.19 | 4.90 / 5.20 / 5.69 |
| Link underline against page (non-text, 3:1) | #CD6679 3.36 | #9C5F4C 3.37 |
| Stamp on the rose copy (large bold text, 3:1) | #B10F2E 5.69 | #E8546D 3.74 before the screen blend, which only lightens it |
| Stamp on page | 6.52 | 4.81 |
| Now-pulse, reading line (accent, non-text) | 6.52 | 6.67 |
| Focus ring against page | 15.78 | 14.68 |
| Skip link (page colour on ink) | 15.78 | 14.68 |
| Code text on slab | #FDFFFF on #280000 19.19 | same |
| Code tokens on slab | unchanged: 28 of 34 at 4.70-16.37; z-4 2.66, z-5 2.82, z-6 3.30, z-9 3.90, z-16 3.09, z-23 3.84 | same |
| Never-as-text pairs (for the rules) | #E8896A on #FAF5F3 2.36; #E8546D on #FAF5F3 3.28 | #B10F2E on #1F1B1C 2.42 |
| Dashboard | ink on bg #F0EAE8 14.33, muted 5.76; lamps run 5.38, wait 5.10, stop 5.92 on bg; stop 5.72 on #F9E3DB | lamps on #1F1B1C: run 9.20, wait 11.27, stop 6.74; stop on surface-2 #432F2A 4.94; ink / muted on surface-2 10.78 / 6.34 |
| Andon text (page on stop lamp) | #F0EAE8 on #B10F2E 5.92 | #1F1B1C on #FF7B6B 6.74 |
| Editor | ok #1D6B4F 5.94 on paper, 6.43 on field; stamp 6.52 on paper; toast (paper on stamp) 6.52 | ok #7FD8B4 10.05 / 8.84; stamp #E8896A 6.67 on paper, 5.87 on field, 5.20 on rose; toast 6.67 |

Thin margins: dark links on peach (4.90) and the dark stamp on rose (3.74, passing only because the stamp is large bold text). Any later tint change needs a new check.

## Acceptance checks
1. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass (the editor CSS is compiled into cms-api).
2. `zola --root site build` succeeds with no warnings. `cargo run -q -p newpost -- --check` passes.
3. No system-driven theme left: `grep -rn "prefers-color-scheme" site/sass site/static/js site/templates` prints nothing, and `grep -c "prefers-color-scheme" site/public/main.css` prints `0`. (`_motion.scss` uses `prefers-reduced-motion`, which this grep does not match.)
4. Old palette gone: `grep -rniE "570000|de7c5a|f5ded6|f2dbe0|f7e5de|cf6f82|e4d9d9|bb9999|4c1912|5f0612|43130e|954a36|482626|684c4c|3a0c09|f3f0f0|rgb\(40 0 0" site/sass site/templates site/static/js scripts tools/factory/dashboard workers/cms-api/admin docs/design-system.md .claude/skills/design-system .claude/skills/ui-ux-review` prints nothing. `grep -rniE "fdffff|280000" site/sass site/templates scripts tools/factory/dashboard workers/cms-api/admin` prints only the `--code-bg` / `--code-ink` lines of `_tokens.scss` (both blocks) and the `.preview pre` line of the editor CSS.
5. `site/sass/_tokens.scss` holds exactly the token table's values in `:root` and `dark-tokens`, including `--accent` in both. `grep -n "var(--stamp)" site/sass/*.scss` prints only the `.stamp` rule in `_home.scss`. `grep -n "var(--accent)" site/sass/*.scss` prints the now-pulse, the reading line and the two alert edges.
6. Layout untouched: `git diff -U0 site/sass tools/factory/dashboard/app.css workers/cms-api/admin/app.css | grep -E "^[-+]\s*(margin|padding|gap|column-gap|row-gap|grid-template|width|max-width|min-width|height|min-height|top|left|right|bottom|inset|translate|rotate|font-size|font-weight|letter-spacing|line-height)\b"` prints nothing.
7. Built head: `grep -o "<meta name=theme-color[^>]*>" site/public/index.html` prints exactly one tag, with `content=#FAF5F3` and no `media`. `grep -o "<meta name=color-scheme[^>]*>" site/public/index.html` shows `content=light`. (Zola minifies, so attributes are unquoted.)
8. Theme behaviour, scripted. Start `zola --root site serve` in the background, then from `tools/shots` run:
   ```bash
   cd /c/Users/Diorama/nows-portfolio/tools/shots && node --input-type=module -e '
   import { chromium } from "playwright";
   const b = await chromium.launch();
   for (const width of [1440, 390]) {
     const c = await b.newContext({ colorScheme: "dark", viewport: { width, height: 900 } });
     const p = await c.newPage();
     const s = () => p.evaluate(() => [getComputedStyle(document.body).backgroundColor, document.documentElement.getAttribute("data-theme"), document.querySelector("[data-theme-label]").textContent, localStorage.getItem("theme")]);
     await p.goto("http://127.0.0.1:1111/");
     console.log(width, "first", await s(), await p.getByRole("button", { name: "Theme: light" }).count());
     await p.click("[data-theme-toggle]"); console.log(width, "click", await s());
     await p.reload(); console.log(width, "reload", await s());
     await p.click("[data-theme-toggle]"); await p.reload(); console.log(width, "back", await s());
     await c.close();
   }
   await b.close();'
   ```
   Expected at both widths, with the system set to dark: `first [ 'rgb(250, 245, 243)', null, 'Theme: light', null ] 1`, `click [ 'rgb(31, 27, 28)', 'dark', 'Theme: dark', 'dark' ]`, `reload` the same as click, `back [ 'rgb(250, 245, 243)', 'light', 'Theme: light', 'light' ]`. Stop the serve process afterwards.
9. `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0004 --drafts --pages /,/work/,/articles/,/about/,/now/,/tags/,/404.html,/articles/how-this-site-is-built/,/articles/writing-reference/,/work/this-website/` (both themes, 390 and 1440) reports no sideways scrolling, no console errors, no CSP blocks, and no axe colour-contrast failures other than spans inside `pre` with the six token classes in the contrast table. Every `-dark.png` really shows the #1F1B1C page (proof that the shoot.mjs change works).
10. Home at 1440. Light: #FAF5F3 page, #2B1618 headline, #6B5557 intro, a white top slip, peach and rose copies that read as three distinct slips against the page and each other, a crimson stamp and a crimson now-pulse. Dark: #1F1B1C page, #2C2527 top slip, #E8546D stamp, #E8896A now-pulse, #C2B6B3 slip labels. Compare with `target/shots/WO-0004-before/`: only colours differ, nothing moves.
11. On `/articles/how-this-site-is-built/` at 1440: the "Writing" crumb, prose links, tag links, the Older/Newer titles, "Back to all writing" and footer links are #B10F2E in light and #E8896A in dark, each underlined. The wordmark, its flow mark and the menu are #2B1618 in light and #F2EDEB in dark. On `/work/` and `/articles/`, list titles are text colour and dates and descriptions are muted, in both themes. The reading line is crimson / #E8896A.
12. On `/articles/writing-reference/`: note, tip and important alerts on peach, warning and caution on rose with an accent edge, readable in both themes. The code slab is #280000 with off-white text in both themes, with a visible 1 px edge in dark.
13. The header at 390 and 1440 shows the Theme button where it was. At 1440 its visible label reads "Theme: light" on a first visit and "Theme: dark" after one click. At 390 the label is visually hidden but is the button's accessible name (check 8, `count` 1).
14. Focus: `--focus` on `/` and `/articles/writing-reference/` shows a visible ring at every stop in both themes, the code Copy button included. Per the WO-0001 lesson, Tab manually from the last prose link of `/articles/how-this-site-is-built/` through the pager, "Back to all writing" and the footer links, in light and dark. Per the WO-0002 lesson, Tab onto a heading anchor (#) and the Copy button and check their contrast by hand (anchor #6B5557 / #C2B6B3 on page, from the table).
15. `site/static/images/og.png` is 1200x630. `python -c "from PIL import Image; print(Image.open('site/static/images/og.png').getpixel((10, 10)))"` prints `(250, 245, 243)`. Opened with Read: "Nows" in #2B1618, muted tag lines, three slips, a crimson stamp, no text hidden behind a slip.
16. With `factory serve` running, http://127.0.0.1:4600 renders the board on #1F1B1C with #F2EDEB text and the lamps unchanged, and the andon bar text is readable. The editor CSS values match the Changes list.
17. The design-system skill and `docs/design-system.md` name the palette per theme, the never-as-text rules, the accent rule with `--accent` and `--stamp`, muted text, theme behaviour (light by default, dark by choice, remembered, label shows the current theme), the token table and the contrast table, all matching `_tokens.scss`. Neither still says dark comes from the system setting or calls dark "maroon". The ui-ux-review `--themes` row is updated.
18. `git diff --stat site/static/_headers site/static/images/nows-flow-favicon.svg site/static/images/nows-flow-mark.svg site/static/images/nows-flow-mark-light.svg site/static/images/nows-flow-mark-dark.svg site/content` prints nothing.

## Risks
- **Design: soft means low separation.** In light, the peach and rose copies sit only 1.14-1.15:1 off the page, and the highlighter 1.05:1. The slips still have their shadow and torn edge. Check in the home shots that the stack still reads as three slips and that the row highlighter is visible on hover. If it does not, the builder may deepen the copy tints. Each tint change needs re-checked contrast numbers.
- **Design: generated look.** A near-neutral warm charcoal (#1F1B1C) with a single coral accent is close to the checklist's "neutral black background with one neon accent". The receipt stack, the warm tints and the crimson-pink stamp keep it specific. The reviewer should judge the dark home and an article against the checklist.
- **Design: the code slab** stays #280000 (see Out). In dark it is a slightly redder block on the charcoal page. Judge it on `/articles/writing-reference/`. A code-theme order can soften it later without failing more token colours.
- **Accessibility:** the thin margins are dark links on peach (4.90) and the dark stamp on rose (3.74, large text only). The Theme button's label changes on click and the button keeps `aria-live="polite"`, so screen readers hear "Theme: dark". Its accessible name must stay the label at 390 px (check 13). Six code token classes stay below 4.5:1, as before.
- **Behaviour:** visitors who saved "dark" keep it, and visitors who saved "light" keep light. Visitors who relied on their dark system setting now get light. That is the request. Without JavaScript the site is always light, and the button (labelled "Theme: light") does nothing, as before. The browser chrome is #FAF5F3 even in dark, because Nows asked the meta to use the light background.
- **Tooling:** if the `shoot.mjs` change is missed, dark shots and dark axe scans silently test the light theme. Check 9's "every -dark.png is dark" catches it.
- **Truthfulness:** the docs describe what this order sets (colours, roles, theme behaviour), in Nows's voice, with no new story about why.
- **Security:** no CSP, header, or storage change beyond the existing `theme` key. `head.js` and `site.js` stay same-origin files. The editor CSS ships inside the cms-api worker, so Nows redeploys it to see the change there.

## Skills the builder needs
design-system, zola-site, ui-ux-review (shots tool), rust-workers (only to rebuild and test cms-api after its CSS changes), factory-protocol.

## Decisions from Nows
None asked. The planner's readings are listed for the reviewers: dark "accent" includes links; the now-pulse, reading line and warning edge follow the accent and the stamp alone takes #E8546D; the code slab stays #280000; the dashboard and editor get the new colours but keep their own theme behaviour.
