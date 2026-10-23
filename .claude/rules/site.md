---
paths:
  - "site/**"
---

# Working in site/

- Load the `zola-site` skill before editing templates, Sass, scripts or `_headers`. Zola 0.23 uses Tera 2, which differs from most examples online.
- Load the `design-system` skill before any change to how a page looks or moves.
- Scripts and styles live in their own files under `static/js/` and `sass/`, because the site's CSP allows same-origin files only.
- Check with `zola --root site build`; look at the result with `node tools/shots/shoot.mjs` (see the `ui-ux-review` skill).
