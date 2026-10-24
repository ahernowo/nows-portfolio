# Design system

## The idea

My work is moving a donation from a form to a report: the donation form records it, the payment gateway settles it, the CRM remembers the donor, and reconciliation checks that all three agree. The site's look comes from the object that did that job before software: a **carbon-copy receipt book**. A white original, a peach copy and a rose copy, written once in dark carbon ink, on fine ledger lines, stamped crimson when it's done.

So the home page opens on three slips, one donation recorded by three systems, landing one after another, and a "Reconciled" stamp coming down once they match. That's the one big moment. Everything else is quiet paper and ink, so the stack stays memorable.

## The pieces

- **Colour**: a soft palette, set out below. A warm off-white page, white slips, a dark warm ink instead of black, crimson for links and the stamp. Light is what everyone sees first; dark is the carbon sheet, there when you pick it.
- **Type**: Inter for everything, headings and reading alike. Headings are large and tightly spaced; text has open line spacing. Moving from browsing to reading is a change of size and line height.
- **Layout**: lists of work and writing are ledger rows (date, entry, detail) ruled in fine warm-grey lines. Pointing at a row draws a highlighter stroke across it.
- **Motion**: the slips and stamp on load; the highlighter, a page-title morph between a list and the page you open, and a reading-progress line on articles, all in answer to what you do. A second moment lives on case studies that switch on the donor journey: a donor on a phone gives one gift, and the scroll walks through it step by step. A short recording of it is switched off for now, until it is re-recorded. The journey's hero keeps moving (a slow breath, a blink), so it has a Pause animation button. With reduced motion turned on in your system, all of it stops.

## Colour

The palette is soft: a warm off-white page, white slips, dark warm ink and crimson. Each theme has its own set, plus tints of them for lines and surfaces:

