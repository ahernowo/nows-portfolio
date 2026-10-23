//! `factory`: the production board behind the Claude Code agent pipeline.
//!
//! Agents never edit `.factory/board.json` themselves. They call this tool,
//! which enforces the rules: reports before passing, findings fixed or waived
//! before approval, no code changes after the build is sealed, and nothing
//! moves while the andon cord is pulled.
//!
//! Run `factory --help` for every command, or see docs/factory.md.

mod board;
mod hooks;
mod seal;
mod server;
mod store;

use std::io::{IsTerminal, Read};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use board::{Board, FindingStatus, Severity, Station, Status};
use store::{display_rel, now, Store};

#[derive(Parser)]
#[command(
    version,
    about = "Work orders, quality gates and a live dashboard for the agent pipeline."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create .factory/ with an empty board. Safe to run again.
    Init,
    /// Open a new work order at the Plan station.
    New {
        title: String,
        /// The full request. Use `--request -` to read it from standard input.
        #[arg(long)]
        request: Option<String>,
    },
    /// Mark that the station's agent has started on an order.
    Start { id: String },
    /// Save a station's report. Pipe the Markdown in: factory report WO-0001 plan < plan.md
    Report { id: String, station: String },
    /// The current station passes the order to the next one.
    Pass {
        id: String,
        #[arg(long, default_value = "")]
        note: String,
    },
    /// The current station rejects the work. Reviews send it back to Build.
    Fail {
        id: String,
        #[arg(long)]
        note: String,
    },
    /// Log a problem found at the current station.
    Finding {
        id: String,
        #[arg(long)]
        severity: String,
        #[arg(long)]
        title: String,
        /// Where, for example site/templates/base.html:12
        #[arg(long)]
        file: Option<String>,
    },
    /// Mark a finding fixed.
    Fix {
        id: String,
        n: u32,
        #[arg(long, default_value = "")]
        note: String,
    },
    /// Accept a finding without fixing it. Needs a reason.
    Waive {
        id: String,
        n: u32,
        #[arg(long)]
        note: String,
    },
    /// Stop the line (pull the andon cord), or clear it with --clear.
    Andon {
        id: Option<String>,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        clear: bool,
        #[arg(long, default_value = "")]
        note: String,
    },
    /// Check every approval gate without changing anything.
    Verify { id: String },
    /// Approve and close the order. Refused unless every gate passes.
    Approve {
        id: String,
        #[arg(long, default_value = "")]
        note: String,
    },
    /// Close an order without finishing it.
    Scrap {
        id: String,
        #[arg(long)]
        reason: String,
    },
    /// List work orders.
    Status {
        /// Include closed orders.
        #[arg(long)]
        all: bool,
        #[arg(long)]
        json: bool,
    },
    /// Everything about one order.
    Show { id: String },
    /// Add a note to an order's history, such as a decision Nows made.
    Note {
        id: String,
        text: String,
        #[arg(long, default_value = "supervisor")]
        by: String,
    },
    /// Add a lesson to .factory/memory/lessons.md.
    Learn {
        text: String,
        #[arg(long)]
        order: Option<String>,
    },
    /// Short summary for the start of a Claude Code session (SessionStart hook).
    Brief,
    /// One-line status for the Claude Code status line.
    Statusline,
    /// PreToolUse hook: block file edits outside --allow paths or inside --deny paths.
    Guard {
        #[arg(long)]
        allow: Vec<String>,
        #[arg(long)]
        deny: Vec<String>,
    },
    /// Hook entry points for Claude Code events.
    Hook { event: String },
    /// Open the live dashboard on http://127.0.0.1:<port>.
    Serve {
        #[arg(long, default_value_t = 4600)]
        port: u16,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Hook commands must never fail loudly: Claude Code runs them constantly.
    match &cli.command {
        Command::Guard { allow, deny } => return ExitCode::from(hooks::guard(allow, deny) as u8),
        Command::Brief => return ExitCode::from(hooks::brief() as u8),
        Command::Statusline => return ExitCode::from(hooks::statusline() as u8),
        Command::Hook { event } if event == "subagent-stop" => {
            return ExitCode::from(hooks::subagent_stop() as u8)
        }
        _ => {}
    }

    match run(cli.command) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// Read text piped into the command (a heredoc or `< file.md`).
///
/// Waits at most 10 seconds, so a command run without any input fails with
/// a clear message instead of hanging an agent's terminal forever.
fn read_piped_stdin() -> Result<Option<String>, String> {
    if std::io::stdin().is_terminal() {
        return Ok(None);
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut text = String::new();
        let result = std::io::stdin()
            .lock()
            .take(store::MAX_REPORT_BYTES as u64 + 1)
            .read_to_string(&mut text)
            .map(|_| text);
        let _ = tx.send(result);
    });
    match rx.recv_timeout(std::time::Duration::from_secs(10)) {
        Ok(Ok(text)) if text.trim().is_empty() => Ok(None),
        Ok(Ok(text)) => Ok(Some(text)),
        Ok(Err(e)) => Err(format!("Could not read standard input: {e}")),
        Err(_) => Err("No text arrived on standard input within 10 seconds. Pipe it in with a heredoc or `< file.md`.".into()),
    }
}

