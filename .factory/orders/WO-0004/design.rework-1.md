# Design review: Soften the palette and make light the default theme

## Verdict
Sent back for finding #1 (medium). Everything else holds: the palette, the light default, the Theme button and the docs match the plan, and both themes read as the receipt book.

## What I looked at
- Diff: every changed file read in full where it affects what people see (`_tokens.scss`, `_home.scss`, `_prose.scss`, `_base.scss`, `head.js`, `site.js`, `base.html`, `header.html`, `shoot.mjs`, `make-og-image.py`, `og.png`, both tool stylesheets, the design-system skill, `docs/design-system.md`).
- `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0004-design --drafts --focus --pages /,/work/,/articles/,/about/,/now/,/tags/,/404.html,/articles/how-this-site-is-built/,/articles/writing-reference/,/work/this-website/`: 40 page shots (390 and 1440, light and dark) plus focus stops. "Console, network and layout problems (0)", "Accessibility scan, WCAG 2.2 AA (0 rule failures)", "Focus (0 problems)". Opened home 1440 light/dark, home 390 dark, writing-reference 1440 light/dark, work/this-website 1440 light.
- Crops in `target/shots/WO-0004-design/crops/` (Playwright against :1111 and a drafts build in `target/wo4-review-site` on :4711): hero stack both themes, row hover both themes, search dialog empty and with results both themes, focus on the pager, "Back to all writing", "Read the source", a heading anchor and a Copy button after the fade settles (`settled-*.png`).
- Reduced motion: no motion changed (only `--stamp` to `--accent` on the pulse and reading line), so not captured.

## Checklist
- Identity and layout: ok. Light home: white, peach and rose slips read as three slips against the page through their shadows (`hero-light.png`); dark: charcoal, brown and plum slips, pink-red stamp. Dark judged against the generated-look checklist: the warm tints, the receipt stack and the coral-underlined links keep it specific, not "black plus neon". Page heights match the before shots (builder's check 10); no layout property in the diff.
- Typography: not affected.
- Colour and contrast: #1. All other pairs recomputed and match the plan's table (below). Grepped every `color:` token use per the WO-0002 lesson: text colours are only `--ink`, `--ink-soft`, `--link`, `--code-ink`, `--paper` (skip link) and `--stamp`; `--rule-strong` is borders and the menu hover underline only. Accent sits only on links, pulse, reading line and warning/caution edges (`grep var(--accent)`).
- Motion: not affected.
- Interaction and focus: ok. Tabbed every stop on both articles (drafts build) in both themes: 30 and 29 stops, each a 2 px outline, rgb(43,22,24) light / rgb(242,237,235) dark. Heading anchor after focus: #6B5557 6.34 / #C2B6B3 8.63 with ring (`settled-dark-anchor.png`); Copy button #FDFFFF on the slab with its ring (`settled-light-copy.png`). Pager and "Back to all writing" rings clear in both themes. Theme button: at 390 with system dark, first visit light, accessible name "Theme: light", one click gives dark + "Theme: dark", the next page stays dark.
- Responsive: ok. No sideways scroll (tool), home 390 dark clean, header unchanged.
- Accessibility: #1 (placeholder is outside axe's reach). Scan 0 failures in both themes, now truly dark in the dark run (shots show the #1F1B1C page).
- Copy: ok. Label "Theme: light" / "Theme: dark" names what is showing; docs speak in Nows's voice with no new facts.

## Contrast table (WCAG formula, from the hex values; re-computed)
| Pair | Light | Dark |
|---|---|---|
| Text on page / top slip | 15.78 / 17.06 | 14.68 / 12.91 |
| Text on peach / rose / highlight | 13.84 / 13.76 / 14.98 | 10.78 / 11.44 / 12.53 |
| Muted on page / top slip (heading anchor #) | 6.34 / 6.86 | 8.63 / 7.59 |
| Muted on peach / rose / highlight | 5.56 / 5.53 / 6.02 | 6.34 / 6.73 / 7.37 |
| Link on page / top slip | 6.52 / 7.05 | 6.67 / 5.87 |
| Link on peach / rose / highlight | 5.72 / 5.69 / 6.19 | 4.90 / 5.20 / 5.69 |
| Link underline against page (3:1) | 3.36 | 3.37 |
| Stamp on rose (large bold) / on page | 5.69 / 6.52 | 3.74 / 4.81 |
| Now-pulse, reading line, warning edge on rose | 6.52, edge 5.69 | 6.67, edge 5.20 |
| Focus ring, skip link | 15.78 | 14.68 |
| Code text on slab | 19.19 | 19.19 |
| **Search placeholder (`--ink-soft` at 0.8 opacity) on page** | **4.00 (fails; was 9.02)** | 5.99 |
| Never-as-text | coral 2.36, #E8546D 3.28 on #FAF5F3 | crimson 2.42 on #1F1B1C |

Separation only (not a WCAG rule): light copies 1.14 / 1.15 off the page, highlighter 1.05; the row hover is still visible in `rowhover-light.png`.

## Acceptance checks
3. Pass: no `prefers-color-scheme` under site/sass, js, templates.
5. Pass: `var(--stamp)` only `_home.scss:151`; `var(--accent)` at `_home.scss:43`, `_prose.scss:381, 387, 487`.
8. Pass: re-run at 390 with system dark (first light/null/"Theme: light" count 1; click dark; next page dark).
9. Pass: my run, 0 problems, 0 axe failures, dark shots are dark.
10. Pass: home 1440 light and dark as described (`home-1440-*.png`, `hero-*.png`).
11. Pass: crumb, prose links, pager title, "Back to all writing", footer links crimson / coral with underline (writing-reference and this-website shots, focus crops); wordmark and menu in ink.
12. Pass: alerts on peach and rose with accent edges, readable in both themes; slab #280000 with its edge in dark (`articles-writing-reference-1440-*.png`).
13. Pass: label and accessible name as above.
14. Pass: see Interaction and focus.
15. Pass: og.png opened: #FAF5F3 page, ink "Nows", three slips, crimson stamp.
16. Not re-run (dashboard/editor are tools; builder's computed values and the CSS diff match the plan).
17. Pass: skill and doc name both palettes, the rules, `--accent` vs `--stamp`, theme behaviour, tokens and contrast; no "maroon" or system-driven wording left.

## Findings
- #1 medium: In light, the search box's placeholder "Search articles and work" is 4.00:1 against the dialog's #FAF5F3, below the 4.5:1 for text. The rule (`site/sass/_chrome.scss:224-227`, `color: var(--ink-soft); opacity: 0.8`) was fine with the old #570000 (9.02:1) and fails only because muted text is now lighter. Visible in `crops/search-light.png`. Fix: drop the `opacity: 0.8` (6.34 light, 8.63 dark), or raise it to 0.9 (5.02 light). Add the placeholder row to the contrast tables in the skill and doc.
