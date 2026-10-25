# Build: Turn the donor-journey recording back on

## What changed
The donor-journey recording is back on the draft case study `/work/donation-website/`, and `journey.mp4` and `journey-poster.jpg` are back in the build. The template flag is `true` and its comment is gone, `ignored_static` and its comment are gone from `zola.toml`, and the recording's `<!-- Nows: ... -->` comment is gone from the page. Text that said the recording was switched off (the `journey.js` header, `docs/design-system.md`, and the design-system and ui-ux-review skills) now describes it as on. The case study's "How this page moves" table has its Recording row back. The video button now reports its state one way, with `data-playing` and its changing label ("Play" / "Pause"), the same as the hero's Pause animation button; it no longer uses `aria-pressed`.

## Files
```
 M .claude/skills/design-system/SKILL.md     motion: the recording plays while half visible; video button no longer "when it returns"
 M .claude/skills/ui-ux-review/SKILL.md      donor journey: check playback in Chrome; Chromium logs ERR_ABORTED
 M docs/design-system.md                     motion: a twelve-second recording plays while on screen
 M site/content/work/donation-website.md     recording Nows comment removed; Recording row added; Reduced motion row mentions autoplay
 M site/sass/journey.scss                    two .jn-vbtn selectors: [aria-pressed] -> [data-playing]
 M site/static/js/journey.js                 header item 4 on one line; paintVideoButton sets data-playing
 M site/templates/partials/journey.html      show_recording = true, comment gone; button data-playing="false"
 M site/zola.toml                            ignored_static and its comment removed
 M .claude/agent-memory/planner/MEMORY.md    (planner's, not mine)
 M .factory/board.json, ?? .factory/orders/WO-0007/   (factory CLI)
?? design/                                   (untracked mock-ups, not touched)
```
That is eight files from the plan's Changes list. The plan says "nine", but its list names eight. Nothing under `site/static/images/` is modified.

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, no warnings
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `zola --root site build`: ok, 5 pages, 2 sections, 0 warnings
- `cargo run -q -p newpost -- --check`: "Checked 7 files. No problems."
- No Rust changed, so the wasm check does not apply.

## Acceptance checks
1. Pass: fmt, clippy and test are green (above).
2. Pass: zola build has 0 warnings, and newpost check reports no problems.
3. Pass: `grep -n show_recording` prints `7:{%- set show_recording = true %}` and `32:{%- if show_recording %}`. The count of "switched off|WO-0006|re-made" is `0`.
4. Pass: the count of "ignored_static|journey" in `zola.toml` is `0`. `git diff site/zola.toml` shows only the five removed lines; `ignored_content` is unchanged (line 21).
5. Pass: in `donation-website.md`, the count of "Stored safely|ignored_static|show_recording" is `0`, the role-title comment count is `1`, and `^draft = true` is `1`.
6. Pass: `site/public/images/journey.mp4` has sha256 `314332fe…f55af76b` and `journey-poster.jpg` has `84287171…a5b13ee2`, both matching the plan. `site/public/work/` contains only `pdf-merger/` and `this-website/` (the draft is not built).
7. Pass: the drafts build went to `target/wo7`. The section text reads "The whole journey in twelve seconds A silent recording of the same illustration. It plays while it is on screen; the button pauses it. Play Illustration with sample data. The steps above describe what it shows.", and the aria-label reads "Recording of the same illustration: a donor gives on a phone and the gift is recorded in three systems". The security regex printed `false`.
8. Pass: `ffprobe` gives duration `12.000000`, h264 1280x720, and 0 audio streams. I looked at the last frame (`target/wo7-last.png`) and the poster: both end with "All three agree / Same gift, same amount, in every system." I read the 1 fps contact sheet (`target/wo7-sheet.png`) frame by frame and found no security text. The padlock icon beside "All three agree" is still there (see Notes).
9. Pass: real Chrome, reduced motion, `section[aria-labelledby=jn-video-title]` at 1280 and 390, light and dark (`target/wo7crops/block-*.png`). Heading, intro, the poster in a 16:9 frame with a 1 px rule edge, the dark Play button with its icon at the bottom left, and the muted caption all show. Sideways overflow is 0 px in all four. Final judgement is for design review.
10. Pass, in Chrome (`target/wo7-check.mjs`):
    - At 30% visible the video stays paused. At 70% it plays: `data-playing="true"`, label "Pause", pause icon `display: block`.
    - Clicking the button pauses it (label "Play", `data-playing="false"`). Scrolling to the top and back leaves it paused at the same time (1.29 s).
    - Clicking Play starts it. Scrolling it off screen pauses it.
    - `hasAttribute("aria-pressed")` is `false` in every state, and there are no console errors.
11. Pass: with reduced motion the video stays paused while fully on screen, and clicking Play plays it. With JavaScript off the button has `hidden` (`display: none`) and the poster attribute is set (`target/wo7crops/nojs-1280.png`).
12. Pass: `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0007 --pages /work/donation-website/ --drafts --widths 390,1280 --reduced --scroll "#journey-story:0.05,0.22,0.5,0.77,1"` reports "Accessibility scan, WCAG 2.2 AA (0 rule failures)". The only problems logged are the eight expected `journey.mp4 (net::ERR_ABORTED)` lines, and there is no sideways scrolling.
13. Pass, at 1280 and 864 (`target/wo7-key864.mjs`; 1280 also in dark):
    - Shift+Tab from the video button lands on step 5 ("Three systems record the gift"). Tab returns to the button, and the next stops are the heading anchors after it; the focus is never `BODY`.
    - The ring is `solid 2px rgb(43,22,24)` with a `0 0 0 2px rgb(250,245,243)` band in light, and `rgb(242,237,235)` / `rgb(31,27,28)` in dark (`target/wo7crops/focus-*.png`).
    - Enter pauses the video and Space plays it again.
    - My first 864 run started from the top of the page because `focus()` on the last step did nothing. Shift+Tab from the button confirmed the order.
14. Pass: the hero Pause animation button takes the hero animations from 2 running to 0 running, and its label changes to "Play animation". The scroll shots at 0.22 and 1 show scene states 2 and 5 with the matching step open (`target/shots/WO-0007/*-scroll-*.png`).
15. Pass: the grep over the two docs, the two skills and `journey.js` prints nothing.
16. Pass: the table on the built drafts page has the rows Reading line, then Recording, then Reduced motion. The Reduced motion row reads "pinning, autoplay and idle motion are off".
17. Pass: `git diff --quiet site/static/_headers` exits 0. The built page has 0 `style=` attributes. Its scripts are `journey.js`, `head.js` and `site.js` (all same-origin files) plus one `application/ld+json`.
18. Pass: `git status --short` lists only the files above (see Files).

## Findings fixed (rework)
None; this is the first build.

## Notes for reviewers
- **Truthfulness:** the recording block's text and the video carry no security claim. A small **padlock icon** still sits beside "All three agree" in the last frame and the poster. It is part of Nows's supplied file, so I left it alone; it is for the reviewer to mention to Nows.
- **The Recording row's wording:** "plays while half of it is on screen" matches the behaviour I measured (paused at 30% visible, playing at 70%). "About 40 lines" matches section 4 of `journey.js` (lines 376 to 420, about 45).
- **Design:** the light poster on the dark page (`block-1280-dark.png`), framed by `--rule`. At 390 the Play button covers part of the phone in the poster (`block-390-dark.png`). That is the WO-0005 layout, unchanged here.
- **Deployment:** with `ignored_static` gone, the next push deploys both media files even though the page is a draft.
- Scratch scripts and shots are in `target/` (git-ignored, outside the seal).
