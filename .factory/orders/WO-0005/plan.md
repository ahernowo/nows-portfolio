# Plan: Port the donor-journey motion to the work page

## Goal
A new draft case study, `/work/donation-website/`, carries the donor-journey from the approved mock-up (`design/journey/`): a hero with the animated scene, a pinned five-step walkthrough driven by scrolling, the 12-second recording with a Play/Pause button, "What changed" bars that stay hidden until Nows gives real figures, and a short "How this page moves" table. It uses the site's header, pager, tokens, Inter and CSP. The other four case studies don't change.

## Scope
In:
- New case study `site/content/work/donation-website.md`, `draft = true`. The journey is switched on from its own front matter (`[extra.journey]`, Q2).
- The journey branch of `work-page.html`. The scene is server-rendered markup in a Tera component (no `innerHTML`). One script, `site/static/js/journey.js`. One stylesheet, `site/sass/journey.scss` → `/journey.css`, loaded only on journey pages.
- The green as tokens `--ok` / `--ok-tint` in both themes (Q6), plus the illustration's own colours as `--jn-*` tokens.
- Byte-for-byte copies of `journey.mp4` and `journey-poster.jpg` in `site/static/images/`.
- Decap fields for the new front matter.
- Two small additions to the screenshot tool so the pinned story and the hidden bars can be reviewed: `--root` and `--scroll` in `shoot.mjs`, and the `.mp4` type in `serve.mjs`.
- Recording the journey in the design-system skill as the second approved moment (Q5), in `docs/design-system.md`, and in the ui-ux-review checklist.

Out:
- The mock-up's own header, Theme button, "Mock-up" note, footer and "Next case study" link: the site header and the existing `.pager` replace them (Q6).
- Splitting the title into three rising lines. The h1 is `page.title` and rises as one line, because line breaks depend on the viewport. The mock-up's headline text is sample copy.
- Re-recording `journey.mp4` / `journey-poster.jpg`. Both still show "Stored safely: Encrypted, with access limited and logged". Nows chose to keep the video, so this is flagged for Nows, not fixed (see Risks, truthfulness).
- `site/static/_headers` (unchanged, per the request), the other case studies, `.pager` rules, and `design/` (left untracked for Nows).
- Real figures, role, year and stack: Nows supplies them later (Q3). The page stays a draft.

## Changes
- `site/content/work/donation-website.md` (new). Front matter: `title = "Donation website"`, `template = "work-page.html"`, `description = "One gift followed from a donor's phone through the donation form, the payment and the three systems that record it."`, `date = 2026-10-24`, `weight = 5`, `draft = true`, `include_in_feeds = false`. `[extra]` has empty `year`/`role`/`status`/`stack`, each with a `# Nows: ...` TOML comment. Put `[extra.journey]` last in the front matter (so test rows can be appended before the closing `+++`). It holds exactly five `[[extra.journey.steps]]` (`title`, `body`) and **no** `compare` rows. The step copy describes only what the illustration shows. It makes no claims about Nows's project:
  1. "Opens the link on her phone" / "A message arrives with a link to the donation page."
  2. "Chooses a gift" / "She picks Rp150.000 and keeps the monthly option."
  3. "Fills in her details" / "Name, email and phone, then a tick for an emailed receipt."
  4. "Pays" / "She chooses an e-wallet and confirms. The button shows the payment is being confirmed, then the page says thank you."
  5. "Three systems record the gift" / "The donation form, the payment gateway and the CRM each hold a record of the same gift. When the three match, the gift is stamped reconciled."
  The mock-up's step 5 ("Her data is safe and processed ... stored with access limited") makes a claim, so it is dropped, as the scene line was in Q4.
  Body:
  - One `<!-- Nows: ... -->` block listing what is missing: what you built and your role; year; stack; a description in your words; one real sentence per step, if wanted; the before/after figures as `[[extra.journey.compare]]` rows (`label`, `value`, `share` 0-100, `after = true` on the new way); the "Stored safely" line baked into the video.
  - Then `## How this page moves`: one sentence ("Each effect uses the lightest tool that can do it. The site's security rules allow no outside scripts, so none of this needs a library.") and a Markdown table (Effect | Built with | JavaScript). Take its rows from the mock-up's table, made true for the built code: title rise (skipped after the title morph), breathing and blinking (stop within 5 s), hero first beat, pinned journey (`position: sticky` + one scroll listener → one progress number), character and hand (inline SVG), reading line and bars (CSS scroll/view timelines; JS only sets bar lengths), recording (plays while half visible, button pauses it), reduced motion. The JavaScript column gives real line counts from `journey.js`.
