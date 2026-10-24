# Security check: Add a back-to-writing link to every article

## Verdict
Pass. No findings.

## Surfaces touched
- Static pages: `site/templates/article.html` (pager `<nav>` guarded by `if page.higher or page.lower`; one new `<p class="back-link">` with a link built by `get_url(path='@/articles/_index.md')`).
- Styles: `site/sass/_prose.scss` (`.back-link` rule, lines 455-462: margin, weight, stretch only).
- Agent memory notes (builder, planner, ui-ux-reviewer), read for agent-safety. No headers, Decap admin, workers, database, content, dependencies, CI or Claude Code settings changed (`git diff HEAD --stat` over `site/static`, Cargo files, `.claude/settings.json`, `.claude/agents`, `.github`, `tools`, `workers`, `crates`: empty).

## Checklist
- Secrets: ok. The token/key regex over `git diff HEAD`: no match. The same over untracked files: no match. `git check-ignore` confirms both `.dev.vars` files are ignored; `.gitignore` still covers `/target/`, `/site/public/`, `.dev.vars`, `.env`, `.env.*`.
- Output encoding and XSS: ok. `git diff HEAD -- site/templates | grep '| safe'`: no match. The new link has a fixed text and an `href` from `get_url` on a fixed internal path, so no visitor or editor data reaches it. The pager values (`page.higher.title`, `.permalink`) are unchanged lines and stay auto-escaped. No JS changed (the innerHTML/eval grep over `*.js`: no match).
- Headers and CSP: ok. `site/static/_headers` unchanged (read: `/*` still `script-src 'self'; style-src 'self'`, HSTS, nosniff, DENY, Referrer-Policy, Permissions-Policy, COOP; `/admin/*` still pins decap-cms@3.17.1). After `zola --root site build`: no bare `<script>`, no inline script other than `application/ld+json`, no `style="` in `site/public`. The built link is `<a href=https://nows-portfolio.pages.dev/articles/>`, same origin.
- Auth, sessions, CSRF: not applicable; no worker, cookie or auth code changed.
- Database: not applicable; no SQL or migrations changed.
- Publishing to git: not applicable; no CMS publishing code changed.
- Dependencies: not applicable; no Cargo or lockfile change, and no third-party script, style or font added to the site.
- Agent and tooling safety: ok. No change to settings, agents, hooks, `factory guard` or the seal. The new memory notes (`.claude/agent-memory/builder/shots-on-windows.md`, `.claude/agent-memory/ui-ux-reviewer/footer-focus-check.md`, three MEMORY.md edits) are how-to notes for screenshots; none tells an agent to loosen a guard, read secrets or touch paths outside the repo and scratch folder. The reviewer note to run Playwright inline rather than write script files respects the Write guard; its output goes to ignored `target/shots/`, outside the seal.
- Privacy: ok. No personal details, analytics or embeds added.

## Findings
None.
