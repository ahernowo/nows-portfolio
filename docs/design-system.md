# Design system

## The idea

My work is moving a donation from a form to a report: the donation form records it, the payment gateway settles it, the CRM remembers the donor, and reconciliation checks that all three agree. The site's look comes from the object that did that job before software: a **carbon-copy receipt book**. An off-white original, a peach copy and a rose copy, written once in maroon carbon, on deep-red ledger lines, stamped crimson when it's done.

So the home page opens on three slips, one donation recorded by three systems, landing one after another, and a "Reconciled" stamp coming down once they match. That's the one big moment. Everything else is quiet paper and ink, so the stack stays memorable.

## The pieces

- **Colour**: five colours, set out below. Off-white paper, maroon ink instead of black, crimson for links and the stamp. Dark mode is the maroon carbon sheet itself, with off-white ink and coral.
- **Type**: Inter for everything, headings and reading alike. Headings are large and tightly spaced; text has open line spacing. Moving from browsing to reading is a change of size and line height.
- **Layout**: lists of work and writing are ledger rows (date, entry, detail) ruled in deep red. Pointing at a row draws a highlighter stroke across it.
- **Motion**: the slips and stamp on load; the highlighter, a page-title morph between a list and the page you open, and a reading-progress line on articles, all in answer to what you do. With reduced motion turned on in your system, all of it stops.

## Colour

Five colours, and only these, plus tints of them for lines and surfaces:

| Colour | Hex | Light theme | Dark theme |
|---|---|---|---|
| Off-white | `#FDFFFF` | The paper | Text, focus rings |
| Crimson | `#B10F2E` | Links in running text, the "Reconciled" stamp, the now-pulse, the reading line, warning alerts | Not used as text |
| Deep red | `#570000` | Secondary text: dates, descriptions, captions, meta | Raised surfaces: the top slip |
| Near-black maroon | `#280000` | Text, focus rings, the code slab | The paper, the code slab |
| Coral | `#DE7C5A` | Not used as text; tints only | Secondary text, links in running text, the stamp, the now-pulse, the reading line |

Two rules hold in both themes:

- Coral `#DE7C5A` is never text on a light background.
- Crimson `#B10F2E` is never text on a dark background.

The accent goes to links in running text only: sentences in articles and pages, the footer, the "Writing" crumb, the older and newer links, "Back to all writing", "All work" and "All articles". The menu, the wordmark, the titles in the work and writing lists, search results and topic chips stay in the text colour. Headings never take the accent.

Secondary text (dates, descriptions, captions, meta) is deep red `#570000` in light and coral `#DE7C5A` in dark. In dark, coral is both the secondary text and the link colour, so links stand apart by their underline.

