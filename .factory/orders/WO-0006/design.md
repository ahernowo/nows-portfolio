# Design review: Reframe the donation case study as an honest plan

## Verdict
Pass. Two low findings (#1, #2) for polish and follow-up. Neither blocks.

## What I looked at
- `git diff` of the ten planned files; read in full: `donation-website.md`, `work-page.html`, `partials/journey.html`, the `journey_scene` component (components.html 35-111), the changed parts of `journey.scss`, `journey.js`, `config.yml`, `zola.toml`, and the skill and doc edits.
- `node tools/shots/shoot.mjs --drafts --reduced --focus --out target/shots/WO-0006-design --pages /work/donation-website/,/work/ --scroll "#journey-story:0.05,0.5,1"`: 48 shots, **0 console/network/layout problems, axe 0 rule failures, every tab stop has a ring**.
- Inline Playwright (real Chrome) against the supervisor's drafts server on :1111: hero crops and measurements at 320, 390, 641, 760, 864, 865, 1024 and 1440, light and dark, plus reduced at 390 and 1440 (`target/shots/WO-0006-design/crops/hero-*.png`); form crops at 866 and 920 (`form-*.png`); a Tab walk with motion on at 390 light and 1440 dark (`focus-pause-*.png`); Pause test with a pixel diff.
- No failed requests at any width; `/images/journey.mp4` returns 404 on :1111.

## Checklist
- Identity and layout: ok. The note and button sit on one line under the scene at 865 px and up (1440: scene x 783-1296, note 783-1123, button 1135-1296). At 864 px and below, the button sits above the scene, right edge on the scene's edge, and the note sits under it. Nothing from the generated-look list.
- Typography: ok. `.jn-sample` is 14 px Inter, sentence case. The walkthrough intro wraps without orphans at 1440. #1: at 865-970 px the long Role value overruns its field.
- Colour and contrast: ok. `.jn-sample` computes to rgb(107,85,87) light and rgb(194,182,179) dark, opacity 1, on `--paper`: 6.34 / 8.63:1. No raw colours added to `journey.scss`. Accent use unchanged.
- Motion: ok. Pause with Enter: label becomes "Play animation", `html.jn-paused` is set, every hero animation is `paused`, and a pixel diff of the hero 3 s apart changes 0 px in dark and 1 px in light (noise). Second Enter restores "Pause animation". Reduced motion: no button, finished pictures, all five steps listed (`*-reduced.png`).
- Interaction and focus: ok. Tab order with motion on: skip link, header, crumb "Work", "Pause animation", step 1, step 2, and so on. The visual move above the scene at narrow widths keeps that order. Button 161x44 px; 2 px `--ink` ring in both themes (crops).
- Responsive: ok, except #1. scrollWidth equals the viewport at every width tested (320-1440). At 390 the button (y 774-818) sits on the first 900 px screen with the scene top 12 px below it. With reduced motion the scene starts 28 px under the form (the hero gap), with no empty band.
- Accessibility: ok. axe 0. One h1. The removed sections take their h2s with them; the walkthrough h2 is followed by the prose h2s. The scene keeps `role="img"` with an "Illustration with sample data: ..." label.
- Copy: ok for the page (see check 9). #2 concerns the /work/ intro, which is outside this order's files.

## Acceptance checks (the ones about what people see)
5. Pass: re-run against the served page. `outside rooms: []`, `rooms: 2`, `visible notes: 2`, `video/claim: []` (I also checked for "Stored safely": none).
9. Pass. I read every sentence on /work/donation-website/ against the request's facts:
   - Dek: "What I will build and run on a donation website from November 2026: its development, documentation and monitoring." Future tense; November 2026 and development/documentation/monitoring are from the facts. "Build and run" is the request's own heading.
   - Form: Role "Donation website development, documentation and monitoring" (the request's fallback, with the `# Nows:` marker); Year 2026; Status "Starting soon"; Built with the four tools exactly as Nows named them.
   - Prose: "From November 2026, I will work on ...", "My first responsibilities will be SOPs ... and monitoring tools.", "I will work with Salesforce (NPSP), the payment gateway, Power BI and Excel automation, and InDiGO.", "There are no results yet, so this page describes the plan, not an outcome. The illustration above follows one gift with sample data; it is not a record from the website." Each sentence is future or present tense and uses only the facts. No numbers, timings, user counts or outcomes.
   - Walkthrough: the intro and the five step texts describe the illustration, not Nows's work. Step 2 is "She picks an amount and keeps the monthly option." Rina, DN-2047 and the Rp amounts appear only inside the two scenes, which carry the visible note and the sample-data label.
   - "How this page moves": seven rows, no bars, no Recording row, no "autoplay". Its line counts describe this page's own code (from WO-0005), not the donation work.
   - HTML comments (the two `<!-- Nows -->` markers) do not reach the output: minify strips them.
10. Pass: `crops/hero-1440-light.png` and `hero-1440-dark.png`. All four form fields are as planned. The note is left and the button right, on one row within the scene's width.
11. Pass: at 390 the button bottom is 818 and the scene top 830 (12 px gap), with right edges both at 370. The note is 12 px under the scene, left-aligned. No sideways scroll. Reduced: no button, gap 28 px. Pause works (see Motion). Tab order is crumb, Pause, steps. 641-864 px use the narrow layout, and 865 px and up the wide one (crops at 641, 760, 864, 865).
12. Pass: see the shots summary above. `/work/` lists the draft with the new description and tools (`work-1440-light.png`, `work-390-dark.png`).
13. Pass: the intro reads as planned. The scroll shots show step 3 at 0.5 (390 light) and step 5 with the panel and stamp at 1 (1440 dark). The reduced shots list all five steps.
14. Pass: in `work-donation-website-390-dark-reduced.png` and `-1440-light.png`, the walkthrough is followed by "What I will build and run", "How this page moves" and the pager. There is no video and no "What changed".
15. Pass: see Colour.

## Findings
- #1 low: in a window 865-970 px wide, the hero's form has three narrow columns, and the word "documentation" in the Role value is wider than its field. It runs up to 16.6 px past the padding, reaching the ruled line into the Year cell at 865-880 px (`crops/form-866-light.png`). The text stays readable. Fix: `hyphens: auto` on `.form-field dd` (the page is `lang="en"`), or `overflow-wrap: break-word`. The problem may also go away once Nows confirms a shorter role title.
- #2 low (follow-up, outside this order's files): the `/work/` intro (`site/content/work/_index.md` line 3) reads "Systems I've built: what the problem was, what I made, and what changed because of it." Once this draft is published, it will sit under that line although nothing is built yet. Before publishing, Nows may want an intro that also fits planned work.

Not logged (pre-existing, unchanged by this order): under reduced motion the hero scene rests on the amount-picker screen while the walkthrough shows the finished thank-you. That is the WO-0005 resting state.
