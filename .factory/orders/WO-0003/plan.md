# Plan: Add the Nows logo to the header and favicon

## Goal
Every page shows Nows's flow mark just before the "Nows" name in the header, in the same colour as the name, as part of the one link home. Browser tabs (site and `/admin/`) show the new crimson favicon. Colours, fonts and the header's look stay as they are.

## Scope
In:
- The mark from `site/static/images/nows-flow-mark.svg`, inlined into the header's home link, hidden from screen readers (`aria-hidden`), sized to the name.
- The favicon link pointing at `site/static/images/nows-flow-favicon.svg`, on the site and in Decap's `/admin/` page; the old `site/static/favicon.svg` removed.
- A visible focus ring on the home link (the global `:focus-visible` ring already applies; the plan keeps it whole around mark and name).
- One header breakpoint moved from 40rem to 43rem (see "Why the breakpoint moves").

Out:
- `nows-flow-mark-dark.svg` and `nows-flow-mark-light.svg`: the request does not mention them (supervisor note). Leave them in place, untouched and unused. They will be copied to the public site as unused files; harmless.
- Editing the logo files themselves (viewBox, paths, colours): they are Nows's artwork. The build only strips the `<title>` on the fly (see Changes).
- The footer, the Open Graph image (`og.png`, `scripts/make-og-image.py`), the dashboard and the stage B editor: not asked for.
- `theme-color` metas, tokens, fonts: unchanged by the request.

### Why the mark is inlined, not an `<img>`
`nows-flow-mark.svg` paints with `currentColor`. That only follows the page's text colour when the SVG is part of the page's HTML; inside `<img src>` it is drawn black (no inherited colour) in both themes. So the SVG must be inline. To keep one source of truth, the header reads the file at build time with Zola's `load_data(..., format="plain")` instead of a pasted copy; if Nows replaces the file, the header follows. (Tested in a scratch Zola 0.23.6 site: it works, and `minify_html` keeps the SVG valid; it lowercases `viewBox`, which the HTML parser corrects for inline SVG, the same as the existing search and theme icons.)

### Why the breakpoint moves
Baseline screenshots (`target/shots/WO-0003-before/home-641-light.png`, `-705-`, etc., taken on the current tree) show the one-row header at 641 px has about 29 px free between "Now" and Search. The mark adds about 42 px (36 px wide at 24 px tall, plus the gap). Left alone, the Search and Theme buttons would wrap onto a second, misaligned row between 641 and about 665 px. Moving the header's narrow-layout query from 40rem (640 px) to 43rem (688 px) lets that band use the existing phone arrangement (nav on its own row, icon-only tools) instead of a broken one. Nothing else about the layout changes. The footer's own 40rem query stays.

