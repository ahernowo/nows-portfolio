# Design review: Add a back-to-writing link to every article

## Verdict
Pass. No findings.

## What I looked at
- Diff: `site/templates/article.html` (pager guarded by `if page.higher or page.lower`, back link as last child of `.article-foot`) and `site/sass/_prose.scss` (`.back-link` at line 458). I read both files in full around the change, plus the global `a` and `:focus-visible` rules in `site/sass/_base.scss`.
- `node tools/shots/shoot.mjs --out target/shots/WO-0001-design --pages /articles/how-this-site-is-built/,/work/this-website/ --focus`: 28 shots, 0 console/network/layout problems, 0 axe WCAG 2.2 AA failures, 0 focus problems.
- Same with `--drafts --pages /articles/how-this-site-is-built/,/articles/writing-reference/,/work/this-website/ --out target/shots/WO-0001-design-drafts`: 12 shots, 0 problems, 0 axe failures.
- The shots tool stops after 10 tab stops, before the article footer, so I built the site twice (with and without drafts) into a scratch folder and ran an inline Playwright script against it. It cropped the footer of each article at 390 and 1440 px in light and dark, measured the link, tabbed onto it, and pressed Enter on it. Crops: `target/shots/WO-0001-design/foot/{nod,dr}-<article>-<width>-<theme>.png` and `...-focus.png`. I opened and looked at the 390 light/dark and 1440 light/dark crops without drafts, the 390 light, 1440 light, 390 dark and 1440 dark crops with drafts, and three focus crops.
- Reduced motion: not captured, because no motion changed.

## Checklist
- Identity and layout: ok. A plain underlined link in the footer's voice. No arrow, no card, no caps label, no red. Left edge at x=20 (390) and x=416 (1440), the same as the prose column.
- Typography: ok. Computed Archivo, 650, 88% stretch, 16 px. It matches the pager titles, and the underline keeps it reading as a link.
- Colour and contrast: ok. Text is `--ink` on `--paper` in both themes (rgb 31,36,99 on 252,252,250; rgb 236,238,249 on 20,25,56), underline `--rule-strong`. No raw colour values. The axe scan is clean.
- Motion: not affected.
- Interaction and focus: ok. Tab order is tags, then pager link (when present), then "Back to all writing". It shows `solid 2px` ink outline, offset 3px, clearly visible in both themes (focus crops). The `<a>` is 118x17 px, but the nearest other target is 40 px above and the site footer is 63 px below, so the WCAG 2.5.8 spacing exception holds.
- Responsive: ok. No sideways scroll at 390 or 1440. The link sits on its own line in every case.
- Accessibility: ok. 0 axe failures. Without drafts the page now has only the "Main" nav landmark, so the empty "More writing" landmark is gone. With drafts both pages have "Main" and "More writing". Headings untouched.
- Copy: ok. "Back to all writing" is sentence case, says where it goes out of context, and matches "Writing" in the header and the breadcrumb.

## Acceptance checks
5. Pass: the no-drafts build has "Back to all writing" once, the last child of `.article-foot`, `href` ending `/articles/`.
6. Pass: 0 `More writing` labels in the no-drafts article build. The DOM nav list is `["Main"]`.
7. Pass: with drafts, "How this site is built" shows rule, "Newer / Writing reference" right-aligned, then the back link on its own line 40 px below. "Writing reference" shows "Older / How this site is built", then the back link (`dr-*` crops).
8. Pass: last item before `.site-footer` in all four width/theme combinations. Left-aligned with the prose, in Archivo with ink colour and the light underline, 40 px (2.5rem) below "Filed under".
9. Pass: Enter on the focused link landed on `/articles/`, titled "Writing | Nows", on both builds.
10. Pass: Tab from the last tag link (or the pager link) reaches the back link, with a 2 px ink outline (focus crops).
11. Pass: 0 axe failures with and without drafts. No empty landmark.
12. Pass: of the site files, the diff touches only `site/templates/article.html` and `site/sass/_prose.scss`. No `style=` and no `<script>` added.
13. Pass: `work-page.html` is unchanged. 0 "Back to all writing" in any built `/work/*/` page. The `.pager` rules are unchanged, and no published case study renders an empty "More work" pager.

## Findings
None.

## Notes
- When only an "Older" link is present, its bold title and the back link stack on the left in the same 650/88% voice. The underline and the 40 px gap tell them apart. I judged this fine, not a defect.
