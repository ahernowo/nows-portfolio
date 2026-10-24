# Security check: Add the Nows logo to the header and favicon

## Verdict
Pass. No findings.

## Surfaces touched
- Static pages: a new `| safe` in `site/templates/partials/header.html` inlines `static/images/nows-flow-mark.svg` (read with `load_data(..., format="plain")`) on every page.
- Favicon links in `site/templates/base.html` and `site/static/admin/index.html` (Decap admin page, one line).
- Sass only in `_chrome.scss` (layout, no `url()`, no imports).
- Four new SVG files in `site/static/images/`; old `site/static/favicon.svg` deleted.
- Agent-memory notes (builder, planner, ui-ux-reviewer).
- Not touched: `_headers`, workers, cms-core, D1, Cargo files, `.claude/settings.json`, agents, skills, hooks, tools (`git diff HEAD --stat` over those paths lists only `admin/index.html` and the deleted favicon).

## Checklist
- Secrets: ok. The diff token grep gave no match (rc=1). The untracked-files grep gave no match (rc=123 from xargs, i.e. clean). `git check-ignore` confirms both `.dev.vars` are ignored.
- Output encoding and XSS: ok. The one new `| safe` (header.html, the `mark` line) outputs a fixed repo file chosen by a constant path, with no visitor or editor input. The plan and build report give the reason. I checked what it inlines:
  - Elements in all four SVGs, by a grep over every tag: only `svg`, `title`, `g`, `circle`, `path`, `rect` (the favicon also nests an `svg`).
  - Attributes: only `xmlns`, `viewBox`, `role`, `aria-label`, `fill`, `stroke*`, `d`, `cx/cy/r`, `x/y/width/height/rx`, `preserveAspectRatio`.
  - None of these appear: `script`, `on*=`, `href`, `xlink`, `style`, `url(`, `@import`, `foreignObject`, `<!`, `<?`, entities, `data:`, `javascript:`.
  - The only URL is the `xmlns` namespace, which is never fetched.
  - Path data holds only M/L/C/Z and numbers, with 0 non-printable or non-ASCII bytes.
  - The sha256 of all four files matches the plan (unedited).
  - Rendered output: all 14 built HTML pages carry the same mark (one sha256). Its tags are `span`, `svg`, `g`, `circle`, `path`, with `<title>` removed.
  - No JS changed.
- Headers and CSP: ok, no change needed. After `zola --root site build`, no inline `<script>` without `src` other than ld+json, and no `style=` attribute in any built page. The site CSP keeps `script-src 'self'` with no `'unsafe-inline'`, so even a hostile SVG would have its inline script and `on*` handlers blocked: a second barrier. The favicon loads as an image (scripts never run there) under `img-src 'self'`, on both the site and the `/admin/*` CSP. The Decap pin and `integrity` hash are untouched (`git diff` of `admin/index.html` is the one favicon line).
- Who can change the inlined file: ok. Decap writes uploads to `site/static/images/uploads` (`config.yml` line 32), a subfolder, so it cannot overwrite `nows-flow-mark.svg`. The stage B worker has no upload route. Changing the logo takes repo write access, which can already change templates.
- Auth, sessions, CSRF: not applicable. No worker, cookie or auth code changed.
- Database: not applicable. No D1 or migration changes.
- Publishing to git: not applicable.
- Dependencies: not applicable. No Cargo changes, and nothing new from a third party: the logo is self-hosted.
- Agent and tooling safety: ok. I read the three memory diffs. They hold build tips: `git archive` into `target/`, stopping one's own scratch server by PID (already settled), and `MSYS_NO_PATHCONV`. None loosens a guard or reaches outside the repo. No settings, agents, hooks or seal changes.
- Privacy: ok. No personal data, trackers or embeds added.

## Findings
None.

Note for Nows (not a defect): from now on, whatever sits in `site/static/images/nows-flow-mark.svg` goes into every page as raw HTML. Replace it only with a file you made or exported yourself, and keep it to plain shapes (no `<script>`, `on...=`, `href` or `<style>`). The CSP would still block scripts.
