//! Commands that Claude Code runs for us: the file guard, the session brief,
//! the subagent log and the status line. Hooks get a JSON object on stdin.

use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use crate::board::{Board, Station, Status};
use crate::store::{now, Store};

#[derive(Deserialize, Default)]
struct HookInput {
    #[serde(default)]
    tool_name: String,
    #[serde(default)]
    tool_input: serde_json::Value,
    #[serde(default)]
    cwd: String,
    #[serde(default)]
    agent_type: String,
    #[serde(default)]
    last_assistant_message: String,
}

fn read_stdin() -> HookInput {
    let mut text = String::new();
    // Hooks always send JSON, but a person running the command by hand may not.
    let _ = std::io::stdin().take(1_000_000).read_to_string(&mut text);
    serde_json::from_str(&text).unwrap_or_default()
}

/// Remove `.` and `..` without touching the disk, so a path such as
/// `site/../.factory/board.json` is judged by where it really points.
fn normalise(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn comparable(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    if cfg!(windows) {
        s.to_lowercase()
    } else {
        s
    }
}

/// The path relative to the project root, or `None` if it is outside it.
fn relative_to_root(root: &Path, cwd: &str, file: &str) -> Option<String> {
    let file = Path::new(file);
    let absolute = if file.is_absolute() {
        file.to_path_buf()
    } else if !cwd.is_empty() {
        Path::new(cwd).join(file)
    } else {
        root.join(file)
    };
    let absolute = comparable(&normalise(&absolute));
    let root = comparable(&normalise(root));
    let root = root.trim_end_matches('/');
    absolute
        .strip_prefix(root)
        .and_then(|rest| rest.strip_prefix('/'))
        .map(str::to_string)
}

fn matches_prefix(rel: &str, prefix: &str) -> bool {
    let prefix = prefix.trim_start_matches("./").replace('\\', "/");
    let prefix = if cfg!(windows) {
        prefix.to_lowercase()
    } else {
        prefix
    };
    let bare = prefix.trim_end_matches('/');
    rel == bare || rel.starts_with(&format!("{bare}/"))
}

/// The decision for one tool call. `Ok` lets it run; `Err` blocks it with a
/// message Claude will read.
pub fn decide(
    root: &Path,
    input_tool: &str,
    tool_input: &serde_json::Value,
    cwd: &str,
    allow: &[String],
    deny: &[String],
) -> Result<(), String> {
    if !matches!(input_tool, "Write" | "Edit" | "MultiEdit" | "NotebookEdit") {
        return Ok(());
    }
    let file = tool_input
        .get("file_path")
        .or_else(|| tool_input.get("notebook_path"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if file.is_empty() {
        return Err("Blocked: could not tell which file this edit targets.".into());
    }
    let Some(rel) = relative_to_root(root, cwd, file) else {
        return Err(format!(
            "Blocked: {file} is outside the project. Only edit files inside the repository."
        ));
    };
    if let Some(hit) = deny.iter().find(|d| matches_prefix(&rel, d)) {
        let why = if hit.trim_start_matches("./").starts_with(".factory") {
            "Factory records change only through the `factory` command.".to_string()
        } else {
            "This station must leave it as it is. If the plan needs it changed, stop and say so in your final message (RESULT: blocked), so Nows can decide.".to_string()
        };
        return Err(format!("Blocked: {rel} is protected ({hit}). {why}"));
    }
    if !allow.is_empty() && !allow.iter().any(|a| matches_prefix(&rel, a)) {
        return Err(format!(
            "Blocked: this station reviews and must not edit {rel}. Write your report with `factory report`, and log problems with `factory finding`. You may edit only: {}.",
            allow.join(", ")
        ));
    }
    Ok(())
}

/// `factory guard`: PreToolUse hook. Exit code 2 blocks the tool call.
pub fn guard(allow: &[String], deny: &[String]) -> i32 {
    let input = read_stdin();
    let root = Store::locate()
        .map(|s| s.root)
        .unwrap_or_else(|_| PathBuf::from("."));
    match decide(
        &root,
        &input.tool_name,
        &input.tool_input,
        &input.cwd,
        allow,
        deny,
    ) {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

/// `factory hook subagent-stop`: note on the board that a station's agent
/// finished, with the start of its last message. Never blocks.
pub fn subagent_stop() -> i32 {
    let input = read_stdin();
    let Ok(store) = Store::locate() else { return 0 };
    let Ok(mut board) = store.load() else {
        return 0;
    };
    let station = Station::LINE
        .iter()
        .copied()
        .find(|s| s.agent() == input.agent_type);
    let Some(station) = station else { return 0 };
    let id = board
        .orders
        .iter()
        .rev()
        .find(|o| o.station == station && o.status == Status::Working)
        .or_else(|| {
            board
                .orders
                .iter()
                .rev()
                .find(|o| o.status != Status::Closed)
        })
        .map(|o| o.id.clone());
    if let Some(id) = id {
        let summary: String = input.last_assistant_message.chars().take(280).collect();
        let _ = board.note(&id, "agent-finished", &input.agent_type, &summary, &now());
        let _ = store.save(&board);
    }
    0
}

fn order_line(o: &crate::board::Order) -> String {
    let open = o
        .findings
        .iter()
        .filter(|f| f.status == crate::board::FindingStatus::Open)
        .count();
    let status = match o.status {
        Status::Waiting => "waiting",
        Status::Working => "in progress",
        Status::Blocked => "BLOCKED",
        Status::Closed => "closed",
    };
    format!(
        "- {} \"{}\": {} ({status}), rework {}, {open} open finding(s)",
        o.id,
        o.title,
        o.station.label(),
        o.rework
    )
}

/// The text a SessionStart hook adds to Claude's context.
pub fn brief_text(board: &Board, lessons: &[String]) -> String {
    let open: Vec<_> = board.open_orders().collect();
    let done = board
        .orders
        .iter()
        .filter(|o| o.station == Station::Done)
        .count();
    let mut out = Vec::new();
    match &board.andon {
        Some(a) => out.push(format!(
            "FACTORY LINE STOPPED: andon on {} at {}: {}. Do not move work orders. Tell the user; a human clears it with `factory andon --clear --note \"...\"`.",
            a.order, a.station, a.reason
        )),
        None => out.push(format!("Factory line running. {} open order(s), {done} done.", open.len())),
    }
    for o in open.iter().take(8) {
        out.push(order_line(o));
    }
    if !open.is_empty() {
        out.push(
            "To continue an order use /factory-resume <id>. For new work use /factory <request>."
                .into(),
        );
    }
    if !lessons.is_empty() {
        out.push("Recent lessons (.factory/memory/lessons.md):".into());
        for l in lessons.iter().rev().take(5).rev() {
            out.push(l.clone());
        }
    }
    out.join("\n")
}

/// `factory brief`
pub fn brief() -> i32 {
    let Ok(store) = Store::locate() else { return 0 };
    match store.load() {
        Ok(board) => println!("{}", brief_text(&board, &store.lessons())),
        Err(_) => println!("Factory not set up in this project yet. Run `factory init`."),
    }
    0
}

/// `factory statusline`: one line under the Claude Code prompt.
pub fn statusline() -> i32 {
    let _ = read_stdin();
    let Ok(store) = Store::locate() else { return 0 };
    let Ok(board) = store.load() else {
        println!("factory: not set up");
        return 0;
    };
    let red = "\x1b[31m";
    let dim = "\x1b[2m";
    let reset = "\x1b[0m";
    if let Some(a) = &board.andon {
        println!(
            "{red}factory: line stopped on {} at {}{reset}",
            a.order,
            a.station.label().to_lowercase()
        );
        return 0;
    }
    let active = board
        .orders
        .iter()
        .filter(|o| o.status == Status::Working)
        .max_by(|a, b| a.updated_at.cmp(&b.updated_at))
        .or_else(|| {
            board
                .open_orders()
                .max_by(|a, b| a.updated_at.cmp(&b.updated_at))
        });
    match active {
        Some(o) => {
            let mut title: String = o.title.chars().take(40).collect();
            if o.title.chars().count() > 40 {
                title.push('…');
            }
            let rework = if o.rework > 0 {
                format!(", rework {}", o.rework)
            } else {
                String::new()
            };
            println!(
                "factory: {} {title} {dim}at{reset} {}{rework}",
                o.id,
                o.station.label().to_lowercase()
            );
        }
        None => {
            let done = board
                .orders
                .iter()
                .filter(|o| o.station == Station::Done)
                .count();
            println!("{dim}factory: idle, {done} done{reset}");
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn root() -> PathBuf {
        PathBuf::from("/work/site-repo")
    }

    fn check(tool: &str, file: &str, allow: &[&str], deny: &[&str]) -> Result<(), String> {
        let allow: Vec<String> = allow.iter().map(|s| s.to_string()).collect();
        let deny: Vec<String> = deny.iter().map(|s| s.to_string()).collect();
        decide(
            &root(),
            tool,
            &json!({ "file_path": file }),
            "/work/site-repo",
            &allow,
            &deny,
        )
    }

    #[test]
    fn non_file_tools_pass() {
        assert!(check("Bash", "anything", &["x"], &[]).is_ok());
        assert!(check("Read", "/etc/passwd", &["x"], &[]).is_ok());
    }

    #[test]
    fn allow_list_limits_reviewers() {
        let allow = [".claude/agent-memory/ui-ux-reviewer"];
        assert!(check(
            "Write",
            ".claude/agent-memory/ui-ux-reviewer/MEMORY.md",
            &allow,
            &[]
        )
        .is_ok());
        assert!(check("Edit", "/work/site-repo/site/sass/_home.scss", &allow, &[]).is_err());
        assert!(check(
            "Write",
            ".claude/agent-memory/ui-ux-reviewer-evil/x.md",
            &allow,
            &[]
        )
        .is_err());
    }

    #[test]
    fn deny_list_protects_board() {
        let deny = [".factory"];
        assert!(check("Write", ".factory/board.json", &[], &deny).is_err());
        assert!(check("Write", "site/../.factory/board.json", &[], &deny).is_err());
        assert!(check("Write", ".factoryx/ok.md", &[], &deny).is_ok());
        assert!(check("Edit", "site/content/a.md", &[], &deny).is_ok());
    }

    #[test]
    fn outside_project_blocked() {
        assert!(check("Write", "/home/me/.bashrc", &[], &[]).is_err());
        assert!(check("Write", "../other/file", &[], &[]).is_err());
    }

    #[test]
    fn missing_path_blocked() {
        let r = decide(&root(), "Write", &json!({}), "", &[], &[]);
        assert!(r.is_err());
    }

    #[test]
    fn brief_mentions_andon() {
        let mut b = Board::default();
        let id = b.new_order("x", "t").expect("new");
        b.pull_andon(&id, "secret found", "t").expect("andon");
        assert!(brief_text(&b, &[]).starts_with("FACTORY LINE STOPPED"));
    }
}
