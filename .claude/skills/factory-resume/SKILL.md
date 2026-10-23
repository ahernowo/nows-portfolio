---
name: factory-resume
description: Continue an open factory work order from the station it is at.
argument-hint: "<WO-id>"
disable-model-invocation: true
---

# Resume a work order

Order: $ARGUMENTS

1. When no order id is given above, run `factory status`. With one open order, use it. With several, ask Nows which one, and continue with that.
2. Run `factory brief` and `factory show <id>`. When the line is stopped, tell Nows why and end here: clearing the andon is their call.
3. Read `.claude/skills/factory/SKILL.md` and follow it from **step 3, Run the line**, with this order id. Steps 4 to 6 of that file apply as written.
