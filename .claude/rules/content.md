---
paths:
  - "site/content/**"
  - "site/static/admin/**"
---

# Writing content

- Load the `cms-content` skill. Start new pieces with `cargo run -q -p newpost -- "Title"` (add `--work` for a case study) and check with `cargo run -q -p newpost -- --check`.
- Every fact about Nows comes from Nows or the repo; mark a missing fact with `<!-- Nows: ... -->`.
