# Design review: Re-skin the site with a new palette and Inter

## Verdict
Sent back for finding #1 (medium). #2, #3 and #4 are low and don't block approval; they are cheap to fix in the same pass.

The re-skin itself is good. The five colours, Inter, the accent rule and the dark coral secondary text all match Nows's decisions. The receipt stack still carries the identity in both themes, and nothing in the layout moved.

## What I looked at
- `git diff` of all of `site/sass/*`, `base.html`, `footer.html`, `favicon.svg`, both admin stylesheets and `docs/design-system.md`. I read all five changed Sass partials in full, plus every `<a>` in `site/templates/` to see which links now take the accent.
- `MSYS_NO_PATHCONV=1 node tools/shots/shoot.mjs --out target/shots/WO-0002-design --drafts --pages /,/work/,/articles/,/about/,/now/,/tags/,/404.html,/articles/how-this-site-is-built/,/articles/writing-reference/,/work/this-website/ --focus` gave 40 shots at 390 and 1440 in light and dark. Console, network and layout problems: 0. Axe WCAG 2.2 AA: 0 rule failures. Focus: every stop shows a ring.
- Full-resolution crops I looked at in `target/shots/WO-0002-design/`: `c-stack-light/dark`, `c-article-head-dark`, `c-article-foot-light/dark`, `c-article-footer-dark`, `c-article-code-dark`, `c-ref-text-dark`, `c-ref-alerts-dark`, `c-ref-code-light`, `c-home-390-light-a`, `c-home-390-dark-b`, `c-case-390-dark`, `c-article-390-light`. Full pages: `home-1440-*`, `about-1440-light`, `work-1440-dark`, `404-390-dark`, `articles-writing-reference-1440-dark`.
- A manual Tab check with inline Playwright on a `--drafts` build, from the last prose link of `/articles/how-this-site-is-built/` through the Copy button, the heading anchors, 4 tags, the pager, "Back to all writing", Email, GitHub and RSS feed. Every stop had a 2px solid ring, #280000 in light and #FDFFFF in dark (the Copy button's ring is #FDFFFF on the slab). Crops: `f-Back-to-all-writing-*`, `f-GitHub-*`.
- The search dialog in both themes (`search-light.png`, `search-dark.png`). `/` opens it. The first Escape clears the typed text (native `type=search` behaviour, unchanged here); the second closes it and returns focus to the wordmark.
- `site/static/images/og.png` and the builder's `target/shots/WO-0002-extra/dashboard-dark.png`.
- Reduced motion: not captured, because no motion changed (the slip and stamp keyframes and `_motion.scss` are untouched).

## Checklist
- Identity and layout: ok. It reads as the receipt book, with a white original, a peach and a rose copy and a crimson stamp in light, and a maroon sheet with a #570000 slip and a coral stamp in dark. None of the generated-look checklist crept in: maroon with coral is warm and specific, not neon on black. Grid, `.wrap` edges and ledger columns are unchanged in every shot.
- Typography: #2, #3. Inter is used throughout, with real italic on `em` and blockquotes and upright standfirsts. The hero is 5 lines at 1440 and 390, headings are balanced, and prose runs about 70 characters per line on /about/.
- Colour and contrast: #1. Axe is clean. I re-computed the thin dark margins: coral on #4C1912 is 4.90 and on #5F0612 is 4.69. Search marks, alerts and the code slab edge all read. Note for Nows, not a finding: in light, secondary text #570000 is only 1.29:1 from body text #280000, so dates and descriptions are told apart by size, not colour. That matches the plan.
- Motion: not affected.
- Interaction and focus: #1 (the anchor is a focus target). Rings show at every stop in both themes, target sizes are unchanged, and the new nav hover rule is decoration only.
- Responsive: ok. At 390 there is no sideways scroll, titles wrap cleanly ("This website and its Rust CMS", "This page isn't here"), and nothing overlaps the stack beyond the deliberate stamp.
- Accessibility: #1. Landmarks and headings are unchanged and the scan is clean.
- Copy: ok. The colophon says "Set in Inter." The doc is in first person and its facts trace to the repo. #4 is about the editor preview's fidelity, not copy.

## Acceptance checks
10. Pass: `home-1440-light/dark`. Off-white page, maroon Inter headline in 5 lines, white, peach and rose slips, crimson stamp inside the stack. Dark: #280000 page, #570000 top slip, coral stamp, coral dates and descriptions.
11. Pass: the computed colours in my Tab run (tags, pager, back link, footer: rgb(177,15,46) in light and rgb(222,124,90) in dark), and the crops show each one underlined. Wordmark, menu and list titles are ink in both themes (`work-1440-dark`, `home-1440-*`).
12. Pass: `c-article-head-dark` shows an upright dek and `c-ref-text-dark` a true italic "emphasis".
13. Pass: the 390 crops and full shots show no clipping or overlap.
14. Pass: `c-article-code-dark` shows the 1px edge, with off-white code on maroon.
15. Pass for rings, by `--focus` and the manual Tab run above. But the heading anchors on that path fail contrast (#1).
16. Pass: `favicon.svg` and the `base.html` diff hold the new hexes.
17. Pass: og.png opened. "Nows" in maroon Inter on off-white, the role line clear of the slips, a crimson stamp.
18. Board read from the builder's `dashboard-dark.png`: Inter, readable. The editor preview's slab sits on `--field` #570000 in dark, so it shows. The builder's note on this doesn't hold.
19. Pass: the doc and the skill name the five colours, the roles, both "never as text" rules, the accent and secondary rules, Inter, and the type and contrast tables.

## Findings
- #1 medium: the "#" heading anchors (`.prose .heading-anchor`, `color: var(--rule-strong)`) show at 2.57:1 in light (#BB9999 on #FDFFFF) and 2.50:1 in dark (#684C4C on #280000). A keyboard visitor who Tabs into an article lands on four of these and sees a faint "#". Axe misses it because the anchor sits at opacity 0 during the scan. It was also low before (2.39/2.45), but this order re-picks every colour and promises AA in both themes. Fix: `color: var(--ink-soft)` (14.89 light, 6.55 dark), and add the pair to the contrast tables.
- #2 low: the standfirst on "How this site is built" ends with "run." alone on its last line at 1440 (`c-article-head-dark.png`). Inter is wider than the old italic, so deks wrap differently now. Fix: `text-wrap: pretty` on `.article-dek, .case-dek` (a type-only property, within Q3).
- #3 low: the pager link's underline now runs under both "Newer" and "Writing reference" as two separate lines (`c-article-foot-*.png`), so one link looks like two. Fix: wrap the title in a span in `article.html` and `work-page.html` and underline only that span, leaving the hint plain.
- #4 low: the stage B editor preview (`.preview`, `font: ... Georgia`) still shows prose in a serif. That serif imitated Newsreader; the site's prose is now Inter, so Nows previews a look the site no longer has. Fix: the system sans stack (`system-ui, -apple-system, "Segoe UI", sans-serif`), keeping the worker's no-webfont rule.
