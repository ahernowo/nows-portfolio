# Approval: Turn the donor-journey recording back on

## Verdict
Approved. Five low findings stay open (#1 to #5). None blocks approval. They are listed under For Nows as a follow-up.

## Checks I ran
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, no warnings
- `cargo test --workspace`: 65 passed (46 + 17 + 1 + 1), 0 failed
- `cargo run -q -p newpost -- --check`: "Checked 7 files. No problems."
- `zola --root site build`: 5 pages, 2 sections, no warnings
- The plan's grep checks 3, 4, 5 and 15: all as expected (output below)
- `sha256sum` of the built and source media: both match the plan's hashes; `git diff --quiet HEAD -- site/static/images site/static/_headers` exits 0
- `ffprobe`: h264 1280x720, duration 12.000000, 0 audio streams. I extracted the last frame and a 1 fps contact sheet and read both, then opened the poster
- My own drafts build in the scratchpad (with `--base-url http://127.0.0.1:4791`), checked in real Chrome (`channel: "chrome"`) with one Playwright script at 1440 light and 390 dark, plus reduced motion and no JS
- `factory verify WO-0007` after my runs: every gate passes, seal intact

## Request against the change
Nows asked for four things, all done:
- `show_recording = true` and its comment removed (`partials/journey.html:7`)
- the `ignored_static` line and its comment removed from `zola.toml`
- the recording's Nows comment removed from `donation-website.md`
- no security claim in the recording block (check 7 below)

`draft = true` and the role-title comment stay. Nothing is committed.

The plan's extras are in the diff, as Nows decided ("Full plan"):
- stale "switched off" text updated in `journey.js`, `docs/design-system.md` and the two skills
- the Recording row restored to the "How this page moves" table
- the video button moved from `aria-pressed` to `data-playing`: template, JS and both Sass selectors

Nothing in the diff goes beyond the plan. The plan says nine files but lists eight; the builder said so, and the diff has exactly those eight. Every word about Nows on the page comes from the repo; the new table row's "About 40 lines" matches section 4 of `journey.js`.

## Acceptance checks
1. Pass (my run): fmt, clippy and test are green, 65 passed.
2. Pass (my run): zola build has no warnings; newpost check reports no problems.
3. Pass (my run): `show_recording` appears at lines 7 (`= true`) and 32 (`if`); the count of "switched off|WO-0006|re-made" is 0.
4. Pass (my run): the count of "ignored_static|journey" in `zola.toml` is 0. The diff is the five removed lines only; `ignored_content` is unchanged.
5. Pass (my run): the count of "Stored safely|ignored_static|show_recording" is 0; the role-title comment count is 1; `draft = true` is 1.
6. Pass (my run): `site/public/images/journey.mp4` is `314332fe...f55af76b` and the poster is `84287171...a5b13ee2`, the same as source and plan. `site/public/work/` holds only `pdf-merger`, `this-website` and `index.html`.
7. Pass (my run, plan's node script on my drafts build): the text is exactly the plan's expected text, the aria-label is "Recording of the same illustration: a donor gives on a phone and the gift is recorded in three systems", and the security regex prints `false`.
8. Pass (my run): duration 12.0 s, no audio. The last frame and the poster end with "All three agree / Same gift, same amount, in every system.". The contact sheet has no security text. The padlock icon is still there (#2, low, Nows's file).
9. Pass: design crops (`target/shots/WO-0007/design/crops/playing-1440-dark.png`, `focus-390-light.png`), which I opened; no sideways overflow in either report. The pale dark-theme button is #1 and the 390 legibility is #5, both low.
10. Pass (my Chrome run, 1440 light and 390 dark):
    - At 30% visible the video is paused. At 70% it plays, with `data-playing="true"`, "Pause" and the pause icon.
    - A click pauses it at 1.23 s. It is still paused at 1.23 s after scrolling to the top and back.
    - Play, then scroll off screen: it pauses.
    - `aria-pressed` is absent in every state, and there are no console errors.
11. Pass (my run): with reduced motion it stays at t=0 while fully visible, and a click plays it. With JS off the button is `hidden` (`display: none`), the poster is set, and there are no `controls`.
12. Pass, from the build and design reports: axe found 0 rule failures across 8 runs. I did not rerun axe; no markup changed beyond one attribute name.
13. Pass (my run, 1440 and 390): Shift+Tab from the button lands on step 5, "Three systems record the gift". Tab goes back to the button, then to `#what-i-will-build-and-run`; focus never lands on BODY. Space pauses and Enter plays again. The builder covered 864.
14. Pass, from the reports: the builder measured the hero Pause animation button and the scroll states. Its code did not change in this diff.
15. Pass (my run): the grep prints nothing (rc 1).
16. Pass (my run): the built table reads Reading line, then Recording, then Reduced motion, and the Reduced motion row says "pinning, autoplay and idle motion are off".
17. Pass (my run): `_headers` is unchanged; the drafts page has 0 `style=`, and its scripts are `journey.js`, `head.js`, `site.js` and one `application/ld+json`.
18. Pass (my run): `git status --short` shows the eight files plus agent memories, `.factory/` and the untracked `design/`; nothing under `site/static/images/` is modified.

## For Nows
Read first in `git diff`:
- `site/templates/partials/journey.html`
- `site/zola.toml`
- `site/content/work/donation-website.md`
- `site/static/js/journey.js` and `site/sass/journey.scss` (the `data-playing` rename)
- the docs and skills last

The agent-memory files and `.factory/` change with every order. `design/` is your untracked mock-up folder and is not part of this commit.

To try by hand, in Chrome:
1. Run `zola --root site serve --drafts` and open `/work/donation-website/`.
2. Scroll to "The whole journey in twelve seconds": the video plays once half of it is on screen.
3. Pause it, scroll away and back: it stays paused.

Deployment: after the next push, `/images/journey.mp4` and `/images/journey-poster.jpg` are public, even though the case study is still a draft. That is what you asked for. The old "Stored safely" video is still in git history (40eac14).

Low follow-ups, for an order of your choice:
- #1: the pale video button in dark
- #2: the padlock beside "All three agree", your file, your call
- #3: the one-word heading line at 1440
- #4: the intro says the video "plays while it is on screen", which is untrue with reduced motion or without JS. The new table row says the same.
- #5: the recording is too small to read at 390

Suggested commit message: `Turn the donor-journey recording back on`
