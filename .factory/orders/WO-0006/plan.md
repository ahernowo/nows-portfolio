# Plan: Reframe the donation case study as an honest plan

## Goal
A visitor to `/work/donation-website/` (still a draft) reads a plan, not a result: Nows starts on a donation website in November 2026 (development, documentation and monitoring), with the tools Nows named. The invented donor, reference and amounts appear only inside the illustration, which says "Illustration with sample data." beside it. The "What changed" bars are gone from the code, and the recording that shows "Stored safely: Encrypted, with access limited and logged" is neither shown nor deployed.

Base: WO-0005 is committed (HEAD `40eac14`) and pushed (`main...origin/main`, no ahead count), so the diff for this order is against HEAD. `design/` is the untracked outside package: leave it alone.

## Scope
In:
- Front matter: description, year, role, status, stack, the step-2 text, and comments.
- Body: a "What I will build and run" section from Nows's facts only; the "How this page moves" table without the bars and recording rows; the `<!-- Nows -->` markers.
- Hero: a visible "Illustration with sample data." line by the scene. At narrow widths the Pause animation button moves above the scene, because the filled-in form pushes the scene down (see Risks).
- Walkthrough intro: drop "Rina"; say "Illustration with sample data."
- Remove the before-and-after bars feature completely: the template block, the JS, the Sass, the Decap field, and the docs and skills that describe it.
- Hide the recording: the video block is switched off in the partial, and `journey.mp4` and `journey-poster.jpg` are left out of the build with `ignored_static`. The files stay byte-identical in the repo for Nows.

Out:
- Editing, re-recording or deleting `journey.mp4` / `journey-poster.jpg`: the request forbids it. Follow-up for Nows.
- The video's CSS (`.jn-video*`, `.jn-vbtn*`) and JS (section 4 of `journey.js`). It already does nothing when the video element is missing (`if (video && videoButton)`, line ~396), so it stays for the re-recorded file. Only its comments change.
- Moving the prose above the walkthrough: that's a template layout change. The hero's description and the Status "Starting soon" carry the framing at the top.
- The home receipt stack (DN-2041, Rp150.000): another page, and part of the site's design.
- The `newpost --work` starter's "## What changed" heading: it serves every future case study.
- WO-0005's other open low findings (#1 aria-pressed on the video button; #5 the dark video button, moot while the video is hidden; #6 small panel text). #7 (the Pause button at the fold at 390) is handled here, because this order makes it worse.
- Publishing: `draft = true` stays.

## Changes
- `site/content/work/donation-website.md`, front matter (other keys unchanged):
  - `description = "What I will build and run on a donation website from November 2026: its development, documentation and monitoring."`
  - Line 7 comment: `# Stays a draft until Nows confirms the role title below.` Keep `draft = true`.
  - `year = "2026"` (from "November 2026"). `role = "Donation website development, documentation and monitoring" # Nows: confirm the exact role title`. `status = "Starting soon"`. `stack = ["Salesforce (NPSP)", "Payment gateway", "Power BI and Excel automation", "InDiGO"]`. Keep Nows's wording: don't split "Power BI and Excel automation", and don't explain InDiGO.
  - Journey comment (lines 17-20): drop the before-and-after sentence. Say the people, amounts and references are sample data and live only inside the illustration.
  - Step 2 body: `She picks an amount and keeps the monthly option.` No Rp outside the scene. Other steps unchanged.
- `site/content/work/donation-website.md`, body: replace the old `<!-- Nows -->` block (lines 44-51) and edit the table. The body becomes, in order:
  1. `<!-- Nows: confirm the exact role title -->` (verbatim)
  2. `<!-- Nows: the recording (site/static/images/journey.mp4 and journey-poster.jpg) still shows the "Stored safely" line about how records are kept, which this page does not claim. It is hidden from this page and left out of the build (ignored_static in site/zola.toml, show_recording in partials/journey.html) until it is re-recorded without that line. -->`
  3. `## What I will build and run`, then these four paragraphs:
     - From November 2026, I will work on a donation website: its development, its documentation and its monitoring.
     - My first responsibilities will be SOPs (standard operating procedures) and monitoring tools.
     - I will work with Salesforce (NPSP), the payment gateway, Power BI and Excel automation, and InDiGO.
     - There are no results yet, so this page describes the plan, not an outcome. The illustration above follows one gift with sample data; it is not a record from the website.
  4. `## How this page moves`, with its intro sentence unchanged. Table changes: the row "Reading line and before-and-after bars" becomes `| Reading line | A CSS scroll timeline | None |`; delete the "Recording" row; in "Reduced motion", "pinning, autoplay and idle motion are off" becomes "pinning and idle motion are off". The other rows stay as they are, seven rows in all.
