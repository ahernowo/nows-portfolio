# Approval: Add a back-to-writing link to every article

## Verdict
Approved.

## Checks I ran
- `cargo fmt --all --check`: exit 0
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0, no warnings
- `cargo test --workspace`: 65 passed, 0 failed (46 + 17 + 1 + 1 across crates)
- `cargo run -q -p newpost -- --check`: "Checked 6 files. No problems."
- `zola --root site build`: 5 pages, 2 sections, no warnings
- `zola --root site check --skip-external-links`: ok
- `zola --root site build --drafts --output-dir <scratch>`: ok, used for checks 7 and 13
- `factory verify WO-0001` after my builds: every gate passes, seal intact

## Request against the change
Asked: a "Back to all writing" link at the end of every article. Built: one `<p class="back-link">` as the last child of the article footer in `site/templates/article.html`, linking via the same `get_url(path='@/articles/_index.md')` the breadcrumb uses, plus a 9-line `.back-link` rule in `site/sass/_prose.scss`. The plan also guards the "More writing" pager so it is not rendered empty; that is in scope with a stated reason (an empty ruled box and empty landmark directly above the new link). Case studies stay out of scope, as the plan says. Nothing in the diff goes beyond the plan. No words about Nows were added.

## Acceptance checks
1. Pass: my `zola build`, no warnings.
2. Pass: my newpost check, no problems.
3. Pass: my fmt, clippy, test runs.
4. Pass: my `zola check --skip-external-links`.
5. Pass: built `articles/how-this-site-is-built/index.html` has "Back to all writing" exactly once, inside `<footer class="article-foot wrap-text">`, `href=https://nows-portfolio.pages.dev/articles/`.
6. Pass: 0 `aria-label="More writing"` in that file.
7. Pass: drafts build, "How this site is built" renders the pager with "Newer / Writing reference", then the back link; "Writing reference" renders "Older / How this site is built", then the back link. I viewed `target/shots/WO-0001-design/foot/dr-writing-reference-1440-light.png`: rule, Older link, back link on its own line.
8. Pass: design report crops; I viewed the 1440 light drafts crop and the 390 dark no-drafts crop: last item before the site footer, aligned with the prose column, Archivo, underlined.
9. Pass: href resolves to `/articles/`, and `site/public/articles/index.html` exists; builder and design both report Enter landing on "Writing | Nows".
10. Pass: I viewed `nod-how-this-site-is-built-390-dark-focus.png`: 2 px outline around the link, visible on dark paper. Tab order from design report.
11. Pass: design and build report 0 axe failures with and without drafts.
12. Pass: `git diff --stat` site files are only `article.html` (+3) and `_prose.scss` (+9); 0 `style=` in the built article; no `<script>` added.
13. Pass: drafts build, 0 "Back to all writing" in all four `/work/*/` pages; `.pager` rules untouched in the diff.

## For Nows
Read first: `git diff site/templates/article.html` (the `if page.higher or page.lower` guard and the new last line), then `git diff site/sass/_prose.scss`. To try by hand: `zola --root site serve`, open "How this site is built", scroll to the end, press Tab past the tags; then `zola --root site serve --drafts` to see it under the pager. The other changes in `git status` are factory records and agent memory notes.

Suggested commit message: `Add a back-to-writing link to every article`
