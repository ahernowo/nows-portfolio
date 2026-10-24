---
name: shots-on-windows
description: Running tools/shots/shoot.mjs from Git Bash on Nows's Windows machine, its limits, and how to crop or drive pages by hand
metadata:
  type: reference
---

- Git Bash rewrites `--pages /articles/x/` into `C:/Program Files/Git/articles/x/`. Prefix with `MSYS_NO_PATHCONV=1`.
- `--focus` stops after 10 tab stops (the `for n <= 10` loop in shoot.mjs). On an article it never reaches the footer. To check a footer link, use a scratch Playwright script: focus the previous link, press Tab, read `getComputedStyle(document.activeElement)`, and screenshot `.article-foot`.
- Full-page phone shots are about 390x4800 and shrink to unreadable in Read. Python has no PIL. To crop, load the PNG as a data URL in Playwright (`setContent`) and take a `clip` screenshot. Opening `file:///x.png` directly does not work, because Chromium's image viewer scales the picture.
- Load Playwright from a scratch script with `createRequire(root + "/tools/shots/package.json")`.
- For a scratch server, build with `zola --root site build --base-url http://127.0.0.1:PORT --output-dir <absolute scratch path> --force`, then serve it with node `http`. A background `zola serve` needs sleeps, which are blocked.
- After `keyboard.press("Enter")` on a link, `waitForLoadState` returns too early. Use `waitForURL`.
- The Write tool is blocked outside the repo (the scratchpad too). Put scratch scripts in `target/` (git-ignored, outside the seal). Reuse the real server: `const { startServer } = await import("file:///" + root + "/tools/shots/serve.mjs")`.
- `page.screenshot({ clip })` below the fold fails ("Clipped area ... outside"). Add `fullPage: true` and add `scrollY` to the box's y.
- An element screenshot right after focus can catch a fade (e.g. `.code-copy` opacity 0.15s). Wait 400 ms first.
- Wait for a background server without sleep: `curl -s --retry 60 --retry-connrefused --retry-delay 1 URL`. Stop it by PID: `netstat -ano | grep "127.0.0.1:PORT .*LISTENING"`, then `taskkill //PID n //F`.