| Role | Light | Dark |
|---|---|---|
| Page | `#FAF5F3` | `#1F1B1C` |
| Surfaces: the top slip | `#FFFFFF` | `#2C2527` |
| Text, focus rings | `#2B1618` | `#F2EDEB` |
| Muted text: dates, descriptions, captions, meta, heading anchors (#) | `#6B5557` | `#C2B6B3` |
| Accent: links in running text, the now-pulse, the reading line, warning alerts | Crimson `#B10F2E` | Coral `#E8896A` |
| The "Reconciled" stamp | Crimson `#B10F2E` | `#E8546D` |
| The code slab | `#280000` with `#FDFFFF` text | the same |

Two rules hold in both themes:

- Coral `#E8896A` and the dark stamp `#E8546D` are never text on a light background.
- Crimson `#B10F2E` is never text on a dark background.

The accent goes to links in running text only: sentences in articles and pages, the footer, the "Writing" crumb, the older and newer links, "Back to all writing", "All work" and "All articles". It also colours the now-pulse, the reading line and the edge of warning and caution alerts (the `--accent` token). The stamp has its own token, `--stamp`, because in dark it is `#E8546D`, not the accent. The menu, the wordmark, the titles in the work and writing lists, search results and topic chips stay in the text colour. Headings never take the accent.

Muted text (dates, descriptions, captions, meta) is `#6B5557` in light and `#C2B6B3` in dark. Links stand apart from it by colour and by their underline, which every running-text link keeps.

### Themes

Light is the theme for everyone, whatever their system is set to. Dark appears when you choose it with the Theme button in the header, and that browser remembers it (the choice is saved as `theme` in local storage, and `head.js` applies it before the page paints, so it never flashes). The button's label says which theme is showing: "Theme: light" or "Theme: dark". Without JavaScript the site stays light.

### Tokens

The tints are sRGB mixes of the colours above:

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#FAF5F3` | `#1F1B1C` | Page background |
| `--slip-original` | `#FFFFFF` | `#2C2527` | The top slip, raised surfaces |
| `--copy-peach` | `#F9E3DB` (coral 24% on white) | `#432F2A` (coral 18% on the dark page) | Second copy, selection, search marks, inline code, note alerts |
| `--copy-rose` | `#F6E2E6` (crimson 12% on white) | `#47262C` (`#E8546D` 20% on the dark page) | Third copy, warning alerts |
| `--highlight` | `#FCEDE9` (coral 15% on white) | `#332624` (coral 10% on the dark page) | Row highlighter, hovers |
| `--ink` | `#2B1618` | `#F2EDEB` | Text, focus rings |
| `--ink-soft` | `#6B5557` | `#C2B6B3` | Muted text, heading anchors (#) |
| `--link` | `#B10F2E` | `#E8896A` | Links in running text |
| `--link-line` | `#CD6679` (crimson 62% on the page) | `#9C5F4C` (coral 62% on the page) | The underline of those links |
| `--accent` | `#B10F2E` | `#E8896A` | Now-pulse, reading line, warning and caution alert edge |
| `--rule` | `#E1DAD9` (text 12% on the page) | `#3D3839` (text 14% on the page) | Ledger lines |
| `--rule-strong` | `#BCB2B1` (text 30%) | `#5A5656` (text 28%) | Borders |
| `--stamp` | `#B10F2E` | `#E8546D` | The "Reconciled" stamp only |
| `--code-bg`, `--code-ink` | `#280000`, `#FDFFFF` | the same | The code slab |
| `--code-edge` | none | `#5A5656` | A 1 px edge so the slab shows on the dark page |
| `--ok` | `#186A41` | `#7FD0A5` | Done and saved, in the donor journey's records |
| `--ok-tint` | `#E3F2EA` | `#1E3A2D` | A record that is saving |

### Contrast

Every pair below meets WCAG 2.2 AA: 4.5:1 for text, 3:1 for large text, underlines and other non-text parts. Computed with the WCAG formula from the hex values above.

| Pair | Light | Dark |
|---|---|---|
| Text on page | 15.78 | 14.68 |
| Text on the top slip | 17.06 | 12.91 |
| Text on peach / rose / highlight | 13.84 / 13.76 / 14.98 | 10.78 / 11.44 / 12.53 |
| Muted text on page (also heading anchors #) | 6.34 | 8.63 |
| Muted text on the top slip | 6.86 | 7.59 |
| Search placeholder (muted text, full opacity, on the page-coloured dialog) | 6.34 | 8.63 |
| Muted text on peach / rose / highlight | 5.56 / 5.53 / 6.02 | 6.34 / 6.73 / 7.37 |
| Link on page | 6.52 | 6.67 |
| Link on the top slip | 7.05 | 5.87 |
| Link on peach / rose / highlight | 5.72 / 5.69 / 6.19 | 4.90 / 5.20 / 5.69 |
| Link underline against page | 3.36 | 3.37 |
| Stamp on the rose copy (large bold text) | 5.69 | 3.74 before the screen blend, which only lightens it |
| Stamp on page | 6.52 | 4.81 |
| Now-pulse and reading line against page | 6.52 | 6.67 |
| Focus ring against page | 15.78 | 14.68 |
| Skip link (page colour on text colour) | 15.78 | 14.68 |
| Code text on the slab | 19.19 | 19.19 |
| Never as text (why the rules exist) | coral 2.36, `#E8546D` 3.28 on the light page | crimson 2.42 on the dark page |

The thin margins are dark links on the peach copy (4.90) and the dark stamp on the rose copy (3.74, which passes only because the stamp is large bold text). Any change to those tints needs a new check.

Code highlighting keeps its `tokyo-night` colours, and the slab keeps its `#280000` in both themes. On that slab, 28 of its 34 token colours reach 4.70 to 16.37; six stay below 4.5:1 (the `z-4`, `z-5`, `z-6`, `z-9`, `z-16` and `z-23` classes, 2.66 to 3.90). A code theme that clears 4.5:1 on `#280000` is a separate change.

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
