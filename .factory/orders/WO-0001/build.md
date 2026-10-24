# Build: Add a back-to-writing link to every article

## What changed
Every article now ends with a "Back to all writing" link to `/articles/`, the last child of the article footer, built with the same `get_url(path='@/articles/_index.md')` as the breadcrumb. The "More writing" pager `<nav>` now renders only when the article has an older or newer neighbour, so the one published article no longer shows an empty ruled box and an empty navigation landmark. One `.back-link` rule in `_prose.scss` gives the link its spacing and the pager's title voice (650 weight, 88% width). Colour, underline and focus come from the global `a` and `:focus-visible` rules.

## Files
- ` M site/templates/article.html`: guard the pager nav with `if page.higher or page.lower`; add the back link paragraph.
- ` M site/sass/_prose.scss`: `.back-link` rule after `.pager-next` (margin 2.5rem top, weight 650, stretch 88%).
- ` M .factory/board.json`, `?? .factory/orders/WO-0001/`: written by the factory CLI.
- ` M .claude/agent-memory/planner/MEMORY.md`: the planner's memory, already modified before this build.

## Checks
- `cargo fmt --all --check`: ok
- `cargo clippy --workspace --all-targets -- -D warnings`: ok, finished with no warnings
- `cargo test --workspace`: 65 passed, 0 failed
- `zola --root site build`: ok, 5 pages, 2 sections, no warnings
- `zola --root site check --skip-external-links`: ok
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- `node tools/shots/shoot.mjs --out target/shots/WO-0001 --pages /articles/how-this-site-is-built/,/work/this-website/ --focus`: 0 console/network/layout problems, 0 axe WCAG 2.2 AA failures, 0 focus problems
- Same with `--drafts --pages /articles/how-this-site-is-built/,/articles/writing-reference/,/work/,/work/this-website/ --out target/shots/WO-0001-drafts`: 0 problems, 0 axe failures

## Acceptance checks
1. Pass: `zola --root site build` has no warnings.
2. Pass: newpost check reports no problems.
3. Pass: fmt, clippy, test are all green (see above).
4. Pass: `zola check --skip-external-links` is ok.
5. Pass: in `site/public/articles/how-this-site-is-built/index.html` "Back to all writing" appears once (`grep -o | wc -l` = 1), inside `<footer class="article-foot wrap-text">`, as `<a href=https://nows-portfolio.pages.dev/articles/>`.
6. Pass: the same file has 0 matches for `aria-label="More writing"`.
7. Pass: with drafts, `articles-how-this-site-is-built-1440-light.png` shows the rule, then "Newer / Writing reference", then the back link on its own line. `articles-writing-reference-390-dark.png` shows "Older / How this site is built", then the back link (both in `target/shots/WO-0001-drafts/`).
8. Pass from builder, design to confirm: `target/shots/WO-0001/articles-how-this-site-is-built-{390,1440}-{light,dark}.png`. The link is the last thing before the site footer, left-aligned with the prose column, ink colour with the light `--rule-strong` underline, about 2.5rem below "Filed under". Computed font is Archivo, weight 650.
9. Pass: in a Playwright script against a local build, pressing Enter on the focused link landed on `http://127.0.0.1:4711/articles/`, titled "Writing | Nows".
10. Pass: the shots tool stops after 10 tab stops (`shoot.mjs` line 201), before the article footer, so I tested this by hand. Focusing the last tag link and pressing Tab lands on "Back to all writing", with `outline: solid 2px`. Screenshots at 390 light and 1440 dark show the ring clearly. With drafts, the pager link sits between the tags and the back link in DOM order.
11. Pass: the axe scan has 0 rule failures with and without drafts. Without drafts the empty pager nav is gone (check 6).
12. Pass: of the site files, `git diff --stat` lists only `site/templates/article.html` and `site/sass/_prose.scss`. `git diff` adds no `style=` and no `<script>`. The other entries in `git status` are factory and planner-memory files, listed above.
13. Pass: `site/public/work/*/index.html` has 0 "Back to all writing". `work-this-website-1440-light.png` (drafts) shows the case-study pager as before. The `.pager` rules were not edited.

## Notes for reviewers
- Start with the footer of `/articles/how-this-site-is-built/` at 390 px, both with and without drafts. Then look at the gap between the tag line and the link when there is no pager (it looks a little loose at 1440, but it matches the pager's 3rem rhythm).
- Tap target: the `<a>` box is 17 px tall, its paragraph 24.8 px. Nothing else is clickable within 2.5rem above it or within the footer gap below, so the WCAG 2.5.8 spacing exception holds. The design reviewer should confirm.
- The link uses the same bold voice as the pager titles, but unlike the pager titles it keeps the underline. The plan asked for that, so it reads as a link rather than a heading.