## Changes
- `site/templates/partials/header.html`: inside the existing `<a class="wordmark" ...>`, before `{{ config.extra.name }}`, add `<span class="wordmark-mark" aria-hidden="true">` holding the inlined mark, then `</span>`. Load the file once at the top of the partial: `{%- set mark = load_data(path="static/images/nows-flow-mark.svg", format="plain") -%}`, then output `{{ mark | replace(from="<title>Nows</title>", to="") | safe }}`. The `replace` drops the SVG's `<title>`, which would otherwise pop up a "Nows" tooltip over the mark. The `aria-hidden` sits on the span, not on the SVG string, so it holds even if a future logo file is shaped differently; the SVG's own `role="img"` and `aria-label` inside a hidden span are ignored by screen readers. Keep `href`, `aria-current` and the link text as they are. Add a one-line Tera comment saying where the mark comes from and why it is inline (`currentColor`).
- `site/sass/_chrome.scss`, `.wordmark`: add `display: inline-flex; align-items: center; gap: 0.375rem;`. Keep every existing declaration (size, weight, tracking, line height, `color: var(--ink)`, no underline, `padding-block`).
- `site/sass/_chrome.scss`, new rules after `.wordmark`: `.wordmark-mark { display: flex; flex: none; }` and `.wordmark-mark svg { width: 1.49em; height: 1em; }` (the file's viewBox is 272.2 by 182.7, so 1.49:1; at the wordmark's 1.5rem that is about 36 by 24 px). No colour on the mark: it inherits `currentColor` from `.wordmark`, which is `--ink` (maroon in light, off-white in dark).
- `site/sass/_chrome.scss`, line 87: `@media (max-width: 40rem)` becomes `@media (max-width: 43rem)` for the header block only (`.header-inner`, `.site-nav`, `.tool-label`, `.tool-kbd`). Add a short comment: the mark made the one-row header need about 42 px more. Leave the footer query at line 151 alone. If the check at 689 px (check 9) still wraps, raise to 44rem and say so in the build report.
- `site/templates/base.html`, line 26: `href="{{ get_url(path='favicon.svg') }}"` becomes `href="{{ get_url(path='images/nows-flow-favicon.svg') }}"`; keep `rel="icon"` and `type="image/svg+xml"`.
- `site/static/admin/index.html`, line 8: `href="/favicon.svg"` becomes `href="/images/nows-flow-favicon.svg"`. Change nothing else in this file (Decap pin and integrity hash stay).
- `site/static/favicon.svg`: delete. Nothing else refers to it (checked with grep across the repo, outside `site/public`, `target` and `.factory`).
- `site/static/images/nows-flow-mark.svg`, `nows-flow-favicon.svg`: add to the change as Nows supplied them, unedited. (They are untracked now; Nows commits them with this order.)

No Rust, content, token, font or `_headers` changes.

## Acceptance checks
Run screenshot commands with `MSYS_NO_PATHCONV=1` in Git Bash, or `--pages /` turns into a Windows path. Baseline shots of the current header are in `target/shots/WO-0003-before/` (home, light, 320/641/700/768; light and dark, 390/705/1024/1440).

