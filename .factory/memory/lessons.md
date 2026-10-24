# Lessons

One line per lesson, newest at the bottom. Written by `factory learn`. Read this before planning or reviewing.

- 2026-10-24 WO-0001: For any change near the end of a page, write the focus acceptance check as a manual Tab-from-the-last-link check, because shots --focus stops after 10 tab stops and never reaches article or page footers.
- 2026-10-24 WO-0002: Tab onto anything hidden at opacity 0 until focus (heading anchors, Copy buttons) and check its contrast by hand, because the axe scan skips it.
- 2026-10-24 WO-0002: When a palette changes, grep every color: use of each token in site/sass and give each text use its own contrast-table row, because tokens meant for borders (like --rule-strong on the heading anchor) can also colour text.
- 2026-10-24 WO-0004: When a text colour token changes, grep opacity in site/sass and recompute faded text (placeholders, line numbers) against 4.5:1; axe does not check ::placeholder.
- 2026-10-24 WO-0004: When a plan changes a text colour token, list every rule that fades that token (opacity, alpha, color-mix) in site/sass as its own row in the plan's contrast table, so the builder fixes it before design review.
- 2026-10-24 WO-0005: When an order keeps self-starting motion longer than 5 s, gate the looping CSS animations on a class that JavaScript adds, so the no-JS page has nothing endless that the JS pause button cannot stop.
- 2026-10-24 WO-0005: In plans, give a toggle button one state signal (a changing label or aria-pressed, never both), render controls that only work with JS as plain text until JS upgrades them rather than shipping disabled buttons, and place any WCAG 2.2.2 pause control inside the first screen at 390.
- 2026-10-24 WO-0006: On pages with pinned or step-by-step content, run the Tab walk at a narrow width (864 px or less) too and confirm document.activeElement is never BODY after a Tab; WO-0005's narrow story hides the focused step.
