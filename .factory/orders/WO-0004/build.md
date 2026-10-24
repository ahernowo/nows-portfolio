# Build: Soften the palette and make light the default theme (rework 1)

## What changed
This round clears design finding #1. The search box's placeholder ("Search articles and work") was muted text at 80% opacity, which fell to 4.00:1 on the new light page. It now uses `--ink-soft` at full opacity: 6.34:1 in light and 8.63:1 in dark. `opacity: 1` also overrides Firefox's built-in 0.54 placeholder fade, which would otherwise have dropped it again. The contrast tables in the design-system skill and `docs/design-system.md` gained a placeholder row, and the skill's muted-text rule now says muted text always runs at full opacity. Everything else is unchanged from the first build (see `build.rework-1.md`).

## Files
This round:
- `site/sass/_chrome.scss`: `.search-input::placeholder` `opacity: 0.8` changed to `1`, with a two-line comment saying why.
- `.claude/skills/design-system/SKILL.md`: placeholder row in the contrast table; "always at full opacity" added to the muted-text rule.
- `docs/design-system.md`: placeholder row in the contrast table.

The rest of `git status --short` is the first build of this order, unchanged (tokens, `_base`/`_home`/`_prose`, `head.js`, `site.js`, `base.html`, `header.html`, `shoot.mjs`, `make-og-image.py`, `og.png`, dashboard and editor CSS, `ui-ux-review` skill, agent memories, `.factory/`).

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, no warnings
- `cargo test --workspace`: 65 passed, 0 failed
- `zola --root site build`: ok, 5 pages, 2 sections, no warnings; built `main.css` holds `search-input::placeholder{color:var(--ink-soft);opacity:1}`
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- `node tools/shots/shoot.mjs --pages /,/articles/` (against :1111): 0 console/network/layout problems, 0 axe failures.
- Contrast script (`target/wo4-contrast.py`, WCAG formula, sRGB alpha mix): light `#6B5557` on `#FAF5F3` 6.34 (4.00 at 0.8, which matches the review); dark `#C2B6B3` on `#1F1B1C` 8.63 (5.99 at 0.8).

## Acceptance checks
Unchanged from `build.rework-1.md`; the only visual change this round is the placeholder.
- #1 evidence: `target/wo4-placeholder.mjs` opened the search dialog on :1111 in Chromium in both themes and read the `::placeholder` computed style. Light: `rgb(107, 85, 87)`, opacity 1, on dialog `rgb(250, 245, 243)`. Dark: `rgb(194, 182, 179)`, opacity 1, on `rgb(31, 27, 28)`. Crops: `target/shots/WO-0004-rework1/search-chromium-light.png` and `search-chromium-dark.png`. I looked at both: the placeholder reads clearly and is still visibly lighter than typed text would be.
- Firefox is not installed for Playwright here, so the Firefox override is from the spec default (UA `::placeholder { opacity: 0.54 }`), not a shot.

## Findings fixed (rework)
- #1: `.search-input::placeholder` is `--ink-soft` at `opacity: 1` (6.34 light / 8.63 dark). Placeholder row added to both contrast tables.

## Notes for reviewers
- Look at the search dialog empty in both themes (Search button or `/`). The placeholder is now as dark as other muted text (dates, descriptions), so it is a little stronger than before. It still reads as a hint because typed text is `--ink`.
- Not changed, for the record: `.giallo-ln` (code line numbers) still has `opacity: 0.6`, but it sits on the code slab, whose colours this order did not touch, and the numbers are `user-select: none` decoration.
