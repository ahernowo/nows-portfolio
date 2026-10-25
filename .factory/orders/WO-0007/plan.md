# Plan: Turn the donor-journey recording back on

## Goal
The draft case study `/work/donation-website/` shows its twelve-second recording again, now that Nows has replaced `journey.mp4` and `journey-poster.jpg` with a version whose last panel says "All three agree" instead of the old "Stored safely" security line. The two files go back into the build, and the comments, docs and skills that said "switched off" are brought up to date.

## Scope
In:
- The three edits Nows listed: `show_recording = true` (and its comment gone) in `partials/journey.html`; the `ignored_static` line and its comment gone from `zola.toml`; the recording `<!-- Nows: ... -->` comment gone from `donation-website.md`.
- The check Nows asked for: the recording block (heading, intro, aria-label, caption) and the video itself carry no security claim. Planner already checked the files (see Risks); the reviewers repeat it.
- Text that would become false once the recording is back: the comment at the top of `journey.js`, one sentence each in `docs/design-system.md`, the `design-system` skill and the `ui-ux-review` skill, and the "How this page moves" table on the case study, which lost its Recording row in WO-0006. The restored row uses the WO-0005 wording, which comes from the code (12 s H.264, ~40 lines of JS in section 4 of `journey.js`).
- The video button gets one state signal, as the WO-0005 lesson asks and as the hero's Pause animation button already does: `data-playing` plus the changing label, no `aria-pressed`. This was a low finding in WO-0005 that "returns with the recording" (ui-ux-reviewer memory); fixing it is three small edits.

Out:
- `draft = true` stays, and the `<!-- Nows: confirm the exact role title -->` comment stays (Nows asked for both).
- The video and poster files: Nows supplied them; not re-encoded or edited. They are already committed in ce9a775 (sha256 below).
- `design/journey/` (untracked mock-up folder, holds the old video): not touched.
- The other old low finding (dark button on the light poster): it reads at 15.78:1 and is a deliberate choice; not reopened.
- The padlock icon beside "All three agree" in the video's last frame: part of Nows's supplied file; noted as a risk, not changed.
- Agent memories other than the planner's: each station updates its own.
- Committing: Nows asked to leave the order uncommitted.

## Changes
- `site/templates/partials/journey.html`: delete the 4-line `{#- The recording is switched off ... (WO-0006). #}` comment (lines 7-10); change line 11 to `{%- set show_recording = true %}`. On the button (line 48) replace `aria-pressed="false"` with `data-playing="false"`. Keep everything else in the recording section as it is (heading, intro, `<video>` attributes, aria-label, caption).
- `site/zola.toml`: delete lines 22-26 (the 4-line `# The donor-journey recording still shows ...` comment and `ignored_static = [...]`). The `ignored_content` line and its comment above stay.
- `site/content/work/donation-website.md`: delete line 45 (the `<!-- Nows: the recording ... -->` comment) and the blank line after it, so one blank line separates the role comment from `## What I will build and run`. In the "How this page moves" table, add before the Reduced motion row: `| Recording | A twelve-second H.264 file that plays while half of it is on screen; the button pauses it | About 40 lines |`, and change the Reduced motion row's middle cell to `The page starts from the finished picture; pinning, autoplay and idle motion are off, and the steps are a plain list`.
- `site/static/js/journey.js`: header comment item 4 becomes one line, `//   4. the recording plays while it is on screen, with a Play/Pause button;` (drop the "it does nothing while switched off" lines 9-10). In `paintVideoButton()` replace `setAttribute("aria-pressed", ...)` with `setAttribute("data-playing", String(playing))`.
- `site/sass/journey.scss`: in the video button rules (around lines 1140-1147) change both `.jn-vbtn[aria-pressed="true"]` selectors to `.jn-vbtn[data-playing="true"]`. No other Sass change.
- `docs/design-system.md` line 14: replace "A short recording of it is switched off for now, until it is re-recorded." with "A twelve-second recording of it plays while it is on screen, with its own Play/Pause button."
- `.claude/skills/design-system/SKILL.md` line 133: replace "The recording that plays while half visible is switched off until it is re-recorded (`show_recording` in `partials/journey.html`, `ignored_static` in `zola.toml`); its CSS and JS stay for it." with "The silent twelve-second recording (`images/journey.mp4`) plays while half visible and loops." and replace "the video, when it returns, has its own Play/Pause button" with "the video has its own Play/Pause button".
- `.claude/skills/ui-ux-review/SKILL.md` line 34: replace the "The recording is switched off ... no request for `journey.mp4`." sentence with: "The recording plays while half visible; Playwright's Chromium can't decode H.264 (shots log `journey.mp4 ERR_ABORTED`), so check playback in Chrome (`C:/Program Files/Google/Chrome/Application/chrome.exe`)."

