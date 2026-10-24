# Approval: Port the donor-journey motion to the work page

## Verdict
Approved. Seven low findings (#1-#7, design review) stay open as polish for a later order; none blocks.

## Checks I ran
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 7 files. No problems."
- `zola --root site build`: ok, 5 pages, no warnings; `zola --root site check --skip-external-links --drafts`: rc 0, 9 pages
- Drafts and normal builds into my scratchpad, with the greps from plan checks 2-7 and 13 (results below)
- Contrast: recomputed every journey pair with a WCAG script from the hex values in `_tokens.scss`. All match the plan and the skill table: ok on panel 6.61/8.19, ok on ok-tint 5.71/6.75, stamp 7.05/4.23, after bar 5.67/5.55, phone muted 6.86/5.97/6.19, mock-up muted 4.45 (hence `#6B5557`), white on dark `--ok` 1.83 (hence the token).
- My own Playwright run in real Chrome against a drafts server (port 1119, stopped afterwards): pause, resume, keyboard step 3, reduced motion, no JS at 390, and the title morph from /work/. No console or page errors.
- Video duration read from the mp4 `mvhd` box: 12.0 s, matching "twelve seconds".
- Opened the shots myself: `target/shots/WO-0005-design/work-donation-website-1440-dark-scroll-1.png` (finished panel, three green rows, stamp, no "Stored safely"), `crops/hero-390-light.png` (Pause button at the fold, as in #7), and `target/wo5crops/bars-work-donation-website-1440-light.png` (grey full bar, accent bar about 6 percent).
- `factory verify` after all runs: every gate passes, seal intact.

## Request against the change
Asked: the mock-up's hero scene, the pinned five-step journey (280svh), the video block, the bars and the "how this page moves" table on the work page. Copy comes from front matter with Nows markers. One JS file, styles in Sass on site tokens, no Google Fonts, media copied, `_headers` unchanged, no `style=""`, a reduced-motion fallback, and shots plus a scan.

Built: all of it, on a new draft case study `/work/donation-website/` (Q2), with steps in front matter and one `<!-- Nows -->` block (Q3), the "Stored safely" line dropped from the scene (Q4), the motion kept as mocked and recorded in the skill (Q5), and the green as `--ok`/`--ok-tint` (Q6). Nows's later override replaced the plan's 5 s cap: endless breath and blink plus the 5.9 s beat, with a hero **Pause animation** button.

Out of scope, with a reason: the mock-up's own header, footer and next link (the site's header and pager replace them); the three-line title split; re-recording the video (Nows kept it); real figures (bars hidden until Nows adds them).

Disclosed small departures, within the request: step 5 targets 1 instead of 0.97, so Enter lands on the finished picture, and the hero label is worded to be true both at rest and when finished. Tooling (`--root`, `--scroll`, `.mp4` type) was planned.

Truthfulness: the steps, description, table and intros describe only the illustration and the code. The invented data is labelled in the aria-labels, the walkthrough intro, the panel ("Sample records for this one gift") and the figcaption. The table's line counts roughly match `journey.js` (scene about 220, beat and pause about 56, story about 58, video about 45, bars 9, motion switch about 34). The one inaccurate row is #4 (low).

## Acceptance checks
1. Pass: my own standard checks plus `zola check --drafts` rc 0.
2. Pass: the drafts build has `work/donation-website/index.html`; the normal build's `work/` holds only `index.html`, `pdf-merger` and `this-website`. `draft = true` count 1, `<!-- Nows:` count 1.
3. Pass: the `jn-|journey` grep prints 0 on this-website, pdf-merger, bank-reconciliation and donation-dashboard.
4. Pass: `git diff --quiet -- site/static/_headers` rc 0. `style="` count 0. The only scripts are head.js, journey.js (defer), site.js and `type=application/ld+json`. The forbidden-API grep on journey.js and the googleapis/gstatic grep print nothing. I read journey.js in full: CSSOM and textContent only.
5. Pass: the hex/rgb grep on journey.scss prints nothing; `_tokens.scss` numstat is 39 added, 0 removed.
6. Pass: sha256 `64dbd92f...73c7c1` and `0974e07c...aa2350`, identical to `design/journey/`.
7. Pass: the sample-text grep gives 0; "Illustration with sample data" is present; `jn-fill` 0.
8. Pass: design shots (axe 0, no CSP or console errors; mp4 aborts are Playwright Chromium's missing H.264), plus my own look at the 1440 dark scroll-1 and 390 hero.
9. Pass: the builder's test-copy bars shot, which I opened.
10. Pass (mine): Enter on step 3 leaves `aria-current="step"` on step 3 only. Step buttons are enabled with motion and all `disabled` under reduced motion.
11. Pass: the builder's and design's real-Chrome runs. In mine, the video button starts at "Play", `aria-pressed=false`.
12. Pass (mine): arriving from /work/ sets `vt-arrival` and the rise animation is `none`.
13. Replaced by Nows's decision; pass (mine). Before: `jn-breathe`/`jn-blink` running in both scenes. After Pause: all paused, `<html class="jn-paused">`, label "Play animation", and the hero is pixel-identical 2.5 s later. Resume: running again, and the beat finishes (Rp150.000 chip selected). `infinite` lists breathe, blink, caret and spinner, as Nows chose.
14. Pass (mine, 390, JS off): no sideways scroll (scrollWidth 390), all 5 step bodies shown, stamp opacity 1, Pause hidden, only the existing scroll-driven reading line animates.
15. Pass: the builder's check; the config diff is plain widgets, and security confirmed /admin/ serves it.
16. Pass: the skill and docs diffs carry the second moment, `--ok`/`--ok-tint`, the accent and motion lines, and the 2.2.2 pause line.

Reduced motion (mine): Pause hidden, story `is-static`, page 3866 px tall, hero rests on the chosen amount, story stamp shown, video "Play", only the reading line running.

## For Nows
- Read first: `site/templates/work-page.html` (the journey branch; the old branch is untouched), `site/templates/partials/journey.html`, `site/static/js/journey.js` (469 lines, commented in five numbered parts), `site/content/work/donation-website.md` (your `<!-- Nows -->` list), then `site/sass/_tokens.scss` (only additions). `site/sass/journey.scss` and the `journey_scene` component in `components.html` are the long mechanical port of the mock-up.
- Decide before publishing: **the video and poster still show "Stored safely: Encrypted, with access limited and logged"**, which the live scene dropped. Re-record it, or confirm the line is true. The page is a draft, so nothing is public yet.
- Seven low polish findings are open (#1-#7). The ones a visitor would notice most are #7 (at 390 the Pause button sits at the fold), #5 (in dark, the video button is pale on the light poster) and #6 (10.35 px panel text on phones). A follow-up order could take them together.
- Try by hand: `zola --root site serve --drafts`, open /work/donation-website/, scroll the story, press Pause animation, then turn off animation effects in Windows (Settings, Accessibility, Visual effects) and reload.
- New files to `git add` explicitly: `site/content/work/donation-website.md`, `site/sass/journey.scss`, `site/static/images/journey.mp4`, `site/static/images/journey-poster.jpg`, `site/static/js/journey.js`, `site/templates/partials/journey.html`. `design/` is the outside mock-up package, still untracked; leave it out unless you want it in the repo (its index.html loads Google Fonts locally). Agent memories, skills and `.factory/` changes come with the line.
- Suggested commit message: `Add a draft donation case study with the donor journey`
