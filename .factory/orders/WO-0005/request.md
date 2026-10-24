# Port the donor-journey motion to the work page

Port the donor-journey motion from design/journey into the real site.

Reference: design/journey/index.html is the approved mock-up; design/journey/src/journey.js and journey.css are the scene, page.js and page.css are the page behaviour and styles. Match what it looks and moves like.

Do:
- Work page template (work-page.html): hero with the animated scene, the pinned five-step journey (scroll height 280svh), the video block, the "what changed" bars, and a short "how this page moves" table. Copy comes from front matter and content, with `<!-- Nows: ... -->` where facts are missing. Keep the mock-up's sample text out of the live page.
- JavaScript: one file, site/static/js/journey.js (scene plus page behaviour). No inline scripts, no outside scripts.
- Styles: move the CSS into site/sass using the site's existing tokens (light default, dark toggle, Inter self-hosted). No Google Fonts link.
- Media: copy journey.mp4 and journey-poster.jpg to site/static/images.
- Security: do not change site/static/_headers. Inline style="" attributes are not allowed by our CSP, so replace the mock-up's style="--w:..." on the bars with classes or data attributes, and set values from JS through element.style only.
- Reduced motion: keep the finished-state fallback, no pinning, no autoplay.
- Check: zola build, cargo run -q -p newpost -- --check, screenshots at 1440 and 390 wide in light and dark, and the accessibility scan. Report anything I need to decide.

Leave the order uncommitted for me to review.

Files referred to:
- design/journey/index.html
- design/journey/src/journey.js, journey.css, page.js, page.css (also present: src/page.html, src/logo.svg.txt)
- design/journey/journey.mp4, journey-poster.jpg
