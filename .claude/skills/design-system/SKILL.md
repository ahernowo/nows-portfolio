---
name: design-system
description: The site's visual identity, the carbon receipt book, with its tokens, type, layout, motion and copy rules. Use before any change to how the site or the admin pages look, move or read.
---

# Design system: the carbon receipt book

Nows builds the systems behind fundraising: donation forms, payment gateways, CRM records and the reconciliation between them. The site borrows its look from the object at the centre of that work, a **carbon-copy receipt book**: an off-white original, peach and rose copies, maroon carbon ink, deep-red ledger lines, and a crimson "Reconciled" stamp. Dark mode is the maroon carbon sheet itself, with off-white ink and coral.

Every visual choice should read as part of that book. When a choice could appear on any portfolio, find the receipt-book version of it, or leave it out.

## Tokens

All colours, type and spacing come from custom properties in `site/sass/_tokens.scss`. Use the token, never a raw value.

The palette is five colours, Nows's choice; everything else is an sRGB tint of them (mix noted in brackets):

| Colour | Hex | Light theme | Dark theme |
|---|---|---|---|
| Off-white | `#FDFFFF` | Paper | Text, focus rings |
| Crimson | `#B10F2E` | Running-text links, stamp, now-pulse, reading line, warning edge | Never as text |
| Deep red | `#570000` | Secondary text | Raised surfaces (top slip) |
| Near-black maroon | `#280000` | Text, focus rings, code slab | Paper, code slab |
| Coral | `#DE7C5A` | Never as text; tints only | Secondary text, running-text links, stamp, now-pulse, reading line |

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#FDFFFF` | `#280000` | Page background |
| `--slip-original` | `#FFFFFF` | `#570000` | The top slip, raised surfaces |
| `--copy-peach` | `#F5DED6` (coral 25%) | `#4C1912` (coral 20% on maroon) | Second copy, selection, search marks, inline code, note alerts |
| `--copy-rose` | `#F2DBE0` (crimson 15%) | `#5F0612` (crimson 40% on maroon) | Third copy, warning alerts |
| `--highlight` | `#F7E5DE` (coral 20%) | `#43130E` (coral 15% on maroon) | Row highlighter, hovers |
| `--ink` | `#280000` | `#FDFFFF` | Text and focus rings: maroon, never black |
| `--ink-soft` | `#570000` | `#DE7C5A` | Secondary text: dates, descriptions, captions, meta, heading anchors (#) |
| `--link` | `#B10F2E` | `#DE7C5A` | Links in running text |
| `--link-line` | `#CF6F82` (crimson 60%) | `#954A36` (coral 60% on maroon) | Their underline, 3:1 against the page |
| `--rule`, `--rule-strong` | `#E4D9D9`, `#BB9999` (deep red 15%, 40%) | `#482626`, `#684C4C` (off-white 15%, 30%) | Ledger lines, borders |
| `--stamp` | `#B10F2E` | `#DE7C5A` | Stamp, now-pulse, reading line, warning-alert edge |
| `--code-bg`, `--code-ink` | `#280000`, `#FDFFFF` | the same | Code slabs, the same in both themes |
| `--code-edge` | `transparent` | `#684C4C` | 1 px inset edge so the slab shows on the dark page |
| `--slip-shadow`, `--dialog-shadow`, `--backdrop` | maroon-tinted | black-tinted | Slips, the search dialog and its backdrop |

Themes: light by default, dark from `prefers-color-scheme`, and `data-theme="light|dark"` on `<html>` when the visitor picks one. A new token gets a value in both the `:root` block and the `dark-tokens` mixin.

Two rules with no exceptions:

- `#DE7C5A` coral is never text on a light background.
- `#B10F2E` crimson is never text on a dark background.

**The accent** (`--link` and `--stamp`) goes to links in running text (sentences in articles and pages, the footer, crumbs, the older/newer pager, "Back to all writing", "All work" / "All articles") and to the stamp, now-pulse, reading line and warning alerts. Never to headings, the menu, the wordmark, list titles, search results or topic chips; those stay `--ink`.

**Secondary text** (dates, descriptions, captions, meta) is `--ink-soft`: deep red in light, coral in dark. In dark, coral is both secondary text and link, so a link is told apart by its underline; keep `--link-line` on every running-text link. The pager underlines its title (`.pager-title`) and leaves the Older/Newer hint plain, so one link reads as one.

Focus rings use `--ink` (on the code slab, `--code-ink`).

Contrast (WCAG formula, from the hex values; every text pair at least 4.5:1):

| Pair | Light | Dark |
|---|---|---|
| Text on page / top slip | 19.19 / 19.26 | 19.19 / 14.89 |
| Text on peach / rose / highlight | 14.95 / 14.66 / 15.79 | 14.38 / 13.75 / 15.64 |
| Secondary on page / top slip | 14.89 / 14.94 | 6.55 / 5.08 |
| Heading anchor (#) on page | 14.89 | 6.55 |
| Secondary on peach / rose / highlight | 11.60 / 11.37 / 12.25 | 4.90 / 4.69 / 5.33 |
| Link on page | 7.03 | 6.55 |
| Link on peach / rose / highlight | 5.48 / 5.37 / 5.78 | 4.90 / 4.69 / 5.33 |
| Link underline against page | 3.35 | 3.04 |
| Code text on slab | 19.19 | 19.19 |

The dark margins on the copies are thin (4.69 on rose); re-check after any tint change. Six `tokyo-night` code token classes stay below 4.5:1 on the slab (`z-4`, `z-5`, `z-6`, `z-9`, `z-16`, `z-23`); a code-theme change is its own order.

## Type

- **Inter**, variable weight (100–900) and optical size (14–32), for everything: headings, navigation, lists, prose, footer, slips. `--font-display` and `--font-text` both name it. Inter has no width axis, so `font-stretch` has no place in the Sass.
- Headings are large with tight tracking; text has open line height. The switch from browsing to reading is size and line height (prose 1.1875 rem / 1.7), not a second family. Standfirsts (`.article-dek`, `.case-dek`) are upright; italic is for `<em>` and blockquotes, which use the real Inter italic.
- **System monospace** for code only, at `letter-spacing: normal` so the body tracking never reaches it.
- Scale: `--step--1` 14 px, `--step-0` 16 px, `--step-1` 21 px, `--step-2` 26 px, `--step-3` 42 px, `--step-4` 40–60 px and `--step-5` 44–84 px fluid. Prose drops to 1.125 rem below 40 rem.
- Measure: `--measure` 38 rem for prose; descriptions 34–44 ch. `text-wrap: balance` on headings, `text-wrap: pretty` on standfirsts so no word sits alone on the last line.
- Labels are sentence case at normal tracking. Emphasis comes from weight, never from colouring one word of a headline.
- White space comes from type only: margins, padding and the grid stay as they are.

| Element | Size | Weight | Tracking | Line height |
|---|---|---|---|---|
| `.hero-title` | `--step-5` | 700 | -0.03em | 1.04 |
| `.page-title` | `--step-4` | 700 | -0.025em | 1.08 |
| `.article-title`, `.case-title` | `clamp(2.6rem, 5vw + 0.6rem, 4.5rem)` | 700 | -0.028em | 1.06 |
| `.block-title` | `--step-3` | 650 | -0.022em | 1.1 |
| `.index-title`, `.prose h2` | `--step-2` | 600 / 650 | -0.017em | 1.2 / 1.25 |
| `.prose h3` | 1.25rem | 600 | -0.012em | 1.35 |
| `.year-title` | `--step-1` | 650 | -0.012em | inherit |
| `.wordmark` | 1.5rem | 750 | -0.025em | 1 |
| `.stamp span` | fluid | 800 | -0.01em | 1 |
| `.slip-system`, `.footer-name`, `.pager-link`, `.back-link` | as set | 650 / 700 / 600 / 600 | -0.01em | inherit |
| body | 16px | 400 | -0.011em | 1.6 |
| `.prose` | 1.1875rem (1.125rem below 40rem) | 400 | -0.014em | 1.7 |
| `.article-dek`, `.case-dek` | 1.375rem, upright, `text-wrap: pretty` | 400 | -0.017em | 1.5 |
| `.hero-sub`, `.page-dek` | `--step-1` | 400 | -0.017em | 1.5 |
| tables / alerts | as set | | | 1.5 / 1.6 |
| `pre`, `code` | as set | | normal | as set |

Fonts are self-hosted woff2 in `site/static/fonts/` with `LICENSE-Inter.txt`: Fontsource `@fontsource-variable/inter` 5.3.0, the `opsz` files (upright and italic, Latin and Latin Extended). `base.html` preloads `inter-latin-opsz-normal.woff2`. A new weight or family means a new file there and a CSP-safe `@font-face`.

## Layout

- `.wrap` caps the page at `--wrap` 72 rem with `--gutter` side padding; `.wrap-text` caps reading columns at `--measure`.
- Lists of work and writing are **ledger rows** (`.index-list`): a date or year column, the title and description, and a meta column, separated by `--rule` lines. Hovering a row draws a highlighter stroke (`--highlight`) behind it.
- Surfaces are flat paper with `--radius` 4 px; slips are the only things with a shadow (`--slip-shadow`).
- The home hero is the receipt stack: one donation (DN-2041, Rp150.000) recorded by the donation form, the payment gateway and the CRM, stamped "Reconciled" when they match. The stack is the site's one memorable element; other pages stay quiet beside it.
- Structure carries meaning: a number marks a real sequence, a rule separates real rows, a label names a real field.

## Motion

One orchestrated moment, then motion only in answer to the visitor:

- **Page load (home only)**: the three slips land one after another (0.15 s apart), then the stamp thumps down at 1.15 s.
- **In answer to the visitor**: the highlighter stroke on list rows, the slip fan on hover, the title morph between a list and its page (cross-document view transitions), the reading-progress line on articles (scroll-driven).
- Easing `--ease-out`; durations 0.15–0.6 s.
- `prefers-reduced-motion: reduce` turns all of it off (`_motion.scss` holds the rule). New motion must be covered by that rule.

Each new animation answers one question: what did the visitor do, and what changed? Entrances on scroll and hover effects on every card fail that test.

## Copy

Words follow the same book: plain, specific, first person for Nows, sentence case.

- Name things by what a visitor understands: "Writing", "Work", "Now".
- Buttons say what happens: "Copy", "Search", "Publish".
- Empty states say what will be there: "The first article is being written."
- Facts about Nows come from Nows or the repo. A placeholder is marked as a placeholder.

## Generated-look checklist

Before passing a visual change, check none of these crept in, since each one makes the site read as a template:

- One word of a headline in a different colour, weight or italic.
- Labels in all caps with wide tracking above headings.
- Content chopped into identical rounded cards with the same soft grey shadow.
- Gradient washes, glows or glassmorphism used as decoration.
- Numbered markers (01 / 02 / 03) on content that is not a sequence.
- Meta joined with middle dots, or an arrow appended to every link.
- Neutral near-black (`#111`) text, or a neutral black background with one neon accent. (Maroon `#280000` with coral is the brand palette, not this pattern; the receipt stack keeps it specific.)
- Fade-and-slide-up on every section as it scrolls into view.

## Admin pages

`workers/cms-api/admin/` (the stage B editor) and `tools/factory/dashboard/` borrow the palette in a denser, tool-like layout. Each stylesheet carries its own copy of the colours at the top (the editor uses system fonts to keep the worker small; the dashboard adds lamp colours for running, waiting and stopped). When a site token changes, change the matching value there too.
