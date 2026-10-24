# Soften the palette and make light the default theme

Soften the colour palette, with a light theme as the default. Colours and theme behaviour only, no layout changes. Update the design-system skill and docs/design-system.md so every later change follows it.

Light theme (the default for every visitor, regardless of their system setting): background #FAF5F3, surfaces #FFFFFF, text #2B1618, muted text #6B5557, accent and links #B10F2E, Reconciled stamp #B10F2E.
Dark theme (only when the visitor chooses it with the Theme button, and remembered afterwards): background #1F1B1C, raised surfaces #2C2527, text #F2EDEB, muted text #C2B6B3, accent #E8896A, Reconciled stamp #E8546D.

The Theme button keeps working. Its label should say which theme is showing, and Light is the starting state. The logo follows the text colour, so it needs no change. Both themes must keep WCAG 2.2 AA contrast, with a contrast table in the design report. The meta theme-color and the OG image script should use the light background.
