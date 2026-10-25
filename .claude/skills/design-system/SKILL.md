---
name: design-system
description: The site's visual identity, the carbon receipt book, with its tokens, type, layout, motion and copy rules. Use before any change to how the site or the admin pages look, move or read.
---

# Design system: the carbon receipt book

Nows builds the systems behind fundraising: donation forms, payment gateways, CRM records and the reconciliation between them. The site borrows its look from the object at the centre of that work, a **carbon-copy receipt book**, in a soft palette: a warm off-white page, a white original, peach and rose copies, dark warm carbon ink, fine ledger lines, and a crimson "Reconciled" stamp. Light is what every visitor sees; dark is the carbon sheet (a warm charcoal page, off-white ink, coral accent and a pink-red stamp), shown only when the visitor picks it.

Every visual choice should read as part of that book. When a choice could appear on any portfolio, find the receipt-book version of it, or leave it out.

## Tokens

All colours, type and spacing come from custom properties in `site/sass/_tokens.scss`. Use the token, never a raw value.

The palette is Nows's choice, one set per theme; everything else is an sRGB tint of it (mix noted in brackets):

| Role | Light | Dark |
|---|---|---|
| Page | `#FAF5F3` | `#1F1B1C` |
| Surfaces, the top slip | `#FFFFFF` | `#2C2527` |
| Text, focus rings | `#2B1618` | `#F2EDEB` |
| Muted text | `#6B5557` | `#C2B6B3` |
| Accent: running-text links, now-pulse, reading line, warning edge | `#B10F2E` crimson | `#E8896A` coral |
| "Reconciled" stamp | `#B10F2E` | `#E8546D` |
| Code slab (kept from the old palette) | `#280000` with `#FDFFFF` text | the same |

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#FAF5F3` | `#1F1B1C` | Page background |
| `--slip-original` | `#FFFFFF` | `#2C2527` | The top slip, raised surfaces |
| `--copy-peach` | `#F9E3DB` (coral 24% on white) | `#432F2A` (coral 18% on `#1F1B1C`) | Second copy, selection, search marks, inline code, note alerts |
| `--copy-rose` | `#F6E2E6` (crimson 12% on white) | `#47262C` (`#E8546D` 20% on `#1F1B1C`) | Third copy, warning alerts |
| `--highlight` | `#FCEDE9` (coral 15% on white) | `#332624` (coral 10% on `#1F1B1C`) | Row highlighter, hovers |
| `--ink` | `#2B1618` | `#F2EDEB` | Text and focus rings: warm, never black |
| `--ink-soft` | `#6B5557` | `#C2B6B3` | Muted text: dates, descriptions, captions, meta, heading anchors (#) |
| `--link` | `#B10F2E` | `#E8896A` | Links in running text |
| `--link-line` | `#CD6679` (crimson 62% on `#FAF5F3`) | `#9C5F4C` (coral 62% on `#1F1B1C`) | Their underline, 3:1 against the page |
| `--accent` | `#B10F2E` | `#E8896A` | Now-pulse, reading line, warning and caution alert edge |
| `--rule`, `--rule-strong` | `#E1DAD9`, `#BCB2B1` (text 12%, 30% on the page) | `#3D3839`, `#5A5656` (text 14%, 28% on the page) | Ledger lines, borders |
| `--stamp` | `#B10F2E` | `#E8546D` | The "Reconciled" stamp only |
| `--code-bg`, `--code-ink` | `#280000`, `#FDFFFF` | the same | Code slabs, the same in both themes |
| `--code-edge` | `transparent` | `#5A5656` | 1 px inset edge so the slab shows on the dark page |
| `--slip-shadow`, `--dialog-shadow`, `--backdrop` | ink-tinted (`rgb(43 22 24 / ...)`) | black-tinted | Slips, the search dialog and its backdrop |
| `--ok` | `#186A41` | `#7FD0A5` | Done, saved, confirmed: the journey's record status text, done dots and meters |
| `--ok-tint` | `#E3F2EA` | `#1E3A2D` | The journey's "Saving" dot |

The **donor-journey illustration** (`journey.scss`) has its own group, `--jn-*`: one token per colour in the picture (skin, hair, mouth, shirt, collar, phone bezel, screen colours, keyboard, lock-screen wallpaper, the phone's green success ring). The phone always shows a light web page, so its screen colours are the same in both themes, like the code slab; only `--jn-shirt` and `--jn-collar` change in dark. `--jn-screen-muted` is `#6B5557` (the mock-up's `#7A6A6C` was 4.45:1 on the URL bar). `journey.scss` holds no raw colours: tokens or `color-mix()` of tokens only.

Themes: light for every visitor, whatever their system setting; the site has no `prefers-color-scheme` rule. Dark appears only when the visitor picks it with the Theme button: `site.js` saves `theme` in localStorage and sets `data-theme="dark"` on `<html>`, and `head.js` re-applies a saved choice before first paint. The button cycles light and dark and its label names the theme showing ("Theme: light" / "Theme: dark"). A new token gets a value in both the `:root` block and the `dark-tokens` mixin. The meta `theme-color` is the light page, `#FAF5F3`.

Two rules with no exceptions:

- `#E8896A` coral and `#E8546D` (the dark stamp) are never text on a light background.
- `#B10F2E` crimson is never text on a dark background.

**The accent** (`--link` and `--accent`) goes to links in running text (sentences in articles and pages, the footer, crumbs, the older/newer pager, "Back to all writing", "All work" / "All articles") and to the now-pulse, reading line, and warning and caution alert edges. Never to headings, the menu, the wordmark, list titles, search results or topic chips; those stay `--ink`. The stamp has its own token, `--stamp`: crimson in light like the accent, `#E8546D` in dark.

**Muted text** (dates, descriptions, captions, meta) is `--ink-soft`: `#6B5557` in light, `#C2B6B3` in dark, always at full opacity (the light value has no room to fade: at 0.8 it falls to 4.00:1). Keep `--link-line` on every running-text link so a link reads as a link without relying on colour. The pager underlines its title (`.pager-title`) and leaves the Older/Newer hint plain, so one link reads as one.

Focus rings use `--ink` (on the code slab, `--code-ink`).

Contrast (WCAG formula, from the hex values; every text pair at least 4.5:1, large text and non-text parts 3:1):

| Pair | Light | Dark |
|---|---|---|
| Text on page / top slip | 15.78 / 17.06 | 14.68 / 12.91 |
| Text on peach / rose / highlight | 13.84 / 13.76 / 14.98 | 10.78 / 11.44 / 12.53 |
| Muted on page / top slip (also heading anchor #) | 6.34 / 6.86 | 8.63 / 7.59 |
| Search placeholder (`--ink-soft` at full opacity on the dialog's `--paper`) | 6.34 | 8.63 |
| Muted on peach / rose / highlight | 5.56 / 5.53 / 6.02 | 6.34 / 6.73 / 7.37 |
| Link on page / top slip | 6.52 / 7.05 | 6.67 / 5.87 |
| Link on peach / rose / highlight | 5.72 / 5.69 / 6.19 | 4.90 / 5.20 / 5.69 |
| Link underline against page | 3.36 | 3.37 |
| Stamp on rose (large bold) / on page | 5.69 / 6.52 | 3.74 (before the screen blend, which only lightens it) / 4.81 |
| Now-pulse, reading line against page | 6.52 | 6.67 |
| Focus ring, skip link | 15.78 | 14.68 |
| Code text on slab | 19.19 | 19.19 |
| Never-as-text pairs | coral 2.36, `#E8546D` 3.28 on `#FAF5F3` | crimson 2.42 on `#1F1B1C` |
| Journey: `--ok` status text on the panel (`--slip-original`) | 6.61 | 8.19 |
| Journey: tick (`--slip-original`) on the `--ok` dot | 6.61 | 8.19 (white would be 1.83, hence the token) |
| Journey: `--ok` on `--ok-tint` (saving dot) | 5.71 | 6.75 |
| Journey: stamp on the panel (bold, at least 1.2rem: large) | 7.05 | 4.23 |
| Journey: video and pause buttons (`--paper` on `--ink`; `--ink` on `--slip-original`) | 15.78; 17.06 | 14.68; 12.91 |
| Journey phone: muted `#6B5557` on screen / URL bar / receipt; accent on chip tint; white on accent | 6.86 / 5.97 / 6.19; 6.12; 7.05 | the same (the phone screen is light in both themes) |

The thin margins are dark links on peach (4.90) and the dark stamp on rose (3.74, large text only); re-check after any tint change. In light the copies sit only 1.14-1.15:1 off the page, so the slips rely on their shadow and edge. Six `tokyo-night` code token classes stay below 4.5:1 on the slab (`z-4`, `z-5`, `z-6`, `z-9`, `z-16`, `z-23`); a code-theme change is its own order.

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

Two orchestrated moments, then motion only in answer to the visitor:

- **Page load (home only)**: the three slips land one after another (0.15 s apart), then the stamp thumps down at 1.15 s.
- **The case-study donor journey** (only on case studies that switch it on with `[[extra.journey.steps]]`; `journey.scss`, `js/journey.js`): the title rises once and the hero scene fades up (under 1.1 s, skipped after the title morph); the hero's first beat (the phone lifts, a thumb picks a gift: 0.7 s wait, 5.2 s beat, then rest); the donor's endless breathing and blinking; and the pinned five-step story drawn from scroll position alone. The silent twelve-second recording (`images/journey.mp4`) plays while half visible and loops. Self-starting motion lasts longer than 5 s, so the hero carries a **Pause animation** button (WCAG 2.2.2) that holds the beat and every CSS animation in the journey; the video has its own Play/Pause button. Without JavaScript the scenes show their finished state and nothing loops. Under reduced motion all of it stops: no pinning, no autoplay, the finished picture, all five steps listed, and no pause button (nothing moves).
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
- Neutral near-black (`#111`) text, or a neutral black background with one neon accent. (Warm carbon `#1F1B1C` with coral `#E8896A` is the brand's dark theme, not this pattern; the receipt stack keeps it specific.)
- Fade-and-slide-up on every section as it scrolls into view.

## Admin pages

`workers/cms-api/admin/` (the stage B editor) and `tools/factory/dashboard/` borrow the palette in a denser, tool-like layout. Each stylesheet carries its own copy of the colours at the top (the editor uses system fonts to keep the worker small; the dashboard adds lamp colours for running, waiting and stopped). When a site token changes, change the matching value there too. Their theme behaviour is their own: the dashboard is dark by default, the editor follows the system setting; neither has a Theme button.
