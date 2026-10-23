---
name: factory-protocol
description: How a station works a factory work order with the `factory` CLI, plus report rules, the severity scale and the standard checks. Use when working, reviewing or moving a factory work order, or logging a finding.
---

# Factory protocol

The factory is a production line for changes to this repo. A **work order** (WO-0001) moves through five **stations**: plan → build → design → security → approve. Each station is one agent. The `factory` CLI is the only way to change the board; `.factory/` is written by the CLI alone.

## Commands for a station

| Do this | Command |
|---|---|
| See an order | `factory show <id>` |
| Say you've started | `factory start <id>` |
| Save your report | `factory report <id> <station> <<'EOF'` … `EOF` |
| Log a problem | `factory finding <id> --severity <low\|medium\|high\|critical> --title "..." [--file path:line]` |
| Mark a finding fixed (builder) | `factory fix <id> <n> --note "..."` |
| Hand on to the next station | `factory pass <id> --note "..."` |
| Send back (reviews return it to build) | `factory fail <id> --note "..."` |
| Stop the line | `factory andon <id> --reason "..."` |
| Check approval gates | `factory verify <id>` |
| Approve (approver only) | `factory approve <id> --note "..."` |
| Save a lesson | `factory learn "<one sentence>" --order <id>` |

Station names for `factory report`: `plan`, `build`, `design`, `security`, `approve`. Only the station the order is at can report, pass or fail it; the CLI refuses anything else, and its message says what to do instead. Read refusals and follow them.

Write reports with a quoted heredoc (`<<'EOF'`), so `$`, backticks and quotes inside the report stay literal. The CLI accepts up to 200 KB; a good report fits on one screen or two.

## Report rules

- **Evidence over claims.** Every "pass" names the command you ran and its result, or the file and line you read, or the screenshot you looked at.
- **One finding per problem**, with a file and line when there is one. The report refers to findings by number (#3).
- **Plain words.** Nows reads these reports to learn. Say what a visitor, an editor or an attacker would experience, then the fix.
- **Secrets stay out.** Refer to a secret by file and line only.

## Severity scale

| Severity | Meaning | Example |
|---|---|---|
| low | Polish. Does not block approval. | Uneven spacing at one breakpoint |
| medium | A real defect a visitor or editor would hit, or a broken repo convention. Blocks approval. | Text below 4.5:1 contrast; missing focus style; inline `style=""` |
| high | A feature broken, or a security weakness needing more than one step to abuse. Blocks approval. | Cookie without `Secure`; publish fails on long titles |
| critical | Exploitable now, a live secret, or data loss. **Stops the line.** | Token committed; SQL built from user input; auth bypass |

## Gates

`factory approve` closes an order only when every gate holds: each station passed in this cycle with its report written, no open findings of medium or worse, the line is running, and the **build seal** still matches. The seal is a fingerprint of every tracked and unignored file taken when the build passed; any later edit breaks it. Reviews read and run; they leave the code as the builder sealed it.

**Rework**: a review that fails an order sends it back to build, archives the old reports (`design.rework-1.md`), and every later station reviews again. More than three reworks stop the line for Nows to decide.

**Andon**: a stopped line refuses every move. Only Nows clears it: `factory andon --clear --note "..."`.

## Standard checks

Include those that apply in every plan's acceptance checks, and run them at build and approval:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
zola --root site build
cargo run -q -p newpost -- --check
```

## Memory

Each agent has a memory folder, `.claude/agent-memory/<agent>/`, whose `MEMORY.md` loads at the start of every run. The line's shared memory is `.factory/memory/lessons.md`, written with `factory learn` and read by the planner and reviewers. A good lesson is one instruction for the next order: "Check `:focus-visible` on every new button before passing build."
