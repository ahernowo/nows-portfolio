# Design review: Re-skin the site with a new palette and Inter (rework 1)

## Verdict
Pass. All four findings from the first review (#1 medium, #2-#4 low) are fixed, and I found nothing new. The rest of the re-skin is the same as what I passed in the first round (see `design.rework-1.md`).

## What I looked at
- Files changed since the last design report (checked by modification time): `site/sass/_prose.scss`, `site/templates/article.html`, `site/templates/work-page.html`, `workers/cms-api/admin/app.css`, `docs/design-system.md`, `.claude/skills/design-system/SKILL.md`. I read the template and editor diffs in full, and the anchor, dek and pager rules in `_prose.scss` along with the `a` rules in `_base.scss` that they build on.
- `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0002-design-r1 --drafts --pages /,/work/,/articles/,/about/,/now/,/tags/,/404.html,/articles/how-this-site-is-built/,/articles/writing-reference/,/work/this-website/ --focus` found 0 console, network or layout problems, 0 axe WCAG 2.2 AA failures, and 0 focus problems.
- An inline Playwright run on a `--drafts` build at 1440 and 390, light and dark. It crops the article heads, case head, pager (at rest, on hover, and keyboard-focused by Tabbing from the last tag) and heading anchors (on hover and keyboard-focused), and reads their computed styles. Crops are `r-*.png` in `target/shots/WO-0002-design-r1/`. I opened `r-how-head-1440-dark`, `r-ref-head-390-dark`, `r-case-head-1440-light`, `r-how-pager-1440-light`, `r-how-pager-hover-1440-dark`, `r-how-foot-focus-390-dark`, `r-ref-foot-focus-1440-light`, `r-anchor-focus-light` and `r-anchor-hover-dark`, plus the full pages `home-1440-dark` and `articles-writing-reference-1440-light`.
- Reduced motion: not captured, because no motion changed.

## Checklist
- Identity and layout: ok. It is still the receipt book (`home-1440-dark`), and this round changed no spacing.
- Typography: ok (#2 and #3 fixed). Standfirsts compute `text-wrap: pretty`. "How this site is built" now ends "All of it free / to run." on 3 lines at 1440 and wraps to 4 at 390. "Writing reference" takes 2 lines at 1440 and 3 at 390. The case dek ends "to learn / the language." No word sits alone on a last line.
- Colour and contrast: ok (#1 fixed). The focused anchor computes rgb(87,0,0) in light (14.89:1) and rgb(222,124,90) in dark (6.55:1), both above 4.5:1. The pager title's underline is `--link-line` (#CF6F82 / #954A36), and the hint is `--ink-soft`.
- Motion: not affected.
- Interaction and focus: ok. A Tab from the last tag lands on the pager link with a 2px solid ring, #280000 in light and #FDFFFF in dark. The ring wraps the hint and the title together. The anchor shows at opacity 1 on focus, with an ink ring. On hover, only the title's underline darkens.
- Responsive: ok. At 390 the pager link stays inside its 48% column, and the title and its underline sit on one line (`r-how-foot-focus-390-dark`). The shots tool reports no sideways scrolling.
- Accessibility: ok. Axe is clean. Each pager link is still one `<a>`, and its accessible name is the hint plus the title ("Newer Writing reference"), as before.
- Copy: ok. Nothing changed. The docs and skill gained the anchor contrast row, the `text-wrap: pretty` note and the pager-underline rule, and all three match the code.

## Acceptance checks
Checks 10-19 passed in round one and still stand: the tokens, layout and templates are the same apart from the lines above, and `home-1440-dark` matches the first round. This round re-checks the following:
11. Pass. The pager link is crimson in light and coral in dark (computed). Only its title is underlined. "Back to all writing" and the tags are unchanged (`r-*-foot-focus-*`).
12. Pass. The standfirsts are upright (`r-how-head-1440-dark`, `r-case-head-1440-light`).
15. Pass. `--focus` found 0 problems. The manual Tab onto the pager and the heading anchor shows a visible ring in both themes, and the anchor now passes contrast.
- Editor preview (#4): judged from code alone, because the editor needs `wrangler dev` with D1. `.preview` now sets `font: 1.0625rem/1.65 var(--sans)`, and `--sans` (line 16) is the system sans stack. No webfont was added.

## Findings
None new. #1 (medium), #2, #3 and #4 (low) were all fixed by the builder and confirmed above.