- `site/templates/work-page.html`: set `journey = page.extra?.journey`. The journey is on when `journey?.steps` is non-empty, so an empty list written by Decap leaves it off. When it is on:
  - `head_extra` adds `journey.css` (`get_url(path='journey.css', cachebust=true)`) and `js/journey.js` (cachebust, `defer`).
  - Add the existing `<div class="reading-progress" aria-hidden="true">`.
  - The header becomes a two-column hero (`.jn-hero`). The left column keeps the existing crumb, `h1.case-title` (keep `data-vt="title"`; wrap the text in `<span class="jn-rise"><span>…</span></span>`), `.case-dek` and `dl.case-form`. The right column is `{{ <journey_scene label="..." hero={true} /> }}`.
  - Then `{% include "partials/journey.html" %}`, the existing `.prose.case-body` and the existing `.pager`.
  - Case studies without a journey render byte-for-byte as today.
- `site/templates/partials/journey.html` (new). Three `<section>`s, each with an `h2` and an intro. The intros are template copy because they describe the fixed illustration and recording:
  - **Walkthrough.** `h2` "One gift, from the donor's side". Intro: "Scroll to follow Rina, an invented donor, through one gift. The pages, names and amounts are an illustration with sample data." Then `#journey-story.jn-story` with a sticky `.jn-stage`, holding the story scene (`journey_scene`, the mock-up's full aria-label) and `ol.jn-steps` rendered from `journey.steps[0..4]`. A missing index fails the build, which is wanted. Each step is `<button class="jn-step" type="button" data-target="…" disabled>` with its number and title, followed by its body. `data-target` comes from a template list `[0.05, 0.22, 0.5, 0.77, 0.97]`. The buttons ship `disabled`, and `journey.js` enables them only when motion is allowed. Add `p.jn-stepcount aria-hidden="true"`.
  - **Video.** `h2` "The whole journey in twelve seconds". Intro: "A silent recording of the same illustration. It plays while it is on screen; the button pauses it." Then a `<figure>` holding `<video muted loop playsinline preload="metadata" width="1280" height="720" poster=…>` with an `aria-label` ("Recording of the same illustration: a donor gives on a phone and the gift is recorded in three systems") and one mp4 `<source>` (URLs via `get_url`). Add a `<button type="button" aria-pressed="false">` with `aria-hidden` play/pause icons and a text label, "Play" or "Pause". `figcaption`: "Illustration with sample data. The steps above describe what it shows."
  - **What changed.** Only `{% if journey.compare %}`. One row per entry: the label and value as text, then `.jn-track > .jn-fill[data-share="{{ row.share }}"]`, with class `jn-fill-after` when `row.after`. No `style=""` anywhere.
- `site/templates/components.html`: new component `journey_scene(label: string, hero: bool = false)`. It renders the mock-up's `TPL` (`design/journey/src/journey.js` lines 18-85) as static markup in its **finished** state, so the page is complete without JS: the success screen is showing, the rows carry `done` with "Saved" / "Confirmed" / "Saved", the stamp is visible, and the arm lines sit at rest coordinates.
  - Outer element: `<figure class="jn-room" role="img" aria-label="Illustration with sample data: {label}">`. Every child is `aria-hidden="true"`. Use `<p>`/`<div>` inside, never `h3`/`h4`.
  - Write out (or loop in Tera) the 8 burst lines and the keyboard keys, so no `createElementNS` is needed.
  - Delete the "Stored safely" block (`.safe` and its lock `.sh`) (Q4).
  - Prefix every class with `jn-` or scope it under `.jn-room`. In particular, `.stamp` becomes `.jn-stamp`, because `_home.scss` styles and animates `.stamp` globally.
- `site/sass/journey.scss` (new; Zola compiles it to `/journey.css`). Port `journey.css` and the page parts of `page.css`:
  - Colours only through tokens or `color-mix()` of tokens. No raw hex or `rgb()`. Font `var(--font-display)`.
  - Site surfaces use `var(--radius)`: the room, panel, video box, bar tracks and video button. The phone keeps its own shapes (bezel, screen, chips, keys). The halo stays a circle. `.jn-stamp` matches the home stamp's 6px.
  - The panel takes `--slip-shadow`. The video box gets a 1px `--rule` border and no shadow.
  - Scene SVGs get `max-width: none`, because `_base.scss` sets `svg { max-width: 100% }`, which would clamp the 150%-wide hands drawing.
  - Done row: `.jn-dot` background `--ok`, tick `--slip-original`.
  - Saving row (`.saving`, set by JS while 0 < r < 1): `--ok-tint` dot with an `--ok` border.
  - Status text and meter fill `--ok`; meter track `--rule`.
  - `.jn-stamp`: `--stamp` on `--slip-original`, bold, font-size at least `1.2rem` (large text).
  - Bars: the track is `color-mix(in srgb, var(--ink) 7%, transparent)`. The fill is `--rule-strong`, or `--accent` for `.jn-fill-after`, with width `var(--w, 0%)` from JS. Tracks are hidden until the block has `.is-ready`.
  - Video button: `--ink` background, `--paper` text, `--radius`, at least 44px tall.
  - Layout as in the mock-up: the 280svh story (fallback 280vh), the sticky 100svh stage, `1.5fr / .5fr` columns, and the room-shape container query. The narrow layout starts at `max-width: 54rem` (mock-up 860px): one column, only the current step, and the step counter shown.
  - Static mode `.jn-story.is-static`, also the no-JS default until JS adds `.is-live`: no pinning, every step listed with its body open. Disabled step buttons keep `--ink` text.
  - Every keyframe sits inside `@media (prefers-reduced-motion: no-preference)`: title rise, scene fade-up, breathe, blink, caret, spinner, and the bars' `view()` timeline inside `@supports`. Add `.vt-arrival .jn-rise > span { animation: none }`.
  - **Auto-start motion ends within 5 s** (WCAG 2.2.2, see Risks). Breathe runs once (4.2 s). Blink runs once, in at most 4.6 s. Only the caret and spinner stay `infinite`, and they appear only at scroll positions the visitor picked.
  - Breakpoints in rem. `font-variant-numeric: tabular-nums` on the bar values.
- `site/sass/_tokens.scss`: add, without changing any existing value:
  - `--ok`: `#186A41` light, `#7FD0A5` dark (Nows's green).
  - `--ok-tint`: `#E3F2EA` light, `#1E3A2D` dark (the mock-up's tints).
  - A commented "journey illustration" group, `--jn-*`, with one token per distinct colour in the mock-up's `journey.css`: skin, skin shade, hair, mouth, bezel, bezel edge, screen, screen-off, screen ink, screen line, screen fills, the screen accent (fixed `#B10F2E`, since the phone always shows a light page), its tint, keyboard, check border, lock-screen wallpaper stops, and the phone's success ring.
  - Exceptions:
    - `--jn-screen-muted` is `#6B5557`, not the mock-up's `#7A6A6C`, which is only 4.45:1 on the URL bar.
    - `--jn-shirt` / `--jn-collar` get their dark values (`#A8394C`, `#F2E6CD`) in `dark-tokens`.
  - Everything else is the same in both themes, like `--code-bg`.
- `site/static/js/journey.js` (new). One IIFE, `"use strict"`, readable and commented like `site.js`. It ports `journey.js` + `page.js` with these differences:
  - `bindScene(root)` queries the server-rendered nodes instead of setting `innerHTML`. It adds `is-live` to the room.
  - Drop the theme code (`site.js` owns the Theme button), the lock-shackle and "safe" code, and the `t` (video-frame) branch.
  - All style changes go through `el.style.x = …` / `el.style.setProperty(…)`. Never `setAttribute('style')`, and no `insertAdjacentHTML`, `eval` or `fetch`.
  - Rows toggle `saving` and `done`.
  - The hero beat starts at 0.6 s and finishes by 5 s (mock-up: 0.7 s + 5.2 s), then rests at 0.25. Under reduced motion it renders 0.25 at once.
  - Story: the same `CUTS`, `aria-current="step"` and the "Step n of 5" counter. Remove `disabled` from the step buttons only when motion is allowed (and put it back if reduced motion switches on). Add `is-static` under reduced motion.
  - Video: autoplay at 50% visible unless reduced motion is set or the visitor paused. The button sets `aria-pressed` and a "Play"/"Pause" label. Swallow the `play()` rejection.
  - Bars: `share = clamp(parseFloat(data-share) || 0, 0, 100)`, then `fill.style.setProperty('--w', share + '%')`. Then add `is-ready`.
  - The scroll listener is passive and rAF-throttled. Exit quietly when the page has no `.jn-room`.
- `site/static/js/head.js`: in the existing `pagereveal` listener, when `e.viewTransition` is set, add class `vt-arrival` to `<html>`. That way the title morph and the title rise never play together. One line plus a comment.
- `site/static/images/journey.mp4`, `site/static/images/journey-poster.jpg` (new): byte copies from `design/journey/`.
- `site/static/admin/config.yml`: in the work collection's `extra` object, add `journey` (object, `required: false`, `collapsed: true`, hint "Only for the donation-website case study"). It contains `steps` (list, `min: 5`, `max: 5`, fields `title`, `body`) and `compare` (list, `required: false`, fields `label` string, `value` string, `share` number int 0-100, `after` boolean, `default: false`).
- `tools/shots/shoot.mjs`:
  - `--root <dir>`: the Zola site folder to build, default `site`. It goes into the existing `spawnSync` args array, with no shell.
  - `--scroll "<selector>:<f1>,<f2>,…"`: after each normal capture (motion runs only), for each fraction, `window.scrollTo(0, top + (el.offsetHeight - innerHeight) * f)`, wait 600 ms, and save a viewport screenshot named with the fraction.
  - Document both flags in the header comment.
- `tools/shots/serve.mjs`: add `".mp4": "video/mp4"` to `TYPES`.
- `.claude/skills/design-system/SKILL.md`:
  - **Tokens**: add rows for `--ok` / `--ok-tint` and a line on the `--jn-*` illustration group.
  - **Contrast**: add the rows from the table below.
  - **Accent**: also used for the "after" bar on the journey.
  - **Motion**: a second approved moment, the case-study journey: hero beat ≤ 5 s, title rise, idle breath and blink that stop within 5 s, the scroll-driven story, the video that plays while visible with a Pause button, and bars on a view timeline. It appears only on case studies that switch it on, and all of it stops under reduced motion.
- `.claude/skills/ui-ux-review/SKILL.md`:
  - **Motion line**: "... or belongs to the home page's load sequence or the case-study journey".
  - **Accent line**: add the journey's "after" bar.
  - A note on `--scroll` and on viewing the hidden bars through a test copy (check 9).
- `docs/design-system.md`: the Motion bullet names the journey, and the Tokens table gains `--ok` / `--ok-tint`.

## Contrast (computed from the hex values; WCAG)
| Pair | Light | Dark |
|---|---|---|
| `--ok` status text on panel (`--slip-original`) | 6.61 | 8.19 |
| Tick `--slip-original` on `--ok` dot (non-text) | 6.61 | 8.19 (white would be 1.83, hence the token) |
| `--ok` on `--ok-tint` (saving dot) | 5.71 | 6.75 |
| `--stamp` on panel (bold, ≥1.2rem = large) | 7.05 | 4.23 |
| Accent "after" bar vs track | 5.67 | 5.55 |
| `--rule-strong` "before" bar vs track | 1.67 | 1.96 (the value is also in text beside it; bar is a duplicate) |
| Video button `--paper` on `--ink` | 15.78 | 14.68 |
| Phone muted `#6B5557` on screen / URL bar / receipt | 6.86 / 5.97 / 6.19 | same (the phone screen is light in both themes) |
| Phone accent `#B10F2E` on chip tint `#FBEBEE`; white on `#B10F2E` | 6.12; 7.05 | same |
No existing token changes, so no faded-text rows change (lessons WO-0004).

## Acceptance checks
`$SCRATCH` is your own scratchpad. In Git Bash, prefix node commands with `MSYS_NO_PATHCONV=1`. Don't use port 1111, where the dev server runs.

1. The standard checks pass: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `zola --root site build` (no warnings), `cargo run -q -p newpost -- --check` (no problems), and `zola --root site check --skip-external-links --drafts`.
2. Draft only. `zola --root site build --drafts --output-dir "$SCRATCH/out" --force` creates `out/work/donation-website/index.html`. The normal build (`--output-dir "$SCRATCH/pub"`) has no `work/donation-website/`. `grep -c '^draft = true' site/content/work/donation-website.md` prints 1, and `grep -c '<!-- Nows:' …` prints at least 1.
3. Other case studies unchanged. `for p in this-website pdf-merger bank-reconciliation donation-dashboard; do grep -c 'jn-\|journey' "$SCRATCH/out/work/$p/index.html"; done` prints 0 four times.
4. CSP. `git diff --quiet -- site/static/_headers` exits 0. `grep -c 'style="' "$SCRATCH/out/work/donation-website/index.html"` prints 0. The page has no `<script>` without `src` except `application/ld+json`. `grep -nE "innerHTML|insertAdjacentHTML|outerHTML|eval\(|new Function|setAttribute\(.style|fetch\(|https?://" site/static/js/journey.js` prints nothing. `grep -rn "googleapis\|gstatic" site/` prints nothing.
5. Tokens only. `grep -nE "#[0-9a-fA-F]{3,8}\b|rgba?\(" site/sass/journey.scss` prints nothing. `git diff site/sass/_tokens.scss` only adds lines.
6. Media are exact copies. `sha256sum site/static/images/journey.mp4 site/static/images/journey-poster.jpg` gives `64dbd92f0f3d8bca34da948badf06842642f694bcda09cfc339ae185b973c7c1` and `0974e07c57a8397381d15acb4ef904c12fb563916fa27d200bcde72fc0aa2350`.
7. No mock-up sample text. In `$SCRATCH/out/work/donation-website/index.html`, `grep -ciE "Mock-up|\[Your |Example figures|Replace (it|them) with|Stored safely|access limited|end to end|Next case study"` prints 0. The illustration label is there: `grep -c "Illustration with sample data"` prints at least 1. The page has no `jn-compare` rows (`grep -c 'jn-fill'` prints 0).
8. Screens. `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --drafts --reduced --out target/shots/WO-0005 --pages /work/donation-website/,/,/work/,/work/this-website/ --scroll "#journey-story:0.05,0.22,0.5,0.77,0.97"`. The report shows no console errors, no CSP violations, no failed requests and no sideways scroll. Axe reports no failures, or each one has a finding. `/`, `/work/` and `/work/this-website/` look as before (compare with `target/shots/WO-0004/` where it exists).
   - At 1440 light: the hero is two columns (title, dek and form fields on the left; the scene on the right, about 520px wide). Square 4px corners on the room and panel. The site header at the top, and the pager at the bottom.
   - At 390: one column with the scene under the form fields, and no overlap.
   - Scroll shots at 1440: 0.05 shows the lock screen with the notification and step 1 open. 0.22 shows the amount chosen and step 2 open. 0.5 shows the form being typed, the keyboard up and step 3. 0.77 shows Pay, the spinner and step 4. 0.97 shows the person moved left and the "Behind the screen" panel on the right with three green "Saved/Confirmed/Saved" rows and the stamp. There is no "Stored safely" line, and step 5 is open.
   - Scroll shots at 390: the same scene states. Only the current step and "Step n of 5" show under the scene. The panel sits at the bottom of the room.
   - Dark: page `#1F1B1C`, panel `#2C2527`, green `#7FD0A5`, stamp `#E8546D`. The phone screen stays white, and the shirt is the lighter dark-theme red.
   - Reduced motion: the story isn't pinned (the page is much shorter), all five steps are listed and open, the story scene is in the finished state, the hero rests on the chosen amount, and the video shows its poster with the button reading "Play".
9. Bars through a test copy, which leaves the repo untouched:
   ```bash
   T="$SCRATCH/wo5-test"; rm -rf "$T"; cp -r site "$T"
   python -I - "$T/content/work/donation-website.md" <<'PY'
   import sys; p = sys.argv[1]; s = open(p, encoding="utf-8").read()
   rows = '\n[[extra.journey.compare]]\nlabel = "Test row: before"\nvalue = "test value 100"\nshare = 100\n\n[[extra.journey.compare]]\nlabel = "Test row: after"\nvalue = "test value 6"\nshare = 6\nafter = true\n'
   i = s.index("+++", 3); open(p, "w", encoding="utf-8").write(s[:i] + rows + s[i:])
   PY
   MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --root "$T" --drafts --out target/shots/WO-0005-bars --pages /work/donation-website/ --widths 390,1440
   ```
   The "What changed" section shows two labelled rows. The first bar fills the track in grey, and the second is about 6% wide in the accent. Both themes are covered, with no console or CSP errors. With `--reduced` added, both bars show at their final lengths with no grow animation.
10. Keyboard, at 1440 in Chrome on a drafts server: `zola --root site serve --drafts --port 1113`, then http://127.0.0.1:1113/work/donation-website/. Tab reaches the five step buttons in order, each with a visible ring. Enter on step 3 scrolls smoothly until the scene shows the form, and step 3 becomes `aria-current="step"`. The video button toggles its label between Play and Pause, and `aria-pressed` with it. Under reduced motion (DevTools > Rendering), the step buttons are disabled and not in the Tab order.
11. Video in a browser with H.264. The Playwright Chromium can't decode it, so the shots show the poster. Use Chrome or Edge on the port-1113 server: scroll the video half into view, and it plays muted. Scroll away, and it pauses. Press Pause, scroll away and back, and it stays paused. With reduced motion on, it never starts by itself.
12. Title. Load the page directly: the title rises once and the scene fades up. Go from `/work/` (drafts server) to "Donation website": the title morphs, and **no** rise plays after it.
13. Auto-start motion stops. After 5 s on the page (hero in view, not scrolling), nothing in the hero moves. `grep -n infinite site/sass/journey.scss` lists only the caret and spinner.
14. Without JavaScript (Chrome DevTools > Disable JavaScript): both scenes show the finished state (success screen, panel with three green rows and the stamp), all five steps show with text, the video shows its poster, no box is empty, and nothing overlaps at 390 or 1440.
15. Decap. http://127.0.0.1:1111/admin/ still loads to the sign-in screen with no configuration error in the page or the console. The work collection YAML has `journey` under `extra` with `steps` (min 5, max 5) and `compare`.
16. Skills and docs. `grep -n "journey" .claude/skills/design-system/SKILL.md .claude/skills/ui-ux-review/SKILL.md docs/design-system.md` shows the second moment, the `--ok` / `--ok-tint` tokens, and the accent and motion lines.

## Risks
- **Truthfulness.**
  - The scene's people, charity, URLs, DN-2047 and Rp150.000 are invented. They stay as a labelled illustration (Q4): the aria-labels, the walkthrough intro, the panel's "Sample records for this one gift" and the figcaption say so. Reviewer: no sentence on the page may claim anything about Nows's project beyond what the repo says. Check the steps, the description and the table.
  - **The video and the poster still show "Stored safely: Encrypted, with access limited and logged"**, which the live scene drops. They are copied unchanged because Nows chose to keep the video. Flag it in the build and approve reports, and in the `<!-- Nows -->` block. The options for Nows: re-record from the ported scene (a later order), or accept it. The page is a draft, so it doesn't go public meanwhile.
- **Accessibility.**
  - (a) WCAG 2.2.2: motion that starts by itself and lasts over 5 s needs a pause control. The mock-up's endless breathing and blinking and its 5.9 s hero beat would fail it. The plan caps them at 5 s, a small departure from "keep it all as mocked", which is flagged to Nows. If Nows wants them endless, the page needs a pause control.
  - (b) The scenes are `role="img"` with every child `aria-hidden`. Axe may still check contrast inside them. Phone text is computed above. The faded "Continue" (disabled-button look, 1.23:1) shows only briefly mid-scroll. If axe flags it, log it as low, with the shot position.
  - (c) At 390 the other step buttons are hidden, so a keyboard user there moves through the story by scrolling. Check that this reads as acceptable.
  - (d) The video has no audio. The steps describe the same content (WCAG 1.2.1 media alternative).
  - (e) The panel text is small (`clamp(11.5px, 1.5cqw, 15px)`, as mocked). Watch it at 390.
- **Design.**
  - The mock-up's 16/14/12px corners become `--radius`. Only the phone keeps its rounded shapes.
  - The lock-screen gradient pictures a phone wallpaper; it isn't a decorative wash.
  - Step numbers 1-5 are a real sequence.
  - The accent on the "after" bar is a new accent use, recorded in the skill.
  - Check the generated-look list and that `/`, `/work/` and the other case studies are unchanged. `_tokens.scss` only gains tokens, but it reaches every page.
- **Layout traps.**
  - Sticky breaks if any ancestor of `.jn-story` gets `overflow: hidden`.
  - `svg { max-width: 100% }` in `_base.scss` will squash the hands unless the scene's SVGs override it.
  - The global `.stamp` in `_home.scss` would restyle and animate an un-renamed stamp.
- **Security.**
  - Markup is server-rendered, so there is no `innerHTML`.
  - JS writes styles only through CSSOM, which `style-src 'self'` allows. Inline `style=""` would be blocked.
  - `data-share` is parsed as a number and clamped.
  - Video and poster are same-origin (`default-src 'self'` covers media).
  - The new `--root` flag goes into an argument array, not a shell string.
  - Check the copied mock-up JS for anything beyond scene logic. The plan's review found only `innerHTML` with a constant template, which is now gone. The mock-up's Google Fonts links aren't ported.
- **Tooling.** Playwright's Chromium has no H.264, so the video plays only in the Chrome or Edge checks (check 11). `--scroll` shots depend on the 600 ms wait; if a shot looks mid-transition, rerun before logging a finding.

## Skills the builder needs
zola-site, design-system, cms-content, ui-ux-review (for the shots tool), factory-protocol. No Rust changes.

## Decisions from Nows
- Q1 The flow-mark SVGs were missing from the working tree. The supervisor restored them with `git restore`, and the build passes again. They aren't part of this change.
- Q2 The journey goes on a new case study, `site/content/work/donation-website.md`, switched on from its own front matter. The other case studies stay unchanged.
- Q3 The page is built with `<!-- Nows: ... -->` markers and stays `draft = true`. The bars stay hidden until real figures are in front matter; reviewers see them only through a test entry (check 9).
- Q4 Invented data stays as a labelled illustration, as with DN-2041 on home. The "Stored safely" line is dropped from the scene. Nows keeps the video, which still shows that line; it is flagged, not changed (Risks).
- Q5 The motion is kept as mocked and recorded in the design-system skill as a second approved moment. One planner default is flagged for Nows: auto-starting motion is capped at 5 s for WCAG 2.2.2.
- Q6 The page follows the design system (site header and pager, 4px corners, no separate next-case-study link), and the green becomes a token in both themes (`--ok`, `--ok-tint`).
