+++
title = "Donation website"
template = "work-page.html"
description = "What I will build and run on a donation website from November 2026: its development, documentation and monitoring."
date = 2026-10-24
weight = 5
# Stays a draft until Nows confirms the role title below.
draft = true
include_in_feeds = false

[extra]
year = "2026"
role = "Donation website development, documentation and monitoring" # Nows: confirm the exact role title
status = "Starting soon"
stack = ["Salesforce (NPSP)", "Payment gateway", "Power BI and Excel automation", "InDiGO"]

# The donor journey. Five steps, one per part of the illustration; the
# walkthrough scrolls through them in this order. The donor, the amounts and
# the reference are sample data and live only inside the illustration.
[extra.journey]

[[extra.journey.steps]]
title = "Opens the link on her phone"
body = "A message arrives with a link to the donation page."

[[extra.journey.steps]]
title = "Chooses a gift"
body = "She picks an amount and keeps the monthly option."

[[extra.journey.steps]]
title = "Fills in her details"
body = "Name, email and phone, then a tick for an emailed receipt."

[[extra.journey.steps]]
title = "Pays"
body = "She chooses an e-wallet and confirms. The button shows the payment is being confirmed, then the page says thank you."

[[extra.journey.steps]]
title = "Three systems record the gift"
body = "The donation form, the payment gateway and the CRM each hold a record of the same gift. When the three match, the gift is stamped reconciled."
+++

<!-- Nows: confirm the exact role title -->

<!-- Nows: the recording (site/static/images/journey.mp4 and journey-poster.jpg) still shows the "Stored safely" line about how records are kept, which this page does not claim. It is hidden from this page and left out of the build (ignored_static in site/zola.toml, show_recording in partials/journey.html) until it is re-recorded without that line. -->

## What I will build and run

From November 2026, I will work on a donation website: its development, its documentation and its monitoring.

My first responsibilities will be SOPs (standard operating procedures) and monitoring tools.

I will work with Salesforce (NPSP), the payment gateway, Power BI and Excel automation, and InDiGO.

There are no results yet, so this page describes the plan, not an outcome. The illustration above follows one gift with sample data; it is not a record from the website.

## How this page moves

Each effect uses the lightest tool that can do it. The site's security rules allow no outside scripts, so none of this needs a library.

| Effect | Built with | JavaScript |
|---|---|---|
| Title rises, scene fades up | CSS keyframes, played once on load; skipped after the title morph from the Work list | None |
| Breathing and blinking | Two looping CSS keyframes on the character; the Pause animation button stops them | None |
| First beat in the hero | The phone lifts, the screen wakes and a thumb picks a gift, once, then the scene rests | About 50 lines, with the pause |
| Pinned journey | `position: sticky` inside a tall section; one scroll listener turns the position into one progress number, and the scene is drawn from that number alone | About 50 lines, plus the scene |
| Character and hand | Inline SVG; the head, eyes and mouth change with progress, and the finger is a line whose end moves to each button | About 190 lines (the scene) |
| Reading line | A CSS scroll timeline | None |
| Reduced motion | The page starts from the finished picture; pinning and idle motion are off, and the steps are a plain list | About 30 lines, shared |
