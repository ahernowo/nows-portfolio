+++
title = "Donation website"
template = "work-page.html"
description = "One gift followed from a donor's phone through the donation form, the payment and the three systems that record it."
date = 2026-10-24
weight = 5
# Stays a draft until Nows has filled in the facts marked below.
draft = true
include_in_feeds = false

[extra]
year = "" # Nows: the year (or years) you built it
role = "" # Nows: your role, in a few words
status = "" # Nows: for example Live, Internal tool, Retired
stack = [] # Nows: the tools and systems you used

# The donor journey. Five steps, one per part of the illustration; the
# walkthrough scrolls through them in this order. Before-and-after bars appear
# only when [[extra.journey.compare]] rows are added (label, value, share 0 to
# 100, after = true on the new way).
[extra.journey]

[[extra.journey.steps]]
title = "Opens the link on her phone"
body = "A message arrives with a link to the donation page."

[[extra.journey.steps]]
title = "Chooses a gift"
body = "She picks Rp150.000 and keeps the monthly option."

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

<!-- Nows: this case study is a draft. Before it goes public, please add:
- What you built and your role on the donation website, in your own words (a paragraph or two here, above "How this page moves").
- The year, your role, the status and the stack, in the front matter above.
- A description in your words, if the one above doesn't fit.
- One real sentence per step, if you want the walkthrough to say more than what the illustration shows.
- The before-and-after figures, as [[extra.journey.compare]] rows (label, value, share 0 to 100, after = true on the new way). The bars stay hidden until then.
- The recording (images/journey.mp4 and its poster) still shows a line about records being encrypted and access-limited, which the live illustration dropped. Re-record it from this page, or confirm the line is true and keep it.
-->

## How this page moves

Each effect uses the lightest tool that can do it. The site's security rules allow no outside scripts, so none of this needs a library.

| Effect | Built with | JavaScript |
|---|---|---|
| Title rises, scene fades up | CSS keyframes, played once on load; skipped after the title morph from the Work list | None |
| Breathing and blinking | Two looping CSS keyframes on the character; the Pause animation button stops them | None |
| First beat in the hero | The phone lifts, the screen wakes and a thumb picks a gift, once, then the scene rests | About 50 lines, with the pause |
| Pinned journey | `position: sticky` inside a tall section; one scroll listener turns the position into one progress number, and the scene is drawn from that number alone | About 50 lines, plus the scene |
| Character and hand | Inline SVG; the head, eyes and mouth change with progress, and the finger is a line whose end moves to each button | About 190 lines (the scene) |
| Reading line and before-and-after bars | CSS scroll and view timelines; shown finished where unsupported | 8 lines, to set the bar lengths |
| Recording | A twelve-second H.264 file that plays while half of it is on screen; the button pauses it | About 40 lines |
| Reduced motion | The page starts from the finished picture; pinning, autoplay and idle motion are off, and the steps are a plain list | About 30 lines, shared |