1. `zola --root site build` succeeds with no warnings.
2. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` and `cargo run -q -p newpost -- --check` pass (nothing in Rust or content changes; they must stay green).
3. Mark on every page: after the build, `for f in $(find site/public -name index.html -not -path '*/admin/*') site/public/404.html; do grep -q 'wordmark-mark' "$f" || echo "missing: $f"; done` prints nothing.
4. Hidden from screen readers, no tooltip: `grep -o '<span[^>]*wordmark-mark[^>]*>' site/public/index.html` prints one tag that contains `aria-hidden=true` (the minifier may reorder attributes). `grep -c '<title>Nows</title>' site/public/index.html` prints `0`.
5. Accessible name: on `/` (1440 px), the home link's accessible name is exactly "Nows" (DevTools > Accessibility pane on the link, or Playwright `page.getByRole('link', { name: 'Nows', exact: true })` matches one element). A screen reader says "Nows, link" once, not twice.
6. Look, light and dark, at 390 and 1440 px on `/`, `/articles/` and `/about/` (`node tools/shots/shoot.mjs --out target/shots/WO-0003 --pages /,/articles/,/about/ --viewport-only`): the mark sits to the left of "Nows", about 24 px tall, vertically centred on the name, in exactly the name's colour (maroon `#280000` in light, off-white `#FDFFFF` in dark; never crimson or coral). Name, nav and tools look as in the baseline shots apart from the mark, and the header's bottom rule is at the same height as in the matching baseline shot (the header stays 4.25rem tall).
7. Theme switch: on `/` at 1440 px, pressing the Theme button through auto, light and dark recolours the mark together with the name, with no flash of black.
8. Click target: clicking the mark (not the name) on `/about/` goes to `/`; the pointer is the link hand over both mark and name.
9. One-row header: at 689, 705, 768 and 1024 px, light theme, with the Theme label set to each of "Theme: auto", "Theme: light" and "Theme: dark", the mark, name, nav, Search and Theme sit on one row with no wrapping (`--widths 689,705,768,1024 --viewport-only`, then click through the theme labels by hand or in DevTools at 689 px).
10. Narrow header: at 688, 641, 390 and 320 px, the first row holds mark + name on the left and the two icon-only tools on the right; the nav sits on its own row below, as in `home-320-light.png`. The shots summary reports no sideways scrolling at any width.
11. Focus ring: `node tools/shots/shoot.mjs --out target/shots/WO-0003-focus --pages /,/about/ --focus`. The home link (first stop after "Skip to content") shows the 2 px `--ink` ring around both mark and name, not clipped by the header, in light and dark; the summary lists no stop without a visible ring.
12. Accessibility scan: the default `--focus` run above (axe, WCAG 2.2 AA, both themes) shows no new violations compared with the order's base; in particular no `svg-img-alt`, `aria-hidden-focus` or duplicate-name findings on the header.
13. Forced colours: in Chrome DevTools > Rendering > "Emulate CSS media feature forced-colors: active", the mark on `/` is still visible and takes the same system colour as the name.
14. CSP: the shots summary reports 0 console, network or CSP problems on every page captured (the inline SVG carries no `style` attribute and no script).
15. Favicon, site: `grep -o '<link[^>]*rel=icon[^>]*>' site/public/index.html` shows `images/nows-flow-favicon.svg` with `type=image/svg+xml`. `ls site/public/favicon.svg` fails (file gone); `site/public/images/nows-flow-favicon.svg` exists. In a browser at `zola --root site serve`, the tab shows the crimson rounded square with the off-white mark, with the OS in light and in dark mode.
16. Favicon, admin: `site/static/admin/index.html` line 8 points at `/images/nows-flow-favicon.svg`; `git diff site/static/admin/index.html` shows that one line only.
17. No leftover references: `grep -rn "favicon.svg" site/templates site/static docs README.md` shows only the two new `nows-flow-favicon.svg` hrefs.
18. Scope: `git status --short` lists only `header.html`, `_chrome.scss`, `base.html`, `admin/index.html`, the deleted `site/static/favicon.svg`, the four untracked `site/static/images/nows-flow-*.svg` files, and `.factory/`. `sha256sum site/static/images/nows-flow-*.svg` still prints these hashes (the logo files stay as Nows supplied them):
    - `73feb7cb...b7f3255` nows-flow-favicon.svg
    - `57f109a9...79c1dc11` nows-flow-mark-dark.svg
    - `8e33268b...19d6bf8f` nows-flow-mark-light.svg
    - `6d422682...fdf55266` nows-flow-mark.svg

## Risks
- Layout (design): the 40rem to 43rem move changes the header between 641 and 688 px to the phone arrangement. The request says "keep the layout unchanged"; this is the smallest change that avoids the tools wrapping onto a broken second row. Reviewer: compare 641/700 px against the baseline shots and confirm the band reads as the existing phone header, not a new design. Report it plainly so Nows sees it.
- Optical alignment (design): the logo's viewBox has more empty space above the drawing than below (about 5 px over 3 px at 24 px tall) and about 3.5 px on the left. The mark may look slightly low against "Nows" and its left edge sits a hair inside the page gutter. Reviewer: zoom the 1440 px shot; a visible misalignment is a low finding (fix with a small `translate` or margin in Sass, never by editing the logo file).
- Colour (design): the mark must be `--ink` like the name, not the accent; the accent never goes to the wordmark (design-system rule). Check both themes.
- Duplicate announcement (accessibility): if `aria-hidden` lands on the wrong element, screen readers hear "Nows, image, Nows, link". Checks 4 and 5 cover it.
- Silent `replace` (maintenance): if Nows later supplies a logo whose `<title>` text differs, the tooltip returns. Harmless to screen readers (the span stays hidden). Reviewer: note only.
- Security: inline SVG goes into every page as raw HTML (`| safe`). The source is a repo file Nows supplied, read at build time, not user input. Security checker: confirm the four SVGs hold only `svg`, `g`, `path`, `circle`, `rect`, `title` (no `script`, `on*` attributes, `href`, `foreignObject` or external references), and that no CSP change was needed. The favicon is loaded as an image, where scripts never run.
- Truthfulness: none; no words about Nows change.

## Skills the builder needs
zola-site, design-system (and ui-ux-review for the screenshot tool when self-checking)
