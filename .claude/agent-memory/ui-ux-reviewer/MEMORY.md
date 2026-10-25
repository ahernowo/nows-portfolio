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
- Donor journey (WO-0005, /work/donation-website/, draft): endless breathing/blinking and the 5.9 s hero beat are Nows's choice, covered by the hero's Pause animation button. The recording is back since WO-0007 (re-recorded, ends "All three agree"; a padlock icon remains, flagged to Nows as WO-0007 #2); the page is a plan from November 2026, not an outcome. The video button uses `data-playing` + label, no `aria-pressed`. The hero text column is vertically centred (mock-up layout).

## Worth checking every time
- Long tag lists and stack lists at 390 px.
- Case-study copy (Nows asked for honesty, WO-0006): only Nows's stated facts, no outcomes or numbers; sample people and amounts stay inside the illustration. Section intros (`/work/` "Systems I've built") can contradict a plan page.
- The journey hero's three-column `.case-form` is narrowest at 865-970 px: long words in Role overrun the field (WO-0006 #1). Measure dd text rects against the field padding there.
- Text on the peach and rose copies in dark (coral link margins 4.90 / 5.20 since WO-0004).
- After any palette change, every `opacity` on a text rule mixes the token with the page: compute it, axe never sees placeholders. WO-0004 #1 failed the search `::placeholder` at 0.8 (4.00:1); it is now opacity 1 (also beats Firefox's 0.54 default). Remaining text fade: `.giallo-ln` 0.6 on the code slab.
- Read `::placeholder` with `getComputedStyle(input, '::placeholder')` after pressing `/`; the input background is transparent, so the dialog's colour is the backdrop.
- Article footers both with and without `--drafts`: only one article is published, so the pager shows only with drafts.
- Things at opacity 0 until hover or focus (`.heading-anchor`, `.code-copy`): axe skips them. Tab onto them and compute contrast by hand. `--rule-strong` as text colour fails in both themes.
- The journey: Pause must stop every journey animation (`el.getAnimations()` playState, and a canvas pixel diff of the hero 2.5 s apart: PNG bytes differ from encoding noise even when frozen). Lows still open after WO-0007: disabled step buttons in static mode, 10.35 px panel text at 390, the pale video button in DARK theme on the always-light video (planners misread this as the light button), the "twelve / seconds" orphan at 1440, the video intro copy untrue under reduced motion/no JS, the tiny recording at 390.
- `.block-title` has no `text-wrap: balance`; new multi-line uses (journey intros) orphan words at 1440.
- Deks and standfirsts for single-word last lines at 1440 after any font or size change (`text-wrap: pretty` is on them since WO-0002).

## How to check
- Header changes: crop `.site-header` at 320/390/641/688/689/705/1440 with an inline Playwright script against `zola serve` (:1111) when it is running; full-page shots are too tall to judge a header. Measure inline SVG ink with `getBBox()` + `getScreenCTM()`.
- Favicons: render the SVG as `<img>` at 16/32 px on light and dark grey strips; headless shows no real tab.
- [Footer focus check](footer-focus-check.md): `--focus` stops at 10 tab stops; inline Playwright for footers, and how to crop screenshots.
- The :1111 dev server runs without drafts: build `zola --root site build --drafts --base-url http://127.0.0.1:4711 --output-dir <ABSOLUTE path under target/> --force` (a relative output dir breaks `cachebust`) and serve it with node `http`.
- H.264 video, autoplay and keyboard checks: launch Playwright with `chromium.launch({ channel: "chrome" })` (real Chrome is installed); the bundled Chromium aborts `.mp4`. The shots tool's `--focus` runs under reduced motion, so motion-only controls need an inline Tab walk.
- Own drafts builds bake the base URL in: serve on the same port you built with (4711), or CSS and theme silently fail to load and crops look unstyled.
- Tab walks: the sequential-focus start point survives `blur()`; to reach a deep control, click/press the control once, then Shift+Tab / Tab to read its neighbours. Clicking a paragraph and pressing Tab can land on a different link.
- Inline Playwright scripts live in the scratchpad and `require("C:/Users/Diorama/nows-portfolio/tools/shots/node_modules/playwright")` by absolute path.
- Long report heredocs sometimes fail in this shell ("unexpected EOF while looking for matching `''"): append the report to a file under target/shots/<id>/ in chunks, then `factory report <id> design < file`. Write is blocked outside this memory folder.
- Opacity-0 elements report `opacity: 0` right after Tab (fade transition): wait ~700 ms before reading styles or shooting.
- On a rework round, scope re-reading with `find <dirs> -newer <ref>`, where `<ref>` is touched to the last design report time (`touch -d "<UTC>Z"`), then re-check the earlier findings with computed styles and crops.
