---
name: ui-ux-review
description: How to review a change for visual design, usability, accessibility (WCAG 2.2 AA) and copy on this site, including the screenshot and accessibility-scan tool. Use when reviewing or checking what a change looks like, or before passing any visual change.
---

# UI and UX review

Review with evidence. Each verdict rests on a screenshot you looked at, a scan result, or code you read.

## Screenshots and scans

`tools/shots/shoot.mjs` builds the site into `target/shots-site` with a local base URL, serves it with the real `_headers` (so the CSP applies), takes full-page screenshots, and runs an axe accessibility scan against WCAG 2.2 AA. Everything it writes is under `target/`, which git ignores, so it leaves the build seal intact.

```bash
node tools/shots/shoot.mjs --out target/shots/WO-0007 --pages /,/articles/how-this-site-is-built/ --focus
```

| Option | Effect |
|---|---|
| `--pages /,/work/` | Paths to capture. Default: home, every top-level section, the 404 page |
| `--widths 390,1440` | Viewport widths (the default pair: a phone and a laptop) |
| `--themes light,dark` | Themes (default both). Dark is set as the visitor's saved choice, the way the Theme button does; the system setting no longer changes the site. |
| `--reduced` | Also capture with reduced motion |
| `--focus` | Tab through each page; photograph every focus stop; flag stops without a visible ring |
| `--drafts` | Include draft pages |
| `--viewport-only` | First screen only, for above-the-fold checks |
| `--scroll "#journey-story:0.05,0.5,0.97"` | After each normal shot (motion on), scroll through the element to each fraction and save a viewport shot; pages without it are skipped |
| `--root <dir>` | Build another Zola site folder, such as a test copy |

The script prints a Markdown summary: screenshot files, console and network problems (including CSP blocks and sideways scrolling), accessibility rule failures, and focus problems. Paste the relevant parts into the design report, then open the screenshots with Read and look at them.

When the command reports that Chromium or the packages are missing, the one-time setup is `cd tools/shots && npm install && npx playwright install chromium`. When it cannot run at all, review from the code and say so in the report.

**The donor journey** (`/work/donation-website/`, a draft): use `--scroll "#journey-story:0.05,0.22,0.5,0.77,1"` for the pinned story. The recording is switched off until it is re-recorded (`show_recording` in `partials/journey.html`, `ignored_static` in `zola.toml`), so the page has no video and no request for `journey.mp4`.

**Which pages**: every page whose template, styles, script or content changed. A change to `base.html`, a partial, `_tokens.scss`, `_base.scss` or `_chrome.scss` affects every page: capture the defaults. A change to `_prose.scss` needs an article and a case study; `/articles/writing-reference/` (a draft, so add `--drafts`) shows every prose element at once.

## Checklist

Give each section a verdict: ok, not affected, or finding numbers.

**Identity and layout**
- The change reads as part of the carbon receipt book (design-system skill), and passes its generated-look checklist.
- Alignment follows the existing grid: `.wrap` edges, the ledger columns of `.index-list`, the `--measure` reading column.
- Spacing uses the same rhythm as neighbouring blocks; nothing crowds the receipt stack on the home page.

**Typography**
- Inter for everything; sizes, weights, tracking and line heights from the type table.
- Line length 45–80 characters in prose; headings balanced, no orphaned single words on wide screens.
- Numbers in tables and lists use tabular figures.

**Colour and contrast**
- Only tokens; both themes defined. Body text at least 4.5:1, large text and UI parts at least 3:1, in light and dark. The scan checks most of this; check text on the peach and rose copies by eye too.
- The accent appears only on running-text links, the stamp, now-pulse, reading line and warning alerts.

**Motion**
- New motion answers a visitor's action or belongs to the home page's load sequence or the case-study journey.
- Motion that starts by itself and lasts over 5 s has a visible pause (WCAG 2.2.2): on the journey, the hero's Pause animation button must stop the beat, the breathing and the blinking.
- With `--reduced`, the page is complete and still: nothing hidden that waits for an animation.

**Interaction and focus**
- Every link and button shows a focus ring (`--focus`), in reading order, and the skip link works.
- Targets at least 24 by 24 px, or spaced so they don't overlap.
- Hover-only information is also available on focus and on touch.
- The search dialog opens with `/`, closes with Escape, and returns focus to where it was.

**Responsive**
- At 390 px: nothing scrolls sideways, nothing overlaps, tap targets comfortable. At 1440 px: no lines stretched past the measure.
- Images have `width`/`height` or an aspect ratio, so nothing jumps as they load.

**Accessibility**
- The scan reports no failures, or each one has a finding.
- One `h1` per page; headings in order; landmarks (`header`, `nav`, `main`, `footer`) intact.
- Images have alt text that says what the image shows; decorative ones have `alt=""` or `aria-hidden`.
- Links say where they go out of context ("Read the reconciliation case study", not "here").

**Copy**
- Plain, specific, sentence case, first person for Nows.
- Buttons name their action; empty and error states tell the visitor what to do next.
- Facts about Nows trace to the request or the repo.

## Turning problems into findings

Write the finding as what a visitor experiences, where, and the fix:

> medium: "On phones the tag list overflows the article footer and the page scrolls sideways (articles at 390 px). Let `.tag-line` wrap."

Severity: a problem a visitor or Nows would hit, or any WCAG 2.2 AA failure, is at least **medium**. Polish that most visitors would never notice is **low**.
