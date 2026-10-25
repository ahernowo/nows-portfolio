# Security check: Turn the donor-journey recording back on

## Verdict
Pass. No findings from security.

## Surfaces touched
Static pages only: one partial (`partials/journey.html`), `journey.js` (one `setAttribute` swap and a comment), `journey.scss` (two selectors), `zola.toml` (`ignored_static` removed, so two media files enter the build), one draft case study, docs, two skills, and three agent-memory notes. No change to headers, the Decap admin, workers, the database, dependencies, the factory or Claude Code settings.

## Checklist
- Secrets: ok. The diff grep for token and key patterns found nothing (rc=1). The grep over untracked text files found nothing (rc=123 = no match). Both `.dev.vars` files are still ignored.
- Output encoding and XSS: ok. No new `| safe` in templates. No `innerHTML`, `eval` or string `setTimeout` in the JS diff. In `journey.js` section 4 (lines 376-420), the button state goes in through `setAttribute("data-playing", String(bool))` and the label through `textContent` with fixed strings. No data from the page reaches markup.
- Headers and CSP: ok. `git diff HEAD --quiet` over `_headers`, `static/admin`, `.gitignore`, `.claude/settings.json`, `.claude/agents`, `.github` and `tools/factory/src` reports them unchanged. I built with drafts to `target/sec-wo7` (since removed). It has 0 inline `<script>` apart from 19 `application/ld+json`, and 0 `style=` attributes in any page. `cmp` shows the built `_headers` is identical to the source. The video loads from the same origin and is covered by `default-src 'self'` (there is no `media-src`); the poster is covered by `img-src 'self'`.
- Media files (newly in the build): ok. The built and source sha256 match the plan (`314332fe…f55af76b`, `84287171…a5b13ee2`), and `site/static/images` is unchanged since ce9a775. JPEG marker walk: APP0 JFIF, DQT x2, SOF0, DHT x4, SOS, EOI. No APP1/EXIF/GPS and 0 bytes after the end marker. MP4 box walk: ftyp, moov (one video `trak`, no audio track), `udta` holding only the `Lavf60.16.100` encoder tag, free, mdat. The box sizes add up to the file size (245225), so nothing is hidden or appended.
- Auth, sessions, CSRF: not applicable. No worker, cookie or admin file changed.
- Database: not applicable. No worker or migration changed.
- Dependencies: not applicable. No `Cargo.toml` or `Cargo.lock` change, and no third-party script, style or font was added.
- Agent and tooling safety: ok. Nothing changed in settings, agents, hooks or the seal. The new notes in builder, planner and ui-ux-reviewer memory are about ffmpeg, Chrome via `executablePath`/`channel` (settled in WO-0005), Tab walks and report chunking. None of them loosens a guard or reaches outside the repo. The untracked `design/journey/` is unchanged from the WO-0005 check: no agent-directed text, no fetch/XHR/eval, and Google Fonts links only, none ported. Its old video is not in the build.
- Privacy: ok. The recording shows only the labelled sample data (settled, WO-0005). No personal details were added.

## Findings
None.

## Note for Nows
With `ignored_static` gone, the next push makes `/images/journey.mp4` and `/images/journey-poster.jpg` public, even though the case study stays a draft. That is what you asked for, and the files hold no secret or personal data. The earlier "Stored safely" video is still in git history (40eac14) and may be in caches from before WO-0006. The padlock icon is design's #2 (low): it is a copy question, not a security one.
