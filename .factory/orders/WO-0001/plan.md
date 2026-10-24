# Plan: Add a back-to-writing link to every article

## Goal
A visitor who reaches the end of any article sees a "Back to all writing" link that takes them to the Writing list (`/articles/`), whether or not the article has an older or newer neighbour.

## Scope
In:
- One link, text exactly "Back to all writing", at the end of the article footer in `site/templates/article.html`, so every article (current and future) gets it with no content edits.
- A small style rule for it in `site/sass/_prose.scss`, next to the pager rules.
- Render the "More writing" pager `<nav>` only when there is an older or newer article. Today the site has one published article, so that nav renders empty: a ruled line with nothing under it, and an empty navigation landmark for screen-reader users. With the new link sitting directly below it, the empty box would show, so this guard is part of doing the link cleanly.

Out:
- Case studies (`work-page.html`): the request says articles. A "Back to all work" link would be a separate order.
- Tag pages, the breadcrumb at the top of the article (already links to Writing), and the Older/Newer labels: none of these were asked for.
- No content files change; no Rust, CMS fields, JS or headers change.

## Changes
- `site/templates/article.html`: inside `<footer class="article-foot wrap-text">`:
  1. Wrap the existing `<nav class="pager" aria-label="More writing"> ... </nav>` in `{%- if page.higher or page.lower %} ... {%- endif %}`. Leave the two pager links inside it exactly as they are.
  2. After that block, as the last child of the footer, add:
     `<p class="back-link"><a href="{{ get_url(path='@/articles/_index.md') }}">Back to all writing</a></p>`
     Use the same `get_url(path='@/articles/_index.md')` call the breadcrumb on line 14 already uses. No arrow character, no icon, no inline `style=""`.
- `site/sass/_prose.scss`: in the "Previous and next links." section (after `.pager-next`, around line 453), add a `.back-link` rule: `margin: 2.5rem 0 0;` and the pager title voice, `font-weight: 650; font-stretch: 88%;`. Colour, underline, hover and focus come from the global `a` and `:focus-visible` rules in `_base.scss` (lines 42-59); do not override them, and use no raw colour values.

## Acceptance checks
1. `zola --root site build` succeeds with no warnings.
2. `cargo run -q -p newpost -- --check` reports no problems.
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass (no Rust changes; they must stay green).
4. `zola --root site check --skip-external-links` passes (a new internal link is added).
5. In the built `site/public/articles/how-this-site-is-built/index.html`, the text "Back to all writing" appears exactly once, inside `<footer class="article-foot`, as an `<a>` whose `href` ends in `/articles/`.
6. The same built file contains no `aria-label="More writing"` (that article has no neighbours, so the pager nav is not rendered).
7. With drafts (`zola --root site serve --drafts`, or `node tools/shots/shoot.mjs --drafts --pages /articles/how-this-site-is-built/,/articles/writing-reference/ --out target/shots/WO-0001`), both articles now have a neighbour: each shows the "More writing" pager with its Older or Newer link above a rule, and "Back to all writing" below the pager, on its own line.
8. On `/articles/how-this-site-is-built/` at 390 px and at 1440 px, in light and dark themes, "Back to all writing" is the last thing in the article before the site footer, left-aligned with the prose column, in Archivo (not Newsreader), ink colour with the usual light underline, and clearly separated from the "Filed under" tag line above it.
9. Clicking the link opens `/articles/` (the Writing list).
10. Tabbing through the article footer reaches the link after the tag links (and pager links when present), and it shows the standard 2 px focus outline (`node tools/shots/shoot.mjs --focus --pages /articles/how-this-site-is-built/`).
11. The accessibility scan from `tools/shots/shoot.mjs` reports no new issues on the article page, and no empty navigation landmark.
12. `git diff --stat` lists only `site/templates/article.html` and `site/sass/_prose.scss`. No `style=""` and no `<script>` added.
13. A case study page (e.g. `/work/` and one entry, drafts included) looks unchanged: no back link, pager as before (the `.pager` rules themselves are not edited).

## Risks
- Design: the link must read as part of the footer, not a button or a card. Reviewer checks no arrow, no all-caps label, no extra red; the stamp red stays rare. Check spacing against the tag line when there is no pager, and against the pager when there is one.
- Accessibility: the link text must make sense out of context (it does: "Back to all writing"). Check the focus ring is visible in both themes and the tap target is at least 24 px tall at 390 px (body line height is 1.55 on 16 px, about 25 px).
- Template: Tera 2 errors on undefined values; `page.higher` and `page.lower` are already tested with `if` on the current lines, so `page.higher or page.lower` is safe. Reviewer confirms the build with and without drafts.
- Security: none expected; one same-site link built by `get_url`, no new scripts, styles from the existing stylesheet, CSP unaffected.
- Truthfulness: no claims about Nows are added.

## Skills the builder needs
zola-site, design-system
