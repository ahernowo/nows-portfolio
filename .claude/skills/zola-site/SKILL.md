---
name: zola-site
description: Conventions for the Zola 0.23 site in site/ (Tera 2 templates, components, content front matter, Sass, static files, headers). Use when editing anything under site/, or when a Zola build fails.
---

# The Zola site

Zola 0.23 builds `site/` into `site/public/` (git-ignored). Build from the repo root with `zola --root site build`; preview with `zola --root site serve` at http://127.0.0.1:1111. Cloudflare Pages runs the same build on every push to `main`.

## Layout

| Path | What lives there |
|---|---|
| `site/zola.toml` | Site settings; `[extra]` holds Nows's public details and the `now_line` |
| `site/content/` | Markdown pages. `_index.md` files are sections; `articles/` and `work/` are the two collections |
| `site/templates/base.html` | The page shell: meta, Open Graph, fonts, scripts, header, footer, search dialog |
| `site/templates/components.html` | Shared components: `work_item`, `article_item`, `figure` |
| `site/templates/partials/` | Header, footer, search dialog, the home hero figure (`receipts.html`) |
| `site/sass/` | `main.scss` imports `_tokens`, `_base`, `_chrome`, `_home`, `_lists`, `_prose`, `_motion` |
| `site/static/` | Copied as is: fonts, `js/head.js`, `js/site.js`, images, `admin/` (Decap CMS), `_headers` |

## Tera 2 templates

Zola 0.23 uses Tera 2, which differs from most Zola examples online:

- **Undefined is an error.** `{{ page.extra.year }}` fails the build when `year` is missing. Write `{{ page.extra.year or "" }}`, or `page.extra?.year`. Only one level may be undefined: guard `a.b.c` with `a?.b?.c or ""`.
- **Components replace macros and shortcodes.** Define once in `components.html`; they are global, with no import:
  ```jinja
  {% component work_item(item: map) %} ... {% endcomponent work_item %}
  ```
  Call with JSX-like tags; non-string values go in braces:
  ```jinja
  {{ <work_item item={page} /> }}
  {{ <figure src="/images/uploads/x.png" alt="..." wide={true} /> }}
  ```
  The same `{{ <figure ... /> }}` syntax works inside Markdown content.
- **Renamed filters**: `escape_html` (was `escape`), `str` (was `as_str`), `divisible_by`, `newlines_to_br`; `truncate(length=n)` needs `length`. `date(format=...)` uses jiff strftime: `%-d %B %Y`.
- **New syntax**: ternary `{{ "a" if x else "b" }}` (both branches required), map literals `{"k": v}`, spread `{...base, "k": v}`, slices `pages[:4]`, list comprehensions `[p.title for p in pages]`. Index arrays with `xs[0]`, never `xs.0`.
- End tags carry the name: `{% endblock content %}`, `{% endcomponent figure %}`.
- Links to content use `get_url(path="@/articles/_index.md")`; tag links use `get_taxonomy_url(kind="tags", term=tag)`; static files use `get_url(path="main.css", cachebust=true)`.

## Content

Articles and case studies set `template` in their front matter; the CMS uses it to tell them apart from the `_index.md` section files in the same folder.

| Kind | Folder | Template | Front matter worth knowing |
|---|---|---|---|
| Article | `content/articles/` | `article.html` | `date`, `description`, `draft`, `[taxonomies] tags` |
| Case study | `content/work/` | `work-page.html` | `weight` (order), `[extra] year, role, status, stack, source` |
| Plain page | `content/` | `page.html` | `about.md`, `now.md` |

Front matter is TOML between `+++` lines. Start new pieces with the CLI so the fields are right: `cargo run -q -p newpost -- "Title" --tags rust,zola` (add `--work` for a case study). Check everything with `cargo run -q -p newpost -- --check`. The `cms-content` skill covers writing.

Markdown extras that are on: smart punctuation, footnotes at the bottom, GitHub alerts (`> [!NOTE]`, `TIP`, `IMPORTANT`, `WARNING`, `CAUTION`), heading anchors on the left, lazy images, `rel="noreferrer"` on external links. Code blocks must name their language (```` ```rust ````); an unknown language fails the build.

## Content Security Policy

`site/static/_headers` sets a strict CSP for every page: scripts and styles from the same origin only. So:

- JavaScript goes in `static/js/` as a file; templates hold no inline `<script>` except `type="application/ld+json"`.
- Styling goes in `sass/`; templates and Markdown hold no `style=""` attributes. Syntax highlighting uses classes (`giallo.css`) for the same reason.
- Images, fonts and media are self-hosted under `static/`. A third-party embed needs a CSP change, which the security checker must review.
- `/admin/*` has its own, looser policy for Decap CMS. Keep that block's rules separate from the site's.

Cloudflare Pages `_headers` rules: at most 100 rules, 2,000 characters per line; `! Header-Name` removes a header set by an earlier matching rule.

## Before you finish

- `zola --root site build` passes with no warnings.
- `zola --root site check --skip-external-links` when you changed links or moved pages.
- `minify_html = true` stays on in `zola.toml` (turn it off only while debugging markup).
- New pages appear in the right list (home, `/work/`, `/articles/`) and in the search index.
