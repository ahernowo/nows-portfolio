# Build: Soften the palette and make light the default theme

## What changed
The site's colour tokens now use Nows's soft palette: a #FAF5F3 page, white slips, #2B1618 text, #6B5557 muted text and crimson in light; #1F1B1C / #2C2527 / #F2EDEB / #C2B6B3 with coral #E8896A and a #E8546D stamp in dark. Light is the default for everyone: the `prefers-color-scheme` block is gone, `head.js` applies only a saved choice, and the Theme button cycles light/dark with the label "Theme: light" / "Theme: dark". A new `--accent` token takes the now-pulse, reading line and warning/caution edges, so `--stamp` colours the stamp alone. The OG image, the theme-color meta, the dashboard and editor CSS, the shots tool's dark run, the design-system skill, `docs/design-system.md` and the ui-ux-review `--themes` row follow.

## Files
- `site/sass/_tokens.scss`: new values in `:root` and `dark-tokens`, `--accent` added, system-driven dark block removed, header comment rewritten.
- `site/sass/_home.scss`: now-pulse uses `--accent`.
- `site/sass/_prose.scss`: reading line and warning/caution edges use `--accent`; slab comment reworded.
- `site/sass/_base.scss`: link comment names coral #E8896A.
- `site/static/js/head.js`: applies a saved theme only; comment says the page stays light.
- `site/static/js/site.js`: light/dark only, no `auto`; first visit stores nothing.
- `site/templates/partials/header.html`: no-JS label "Theme: light".
- `site/templates/base.html`: `color-scheme: light`, one theme-color `#FAF5F3`.
- `tools/shots/shoot.mjs`: dark run saves `theme=dark` with an init script.
- `scripts/make-og-image.py`, `site/static/images/og.png`: new colours, regenerated.
- `tools/factory/dashboard/app.css`, `workers/cms-api/admin/app.css`: new palette values, comments, backdrop.
- `.claude/skills/design-system/SKILL.md`, `docs/design-system.md`, `.claude/skills/ui-ux-review/SKILL.md`: palette, themes, tokens, contrast.
- `.claude/agent-memory/planner/MEMORY.md`, `.factory/board.json`, `.factory/orders/WO-0004/`: from the planner and the CLI, not this build.

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, 0 warnings
- `cargo test --workspace`: 65 passed, 0 failed (46 + 17 + 1 + 1)
- `cargo check -p cms-api --target wasm32-unknown-unknown`: ok (the editor CSS is compiled in)
- `zola --root site build`: ok, no warnings; `zola --root site check --skip-external-links`: ok
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."

## Acceptance checks
1. Pass: fmt, clippy, test above.
2. Pass: zola build and newpost check above.
3. Pass: `grep -rn prefers-color-scheme site/sass site/static/js site/templates` prints nothing; `grep -c` on `site/public/main.css` prints 0.
4. Pass: the old-palette grep prints nothing. The `fdffff|280000` grep prints only `_tokens.scss` lines 68-69 and 94-95 (`--code-bg`/`--code-ink`, both blocks) and `workers/cms-api/admin/app.css:170` (`.preview pre`).
5. Pass: `_tokens.scss` holds the plan's values (every mix re-derived by script; all identical). `var(--stamp)` only at `_home.scss:151` (`.stamp`); `var(--accent)` at `_home.scss:43` and `_prose.scss:381, 387, 487`.
6. Pass: the layout-property diff grep prints nothing.
7. Pass, with a note: Zola's minifier puts `content` first, so the plan's pattern `<meta name=theme-color[^>]*>` matches nothing. `grep -o "<meta[^>]*\(theme-color\|color-scheme\)[^>]*>" site/public/index.html` prints `<meta content=light name=color-scheme>` and exactly one `<meta content=#FAF5F3 name=theme-color>`, with no `media`.
8. Pass, against the supervisor's server on 1111 (system set to dark), at 1440 and 390: `first ['rgb(250, 245, 243)', null, 'Theme: light', null] 1`, `click ['rgb(31, 27, 28)', 'dark', 'Theme: dark', 'dark']`, `reload` the same, `back ['rgb(250, 245, 243)', 'light', 'Theme: light', 'light']`. Matches the plan exactly.
9. Pass: 40 shots in `target/shots/WO-0004/`, "Console, network and layout problems (0)", "Accessibility scan, WCAG 2.2 AA (0 rule failures)" (not even the six code classes this time). The bottom-left pixel of every `-dark.png` is (31, 27, 28), of every `-light.png` (250, 245, 243). One flake: in the first run the lazy `og.png` figure on `writing-reference-1440-dark` had not loaded; two re-runs (`target/shots/WO-0004-recheck1`, `-recheck2`) show it in both themes. That is lazy-load timing in a full-page shot, not a colour change.
10. For design review, plus what I saw: `home-1440-light.png` / `-dark.png`. Light: the three slips read as distinct (white, peach, rose) with their shadows; crimson stamp and pulse. Dark: #2C2527 top slip, brown peach and plum rose copies, #E8546D stamp, coral pulse. Page sizes match `target/shots/WO-0004-before/` exactly (home 1440x2329 and 390x2989; writing-reference 1440x3921 and 390x4103).
11. Pass (computed styles, `/articles/how-this-site-is-built/` at 1440): crumb, prose links, tag links, footer links and "Back to all writing" are rgb(177,15,46) with underline rgb(205,102,121) in light, rgb(232,137,106) with rgb(156,95,76) in dark. Wordmark and menu are rgb(43,22,24) / rgb(242,237,235). The dek is muted. The reading line background is crimson / coral. Pager (drafts build, `target/wo4-site`): `.pager-title` is the accent and underlined, `.pager-hint` muted and plain, in both themes.
12. For design review: `target/wo4-crops/wr-both.png` (light left, dark right). Note/tip/important on peach, warning/caution on rose with an accent edge; the slab is #280000 in both, with the 1 px edge visible in dark.
13. Pass: the label reads "Theme: light" first and "Theme: dark" after a click (check 8). At 390, `getByRole("button", { name: "Theme: light" })` counts 1, so the visually hidden label is still the accessible name.
14. Pass: `--focus` on `/` and `/articles/writing-reference/`: "every tab stop shows a focus ring" (`target/shots/WO-0004-focus/`). Manual Tab from the last prose link of `/articles/how-this-site-is-built/`: tags, "Back to all writing", Email, GitHub, RSS feed, "Read the source", the skip link and the wordmark each get a 2 px solid outline, rgb(43,22,24) in light and rgb(242,237,235) in dark. Heading anchor on focus: rgb(107,85,87) light (6.34:1), rgb(194,182,179) dark (8.63:1), opacity 1, ink outline. Copy button: #FDFFFF on the slab with a #FDFFFF outline, unchanged.
15. Pass: og.png is 1200x630, pixel (10,10) = (250, 245, 243). Opened it: "Nows" in #2B1618, muted tag lines, three slips, crimson stamp. The slips cover the row labels "Plan", "Status", "Donor", "Gift" exactly as the previous og.png did (compared with `git show HEAD:site/static/images/og.png`); that is the stacked design, and the layout is unchanged.
16. Pass: the dashboard, freshly built and served on 4601 (then stopped): body rgb(31,27,28) with rgb(242,237,235) text in dark; rgb(240,234,232) / rgb(43,22,24) in light. Andon: rgb(31,27,28) on rgb(255,123,107) dark (6.74), rgb(240,234,232) on rgb(177,15,46) light (5.92). Lamps unchanged. Screenshot `target/wo4-crops/dash-dark.png`. The editor CSS values match the plan line by line.
17. For design review: the skill and the doc name the palette per theme, both never-as-text rules, `--accent` against `--stamp`, muted text, theme behaviour, tokens and the contrast table. A case-insensitive grep for "maroon" and "deep red" on both prints nothing; "system setting" appears only to say it no longer applies.
18. Pass: `git diff --stat` on `_headers`, the favicon, the three mark SVGs and `site/content` prints nothing.

