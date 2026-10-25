# Design review: Turn the donor-journey recording back on

## Verdict
Pass. Five low findings (#1-#5), none blocking. #2 (the padlock) is for Nows to decide.

## What I looked at
- The diff: 8 files. I read the full recording section of `partials/journey.html`, section 4 and `applyMotionPreference` of `journey.js`, the recording rules of `journey.scss` (1081-1153), the whole of `donation-website.md`, and the changed lines in the docs and the two skills.
- Shots tool, drafts, `/work/donation-website/` at 390 and 1440, light and dark, plus reduced motion and scroll states 0.05/0.5/1: `target/shots/WO-0007/design/`.
- Real Chrome (inline Playwright, `channel: "chrome"`, my own drafts build on :4711 with Range support), crops in `target/shots/WO-0007/design/crops/`:
  - `playing-*` and `paused-*` for each width and theme
  - `frame-*-{0.5,3,6,9,11.8}`: the button over five points of the video
  - `focus-*`, `reduced-{390,1440}`, `nojs-1440`, `table-1440-dark`
- The poster file `site/static/images/journey-poster.jpg`, opened directly.

## Checklist
- Identity and layout: ok. The section is the WO-0005 layout: 16:9 frame, 1 px `--rule` edge, `--radius`, caption in `--ink-soft`. Its intro sits on the same column as the walkthrough intro.
- Typography: #3. The heading breaks "...in twelve / seconds" at 1440 (2 lines; `text-wrap: wrap`).
- Colour and contrast: #1. Label text is fine (`--paper` on `--ink`: 15.78 light, 14.68 dark). In dark, though, the button fill (#F2EDEB) sits on the light video with almost no edge (`playing-1440-dark.png`, `frame-1440-dark-9.png`).
- Motion: ok. It plays at 50 % visible (in view: `paused:false`, `data-playing="true"`, "Pause"). A click pauses it and it stays paused after scrolling away and back. Play then scroll away: it pauses. With reduced motion it stays paused (`t:0`) and clicking Play plays it. Without JS the button is `hidden` and the poster shows. The looping video has its own pause, so WCAG 2.2.2 is met.
- Interaction and focus: ok. Shift+Tab from the button lands on step 5, "Three systems record the gift". Tab goes on to the `#what-i-will-build-and-run` heading anchor, and focus never lands on BODY. The ring and page-coloured band show over the frame in both themes (`focus-1440-dark.png`, `focus-390-light.png`). Space plays, Enter pauses. The button is 83x44. `aria-pressed` is absent in every state, and the icons swap (play/pause `display` block/none).
- Responsive: #5. No sideways scroll at 390 or 1440 (overflow 0 in all four runs). The video has width/height and an aspect ratio. The table at 390 fits (350/350 px).
- Accessibility: ok. axe found 0 rule failures across 8 runs. The only logs are the expected 8x `journey.mp4 ERR_ABORTED` from Playwright's Chromium. One h2 for the section; `aria-label` on the video; the caption says the data is sample data.
- Copy: #2, #4. No security words in the heading, intro, aria-label or caption. The poster and the last frame end with "All three agree / Same gift, same amount, in every system." The new table row (Recording) reads plainly and sits between Reading line and Reduced motion; the Reduced motion row now mentions autoplay.

## Acceptance checks
9. Pass: at 390 and 1440, light and dark, I saw the heading, the intro and the poster or frame in the 16:9 frame with its rule edge. In light the button is dark with a play icon at the bottom left; in dark it is pale (#1). The muted caption sits under the frame, and nothing overflows.
10. Pass in real Chrome. Autoplay at half visible gives "Pause" / `data-playing="true"`. A click gives "Play" / `false`, and the video does not restart after scrolling away and back. Play, then off screen, pauses it. `hasAttribute("aria-pressed")` is false every time.
11. Pass. Reduced motion: no autoplay, and Play works. No JS: the button is `hidden` (`display: none`), the poster shows, and the video has no `controls`.
12. Pass: axe found 0 failures, light and dark, at 390 and 1440, motion on and reduced.
13. Pass at 390 and 1440 in both themes (the builder covered 864). The order runs step 5, video button, heading anchor, with no BODY stop. Ring visible; Enter and Space toggle.
14. Pass from the scroll shots at 0.05, 0.5 and 1 (states 1, 3 and 5). I did not re-measure the hero Pause animation button; nothing in its code changed.
15. Pass: the grep over the docs, skills and `journey.js` printed nothing.
16. Pass: rows Reading line, Recording, Reduced motion, in that order (`table-1440-dark.png`). "pinning, autoplay and idle motion are off".
17. Pass (design side): 0 `style=` attributes in the built drafts page.

## Findings
- #1 low: In dark, the video button keeps the theme's light `--ink` fill, but the recording is light in both themes. The button shows as a pale box with almost no edge, and only the label tells you it is a button. Give it theme-independent colours, like the phone (for example `--jn-bezel` fill with `--jn-screen` text), or a `--rule-strong` edge (journey.scss:1103-1120). This is WO-0005 #5, back with the recording; the plan's "15.78:1" is the light-theme figure.
- #2 low, for Nows: The words no longer make a security claim, but the last frame and the poster still put a padlock icon beside "All three agree". A visitor may read a lock as "stored securely". The icon is in Nows's supplied file, so keep it, or swap it for a tick or an equals sign in the next re-record.
- #3 low: At 1440 the heading "The whole journey in twelve seconds" breaks as "...in twelve / seconds", one word alone on the last line. Add `text-wrap: balance` to `.jn-intro .block-title` (or to `.block-title`). This is WO-0005 #2.
- #4 low: The intro says "It plays while it is on screen; the button pauses it." With reduced motion it never starts by itself. Without JavaScript there is no button and the video cannot be played at all (no `controls`). Reword it, for example "It plays while it is on screen unless you have reduced motion on; the button plays and pauses it". Or give no-JS visitors a link to the file (partials/journey.html:36). This is related to WO-0005 #4.
- #5 low: At 390 the recording is about 348 px wide. The records panel text in it is a few pixels tall and unreadable, and the 83x44 button covers the lower part of the phone (`playing-390-light.png`, `reduced-390.png`). The caption points to the steps above, so nothing is lost. Consider moving the button below the frame on narrow screens, or showing the recording only from 40rem up.
