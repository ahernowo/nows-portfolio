# Build: Reframe the donation case study as an honest plan

## What changed
`/work/donation-website/` (still a draft) now reads as a plan: Nows starts on a donation website in November 2026 (development, documentation and monitoring), with SOPs and monitoring tools first, and the four tools Nows named in "Built with". Rina, DN-2047 and the Rp amounts appear only inside the two illustrations; a visible "Illustration with sample data." line sits under the hero scene and in the walkthrough intro. The "What changed" bars feature is gone from template, JS, Sass, Decap and the docs. The recording is switched off in the partial (`show_recording = false`) and left out of the build with `ignored_static`; the mp4 and poster are byte-identical in the repo. At 54rem and below, the Pause animation button now sits above the hero scene.

## Files
```
 M .claude/skills/design-system/SKILL.md   bars removed from accent rule, contrast table and motion; recording "switched off"
 M .claude/skills/ui-ux-review/SKILL.md    bars recipe removed; says the recording is switched off
 M docs/design-system.md                   motion line: no bars, recording switched off for now
 M site/content/work/donation-website.md   front matter (description, year, role, status, stack, step 2, comments); new body
 M site/sass/journey.scss                  .jn-hero-fig grid (scene / note + button), .jn-sample, bars and jn-fillin removed
 M site/static/admin/config.yml            Decap `compare` field and its comment removed
 M site/static/js/journey.js               section 5 (bars) removed; header item 4 notes the recording is off
 M site/templates/partials/journey.html    new walkthrough intro; show_recording switch around the video; bars section removed
 M site/templates/work-page.html           <p class="jn-sample">Illustration with sample data.</p> in the hero figure
 M site/zola.toml                          ignored_static for journey.mp4 and journey-poster.jpg, with the why
```
Also changed, not part of the code: `.factory/` (CLI), `.claude/agent-memory/planner/MEMORY.md` (planner's, already modified before this build). `design/` untouched and untracked.

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, 0 warnings
- `cargo test --workspace`: 65 passed, 0 failed
- `zola --root site build`: ok, no warnings (5 pages)
- `zola --root site check --skip-external-links`: ok
- `cargo run -q -p newpost -- --check`: "Checked 7 files. No problems."
- `node --check site/static/js/journey.js`: ok
- Line endings: all edited files are LF like HEAD (checked byte counts against `git show HEAD:<file>`).

## Acceptance checks
1. Pass: fmt, clippy, tests as above.
2. Pass: zola build and newpost --check as above.
3. Pass: `grep -c '^draft = true'` gives 1; after the plain build `site/public/work/donation-website/` does not exist.
4. Pass: after `zola --root site build --drafts`, `site/public/images/` holds no journey.* file. `sha256sum` gives `64dbd92f...73c7c1` (mp4) and `0974e07c...aa2350` (poster), as in the plan; `git status --short site/static/images` prints nothing.
5. Pass: the node script prints `outside rooms: []`, `rooms: 2`, `visible notes: 2`, `video/claim: []`.
6. Pass: the encrypt / access limited / logged grep finds nothing.
7. Pass: both bars greps find nothing.
8. Pass: role, status, year, stack exactly as planned; role line carries `# Nows: confirm the exact role title`; `grep -c "<!-- Nows"` gives 2 (role marker verbatim, recording marker naming both switches).
9. For design/approver to judge copy. Every body sentence is future or present tense and uses only the request's facts; no numbers except "November 2026" and the year. First prose h2 is "What I will build and run"; "How this page moves" has seven rows, no bars, no Recording, no "autoplay".
10. Pass (`target/shots/WO-0006/hero-1440-light.png`, `hero-1440-dark.png`): form shows Role, Year 2026, Status Starting soon, Built with "Salesforce (NPSP), Payment gateway, Power BI and Excel automation, InDiGO". Measured at 1440: scene x 783-1296; note x 783-1123 at y 746; button x 1135-1296 at y 736-780. One row, inside the scene's width.
11. Pass (scratch script `target/wo6-measure.mjs`, shots `hero-390-*.png`): at 390 the button is at y 774-818, right edge 370 = scene right edge 370; scene top 830, so 12 px gap (limit 16). Note at y 1234, 12 px under the scene, left-aligned. scrollWidth overflow 0 at every width tested. Reduced motion and no-JS: button `display: none`, scene top 774 - form bottom 746 = 28 px = hero row gap (28px); no extra band. Pause at 1440: label becomes "Play animation", every hero animation reports `paused`, and two element screenshots 1.5 s apart are byte-identical. Tab order after the header: crumb "Work", "Pause animation", step 1, step 2.
    Switch range: 641, 700, 760, 864 use the narrow layout (button above, centred 520 px column); 865 and 900 the wide one. At 865 I first saw the note wrap to two lines beside the button, so the column gap is 0.75rem (was 1rem); it now fits on one line at 865 (y 647-669).
12. Pass: shoot.mjs with the planned flags: axe 0 failures, 0 console/network/layout problems (the baseline in `target/shots/WO-0006-before/` had nine `journey.mp4 ERR_ABORTED`; now none), every tab stop shows a focus ring. `/work/` lists the draft with the new description and stack (`work-1440-light.png`).
13. Pass: intro and step 2 read as planned (`work-donation-website-1440-dark-reduced.png`); scroll shots 0.05 / 0.5 / 1 show steps 1, 3, 5 at 390 and 1440; reduced-motion shots list all five steps.
14. Pass: in the full-page shots the walkthrough is followed by "What I will build and run", "How this page moves" and the pager. No video section, no "What changed".
15. Pass: `.jn-sample` is `color: var(--ink-soft)`, no opacity rule; computed colour rgb(107,85,87) = #6B5557 light, rgb(194,182,179) = #C2B6B3 dark, opacity 1. On `--paper`: 6.34:1 light, 8.63:1 dark (design-system table). axe passes.
16. Pass: `git status --short` lists the ten planned files plus `.factory/` and the planner's memory; `design/` stays untracked.

## Remaining `<!-- Nows -->` gaps
1. Confirm the exact role title (front matter comment on `role` and the body marker).
2. Re-record `site/static/images/journey.mp4` and `journey-poster.jpg` without the "Stored safely" line; then set `show_recording = true` in `site/templates/partials/journey.html` and remove both files from `ignored_static` in `site/zola.toml`.

## Notes for reviewers
- Look first at the hero at 390 (button above the scene, note under it) and at 1440 (note left, button right, one line). The layout switches at 54rem (864/865 px).
- DOM order is scene, note, button; at narrow widths CSS `order: -1` puts the button visually above the scene. Tab order is unchanged (the scene has no focusable parts), so visual and focus order still agree: crumb, then the button, then the steps.
- `ignored_static` is a live setting (not commented out), as it must be to drop the files. The supervisor's `zola serve` on :1111 needs a restart to pick up the `zola.toml` change; until then it still serves journey.mp4 (unlinked).
- The video CSS (`.jn-video*`, `.jn-vbtn*`) and JS section 4 stay, dormant: `if (video && videoButton)` finds nothing.
- The media was already deployed with 40eac14 (unlinked); this order drops it from the next deploy only. It stays in git history.
- Out of scope but noticed: the `/work/` section intro still says "what changed because of it", which this page no longer fits; a copy change for another order if Nows wants it.
