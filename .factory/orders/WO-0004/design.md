# Design review: Soften the palette and make light the default theme (rework 1)

## Verdict
Pass. Finding #1 is fixed and nothing else visible changed since my first review (`design.rework-1.md`), whose verdicts stand.

## What I looked at
- Scope: `find site .claude/skills docs tools/shots tools/factory/dashboard workers/cms-api/admin scripts -newer <ref>` with `<ref>` touched to the last design report (2026-10-24T05:50:13Z) lists only `site/sass/_chrome.scss`, `.claude/skills/design-system/SKILL.md`, `docs/design-system.md`. `git diff site/sass/_chrome.scss`: `.search-input::placeholder` `opacity: 0.8` became `opacity: 1`, plus a two-line comment. Grep of `opacity` across `site/sass/`: the only other text fade is `.giallo-ln` at 0.6 on the code slab, untouched by this order.
- Search dialog opened with `/` on :1111 (inline Playwright, saved choice for dark), at 390 and 1440, light and dark, empty and with "rust" typed: `target/shots/WO-0004-design2/search-{light,dark}-{390,1440}.png` and `search-typed-*.png`. Opened `search-light-390.png`, `search-dark-1440.png`, `search-typed-light-1440.png`.
- `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0004-design2/scan --pages /,/articles/`: 8 shots, "Console, network and layout problems (0)", "Accessibility scan, WCAG 2.2 AA (0 rule failures)".

## Checklist
- Identity and layout: ok (unchanged since first review).
- Typography: not affected.
- Colour and contrast: ok. Placeholder computed style: light `rgb(107, 85, 87)` opacity 1 on dialog `rgb(250, 245, 243)` = 6.34:1; dark `rgb(194, 182, 179)` opacity 1 on `rgb(31, 27, 28)` = 8.63:1 (recomputed with the WCAG formula). Input background is transparent, so the dialog colour is the true backdrop.
- Motion: not affected.
- Interaction and focus: ok. `/` opens the dialog with focus in the input in all four runs.
- Responsive: ok. Dialog at 390 fits the width; placeholder on one line.
- Accessibility: ok. Scan 0 failures; the placeholder (outside axe's reach) checked by hand above.
- Copy: ok. Placeholder text unchanged, "Search articles and work". The placeholder still reads as a hint: typed "rust" in `--ink` is visibly darker than the muted placeholder and result snippets.

## Acceptance checks
Unchanged from `design.rework-1.md` (all pass there except the placeholder). The contrast tables now carry the placeholder row: `SKILL.md:66` and `docs/design-system.md:76`, 6.34 / 8.63, matching my figures; `SKILL.md:55` states muted text always runs at full opacity.

## Findings
- #1 medium (fixed, verified): placeholder now 6.34:1 light, 8.63:1 dark (`site/sass/_chrome.scss:226-229`).
- No new findings.