fn with_board<T>(
    store: &Store,
    f: impl FnOnce(&mut Board) -> Result<T, board::Refused>,
) -> Result<T, String> {
    let mut board = store.load()?;
    let out = f(&mut board).map_err(|r| r.0)?;
    store.save(&board)?;
    Ok(out)
}

fn print_gates(report: &board::GateReport) {
    for (ok, text) in &report.checks {
        println!("  [{}] {text}", if *ok { "x" } else { " " });
    }
}

fn run(command: Command) -> Result<ExitCode, String> {
    let store = Store::locate()?;
    let t = now();

    match command {
        Command::Init => {
            let created = store.init()?;
            if created {
                println!(
                    "Created {}. Open the dashboard with `factory serve`.",
                    display_rel(&store.root, &store.factory_dir())
                );
            } else {
                println!(
                    "The factory is already set up in {}.",
                    display_rel(&store.root, &store.factory_dir())
                );
            }
        }

        Command::New { title, request } => {
            let request = match request.as_deref() {
                Some("-") => {
                    read_piped_stdin()?.ok_or("Pipe the request in after `--request -`.")?
                }
                Some(r) => r.to_string(),
                None => title.clone(),
            };
            let id = with_board(&store, |b| b.new_order(&title, &t))?;
            store.write_doc(
                &id,
                "request.md",
                &format!("# {title}\n\n{}\n", request.trim()),
            )?;
            println!("{id}");
            eprintln!("Opened {id} at the Plan station.");
        }

        Command::Start { id } => {
            let station = with_board(&store, |b| b.start(&id, &t))?;
            println!(
                "{} started at {}.",
                board::normalise_id(&id),
                station.label()
            );
        }

        Command::Report { id, station } => {
            let station = Station::parse(&station)
                .filter(|s| s.report_file().is_some())
                .ok_or("Station must be one of: plan, build, design, security, approve.")?;
            let mut board = store.load()?;
            let (order_id, at) = {
                let order = board.order(&id).map_err(|r| r.0)?;
                (order.id.clone(), order.station)
            };
            if at != station {
                return Err(format!(
                    "{order_id} is at {}, not {}. Only the current station writes its report.",
                    at.label(),
                    station.label()
                ));
            }
            let text = read_piped_stdin()?
                .ok_or("Pipe the report in, for example: factory report WO-0001 plan < plan.md")?;
            let file = station.report_file().unwrap_or("report.md");
            let path = store.write_doc(&order_id, file, &text)?;
            let _ = board.note(&order_id, "report", station.agent(), file, &t);
            store.save(&board)?;
            println!("Saved {}.", display_rel(&store.root, &path));
        }

        Command::Pass { id, note } => {
            let current = store.load()?.order(&id).map_err(|r| r.0)?.station;
            let has_report = store.has_report(&id, current);
            let seal = if current == Station::Build {
                Some(seal::fingerprint(&store.root, &t).ok_or(
                    "Could not fingerprint the code. Is this a git repository with git installed?",
                )?)
            } else {
                None
            };
            let next = with_board(&store, |b| b.pass(&id, &note, has_report, seal, &t))?;
            println!(
                "{} passed {}. Now at {}.",
                board::normalise_id(&id),
                current.label(),
                next.label()
            );
        }

        Command::Fail { id, note } => {
            let to = with_board(&store, |b| b.fail(&id, &note, &t))?;
            let board = store.load()?;
            let order = board.order(&id).map_err(|r| r.0)?;
            if to == Station::Build {
                store.archive_reports(&order.id, order.rework)?;
            }
            println!("{} sent to {}.", order.id, to.label());
            if let Some(a) = &board.andon {
                println!("The line is now stopped: {}", a.reason);
            }
        }

        Command::Finding {
            id,
            severity,
            title,
            file,
        } => {
            let severity = Severity::parse(&severity)
                .ok_or("Severity must be low, medium, high or critical.")?;
            let n = with_board(&store, |b| b.add_finding(&id, severity, &title, file, &t))?;
            println!("Logged finding #{n} ({severity:?}).");
            if severity == Severity::Critical {
                println!("Critical: the line is stopped until a human clears it.");
            }
        }

        Command::Fix { id, n, note } => {
            with_board(&store, |b| {
                b.resolve_finding(&id, n, FindingStatus::Fixed, &note, &t)
            })?;
            println!("Finding #{n} marked fixed.");
        }

        Command::Waive { id, n, note } => {
            with_board(&store, |b| {
                b.resolve_finding(&id, n, FindingStatus::Waived, &note, &t)
            })?;
            println!("Finding #{n} waived.");
        }

        Command::Andon {
            id,
            reason,
            clear,
            note,
        } => {
            if clear {
                let order = with_board(&store, |b| b.clear_andon(&note, &t))?;
                println!("Line running again. {order} is back in its queue.");
            } else {
                let id = id.ok_or("Name the order: factory andon WO-0001 --reason \"...\"")?;
                let reason = reason.ok_or("Say why: --reason \"...\"")?;
                with_board(&store, |b| b.pull_andon(&id, &reason, &t))?;
                println!("Line stopped. A human clears it with `factory andon --clear --note \"what was done\"`.");
            }
        }

        Command::Verify { id } => {
            let board = store.load()?;
            let current = seal::fingerprint(&store.root, &t).map(|s| s.hash);
            let report = board
                .gates(&id, &store.reports(&id), current.as_deref())
                .map_err(|r| r.0)?;
            print_gates(&report);
            if !report.passed() {
                println!("Not ready for approval.");
                return Ok(ExitCode::FAILURE);
            }
            println!("Every gate passes.");
        }

        Command::Approve { id, note } => {
            if !store.has_report(&id, Station::Approve) {
                return Err(format!(
                    "Write the approval report first: factory report {} approve < approval.md",
                    board::normalise_id(&id)
                ));
            }
            let current = seal::fingerprint(&store.root, &t).map(|s| s.hash);
            let reports = store.reports(&id);
            let report = with_board(&store, |b| {
                b.approve(&id, &note, &reports, current.as_deref(), &t)
            })?;
            print_gates(&report);
            if report.passed() {
                println!(
                    "{} approved and closed. Ready for the human to review and commit.",
                    board::normalise_id(&id)
                );
            } else {
                println!("Not approved: fix the unchecked gates, or send the order back with `factory fail`.");
                return Ok(ExitCode::FAILURE);
            }
        }

        Command::Scrap { id, reason } => {
            with_board(&store, |b| b.scrap(&id, &reason, &t))?;
            println!("{} scrapped.", board::normalise_id(&id));
        }

        Command::Status { all, json } => {
            let board = store.load()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&board).map_err(|e| e.to_string())?
                );
                return Ok(ExitCode::SUCCESS);
            }
            if let Some(a) = &board.andon {
                println!(
                    "LINE STOPPED on {} at {}: {}\n",
                    a.order,
                    a.station.label(),
                    a.reason
                );
            }
            let orders: Vec<_> = board
                .orders
                .iter()
                .filter(|o| all || o.status != Status::Closed)
                .collect();
            if orders.is_empty() {
                println!("No {}orders. Start one with /factory in Claude Code, or `factory new \"title\"`.", if all { "" } else { "open " });
            }
            for o in orders {
                let open = o
                    .findings
                    .iter()
                    .filter(|f| f.status == FindingStatus::Open)
                    .count();
                println!(
                    "{}  {:<15} {:<8} rework {}  findings {}  {}",
                    o.id,
                    o.station.label(),
                    format!("{:?}", o.status).to_lowercase(),
                    o.rework,
                    open,
                    o.title
                );
            }
        }

        Command::Show { id } => {
            let board = store.load()?;
            let o = board.order(&id).map_err(|r| r.0)?;
            println!("{} {}\n", o.id, o.title);
            println!(
                "Station: {} ({:?}), rework {}",
                o.station.label(),
                o.status,
                o.rework
            );
            if let Some(s) = &o.seal {
                println!(
                    "Build seal: {}… over {} files, {}",
                    &s.hash[..12.min(s.hash.len())],
                    s.files,
                    s.at
                );
            }
            let reports: Vec<_> = store
                .reports(&o.id)
                .iter()
                .map(|s| s.report_file().unwrap_or_default())
                .collect();
            println!(
                "Reports: {}",
                if reports.is_empty() {
                    "none yet".to_string()
                } else {
                    reports.join(", ")
                }
            );
            if !o.findings.is_empty() {
                println!("\nFindings:");
                for f in &o.findings {
                    println!(
                        "  #{} {:?} {:?} at {}: {}{}",
                        f.n,
                        f.severity,
                        f.status,
                        f.station.label(),
                        f.title,
                        f.file
                            .as_ref()
                            .map(|p| format!(" ({p})"))
                            .unwrap_or_default()
                    );
                }
            }
            println!("\nHistory:");
            for e in &o.events {
                println!(
                    "  {}  {:<9} {:<8} {}  {}",
                    e.at,
                    e.station.as_str(),
                    e.kind,
                    e.actor,
                    e.note
                );
            }
        }

        Command::Note { id, text, by } => {
            if text.trim().is_empty() {
                return Err("Write the note as text after the order id.".into());
            }
            let by: String = by
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
                .take(40)
                .collect();
            with_board(&store, |b| b.note(&id, "note", &by, &text, &t))?;
            println!("Note added to {}.", board::normalise_id(&id));
        }

        Command::Learn { text, order } => {
            let text = text.trim().replace('\n', " ");
            if text.is_empty() {
                return Err("Write the lesson as one sentence.".into());
            }
            let date = &t[..10];
            let tag = order
                .map(|o| format!(" {}:", board::normalise_id(&o)))
                .unwrap_or_default();
            store.append_lesson(&format!("- {date}{tag} {text}"))?;
            println!(
                "Lesson saved to {}.",
                display_rel(&store.root, &store.lessons_path())
            );
        }

        Command::Serve { port } => {
            if !store.is_initialised() {
                store.init()?;
            }
            server::serve(store, port)?;
        }

        Command::Brief | Command::Statusline | Command::Guard { .. } => {
            unreachable!("handled before run")
        }
        Command::Hook { event } => {
            return Err(format!("Unknown hook event {event}. Known: subagent-stop."))
        }
    }
    Ok(ExitCode::SUCCESS)
}