- `site/templates/work-page.html`: in `.jn-hero-fig`, after the scene component and before the button, add `<p class="jn-sample">Illustration with sample data.</p>`. The button markup stays the same.
- `site/templates/partials/journey.html`:
  - Top comment: drop the bars clause.
  - Walkthrough intro `<p>`: `Follow one gift from a donor's phone to the records behind it. Illustration with sample data.`
  - Add `{%- set show_recording = false %}`, with a Tera comment saying why: the file shows a line this page no longer makes, and it is re-enabled together with `ignored_static`. Wrap the whole video `<section>` in `{%- if show_recording %} ... {%- endif %}`. Mind the nested `{#- ... #}` comment: use `if`, not a Tera comment around it.
  - Delete the `{%- if page.extra.journey?.compare %}` section (lines 53-69).
- `site/sass/journey.scss`: delete the bars block (comment at 1127 through `.jn-fill-after`, about lines 1127-1179). Delete the `@supports (animation-timeline: view())` `.jn-fill` block inside the motion media query (about 1265-1271) and `@keyframes jn-fillin`. Add `.jn-sample` (margin 0, `--step--1`, `--ink-soft`, full opacity). Lay out `.jn-hero-fig` as a grid in DOM order scene, note, button:
  - Above 54rem: the scene spans the row; under it, the note sits left and the button right on one line. The row is as wide as the scene (max 520px, aligned with its right edge, as today). Move the button's `margin-top: 0.75rem` into that layout.
  - At 54rem and below (the existing media query): the button sits above the scene, right-aligned to the scene's edge, and the note sits under the scene, left-aligned. When the button is `hidden` (no JS, reduced motion), it leaves no empty band.
  - Colours are tokens only; no raw values in `journey.scss`.
- `site/static/js/journey.js`: delete section "5. The bars" (lines ~423-433) and item 5 of the header comment. Item 4 of the header gets a note: it does nothing while the recording is switched off.
- `site/static/admin/config.yml`: delete the `compare` list field (lines ~124-134). The comment at 106-107 drops "bars show only when compare rows exist".
- `site/zola.toml`: under `ignored_content`, add a commented line `ignored_static = ["images/journey.mp4", "images/journey-poster.jpg"]`. The comment says the recording shows a line the page no longer makes and is left out until re-recorded (WO-0006). I tested this on Zola 0.23.6: both files disappear from the output.
- `.claude/skills/design-system/SKILL.md`: line 57, drop the "after" bar clause. Line 84, delete the bars contrast row. Line 134, drop the bars, and say the recording is switched off until it is re-recorded.
- `.claude/skills/ui-ux-review/SKILL.md`: line 34, delete the sentence about the "What changed" bars and the `--root` copy for them, and add that the recording is switched off. Line 54, drop "and the journey's "after" bar".
- `docs/design-system.md` line 14: drop "and before-and-after bars grow", and say the recording is switched off for now.

## Acceptance checks
Before building, take baseline shots into `target/shots/WO-0006-before/` (same command as check 12). The dev server on :1111 belongs to the supervisor: don't start another on that port. A `zola.toml` change may need that server restarted to show.

1. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` all pass. No Rust changes, so these only confirm nothing broke.
2. `zola --root site build` succeeds with no warnings. `cargo run -q -p newpost -- --check` reports no errors.
3. Still a draft: `grep -c '^draft = true' site/content/work/donation-website.md` gives 1, and after check 2's build, `site/public/work/donation-website/` doesn't exist.
4. Recording not deployed: after `zola --root site build --drafts`, neither `site/public/images/journey.mp4` nor `site/public/images/journey-poster.jpg` exists. Both source files are unchanged: `sha256sum site/static/images/journey.mp4 site/static/images/journey-poster.jpg` gives `64dbd92f0f3d8bca34da948badf06842642f694bcda09cfc339ae185b973c7c1` and `0974e07c57a8397381d15acb4ef904c12fb563916fa27d200bcde72fc0aa2350`, and `git status --short site/static/images` prints nothing.
5. Sample data stays inside the scene. On the drafts build, run:
   `node -e 'const h=require("fs").readFileSync(process.argv[1],"utf8");const t=h.replace(/<figure[^>]*jn-room[\s\S]*?<\/figure>/g,"");console.log("outside rooms:",t.match(/Rina|DN-\d+|Rp\s?\d/g)||[]);console.log("rooms:",(h.match(/<figure[^>]*jn-room/g)||[]).length);console.log("visible notes:",(t.match(/Illustration with sample data\./g)||[]).length);console.log("video/claim:",h.match(/<video|journey\.mp4|journey-poster|[Ee]ncrypt|access limited|What changed/g)||[])' site/public/work/donation-website/index.html`
   Expected: `outside rooms: []`, `rooms: 2`, `visible notes: 2`, `video/claim: []`. On today's tree it prints `[ 'Rina', 'Rp1' ]`, 2, 1, and three video hits.
6. No "Stored safely" claim in the source: `grep -rn -i "encrypt\|access limited\|logged" site/templates site/static/js site/sass site/content` finds nothing.
7. The bars are gone: `grep -rn -i "compare\|jn-crow\|jn-track\|jn-fill\|fillin\|before-and-after\|What changed" site/templates site/sass site/static/js site/static/admin/config.yml site/content/work/donation-website.md` finds nothing. `grep -n -i "before-and-after\|\"after\" bar\|What changed\" bars\|What changed\" block" .claude/skills/design-system/SKILL.md .claude/skills/ui-ux-review/SKILL.md docs/design-system.md` finds nothing.
8. Front matter (read the file): role, status, year and stack are exactly as in Changes. The role line carries `# Nows: confirm the exact role title`, and the body holds `<!-- Nows: confirm the exact role title -->` verbatim plus the recording marker. `grep -c "<!-- Nows" site/content/work/donation-website.md` gives 2.
9. Copy (a reviewer reads `donation-website.md` against the request's facts): every sentence about Nows is present or future tense and uses only these facts: November 2026; development, documentation and monitoring; SOPs and monitoring tools first; Salesforce (NPSP), the payment gateway, Power BI and Excel automation, InDiGO. There are no numbers, timings, user counts or outcomes. The first prose h2 is "What I will build and run". "How this page moves" has seven rows: no bars, no Recording, no "autoplay".
10. Hero at 1440, light and dark: the form shows Role, Year, Status and Built with ("Salesforce (NPSP), Payment gateway, Power BI and Excel automation, InDiGO"). Under the scene, one row has "Illustration with sample data." on the left in muted text and the Pause animation button on the right, both inside the scene's width.
11. Hero at 390, light and dark: the button sits directly above the scene, right-aligned to the scene's edge, with its bottom no more than 16 px above the scene's top. The note sits directly under the scene. Nothing scrolls sideways. Under reduced motion (and without JS) the button is gone, and the gap between the form and the scene equals the hero grid gap, with no extra empty band. Pressing Pause animation (real Chrome or Playwright, 1440) still changes its label to "Play animation" and holds the breathing and blinking. Tab order: crumb, Pause animation, step buttons.
12. Shots and scan: `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --drafts --reduced --focus --out target/shots/WO-0006 --pages /work/donation-website/,/work/ --scroll "#journey-story:0.05,0.5,1"` covers 390 and 1440, light and dark. Expected: axe 0 failures, no console errors, no CSP violations, no sideways scroll, and no failed requests. The video is gone, so the `journey.mp4` aborts in WO-0005's runs must not appear. `/work/` lists the draft with the new description.
13. Walkthrough at 390 and 1440: the intro reads "Follow one gift from a donor's phone to the records behind it. Illustration with sample data." Step 2 reads "She picks an amount and keeps the monthly option." The pinned story still moves through all five steps (scroll shots 0.05 / 0.5 / 1). Under reduced motion all five steps are listed.
14. Below the walkthrough come straight away the prose ("What I will build and run", then "How this page moves") and the pager. There is no video section and no "What changed" heading, at either width or in either theme.
15. Contrast: `.jn-sample` is `--ink-soft` on `--paper` at full opacity: 6.34:1 light, 8.63:1 dark. axe covers it; confirm by reading the Sass.
16. Diff scope: `git status --short` lists only the ten files in Changes, plus `.factory/` and agent memories. `design/` stays untracked and unchanged.

## Risks
- **Truthfulness.** All copy comes from the request's facts. "Year: 2026" comes from "November 2026", and "Payment gateway" from "the payment gateway". Reviewer: check each sentence in the body and the description against the facts list. Nothing should say the website exists, has users, or produced anything. The illustration's step 5 describes the picture ("each hold a record"), not Nows's system. It is presented as sample data in four places.
- **The media was already public.** `40eac14` is on origin/main, so Cloudflare Pages has served `/images/journey.mp4` and `/images/journey-poster.jpg` (unlinked) since that push. `ignored_static` drops them from the next deploy. They stay in git history, and caches may keep them for a while. Security/approver: say this to Nows plainly. Nothing secret is in them; the issue is the unconfirmed "Stored safely" claim.
- **Two switches for one recording.** `show_recording` in the partial and `ignored_static` in `zola.toml` must be flipped together when the recording returns. The Nows marker names both. Reviewer: check both comments say so.
- **Pause button (WCAG 2.2.2, lesson WO-0005).** The filled-in form pushes the 390 scene below the first screen; the mock-up shot puts the scene's top at about 810 px. The hero's motion starts on load, so the control must arrive with the scene. Placing it above the scene at narrow widths keeps them together. Design: check 11, plus a look at 641-864 px, where the layout switches.
- **Dormant video code.** Its CSS and JS stay on purpose for the re-recorded file. Security can confirm it does nothing without the `<video>` element.
- **Decap.** Removing the `compare` field is safe: no content uses it, and the stage B CMS and cms-core have no journey fields (grep: none).
- **Supervisor note mismatch.** WO-0005 is not uncommitted: it is HEAD `40eac14`. Diffs are against HEAD.

## Skills the builder needs
zola-site, design-system, cms-content, ui-ux-review (for the shots)

## Remaining `<!-- Nows -->` gaps after this order (for the build report)
1. Confirm the exact role title (front matter comment and body marker).
2. Re-record `journey.mp4` and `journey-poster.jpg` without the "Stored safely" line. Then set `show_recording = true` and remove the two files from `ignored_static`.
