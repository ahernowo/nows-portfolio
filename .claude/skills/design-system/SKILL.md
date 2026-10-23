---
name: design-system
description: The site's visual identity, the carbon receipt book, with its tokens, type, layout, motion and copy rules. Use before any change to how the site or the admin pages look, move or read.
---

# Design system: the carbon receipt book

Nows builds the systems behind fundraising: donation forms, payment gateways, CRM records and the reconciliation between them. The site borrows its look from the object at the centre of that work, a **carbon-copy receipt book**: a white original, yellow and pink copies, indigo carbon ink, blue ledger lines, and a red "paid" stamp. Dark mode is the carbon sheet itself.

Every visual choice should read as part of that book. When a choice could appear on any portfolio, find the receipt-book version of it, or leave it out.

## Tokens

All colours, type and spacing come from custom properties in `site/sass/_tokens.scss`. Use the token, never a raw value.

| Token | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#fcfcfa` | `#141938` | Page background |
| `--slip-original` | `#ffffff` | `#1d2350` | The top slip, raised surfaces |
| `--copy-yellow` | `#fbf0b4` | `#3a3826` | Second copy; also `--highlight` in light |
| `--copy-pink` | `#f9dce2` | `#3b2640` | Third copy |
| `--ink` | `#1f2463` | `#eceef9` | Text: carbon indigo, never black |
| `--ink-soft` | `#4a4f86` | `#aeb3da` | Secondary text |
| `--rule`, `--rule-strong` | `#b9c9e6`, `#8fa6d3` | `#2e356f`, `#4a52a0` | Ledger lines, borders |
| `--stamp` | `#b4302a` | `#ff8273` | The one accent: stamp, now-pulse, focus |
| `--code-bg`, `--code-ink` | `#161b45`, `#e6e8f6` | `#0e1230`, `#e6e8f6` | Code slabs, the same in both themes |

Themes: light by default, dark from `prefers-color-scheme`, and `data-theme="light|dark"` on `<html>` when the visitor picks one. A new token gets a value in both the `:root` block and the `dark-tokens` mixin.

The stamp red is rare on purpose: one stamp, the now-pulse, focus rings, and alerts that warn. A second red thing on a page competes with the stamp.

## Type

- **Archivo**, variable width and weight (`font-stretch` 62–125%), is the voice of the interface: headings, navigation, lists, descriptions, footer, slips. Headings run narrow and heavy: hero 70% / 760, page titles 76% / 750, section titles 78% / 720, list titles 84% / 680. Interface text sits at 100% width.
- **Newsreader**, variable optical size, is the voice of reading: article and case-study prose (`.prose`, 1.1875 rem / 1.62) and the italic standfirst under a title (`.article-dek`, `.case-dek`). The switch from Archivo to Newsreader tells the visitor they have moved from browsing to reading.
- **System monospace** for code only.
- Scale: `--step--1` 14 px, `--step-0` 16 px, `--step-1` 21 px, `--step-2` 24 px, `--step-3` 36 px, `--step-4` and `--step-5` fluid display sizes. Prose drops to 1.125 rem on small screens.
- Measure: `--measure` 38 rem for prose; descriptions 34–44 ch. `text-wrap: balance` on headings.
- Labels are sentence case at normal tracking. Emphasis comes from weight and width, never from colouring one word of a headline.

Fonts are self-hosted woff2 in `site/static/fonts/` with their OFL licences. A new weight or family means a new file there and a CSP-safe `@font-face`.

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
- Near-black (`#111`) text or a near-black background with one neon accent.
- Fade-and-slide-up on every section as it scrolls into view.

## Admin pages

`workers/cms-api/admin/` (the stage B editor) and `tools/factory/dashboard/` borrow the palette in a denser, tool-like layout. Each stylesheet carries its own copy of the colours at the top (the editor uses system fonts to keep the worker small; the dashboard adds lamp colours for running, waiting and stopped). When a site token changes, change the matching value there too.
