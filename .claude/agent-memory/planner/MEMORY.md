# Planner memory

Notes that make future plans better. Keep under 150 lines; merge and prune.

## The repo
- Drafts on purpose: `site/content/articles/writing-reference.md` (shows every prose element; use it to check prose styling) and the case studies `bank-reconciliation.md` and `donation-dashboard.md` (waiting for Nows's facts).
- A change to `base.html`, a partial, `_tokens.scss`, `_base.scss` or `_chrome.scss` affects every page; plan design review across all pages.
- `.pager` (in `_prose.scss`) is shared by `article.html` and `work-page.html`; editing its rules changes case studies too. Its links are `max-width: 48%` + 1rem gap, so adding `flex-wrap` wraps them below ~400 px. Prefer a new class.
- Few published pieces: an article can have no neighbours, so plan checks for both "pager empty" and "pager present" (use `--drafts` to get neighbours).
- Content rules are defined once in `crates/cms-core/src/article.rs`; the Decap fields in `site/static/admin/config.yml` mirror them. Plan both when a rule changes.
