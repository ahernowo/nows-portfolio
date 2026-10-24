# Re-skin the site with a new palette and Inter

Re-skin the site with a new palette and typeface, and record them in the design-system skill and docs/design-system.md so every later change follows them. Change colours and type only, not the layout.

Palette (the owner's choice, use exactly these):
- #FDFFFF off-white
- #B10F2E crimson
- #570000 deep red
- #280000 near-black maroon
- #DE7C5A coral

Roles: Light theme: background #FDFFFF, text #280000, links, buttons and the Reconciled stamp #B10F2E. Dark theme: background #280000, raised surfaces #570000, text #FDFFFF, accent #DE7C5A. #DE7C5A is never used for text on light backgrounds, and #B10F2E is never used for text on dark ones. Tints of these colours are allowed for borders and surfaces. Both themes must keep WCAG 2.2 AA contrast, and the reports must include a contrast table.

Typeface: Inter, variable, self-hosted as woff2 from site/static/fonts with its OFL licence file next to it, for headings and body. The look is similar to apple.com: large headings with tight letter-spacing, generous white space. Remove the old fonts, and update the OG image script and the CSP font rules if they need it.
