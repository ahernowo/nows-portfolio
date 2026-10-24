# Build: Port the donor-journey motion to the work page

## What changed
A new draft case study, `/work/donation-website/`, switches the donor journey on from its own front matter (`[extra.journey]`, five steps, no compare rows). `work-page.html` gets a journey branch: a two-column hero with the animated scene and a **Pause animation** button, then the pinned five-step walkthrough, the 12-second recording with Play/Pause, the "What changed" bars (shown only once `[[extra.journey.compare]]` rows exist), the Markdown "How this page moves" table and the usual pager. The scene is server-rendered in a `journey_scene` Tera component in its finished state, so the page is complete without JavaScript; `js/journey.js` (one IIFE, no `innerHTML`/`eval`/`fetch`) drives it from one progress number, and `journey.scss` → `/journey.css` styles it from tokens only (`--ok`, `--ok-tint` and the `--jn-*` illustration group, all new). Case studies without a journey render byte-for-byte as before (only cachebust hashes differ).

**Departure from the plan (Nows's later decision, overrides the 5 s cap):** the endless breathing and blinking and the full 5.9 s hero beat (0.7 s wait + 5.2 s) are kept as mocked, and the hero has a visible pause control for WCAG 2.2.2. It is a real `<button class="jn-pause" type="button">` under the hero scene, keyboard reachable, label switching "Pause animation" / "Play animation" (plus icon swap; no `aria-pressed`, so the label alone carries the state), site focus ring. Pressing it cancels the JS beat (rAF) and sets `.jn-paused` on `<html>`, which puts every CSS animation in the journey (breathing, blinking, title rise, scene fade-up, caret, spinner, in both scenes) on `animation-play-state: paused`; pressing again resumes from the same frame. It ships `hidden` and `journey.js` shows it only when motion is allowed; under reduced motion it stays hidden (CSS backs this up) because nothing moves. The looping breath and blink run only on `.jn-room.is-live` (set by JS), so without JavaScript nothing loops that the button couldn't stop. Evidence under Acceptance checks 13.

## Files
```
 M .claude/skills/design-system/SKILL.md   --ok/--ok-tint rows, --jn-* group, contrast rows, accent "after" bar, second motion moment with the pause
 M .claude/skills/ui-ux-review/SKILL.md    accent + motion lines, 2.2.2 pause line, --scroll/--root rows, how to see the bars via a test copy
 M docs/design-system.md                   Motion bullet names the journey; --ok/--ok-tint token rows
 M site/sass/_tokens.scss                  only additions: --ok, --ok-tint, --jn-* (both themes where they differ)
 M site/static/admin/config.yml            Decap: extra.journey (steps min/max 5, compare rows)
 M site/static/js/head.js                  pagereveal with a view transition adds .vt-arrival (skips the title rise)
 M site/templates/components.html          new journey_scene component (finished-state markup)
 M site/templates/work-page.html           journey branch; the non-journey branch is the old markup unchanged
 M tools/shots/serve.mjs                   .mp4 content type
 M tools/shots/shoot.mjs                   --root (argument array, no shell) and --scroll
?? site/content/work/donation-website.md   the draft case study, Nows markers, How-this-page-moves table
?? site/sass/journey.scss                  the scene and page styles
?? site/static/images/journey-poster.jpg   byte copy
?? site/static/images/journey.mp4          byte copy
?? site/static/js/journey.js               scene, hero beat + pause, story, video, bars
?? site/templates/partials/journey.html    walkthrough, video, bars sections
```
(`.claude/agent-memory/planner/MEMORY.md`, `.factory/`, `design/` were already modified/untracked before this build.)

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok (no Rust changed)
- `cargo test --workspace`: 65 passed, 0 failed
- `zola --root site build`: ok, 5 pages, no warnings
- `zola --root site check --skip-external-links --drafts`: ok, 9 pages
- `cargo run -q -p newpost -- --check`: "Checked 7 files. No problems."
- No wasm/worker changes.

## Acceptance checks
1. Pass: the standard checks above.
2. Pass: drafts build has `work/donation-website/index.html`; the normal build's `work/` holds only `index.html`, `pdf-merger`, `this-website` (and no `donation-website` in `search_index.en.json`). `grep -c '^draft = true'` → 1; `grep -c '<!-- Nows:'` → 1.
3. Pass: the `grep -c 'jn-\|journey'` loop prints 0 four times. Also built HEAD (`git archive HEAD site`) and compared HTML with cachebust hashes stripped: this-website, pdf-merger, bank-reconciliation, articles and home are identical; donation-dashboard differs only by its new "Next: Donation website" pager link (drafts build only).
4. Pass: `git diff --quiet -- site/static/_headers` exits 0; `grep -c 'style="'` → 0; only `<script type=application/ld+json>` lacks `src`; the journey.js forbidden-pattern grep prints nothing; no `googleapis`/`gstatic` under `site/`.
5. Pass: hex/rgb grep on `journey.scss` prints nothing; `git diff site/sass/_tokens.scss` has 0 removed lines (39 added).
6. Pass: `sha256sum` gives `64dbd92f...c7c1` (mp4) and `0974e07c...2350` (poster), as in the plan.
7. Pass: sample-text grep → 0 (the `<!-- Nows -->` block is stripped by `minify_html`); "Illustration with sample data" present (hero, story, figcaption); `jn-fill` → 0.
8. Shots: `target/shots/WO-0005/` (command as in the plan). Axe: 0 failures. Console/CSP: none. Sideways scroll: none. Failed requests: 8 × `/images/journey.mp4 (net::ERR_ABORTED)`: Playwright's Chromium cannot decode H.264 and drops the request; the same server plays it in real Chrome (check 11), so this is tooling, not a defect. `/` and `/work/this-website/` are pixel-identical to `target/shots/WO-0004/`; `/work/` is 130 px taller in the drafts build because the new draft row is listed.
   - 1440 light: two-column hero, scene about 520 px wide, square 4 px corners, Pause animation button under the scene, site header and pager. The full-page shot catches the hero mid-beat (shots settle at 1.9 s).
   - 390: one column; the scene under the dek (no form fields exist yet), pause button centred under it, no overlap.
   - Scroll 1440: 0.05 lock screen waking (dimmed: the screen is still turning on at 0.05, as mocked) with the notification and step 1; 0.22 amount chosen, step 2; 0.5 form typing with keyboard up, step 3; 0.77 Pay screen with the thumb on Pay, step 4 (the spinner starts at 0.782, so 0.77 shows no spinner); 0.97 person moved left, panel right, step 5, **records mid-save** ("Saved / Saving / Saving", no stamp yet: in the mock-up's timing the rows finish at 0.987 and the stamp lands at 0.992-1). Shots at 1.0 (`target/shots/WO-0005-bars/*-scroll-1.png`, `target/shots/WO-0005-final/`) show three green Saved/Confirmed/Saved rows and the stamp; no "Stored safely" line anywhere in the live scene. Because of this I set step 5's button target to **1** instead of 0.97, so Enter on step 5 lands on the finished picture.
   - Scroll 390: same scene states, "Step n of 5" plus only the current step under the scene; the panel sits at the bottom of the room.
   - Dark: page `#1F1B1C`, panel `#2C2527`, green `#7FD0A5`, stamp `#E8546D`, phone screen white, lighter shirt.
   - Reduced: story not pinned (page 3911 px vs 5767 px at 1440), all five steps listed and open, story scene finished, hero rests on the chosen amount, video poster with "Play", no pause button.
9. Pass: test copy in my scratchpad with two rows, `--root <copy> --drafts --reduced` → `target/shots/WO-0005-bars/`. Two labelled rows; the "before" bar fills the track in grey (`--rule-strong`), the "after" bar is about 6% wide in the accent; both themes; reduced shots show both bars at final length; axe 0; no console/CSP errors. `git status site/content` shows only the new file (repo untouched).
10. Pass (scratch Playwright script, drafts build on port 4733): after the pause button, Tab goes step 1 → 2 → 3 → 4 → 5 → video button, each with a 2 px solid outline (`target/wo5crops/step3-focus.png`). Enter on step 3 smooth-scrolled and set `aria-current="step"` on step 3 only. Under reduced motion all step buttons are `disabled` and absent from the Tab order (stops go: skip link, header, crumb, video button, heading anchor, pager).
11. Pass in real Chrome (Playwright driving `C:/Program Files/Google/Chrome/Application/chrome.exe`): video plays when scrolled half into view (t 1.51 s, label "Pause", `aria-pressed=true`), pauses when scrolled away, resumes on return; after pressing Pause, away-and-back keeps it paused; Enter on the button plays it. Reduced motion: never starts by itself; pressing Play plays it. Design review may still want a hand check in Edge.
12. Pass: direct load runs the `jn-rise` animation and `<html>` has no `vt-arrival`; clicking "Donation website" on `/work/` gives `vt-arrival` and no rise animation on the title.
13. **Changed by Nows's decision** (no 5 s cap; `grep -n infinite` lists breathe, blink, caret, spinner). Instead, the pause evidence: before pressing, hero animations `jn-breathe:running, jn-blink:running`; the button is reached by Tab (10th stop) and shows the 2 px `--ink` ring (dark: `target/wo5crops/pause-focus-dark.png`); Enter → label "Play animation", `data-paused=true`, `<html class="jn-paused">`, animations `jn-breathe:paused, jn-blink:paused`; the beat state (phone lift, arm end, chip) and a hero screenshot are identical 2.5 s later (pixel compare: same). Space resumes: animations running again and the beat finishes (chip selected). Unpaused at rest, the hero keeps moving (two shots 1.3 s apart differ), as Nows chose.
14. Pass: JavaScript disabled, 1440 and 390 (`target/wo5crops/nojs-*.png`): both scenes in the finished state (thank-you screen, three green rows, stamp), five steps with text, poster, pause and video buttons hidden, no sideways scroll, and no animation running except the scroll-driven reading line. For design review: in the portrait hero room the panel layers over the lower phone, as the mock-up's narrow layout does at p = 1.
15. Pass: http://127.0.0.1:1111/admin/ loads to "Login with GitHub" with no configuration error. The only console error is `localhost:8081/api/v1` refused, Decap's local-backend probe, which happens without this change too. `steps` also has `required: false`, so other case studies (journey empty) still save; min/max 5 apply once there are steps.
16. Pass: the grep shows the second moment, `--ok` / `--ok-tint`, the accent and motion lines, and the 2.2.2 pause line.

## Notes for reviewers
- **Truthfulness.** Steps, description and table describe only the illustration and the code. The video and poster still show "Stored safely: Encrypted, with access limited and logged", which the live scene drops; this is flagged in the `<!-- Nows -->` block (Nows: re-record or confirm). The page stays a draft. The year, role, status and stack are empty with `# Nows:` comments; the empty form box is not rendered on the journey page.
- **Video button**: it follows the plan (label "Play"/"Pause" and `aria-pressed`), which a screen reader announces as "Pause, toggle button, pressed" while playing. The hero pause uses a label change only. Design may want the video button to match (drop `aria-pressed`); I left it as planned.
- **Small departures**: step 5 target 1 (not 0.97), explained in check 8; the hero scene label is "a donor gives a monthly gift to a charity on her phone." so it's true both at rest (JS) and finished (no JS).
- Look at: `/work/donation-website/` at 1440 and 390, both themes, motion and reduced; the scroll shots; the pause button's placement under the hero scene; `target/wo5crops/` for the scratch evidence. Pinning breaks if any ancestor of `.jn-story` gets `overflow: hidden`.
- Shots tool: `--scroll` skips pages without the selector and runs after the axe scan (axe sees the page at the top).
