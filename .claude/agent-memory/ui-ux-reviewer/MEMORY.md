# UI/UX reviewer memory

Patterns worth checking and decisions already made. Keep under 150 lines; merge and prune.

## Deliberate, not defects
- The "Reconciled" stamp overlaps the CRM slip's labels: a real stamp lands on the paper. The figure's `aria-label` carries the full meaning.
- List rows draw their focus ring on the row (`.index-item:has(.index-link:focus-visible)`), not on the link.

## Worth checking every time
- Long tag lists and stack lists at 390 px.
- Text on the yellow and pink copies in dark mode.
- Article footers both with and without `--drafts`: only one article is published, so the pager shows only with drafts.

## How to check
- [Footer focus check](footer-focus-check.md): `--focus` stops at 10 tab stops, so footer links need an inline Playwright check.