The tints are sRGB mixes of the five colours:

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#FDFFFF` | `#280000` | Page background |
| `--slip-original` | `#FFFFFF` | `#570000` | The top slip, raised surfaces |
| `--copy-peach` | `#F5DED6` (coral 25% on off-white) | `#4C1912` (coral 20% on maroon) | Second copy, selection, search marks, inline code, note alerts |
| `--copy-rose` | `#F2DBE0` (crimson 15%) | `#5F0612` (crimson 40% on maroon) | Third copy, warning alerts |
| `--highlight` | `#F7E5DE` (coral 20%) | `#43130E` (coral 15% on maroon) | Row highlighter, hovers |
| `--ink` | `#280000` | `#FDFFFF` | Text, focus rings |
| `--ink-soft` | `#570000` | `#DE7C5A` | Secondary text, heading anchors (#) |
| `--link` | `#B10F2E` | `#DE7C5A` | Links in running text |
| `--link-line` | `#CF6F82` (crimson 60%) | `#954A36` (coral 60% on maroon) | The underline of those links |
| `--rule` | `#E4D9D9` (deep red 15%) | `#482626` (off-white 15% on maroon) | Ledger lines |
| `--rule-strong` | `#BB9999` (deep red 40%) | `#684C4C` (off-white 30%) | Borders |
| `--stamp` | `#B10F2E` | `#DE7C5A` | Stamp, now-pulse, reading line, warning edge |
| `--code-bg`, `--code-ink` | `#280000`, `#FDFFFF` | the same | The code slab |
| `--code-edge` | none | `#684C4C` | A 1 px edge so the slab shows on the maroon page |

### Contrast

Every pair below meets WCAG 2.2 AA: 4.5:1 for text, 3:1 for underlines and other non-text parts. Computed with the WCAG formula from the hex values above.

| Pair | Light | Dark |
|---|---|---|
| Text on page | 19.19 | 19.19 |
| Text on the top slip | 19.26 | 14.89 |
| Text on peach / rose / highlight | 14.95 / 14.66 / 15.79 | 14.38 / 13.75 / 15.64 |
| Secondary text on page | 14.89 | 6.55 |
| Secondary text on the top slip | 14.94 | 5.08 |
| Heading anchor (#) on page | 14.89 | 6.55 |
| Secondary text on peach / rose / highlight | 11.60 / 11.37 / 12.25 | 4.90 / 4.69 / 5.33 |
| Link on page | 7.03 | 6.55 |
| Link on peach / rose / highlight | 5.48 / 5.37 / 5.78 | 4.90 / 4.69 / 5.33 |
| Link underline against page | 3.35 | 3.04 |
| Stamp on the rose copy | 5.37 | 4.69 before the screen blend, which only lightens it |
| Focus ring against page | 19.19 | 19.19 |
| Code text on the slab | 19.19 | 19.19 |

The dark coral margins are thin: 4.69 on the rose copy and 4.90 on peach. Any change to those tints needs a new check.

Code highlighting keeps its `tokyo-night` colours. On the maroon slab, 28 of its 34 token colours reach 4.70 to 16.37; six stay below 4.5:1 (the `z-4`, `z-5`, `z-6`, `z-9`, `z-16` and `z-23` classes, 2.66 to 3.90). A code theme that clears 4.5:1 on `#280000` is a separate change.

## Type

Inter, one family for everything. Weights come from its variable weight axis, and its optical-size axis adjusts letter shapes for small and large text on its own.

| Element | Size | Weight | Letter-spacing | Line height |
|---|---|---|---|---|
| Home headline | `--step-5`, 44 to 84 px | 700 | -0.03em | 1.04 |
| Page titles | `--step-4`, 40 to 60 px | 700 | -0.025em | 1.08 |
| Article and case-study titles | 41.6 to 72 px | 700 | -0.028em | 1.06 |
| Section titles | 42 px | 650 | -0.022em | 1.1 |
| List titles, article `h2` | 26 px | 600 / 650 | -0.017em | 1.2 / 1.25 |
| Article `h3` | 20 px | 600 | -0.012em | 1.35 |
| Year headings | 21 px | 650 | -0.012em | inherited |
| Wordmark | 24 px | 750 | -0.025em | 1 |
| Body | 16 px | 400 | -0.011em | 1.6 |
| Article text | 19 px (18 px on phones) | 400 | -0.014em | 1.7 |
| Standfirst under a title | 22 px, upright | 400 | -0.017em | 1.5 |
| Home and page introductions | 21 px | 400 | -0.017em | 1.5 |
| Code | unchanged | | normal | unchanged |

Space on the page comes from this type: bigger headings, tighter letter-spacing and more line height. Margins, padding and the grid stay as they were.

## Where it lives

- Tokens (colours, type scale, spacing, easing): `site/sass/_tokens.scss`.
- The receipt stack: `site/templates/partials/receipts.html` and `site/sass/_home.scss`.
- The full rules the agents follow, including the checklist of generic patterns to keep out: `.claude/skills/design-system/SKILL.md`. That file is the single source; change the rules there.

## Fonts

Inter is self-hosted from `site/static/fonts/` under the SIL Open Font License (`LICENSE-Inter.txt` alongside). The files come from Fontsource, `@fontsource-variable/inter` version 5.3.0: upright and italic, each split into Latin and Latin Extended, with weights 100 to 900 and optical sizes 14 to 32. Self-hosting keeps the site's Content Security Policy strict and avoids sending visitors' addresses to a font service.

## Link previews

`site/static/images/og.png` is the card shown when the site is shared. `scripts/make-og-image.py` regenerates it; see the comment at the top of the script.