## Acceptance checks
Standard (run from the repo root):
1. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`: all pass (no Rust changes; they must stay green).
2. `zola --root site build` succeeds with no warnings, and `cargo run -q -p newpost -- --check` reports no problems.

The three edits Nows asked for:
3. `grep -n "show_recording" site/templates/partials/journey.html` prints exactly two lines: `{%- set show_recording = true %}` and `{%- if show_recording %}`. `grep -c "switched off\|WO-0006\|re-made" site/templates/partials/journey.html` prints `0`.
4. `grep -c "ignored_static\|journey" site/zola.toml` prints `0`; `git diff site/zola.toml` shows only the five removed lines (comment + `ignored_static`), and `ignored_content = [...]` is unchanged.
5. In `site/content/work/donation-website.md`: `grep -c "Stored safely\|ignored_static\|show_recording"` prints `0`; `grep -c "<!-- Nows: confirm the exact role title -->"` prints `1`; `grep -c "^draft = true"` prints `1`.
6. After `zola --root site build` (no drafts): `site/public/images/journey.mp4` and `site/public/images/journey-poster.jpg` exist, and `sha256sum` gives `314332fe08ed813a301f182062a56f15ce4fad47d7f210783c82c7c0f55af76b` and `84287171aae382598c71b7b1ea7d32ef3052973aa68bef986fd6e4f4a5b13ee2` (Nows's files from ce9a775, unchanged). `site/public/work/donation-website/` does not exist (still a draft).

No security claim in the recording block:
7. Build with drafts to a scratch folder (`zola --root site build --drafts --output-dir <scratch>/out --force`), then run this on `<scratch>/out/work/donation-website/index.html`:
   ```bash
   node -e 'const h=require("fs").readFileSync(process.argv[1],"utf8");const m=h.match(/<section[^>]*jn-video-title[\s\S]*?<\/section>/);if(!m)throw "no section";const t=m[0].replace(/<[^>]+>/g," ").replace(/\s+/g," ");const al=(m[0].match(/aria-label="?([^">]*)/)||[])[1];console.log(t,"|",al);console.log(/stored safely|encrypt|access|logged|secur|safe|protect|privacy/i.test(t+" "+al))' <file>
   ```
   Expected: the text reads "The whole journey in twelve seconds / A silent recording of the same illustration. It plays while it is on screen; the button pauses it. / Play / Illustration with sample data. The steps above describe what it shows.", the aria-label is "Recording of the same illustration: a donor gives on a phone and the gift is recorded in three systems", and the last line prints `false`. (Planner ran this on a mock of the change: as expected.)
8. The video itself: `ffmpeg -v error -sseof -0.5 -i site/static/images/journey.mp4 -frames:v 1 <scratch>/last.png` and open it, and open `site/static/images/journey-poster.jpg`: the records panel ends with "All three agree / Same gift, same amount, in every system."; no "Stored safely", "Encrypted" or "access limited" anywhere. A 1 fps contact sheet (`-vf "fps=1,scale=640:-1,tile=4x3" -frames:v 1`) shows no other text about security. `ffprobe` gives duration 12.0 s (the heading says twelve seconds) and no audio stream (`ffprobe -v error -select_streams a -show_entries stream=index -of csv=p=0 <mp4> | wc -l` prints `0`; the intro says silent).

On the page (`zola --root site serve --drafts`, `/work/donation-website/`):
9. At 1280 and at 390 px wide, in light and in dark (Theme button): below the walkthrough, a section headed "The whole journey in twelve seconds" shows the poster in a 16:9 frame with a 1 px rule edge, a dark "Play" button with a play icon at its bottom left, and the caption under it in muted text. Nothing overflows sideways at 390.
10. Playback, in Chrome (Playwright's Chromium cannot decode H.264; use `C:/Program Files/Google/Chrome/Application/chrome.exe`): scroll until at least half the video is on screen: it plays, the button reads "Pause" with a pause icon and has `data-playing="true"`. Click it: the video stops, the button reads "Play", `data-playing="false"`; scrolling away and back does not restart it. Click Play, then scroll the video off screen: it pauses. The button has no `aria-pressed` attribute (`document.querySelector(".jn-vbtn").hasAttribute("aria-pressed")` is `false`).
11. With reduced motion emulated: the video does not start on its own; the poster stays; clicking Play plays it. With JavaScript off: the poster shows and the button is not shown (`hidden`).
12. `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs` on the drafts build of `/work/donation-website/`, light and dark: axe reports 0 violations (the `journey.mp4 ERR_ABORTED` log line from Playwright's Chromium is expected, not a failure).
13. Keyboard, at 1280 and at 864 px: Tab from the last walkthrough step reaches the video button, then the page content after it; the button shows the `--ink` ring with the page-coloured band; `document.activeElement` is never `BODY` after a Tab (lesson WO-0006). Enter and Space toggle play and pause.
14. The rest of the journey is unchanged: the hero Pause animation button still stops the journey CSS animations, and the walkthrough still pins and steps (`--scroll "#journey-story:0.05,0.22,0.5,0.77,1"` shots show scene states 1 to 5).

Docs and scope:
15. `grep -rn "switched off\|when it returns\|no request for" docs/design-system.md .claude/skills/design-system/SKILL.md .claude/skills/ui-ux-review/SKILL.md site/static/js/journey.js` prints nothing.
16. The "How this page moves" table on the drafts page has a "Recording" row (between "Reading line" and "Reduced motion"), and the Reduced motion row mentions autoplay.
17. `git diff --quiet site/static/_headers` exits 0 (CSP unchanged: `default-src 'self'` covers same-origin media); the built drafts page has no `style=` attribute and no inline `<script>` other than `application/ld+json`.
18. `git status --short` lists only the nine files in Changes (plus `.factory/`, the untracked `design/`, and agent-memory folders); nothing under `site/static/images/` is modified.

## Risks
- **Truthfulness (the main risk here).** The old recording claimed records were "Stored safely: encrypted, with access limited and logged", which Nows does not claim. Planner checked the new files: the last frame and the poster end with "All three agree / Same gift, same amount, in every system.", and the 1 fps contact sheet shows no security text. One thing to look at: a small **padlock icon** still sits beside "All three agree". It makes no claim in words, but a visitor may read a lock as "secure". It is in Nows's supplied file, so the reviewer should mention it for Nows (low at most) rather than block on it. The video also shows sample data (the donor "Rina", Rp150.000, DN-2047, rina@example.com); the live scene shows the same, and the caption labels it "Illustration with sample data" (Nows's WO-0005 decision).
- **Deployment.** With `ignored_static` gone, the next push deploys `/images/journey.mp4` and `/images/journey-poster.jpg` even though the page is a draft (drafts do not hide static files). That is what Nows asked for; older files with the same names were deployed before WO-0006. The approver should say so in the hand-off.
- **Accessibility.** The video loops endlessly once playing, so its own Play/Pause button is the WCAG 2.2.2 control: check it is reachable, visible over every frame (focus band), and that Space and Enter work. The hero Pause animation button does not stop the video, by design. Under reduced motion nothing may autoplay.
- **Button state.** The rename from `aria-pressed` to `data-playing` must happen in all three files (template, JS, both Sass selectors) or the icon stops swapping; check 10 tests it. The changing label ("Play" / "Pause") is the one state signal for screen readers, as on the hero button.
- **Design.** No new styles; the recording section is the layout reviewed in WO-0005. Look at the light poster on the dark theme, framed by `--rule`.
- **Security.** No new code paths and no CSP change; the video is same-origin. The security checker should confirm `_headers` is unchanged and the media files match the hashes in check 6.

## Skills the builder needs
zola-site, design-system, cms-content, ui-ux-review (for checks 9-14)
