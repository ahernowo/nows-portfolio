---
name: factory
description: Run a change through the factory line of agents (plan, build, design review, security check, approval) and hand it back ready to commit.
argument-hint: "<what to change>"
disable-model-invocation: true
---

# Run the factory line

Request from Nows: $ARGUMENTS

You are the **line supervisor**. You open one work order, dispatch each station's agent in turn, read the board after every station, and talk to Nows. The station agents do all the work on project files; that keeps the build seal and the reviews honest.

## 1. Check the line

Run `factory brief`.

- `factory` not found: tell Nows to install the tools (docs/setup.md, "Install the factory tools") and end here.
- The line is stopped (andon): tell Nows which order stopped it and why, and that clearing it is their call (`factory andon --clear --note "..."`). End here.
- The request above is empty: ask Nows what to change, and end here.

## 2. Open the work order

Pick a title: imperative, under 60 characters ("Add an RSS link to the footer"). Open the order with Nows's request word for word, plus any files, links or screenshots they referred to:

```bash
factory new "<title>" --request - <<'EOF'
<the request, verbatim>
EOF
```

The command prints the order id. Tell Nows the id and that `factory serve` shows the line live at http://127.0.0.1:4600.

## 3. Run the line

Repeat until the order reaches Done or the line stops:

1. Run `factory show <id>` and read the `Station:` line.
2. Dispatch that station's agent with the Agent tool, one station at a time:

   | Station | Agent |
   |---|---|
   | Plan | `planner` |
   | Build | `builder` |
   | Design review | `ui-ux-reviewer` |
   | Security check | `security-checker` |
   | Approval | `approver` |

   Prompt: `Work order <id>. Do your station's work and end with your RESULT line.` Add any decisions from Nows (step 4) under `Decisions from Nows:`.
3. Read the agent's RESULT line, then run `factory show <id>` again. The board is the truth; the message is a summary.
   - Moved to the next station, or sent back to Build: continue the loop. Tell Nows in one line when work goes back for rework, naming the finding numbers.
   - `RESULT: needs-decision` or `RESULT: blocked`: go to step 4.
   - Line stopped: go to step 5.
   - Still at the same station with no RESULT line: dispatch the same agent once more with `Finish work order <id>: your report or pass is missing.` A second miss is a block: go to step 4.

## 4. Decisions

Ask Nows with AskUserQuestion, using the agent's questions and options as written. Record each answer on the board:

```bash
factory note <id> "Decision: <question> → <answer>" --by nows
```

Then dispatch the same agent again with the answers. When Nows wants to drop the order: `factory scrap <id> --reason "<their reason>"`, and end here.

## 5. Line stopped

Tell Nows: the order, the station, the reason (from `factory show <id>`), and what you suggest. End here; Nows clears the andon, then runs `/factory-resume <id>`.

## 6. Hand over

When the order reaches Done:

1. Run `git status --short` and `git diff --stat`.
2. Tell Nows, in a few lines: what changed, how many reworks and findings there were, where the reports are (`.factory/orders/<id>/`), and the commit message the approver suggested.
3. Next step for Nows: read `git diff`, then commit and push. Pushing to `main` deploys the site.

Committing and pushing are Nows's steps, so the line ends at the hand-over.
