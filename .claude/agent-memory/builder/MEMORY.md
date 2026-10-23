# Builder memory

Gotchas that cost time once. Keep under 150 lines; merge and prune.

## Zola and Tera 2
- `insert_anchor_links = "left"` keeps heading text plain; "heading" mode wraps the text in a link with an aria-label that hides it from screen readers.
- GitHub alerts render as `blockquote.markdown-alert-note` and friends, with no shared base class; style them with `blockquote[class^="markdown-alert-"]`.
- `zola build --output-dir` needs an absolute path, or `get_url(..., cachebust=true)` cannot find `main.css`.

## Workers
- `Response::redirect` headers are immutable; build a 302 by hand when it must set cookies.
- The wasm target may be missing locally; `cargo clippy` natively still catches most errors, and CI builds the wasm.
