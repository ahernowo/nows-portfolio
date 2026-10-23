# Planner memory

Notes that make future plans better. Keep under 150 lines; merge and prune.

## The repo
- Drafts on purpose: `site/content/articles/writing-reference.md` (shows every prose element; use it to check prose styling) and the case studies `bank-reconciliation.md` and `donation-dashboard.md` (waiting for Nows's facts).
- A change to `base.html`, a partial, `_tokens.scss`, `_base.scss` or `_chrome.scss` affects every page; plan design review across all pages.
- Content rules are defined once in `crates/cms-core/src/article.rs`; the Decap fields in `site/static/admin/config.yml` mirror them. Plan both when a rule changes.
