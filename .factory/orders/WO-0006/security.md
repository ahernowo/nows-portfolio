# Security check: Reframe the donation case study as an honest plan

## Verdict
Pass. No findings from this station.

## Surfaces touched
Static pages only: one draft case study (`site/content/work/donation-website.md`), two templates (`work-page.html`, `partials/journey.html`), `journey.scss`, `journey.js`, the Decap field list (`site/static/admin/config.yml`), `site/zola.toml` (`ignored_static`), two skills, one doc and three agent memories. Not touched (one `git diff HEAD --quiet` over them, "unchanged"): `site/static/_headers`, `site/static/admin/index.html`, `.gitignore`, `.claude/settings.json`, `.claude/agents`, `.github`, `tools/factory/src`, `Cargo.toml`, `Cargo.lock`, `workers`, `crates`.

## Checklist
- Secrets: ok. The token/key grep over `git diff HEAD` found nothing (rc 1). The untracked-file grep (`.factory/orders/WO-0006/`, `design/journey/` text files) found nothing (rc 123 = no match). `.dev.vars` in both workers still ignored.
- Output encoding and XSS: ok. No new `| safe` in templates. No `innerHTML`, `eval` or similar in the JS diff. The change only removes code: the bars section, which took `row.label` / `row.value` (Tera-escaped) and `data-share` (parsed and clamped). The new `<p class="jn-sample">` is fixed text. `node --check site/static/js/journey.js` ok.
- Headers and CSP: ok. `_headers` unchanged, and byte-identical in the build output (`cmp`). After `zola build --drafts` into a scratch dir: no inline `<script>` without `src` except ld+json, no `style="` in any HTML. The page has no `<video>`, no `journey.mp4` or `journey-poster` reference, no `data-share`; 0 HTML comments (the `<!-- Nows -->` markers are stripped). The dormant video code (`journey.js` section 4) runs only inside `if (video && videoButton)`, and the motion handler is guarded by `if (video && ...)`, so it does nothing without the element.
- Auth, sessions, CSRF: not applicable. No worker, cookie or auth file changed.
- Database: not applicable. No worker or migration changed.
- Publishing to git: not applicable. `cms-core` unchanged. Removing the Decap `compare` field only narrows what an editor can write; no content uses it.
- Dependencies: not applicable. No Cargo or npm change; no third-party script, style or font added.
- Agent and tooling safety: ok. Settings, agents, hooks and the seal are unchanged. I read the three memory diffs: builder (grid note, `ignored_static`, Python LF/CRLF handling in `target/`), planner (mock in the scratchpad, `ignored_static`, stale supervisor notes), ui-ux-reviewer (Playwright `require` by absolute path to the repo's own `tools/shots/node_modules`). None loosens a guard or reaches outside the repo or scratchpad. `design/` is untracked and unchanged since the order started (`find design -newer request.md` is empty).
- Privacy: ok. The new facts (start in November 2026, the role's scope, the four tools) are the ones the request lists. No employer name, email or phone added. Rina, DN-2047 and the Rp amounts are labelled sample data.

## Deploy note for Nows (not a finding)
`ignored_static` works: a plain build into a scratch dir has no `journey.*` file in `images/`, and the draft page is not built. The source files keep their hashes (`64dbd92f...` mp4, `0974e07c...` poster), and `git status site/static/images` is clean. But `40eac14` is already on origin/main, so the next deploy removes the files from the live site, while git history and caches keep them for a while. They hold no secret. The only problem is the "Stored safely" line in them, which nobody has confirmed. If that matters, deploy soon after this order is approved.

## Findings
None from this station. Design's #1 and #2 are low and outside security.
