---
name: kaizen
description: Review how the factory line has been running and propose improvements to its agents, skills and checks.
disable-model-invocation: true
---

# Kaizen review

Kaizen is the line improving itself: each rework, finding and lesson is evidence about where the process lets defects through. You read that evidence, find the patterns, and propose small, specific changes. Nows chooses which ones happen.

## 1. Gather the evidence

- `factory status --all --json`: every order, its events (`fail` notes, `andon` reasons), findings and rework count.
- `.factory/memory/lessons.md`: the lessons the approver saved.
- `.claude/agent-memory/*/MEMORY.md`: what each station has been teaching itself.
- `.factory/orders/*/approval.md` for the most recent orders.

Done when you have read every closed order since the last kaizen review (find it in lessons.md: "Kaizen review held").

## 2. Measure

Work out, and show Nows in a short table:

- Orders done and scrapped; first-pass yield (done with zero rework, over all done).
- Reworks by the station that sent the work back, and the finding titles behind them.
- Findings by severity and by station.
- Line stops and their reasons.

## 3. Find the patterns

A pattern is the same kind of defect at least twice, or one defect that reached approval or production. For each, ask: which station could have prevented it earliest, and what was missing there: a check in a skill, a step in an agent, a test in `cms-core`, a CI job, a lint?

## 4. Propose

Up to five proposals, most valuable first. Each one:

- **Evidence**: the orders and findings behind it (WO-0004 #2, WO-0007 #1).
- **Change**: the file and the exact edit, quoted, kept small.
- **Effect**: which defect it prevents and at which station.

Prefer moving a check earlier on the line (into the planner's or builder's steps) over adding a later review. Prefer a test or lint, which runs every time, over an instruction, which an agent may skip. When a lesson is now covered by a proposal, say so.

Ask Nows which to apply with AskUserQuestion (multiple choice, one option per proposal).

## 5. Apply what Nows chose

- Changes to skills, CI, tests or tools: open a work order through `/factory`, with the chosen proposals as the request.
- Changes to `.claude/agents/` or `.claude/settings.json`: these are the line's guardrails, which the builder cannot edit. Show Nows each diff and apply it yourself once approved (the settings ask before those edits).

Finish with `factory learn "Kaizen review held: <one line on what changed>"`, so the next review knows where to start.
