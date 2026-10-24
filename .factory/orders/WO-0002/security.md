# Security check: Re-skin the site with a new palette and Inter

## Verdict
Pass. No findings.

## Surfaces touched
- Static site: Sass (colours, type, four local `@font-face` rules), three templates (theme-color metas, font preload, footer colophon, pager title span), `favicon.svg`, `og.png`.
- Third-party binaries: four Inter woff2 files and their OFL licence, taken from Fontsource on jsdelivr. Six Archivo/Newsreader files deleted.
- Tool CSS: the factory dashboard `app.css` and the cms-api editor `admin/app.css`. Both are compiled in with `include_str!`. No Rust source changed.
- `scripts/make-og-image.py`, the design-system and ui-ux-review skills, `docs/design-system.md`, and the agent-memory notes for builder, planner and ui-ux-reviewer.
- Not touched: `_headers`, `.gitignore`, the Decap admin, worker `src/`, `wrangler.toml`, D1, Cargo manifests and lock, `.github`, `.claude/settings.json`, agents, `tools/factory/src` (checked with `git diff HEAD --stat` on those paths: empty).

## Checklist
- Secrets: ok. The diff token grep gave rc=1 (no match). The untracked-files grep gave rc=123 (no match). The four woff2 files were grepped separately: 0. `.dev.vars ignored` printed for both workers, and `.gitignore` is unchanged.
- Output encoding and XSS: ok. There is no new `| safe` in `git diff HEAD -- site/templates`, and no JS changed. The new `<span class="pager-title">{{ page.higher.title }}</span>` (`site/templates/article.html:36-37`, `work-page.html:34-35`) uses Tera's default escaping, the same as the bare title before it.
- Headers and CSP: ok. `zola --root site build` succeeds. In `site/public` there is no bare `<script>`, no inline script other than ld+json, and no `style="` (all three greps rc=1). The site CSP keeps `font-src 'self'`. Every font URL in `main.css`, `index.html` and the dashboard CSS is relative or same-origin (`fonts/inter-*.woff2`, `/fonts/inter-latin-opsz-normal.woff2`). There is no `@import` and no external URL in the CSS. The new files have new names, so the `/fonts/*` immutable cache rule stays correct. The dashboard CSP (`tools/factory/src/server.rs:20`) and `ADMIN_CSP` (`workers/cms-api/src/http.rs:137`) are unchanged. Both allow `font-src 'self'`. The dashboard's `/fonts/` handler (`server.rs:176-189`) still accepts only lowercase names ending in `.woff2` with no `..`, and the new file names fit it. The Decap pin (3.17.1) is unchanged in both `_headers` and `admin/index.html:18`.
- Auth, sessions, CSRF: not applicable. No worker source, cookie or OAuth code changed. The editor change is CSS only.
- Database: not applicable. No SQL, migrations or `db.rs` changed.
- Publishing to git: not applicable. No change to path or front-matter code.
- Dependencies: ok. No Cargo change. The fonts are the only new third-party input. I downloaded the four woff2 files and `LICENSE` from `cdn.jsdelivr.net/npm/@fontsource-variable/inter@5.3.0` and compared SHA-256: all five match the repo files byte for byte. The byte sizes match the plan, and all four start with the `wOF2` magic. They are self-hosted, so the public site still loads no third-party fonts at runtime. Fontsource and Inter are widely used and under OFL 1.1.
- Agent and tooling safety: ok. No settings, hooks, guard or seal logic changed. I read the new memory and skill text. It holds gotchas and design rules, and nothing that addresses agents or loosens a guard. Two notes worth knowing:
  - The builder note that `.claude/agent-memory/` sits outside the seal is true (`tools/factory/src/seal.rs:17`). That is existing design, not a new gap.
  - The builder's "stop a scratch server with `taskkill //PID n //F`" note scopes the kill to the scratch port the agent opened. It stays inside the machine's own processes and the repo, so it is acceptable.
  - The OG script still reads only `site/static/fonts` and writes TTFs to a temp folder, as before.
- Privacy: ok. No personal data was added (grep of the added lines for emails and phone numbers: only a false match on "100-900"). The OG image text ("Nows", role, tag line) and the footer differ only by font name.

## Findings
None.
