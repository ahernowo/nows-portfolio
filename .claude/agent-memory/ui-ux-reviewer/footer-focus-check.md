---
name: footer-focus-check
description: The shots tool's --focus stops after 10 tab stops, so article and case-study footer links need a separate check; how to do it within the reviewer's write limits
metadata:
  type: reference
---

`tools/shots/shoot.mjs --focus` photographs only the first 10 tab stops. On an article it never reaches `.article-foot` (tags, pager, back link), so "0 focus problems" says nothing about footer links.

**How to apply:** when a change touches a footer or anything low on a long page, also run an inline Playwright script (`node --input-type=module - <<'EOF'`). The Write guard limits this station to its memory folder, so write no script files. Load Playwright with `createRequire(root + "/tools/shots/package.json")`. Build with `zola --root site build [--drafts] --base-url http://127.0.0.1:PORT --output-dir <scratch> --force`, serve with node `http`, focus the link before the target, press Tab, read `getComputedStyle(document.activeElement)`, and clip-screenshot the area into `target/shots/<id>/`. Full-page phone shots are too tall to read: crop with `clip` + `fullPage: true`.

Articles need checking both with and without `--drafts`: only one article is published, so the pager appears only with drafts. See [[builder shots-on-windows]] for Git Bash path quirks (`MSYS_NO_PATHCONV=1`).
