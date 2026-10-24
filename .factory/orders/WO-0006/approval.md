# Approval: Reframe the donation case study as an honest plan

## Verdict
Approved. The draft now reads as a plan built only from Nows's facts, the sample data stays inside the illustration with a visible note, the bars are gone everywhere, and the recording is neither shown nor built. Two low findings stay open (#1, #2). One older keyboard problem from WO-0005 turned up in my own run; it is not caused by this order and is listed below as a follow-up.

## Checks I ran
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 7 files. No problems."
- `zola --root site build`: ok, 5 pages, no warnings; `site/public/work/` holds no donation-website and `site/public/images/` no journey.* file
- `zola --root site build --drafts --output-dir <scratchpad>/drafts --force`: no journey.* in images/; plan check 5 node script prints `outside rooms: []`, `rooms: 2`, `visible notes: 2`, `video/claim: []` (I added "Stored safely" to the pattern: none)
- `sha256sum` of journey.mp4 / journey-poster.jpg: 64dbd92f...73c7c1 / 0974e07c...aa2350, as planned; `git status --short site/static/images` empty
- Plan greps 6 and 7 (encrypt/access limited/logged; compare/jn-fill/before-and-after/What changed, plus the skills and docs): nothing found
- `grep -c "<!-- Nows"` = 2; `grep -c '^draft = true'` = 1
- Own Playwright (real Chrome) against the supervisor's :1111 drafts server, 390 and 1440, light and dark (theme set through localStorage, since the site ignores the OS scheme), motion / reduced / no-JS: 0 failed requests, 0 console errors, 0 sideways overflow, no `<video>`. Hero shots in the scratchpad (`shots/t-hero-390-dark.png`, `t-hero-1440-light.png`, opened).
- `factory verify WO-0006` after my runs: every gate passes, seal intact.

## Request against the change
1. Plan framing: done. Description, "What I will build and run" and four future/present-tense paragraphs, each from the facts list (November 2026; development, documentation and monitoring; SOPs and monitoring tools first; the four tools). No results, numbers, timings or user counts.
2. Bars removed: done in template, JS, Sass (incl. `jn-fillin`), Decap `compare` field, skills, docs, and the "How this page moves" table (bars row reduced to "Reading line", Recording row and "autoplay" gone; seven rows).
3. Built with: the four tools as Nows named them. Role uses the fallback wording with `# Nows: confirm the exact role title` and the verbatim body marker.
4. Rina, DN-2047 and Rp amounts only inside the two scenes (step 2 now "She picks an amount..."); "Illustration with sample data." under the hero scene and in the walkthrough intro.
5. The claim is in the mp4, so the video block is switched off (`show_recording = false`) and both files are left out of the build (`ignored_static`); files byte-identical. Follow-up for Nows recorded in the second marker.
6. `draft = true` kept. Remaining gaps: (a) confirm the role title; (b) re-record journey.mp4 and its poster without "Stored safely", then flip `show_recording` and remove the two `ignored_static` entries together.
Beyond the request, from the plan: the Pause button moves above the scene at 54rem and below (WO-0005 lesson on WCAG 2.2.2); the `.jn-hero-fig` column gap is 0.75rem (disclosed by the builder). Nothing in the diff goes beyond the plan.

## Acceptance checks
1. Pass: fmt, clippy, 65 tests (my run).
2. Pass: zola build, newpost --check (my run).
3. Pass: draft count 1; no built page after the plain build (my run).
4. Pass: no journey.* in either build; hashes match; images clean (my run).
5. Pass: node script output as expected (my run, drafts build).
6. Pass: grep empty (my run).
7. Pass: both greps empty (my run).
8. Pass: front matter read in `git diff`; markers = 2 (my run).
9. Pass: I read every sentence of the dek, form and prose against the facts list (see above); design report confirms the walkthrough text.
10. Pass: 1440 light and dark, scene x 783-1296, note 783-1123, button 1135-1296 on one row; form reads Role / 2026 / Starting soon / the four tools (my Playwright).
11. Pass: 390: button y 774-818, scene top 830 (12 px), right edges 370/370; note 12 px under the scene; reduced and no-JS: no button, scene 28 px under the form. Pause: label becomes "Play animation", only the scroll-driven `reading-progress` still runs, hero crops 2.5 s apart differ by 4 of 295k pixels at 1440 light (anti-aliasing) and 0 elsewhere; second press restores "Pause animation". Tab order crumb, Pause, steps: holds at 865 px and up; see the note below for 864 px and down.
12. Pass: design's shoot.mjs run (axe 0, no problems, no mp4 aborts) plus my own 0 failed requests; drafts /work/ carries the new description.
13. Pass: design report scroll shots; intro and step 2 wording confirmed in the source.
14. Pass: h2 list on the page is "One gift, from the donor's side", "What I will build and run", "How this page moves" (my Playwright); no video, no "What changed".
15. Pass: `.jn-sample` computes to rgb(107,85,87) light and rgb(194,182,179) dark, no opacity rule in the Sass (my run): 6.34 / 8.63:1.
16. Pass: `git status --short` shows the ten planned files plus agent memories and `.factory/`; `design/` untracked.

## Older problem found (WO-0005, not this order)
At 864 px wide and below, Tab from "Pause animation" to step 1 drops focus to the page body: focusing step 1 scrolls the page, the story moves to step 2, and the narrow layout hides every step except the current one (`journey.scss`, `.jn-story.is-live .jn-steps li:not(:has(...aria-current))` display none), so the focused button disappears. A keyboard visitor on a narrow window loses their place. I reproduced the same thing on HEAD 40eac14 served from a scratch copy (390, 700 and 864 px), so this order neither caused nor worsened it, and its fix is outside this plan. Not logged against WO-0006; suggested as its own order (keep focusable steps visible, or move `aria-current` with focus).

## For Nows
Read first in `git diff`: `site/content/work/donation-website.md` (all the words), `site/templates/partials/journey.html` (the `show_recording` switch), `site/zola.toml` (`ignored_static`), then `site/sass/journey.scss` (hero layout). The skills, `docs/design-system.md` and `config.yml` only remove the bars.
Try by hand: `zola --root site serve --drafts`, open /work/donation-website/ at phone width and desktop, press Pause animation.
Before publishing: confirm the role title; re-record the video; consider design #2 (the /work/ intro says "Systems I've built"). `journey.mp4` and its poster are live (unlinked) since 40eac14; the next deploy removes them, git history keeps them.
Not part of the commit: `design/` (outside package). Agent memories and `.factory/` are the line's own files.
Suggested commit message: `Reframe the donation case study as a plan`