## Contrast table (recomputed from the final hex values, WCAG formula)

| Pair | Light | Dark |
|---|---|---|
| Text on page | 15.78 | 14.68 |
| Text on top slip | 17.06 | 12.91 |
| Text on peach / rose / highlight | 13.84 / 13.76 / 14.98 | 10.78 / 11.44 / 12.53 |
| Muted on page (heading anchor #) | 6.34 | 8.63 |
| Muted on top slip | 6.86 | 7.59 |
| Muted on peach / rose / highlight | 5.56 / 5.53 / 6.02 | 6.34 / 6.73 / 7.37 |
| Link on page / top slip | 6.52 / 7.05 | 6.67 / 5.87 |
| Link on peach / rose / highlight | 5.72 / 5.69 / 6.19 | 4.90 / 5.20 / 5.69 |
| Link underline against page (3:1) | 3.36 | 3.37 |
| Stamp on rose (large bold, 3:1) / on page | 5.69 / 6.52 | 3.74 / 4.81 |
| Now-pulse, reading line against page | 6.52 | 6.67 |
| Focus ring, skip link | 15.78 | 14.68 |
| Code text on slab | 19.19 | 19.19 |
| Never-as-text | #E8896A 2.36, #E8546D 3.28 on #FAF5F3 | #B10F2E 2.42 on #1F1B1C |
| Dashboard | ink/bg 14.33, muted/bg 5.76; run 5.38, wait 5.10, stop 5.92; stop on yellow 5.72 | run 9.20, wait 11.27, stop 6.74; stop on surface-2 4.94; ink / muted on surface-2 10.78 / 6.34 |
| Editor | ok 5.94 on paper, 6.43 on field; stamp 6.52 paper / 7.05 field / 5.69 rose | ok 10.05 / 8.84; stamp #E8896A 6.67 / 5.87 / 5.20 (#E8546D on the field would be 4.23) |

Separation (not a WCAG rule): in light the copies sit 1.14 (peach), 1.15 (rose), 1.05 (highlight) and 1.08 (white slip) off the page; in dark 1.36 / 1.28 / 1.17 / 1.14.

## Notes for reviewers
- One small change from the plan's wording in `site.js`: `readTheme()` reads the `data-theme` attribute that `head.js` set, not localStorage. Both give the same answer, but reading the attribute keeps the button toggling both ways when storage is blocked; reading storage, it would stick on dark. First visits still store nothing (check 8 shows `null`).
- In `docs/design-system.md` I also reworded "The idea" sentence ("written once in maroon carbon, on deep-red ledger lines") and the layout bullet ("ruled in deep red"), because the ledger lines are no longer deep red. They now say "dark carbon ink" and "fine (warm-grey) ledger lines". No new facts about Nows.
- Judge the dark stamp: #E8546D with the screen blend over the plum rose copy reads softer than the light one (3.74:1 before blending, large bold text only).
- Judge the light row highlighter on hover (1.05:1 off the page) on `/work/` and `/articles/`; I did not deepen any tint.
- The dashboard picks up its CSS on the next `factory` rebuild; the editor only after cms-api is redeployed.
