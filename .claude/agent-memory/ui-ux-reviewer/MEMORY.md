# UI/UX reviewer memory

Patterns worth checking and decisions already made. Keep under 150 lines; merge and prune.

## Deliberate, not defects
- The "Reconciled" stamp overlaps the CRM slip's labels: a real stamp lands on the paper. The figure's `aria-label` carries the full meaning.
- List rows draw their focus ring on the row (`.index-item:has(.index-link:focus-visible)`), not on the link.
- Since WO-0004 (soft palette): light is the default whatever the system; dark only via the Theme button (saved `theme`). Muted text is its own colour in both themes, so links differ from it by colour and underline.
- The dark stamp #E8546D on the plum rose copy is 3.74:1: passes only as large bold text, reads softer than light. Accepted.
- Pager: only `.pager-title` is underlined; the Older/Newer hint stays plain (WO-0002 #3). In dark, hint and title are both coral, so the underline is the only difference.
- Search: the first Escape clears a `type=search` input, the second closes the dialog and restores focus. Native, fine.
- Header phone layout starts at 43rem (not 40rem) since WO-0003's flow mark; 641-688 px is the phone header by Nows's choice. Footer query stays 40rem.
- The flow mark's ink sits ~1.5 px low against the caps (viewBox has extra top space). Read as natural at 1x; nudge only if Nows asks.

## Worth checking every time
- Long tag lists and stack lists at 390 px.
- Text on the peach and rose copies in dark (coral link margins 4.90 / 5.20 since WO-0004).
- After any palette change, every `opacity` on a text rule mixes the token with the page: compute it, axe never sees placeholders. WO-0004 #1 failed the search `::placeholder` at 0.8 (4.00:1); it is now opacity 1 (also beats Firefox's 0.54 default). Remaining text fade: `.giallo-ln` 0.6 on the code slab.
- Read `::placeholder` with `getComputedStyle(input, '::placeholder')` after pressing `/`; the input background is transparent, so the dialog's colour is the backdrop.
- Article footers both with and without `--drafts`: only one article is published, so the pager shows only with drafts.
- Things at opacity 0 until hover or focus (`.heading-anchor`, `.code-copy`): axe skips them. Tab onto them and compute contrast by hand. `--rule-strong` as text colour fails in both themes.
- Deks and standfirsts for single-word last lines at 1440 after any font or size change (`text-wrap: pretty` is on them since WO-0002).

## How to check
- Header changes: crop `.site-header` at 320/390/641/688/689/705/1440 with an inline Playwright script against `zola serve` (:1111) when it is running; full-page shots are too tall to judge a header. Measure inline SVG ink with `getBBox()` + `getScreenCTM()`.
- Favicons: render the SVG as `<img>` at 16/32 px on light and dark grey strips; headless shows no real tab.
- [Footer focus check](footer-focus-check.md): `--focus` stops at 10 tab stops; inline Playwright for footers, and how to crop screenshots.
- The :1111 dev server runs without drafts: build `zola --root site build --drafts --base-url http://127.0.0.1:4711 --output-dir <ABSOLUTE path under target/> --force` (a relative output dir breaks `cachebust`) and serve it with node `http`.
- Opacity-0 elements report `opacity: 0` right after Tab (fade transition): wait ~700 ms before reading styles or shooting.
- On a rework round, scope re-reading with `find <dirs> -newer <ref>`, where `<ref>` is touched to the last design report time (`touch -d "<UTC>Z"`), then re-check the earlier findings with computed styles and crops.
