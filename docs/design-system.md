# Design system

## The idea

My work is moving a donation from a form to a report: the donation form records it, the payment gateway settles it, the CRM remembers the donor, and reconciliation checks that all three agree. The site's look comes from the object that did that job before software: a **carbon-copy receipt book**. One original, a yellow copy and a pink copy, written once in indigo carbon, on blue ledger lines, stamped red when it's done.

So the home page opens on three slips, one donation recorded by three systems, landing one after another, and a "Reconciled" stamp coming down once they match. That's the one big moment. Everything else is quiet paper and ink, so the stack stays memorable.

## The pieces

- **Colour**: paper white, two copy colours (yellow `#fbf0b4`, pink `#f9dce2`), carbon indigo ink (`#1f2463`) instead of black, ledger blue for lines, and stamp red (`#b4302a`) used rarely. Dark mode is the carbon sheet itself: deep indigo paper with pale ink.
- **Type**: Archivo, a variable font whose width I narrow for headings, for everything you navigate; Newsreader, a serif with optical sizes, for everything you read. Moving from one to the other tells you you've gone from browsing to reading.
- **Layout**: lists of work and writing are ledger rows (date, entry, detail) ruled in blue. Pointing at a row draws a highlighter stroke across it.
- **Motion**: the slips and stamp on load; the highlighter, a page-title morph between a list and the page you open, and a reading-progress line on articles, all in answer to what you do. With reduced motion turned on in your system, all of it stops.

## Where it lives

- Tokens (colours, type scale, spacing, easing): `site/sass/_tokens.scss`.
- The receipt stack: `site/templates/partials/receipts.html` and `site/sass/_home.scss`.
- The full rules the agents follow, including the checklist of generic patterns to keep out: `.claude/skills/design-system/SKILL.md`. That file is the single source; change the rules there.

## Fonts

Archivo and Newsreader are self-hosted from `site/static/fonts/` under the SIL Open Font License (licence files alongside). Self-hosting keeps the site's Content Security Policy strict and avoids sending visitors' addresses to a font service.

## Link previews

`site/static/images/og.png` is the card shown when the site is shared. `scripts/make-og-image.py` regenerates it; see the comment at the top of the script.
