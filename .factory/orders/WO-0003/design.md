# Design review: Add the Nows logo to the header and favicon

## Verdict
Pass. No findings.

## What I looked at
- Diff: `site/templates/partials/header.html`, `site/sass/_chrome.scss`, `site/templates/base.html`, `site/static/admin/index.html`, deleted `site/static/favicon.svg`, and the new `nows-flow-mark.svg` and `nows-flow-favicon.svg`. I read all of these in full.
- `node tools/shots/shoot.mjs --out target/shots/WO-0003-design --focus`: every default page (home, work, articles, about, now, 404) at 390 and 1440, light and dark. Result: 0 console, network, CSP or sideways-scroll problems, 0 axe failures (WCAG 2.2 AA), 0 focus problems.
- Header crops of `/about/` at 320, 390, 641, 688, 689, 705, 768, 1024 and 1440, light and dark: `target/shots/WO-0003-design/hdr-<w>-<theme>.png`.
- The wordmark at 4x zoom (`wm-zoom-{light,dark}.png`), with focus (`wm-focus-{light,dark}.png`), and in forced colours (`wm-forced.png`).
- The favicon rendered at 16 and 32 px on light and dark tab-strip greys, at 1x and 2x (`favicon-16-1x.png`, `favicon-tabs-2x.png`), served by the running `zola serve` on :1111.
- The baseline `target/shots/WO-0003-before/home-641-light.png`, to compare with the new 641 px header.

## Checklist
- Identity and layout: ok. The mark is drawn in `--ink`, never the accent: computed fill rgb(40,0,0) in light and rgb(253,255,255) in dark, the same as the name. It reads as part of the name, with no extra decoration. The box starts on the `.wrap` edge (x=144 at 1440). The drawn ink starts 3.5 px in, because the logo's viewBox has empty space on the left. That's about the same as a letter's side space, so it's not a defect.
- Typography: ok. The name is unchanged: 1.5rem, 750, -0.025em, line height 1. The header is still 69 px tall at 689 px and above, and 93.77 px on phones, the same as the baseline.
- Colour and contrast: ok. The mark has the name's colour, 19.19:1 in both themes. The favicon is crimson with an off-white mark, the same in both OS themes, and the rounded square stands out on both light and dark tab strips.
- Motion: not affected. No motion or view-transition changes. The wordmark has no `view-transition-name`.
- Interaction and focus: ok. One 2 px ring wraps both the mark and the name, `--ink` in each theme (rgb(40,0,0) / rgb(253,255,255) solid 2px), and nothing clips it. It is the second tab stop, after the skip link. The link target is 105 by 40 px. Clicking the mark goes home (builder check 8, and the mark is inside the `<a>`).
- Responsive: ok. At 689 to 1440 everything sits on one row, with about 40 px spare between "Now" and Search at 689. At 688, 641, 390 and 320 the header uses the existing phone layout: mark, name and icon tools on the first row, the nav below. The 641 px band now matches the phone header instead of the old one-row header. Nows approved this move, and it reads as the existing design, not a new one. Nothing scrolls sideways.
- Accessibility: ok. axe found 0 failures. The accessibility tree shows `link "Nows"` once and no image with that name (`getByRole('img', {name:'Nows'})` count 0 on `/` and `/about/`). The span is `aria-hidden`, and the `<title>` is stripped, so there's no tooltip. In forced colours the mark and the name both take the system link colour.
- Copy: not affected. No words changed.

## Acceptance checks
6. Pass: the mark is left of "Nows" and 24 px tall (35.75 by 24 box), the same colour as the name in both themes, and the header height is unchanged. Optically, the drawn ink runs from 27.1 to 43.1 px, while the capitals run from about 24.8 to 42.3. The rising stroke starts about 1 px below the baseline and its tip ends about 2 px below cap height. At 1x it reads as a natural upward stroke, not as a sag, so I logged no finding. If Nows wants it to sit higher, `translate: 0 -0.06em` on `.wordmark-mark svg` would do it.
7. Pass: clicking Theme through light, dark and auto changed the name and mark colours together at each step (computed values identical).
9. Pass: one row at 689, 705, 768 and 1024 (`hdr-689-light.png` and the others).
10. Pass: phone layout at 688, 641, 390 and 320 (`hdr-688-light.png`, `hdr-641-dark.png`, `hdr-390-light.png`, `hdr-320-dark.png`).
11. Pass: the ring wraps the mark and the name in light and dark (`wm-focus-*.png`). The shots tool reports 0 focus problems.
12. Pass: 0 axe failures on all six default pages, in both themes.
13. Pass: in forced colours the mark and the name share one system colour (`wm-forced.png`).
14. Pass: 0 console, network or CSP problems.
15. Pass: on :1111, `<link rel=icon>` points at `images/nows-flow-favicon.svg`, which serves 200 `image/svg+xml`, and `/favicon.svg` now returns 404. At 16 px the mark is small but recognisable. I couldn't see a real browser tab (headless), so I rendered the icon at tab sizes instead.
16. Pass: `/admin/` serves `<link rel="icon" href="/images/nows-flow-favicon.svg">`.

## Findings
None.

Notes for Nows, not defects:
- Between 641 and 688 px the header now uses the phone layout (approved).
- Each page now carries the mark's path data inline, about 4.5 KB before compression. That's the cost of a mark that follows the theme colour.
- `nows-flow-mark-dark.svg` and `nows-flow-mark-light.svg` are published but unused, as the plan says.
