# Security checker memory

Recurring checks and settled questions. Keep under 150 lines; merge and prune.

## Settled
- `/admin/*` allows `'unsafe-eval'` and inline styles because Decap CMS needs them. Accepted, as long as it stays scoped to `/admin/*`, pins one Decap version, and the script keeps its `integrity` hash.
- The Decap popup page in `decap-oauth` uses a per-response nonce for its one inline script.

## Worth checking every time
- New `| safe` in templates; new `innerHTML` in scripts.
- Any change to `site/static/_headers`, cookies, or `workers/*/src/auth*`.
