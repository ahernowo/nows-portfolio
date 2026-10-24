# UI/UX reviewer memory

Patterns worth checking and decisions already made. Keep under 150 lines; merge and prune.

## Deliberate, not defects
- The "Reconciled" stamp overlaps the CRM slip's labels: a real stamp lands on the paper. The figure's `aria-label` carries the full meaning.
- List rows draw their focus ring on the row (`.index-item:has(.index-link:focus-visible)`), not on the link.
- Light secondary text #570000 is only 1.29:1 from body #280000: hierarchy comes from size. Planned in WO-0002; mention it, don't log it.
- In dark, coral is both secondary text and link colour; links are told apart only by `--link-line` underline. Check the underline is there.
- Pager: only `.pager-title` is underlined; the Older/Newer hint stays plain (WO-0002 #3). In dark, hint and title are both coral, so the underline is the only difference.
- Search: the first Escape clears a `type=search` input, the second closes the dialog and restores focus. Native, fine.

## Worth checking every time
- Long tag lists and stack lists at 390 px.
- Text on the peach and rose copies in dark (coral margins are 4.90 / 4.69).
- Article footers both with and without `--drafts`: only one article is published, so the pager shows only with drafts.
- Things at opacity 0 until hover or focus (`.heading-anchor`, `.code-copy`): axe skips them. Tab onto them and compute contrast by hand. `--rule-strong` as text colour fails in both themes.
- Deks and standfirsts for single-word last lines at 1440 after any font or size change (`text-wrap: pretty` is on them since WO-0002).

## How to check
- [Footer focus check](footer-focus-check.md): `--focus` stops at 10 tab stops; inline Playwright for footers, and how to crop screenshots.
- On a rework round, scope re-reading with `find <dirs> -newer <ref>`, where `<ref>` is touched to the last design report time (`touch -d "<UTC>Z"`), then re-check the earlier findings with computed styles and crops.
