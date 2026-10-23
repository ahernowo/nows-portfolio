//! The production board: work orders, the stations they move through, and
//! the rules for moving them. Everything here is pure data and logic, so the
//! rules are covered by unit tests. Saving and loading live in `store.rs`.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Stations in line order. An order visits each one; any review station can
/// send it back to `Build`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Station {
    Plan,
    Build,
    Design,
    Security,
    Approve,
    Done,
    Scrapped,
}

impl Station {
    pub const LINE: [Station; 5] = [
        Station::Plan,
        Station::Build,
        Station::Design,
        Station::Security,
        Station::Approve,
    ];

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "plan" | "planner" => Station::Plan,
            "build" | "builder" => Station::Build,
            "design" | "ui-ux" | "ui-ux-reviewer" | "ux" => Station::Design,
            "security" | "security-checker" => Station::Security,
            "approve" | "approval" | "approver" => Station::Approve,
            "done" => Station::Done,
            "scrapped" => Station::Scrapped,
            _ => return None,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Station::Plan => "plan",
            Station::Build => "build",
            Station::Design => "design",
            Station::Security => "security",
            Station::Approve => "approve",
            Station::Done => "done",
            Station::Scrapped => "scrapped",
        }
    }

    /// The Claude Code subagent that works this station.
    pub fn agent(self) -> &'static str {
        match self {
            Station::Plan => "planner",
            Station::Build => "builder",
            Station::Design => "ui-ux-reviewer",
            Station::Security => "security-checker",
            Station::Approve => "approver",
            Station::Done | Station::Scrapped => "human",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Station::Plan => "Plan",
            Station::Build => "Build",
            Station::Design => "Design review",
            Station::Security => "Security check",
            Station::Approve => "Approval",
            Station::Done => "Done",
            Station::Scrapped => "Scrapped",
        }
    }

    /// The report file each station writes.
    pub fn report_file(self) -> Option<&'static str> {
        match self {
            Station::Plan => Some("plan.md"),
            Station::Build => Some("build.md"),
            Station::Design => Some("design.md"),
            Station::Security => Some("security.md"),
            Station::Approve => Some("approval.md"),
            Station::Done | Station::Scrapped => None,
        }
    }

    pub fn next(self) -> Station {
        match self {
            Station::Plan => Station::Build,
            Station::Build => Station::Design,
            Station::Design => Station::Security,
            Station::Security => Station::Approve,
            Station::Approve | Station::Done => Station::Done,
            Station::Scrapped => Station::Scrapped,
        }
    }

    pub fn is_review(self) -> bool {
        matches!(self, Station::Design | Station::Security | Station::Approve)
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Queued at its station, nobody working on it.
    Waiting,
    /// An agent has started on it.
    Working,
    /// The line is stopped on this order. Only a human clears it.
    Blocked,
    /// Finished: approved or scrapped.
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "low" => Severity::Low,
            "medium" | "med" => Severity::Medium,
            "high" => Severity::High,
            "critical" | "crit" => Severity::Critical,
            _ => return None,
        })
    }

    /// Medium and above must be fixed or waived before approval.
    pub fn blocks_approval(self) -> bool {
        self >= Severity::Medium
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingStatus {
    Open,
    Fixed,
    Waived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub n: u32,
    pub station: Station,
    pub severity: Severity,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    pub status: FindingStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub at: String,
    pub station: Station,
    pub kind: String,
    pub actor: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
}

/// A fingerprint of the working tree, taken when the build passes. Review
/// stations must not change code, so at approval the tree must still match.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seal {
    pub hash: String,
    pub files: usize,
    pub at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub station: Station,
    pub status: Status,
    /// Times the order was sent back to Build.
    pub rework: u32,
    /// Stations passed since the last rework, in order.
    #[serde(default)]
    pub passed: Vec<Station>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seal: Option<Seal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(default)]
    pub findings: Vec<Finding>,
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Andon {
    pub order: String,
    pub station: Station,
    pub reason: String,
    pub at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub version: u32,
    pub next_id: u32,
    /// Rework loops allowed before the line stops for a human.
    pub max_rework: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub andon: Option<Andon>,
    pub orders: Vec<Order>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            version: 1,
            next_id: 1,
            max_rework: 3,
            andon: None,
            orders: Vec::new(),
        }
    }
}

/// Why a command was refused. The message tells the agent what to do instead.
#[derive(Debug, PartialEq, Eq)]
pub struct Refused(pub String);

impl fmt::Display for Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn refuse<T>(msg: impl Into<String>) -> Result<T, Refused> {
    Err(Refused(msg.into()))
}

/// Result of checking the approval gates.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct GateReport {
    pub checks: Vec<(bool, String)>,
}

impl GateReport {
    fn check(&mut self, ok: bool, text: impl Into<String>) {
        self.checks.push((ok, text.into()));
    }

    pub fn passed(&self) -> bool {
        self.checks.iter().all(|(ok, _)| *ok)
    }
}

impl Board {
    pub fn order(&self, id: &str) -> Result<&Order, Refused> {
        let id = normalise_id(id);
        self.orders.iter().find(|o| o.id == id).ok_or_else(|| {
            Refused(format!(
                "No work order {id}. Run `factory status` to list them."
            ))
        })
    }

    fn order_mut(&mut self, id: &str) -> Result<&mut Order, Refused> {
        let id = normalise_id(id);
        self.orders.iter_mut().find(|o| o.id == id).ok_or_else(|| {
            Refused(format!(
                "No work order {id}. Run `factory status` to list them."
            ))
        })
    }

    fn line_must_run(&self) -> Result<(), Refused> {
        match &self.andon {
            Some(andon) => refuse(format!(
                "The line is stopped (andon on {} at {}: {}). A human must run `factory andon --clear` first.",
                andon.order, andon.station, andon.reason
            )),
            None => Ok(()),
        }
    }

    /// Open a new work order at the Plan station.
    pub fn new_order(&mut self, title: &str, now: &str) -> Result<String, Refused> {
        let title = title.trim();
        if title.is_empty() {
            return refuse("Give the work order a title.");
        }
        let id = format!("WO-{:04}", self.next_id);
        self.next_id += 1;
        self.orders.push(Order {
            id: id.clone(),
            title: title.to_string(),
            created_at: now.into(),
            updated_at: now.into(),
            station: Station::Plan,
            status: Status::Waiting,
            rework: 0,
            passed: Vec::new(),
            seal: None,
            closed_at: None,
            findings: Vec::new(),
            events: vec![Event {
                at: now.into(),
                station: Station::Plan,
                kind: "created".into(),
                actor: "supervisor".into(),
                note: title.to_string(),
            }],
        });
        Ok(id)
    }

    /// An agent starts work at the order's current station.
    pub fn start(&mut self, id: &str, now: &str) -> Result<Station, Refused> {
        self.line_must_run()?;
        let order = self.order_mut(id)?;
        if order.status == Status::Closed {
            return refuse(format!("{} is closed.", order.id));
        }
        order.status = Status::Working;
        order.updated_at = now.into();
        let station = order.station;
        order.events.push(Event {
            at: now.into(),
            station,
            kind: "start".into(),
            actor: station.agent().into(),
            note: String::new(),
        });
        Ok(station)
    }

    /// The current station passes the order on. `report_exists` says whether
    /// the station's report file is on disk; `seal` is the fingerprint to
    /// record when Build passes.
    pub fn pass(
        &mut self,
        id: &str,
        note: &str,
        report_exists: bool,
        seal: Option<Seal>,
        now: &str,
    ) -> Result<Station, Refused> {
        self.line_must_run()?;
        let order = self.order_mut(id)?;
        let station = order.station;
        if order.status == Status::Closed {
            return refuse(format!("{} is closed.", order.id));
        }
        if station == Station::Approve {
            return refuse(
                "Approval passes with `factory approve`, which checks every gate first.",
            );
        }
        if !report_exists {
            return refuse(format!(
                "Write the {} report first: factory report {} {} < report.md",
                station.label().to_lowercase(),
                order.id,
                station
            ));
        }
        if station.is_review() {
            let open_here: Vec<_> = order
                .findings
                .iter()
                .filter(|f| {
                    f.station == station
                        && f.status == FindingStatus::Open
                        && f.severity.blocks_approval()
                })
                .collect();
            if !open_here.is_empty() {
                return refuse(format!(
                    "This station logged {} open finding(s) of medium severity or worse. Use `factory fail` to send the order back to Build.",
                    open_here.len()
                ));
            }
        }
        if station == Station::Build {
            order.seal = seal;
        }
        order.passed.push(station);
        let next = station.next();
        order.station = next;
        order.status = Status::Waiting;
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station,
            kind: "pass".into(),
            actor: station.agent().into(),
            note: note.trim().to_string(),
        });
        Ok(next)
    }

    /// A station rejects the work. Reviews send it back to Build; Plan and
    /// Build stay where they are for another try. Too many loops stop the line.
    pub fn fail(&mut self, id: &str, note: &str, now: &str) -> Result<Station, Refused> {
        self.line_must_run()?;
        if note.trim().is_empty() {
            return refuse("Say why it failed with --note, so the next station knows what to fix.");
        }
        let max_rework = self.max_rework;
        let order = self.order_mut(id)?;
        if order.status == Status::Closed {
            return refuse(format!("{} is closed.", order.id));
        }
        let from = order.station;
        let to = if from.is_review() {
            Station::Build
        } else {
            from
        };
        if from.is_review() {
            order.rework += 1;
            order.passed.retain(|s| *s == Station::Plan);
            order.seal = None;
        }
        order.station = to;
        order.status = Status::Waiting;
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station: from,
            kind: "fail".into(),
            actor: from.agent().into(),
            note: note.trim().to_string(),
        });
        let rework = order.rework;
        let order_id = order.id.clone();
        if rework > max_rework {
            self.pull_andon(
                &order_id,
                &format!("Rework limit reached ({rework} loops). A human should decide what happens next."),
                now,
            )?;
        }
        Ok(to)
    }

    pub fn add_finding(
        &mut self,
        id: &str,
        severity: Severity,
        title: &str,
        file: Option<String>,
        now: &str,
    ) -> Result<u32, Refused> {
        if title.trim().is_empty() {
            return refuse("Describe the finding with --title.");
        }
        let order = self.order_mut(id)?;
        if order.status == Status::Closed {
            return refuse(format!("{} is closed.", order.id));
        }
        let n = order.findings.iter().map(|f| f.n).max().unwrap_or(0) + 1;
        let station = order.station;
        order.findings.push(Finding {
            n,
            station,
            severity,
            title: title.trim().to_string(),
            file,
            status: FindingStatus::Open,
            note: None,
            at: now.into(),
        });
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station,
            kind: "finding".into(),
            actor: station.agent().into(),
            note: format!("#{n} {severity:?}: {}", title.trim()),
        });
        let order_id = order.id.clone();
        if severity == Severity::Critical {
            self.pull_andon(
                &order_id,
                &format!("Critical finding #{n}: {}", title.trim()),
                now,
            )?;
        }
        Ok(n)
    }

    pub fn resolve_finding(
        &mut self,
        id: &str,
        n: u32,
        status: FindingStatus,
        note: &str,
        now: &str,
    ) -> Result<(), Refused> {
        if status == FindingStatus::Waived && note.trim().is_empty() {
            return refuse("A waiver needs a reason in --note.");
        }
        let order = self.order_mut(id)?;
        let station = order.station;
        let finding = order
            .findings
            .iter_mut()
            .find(|f| f.n == n)
            .ok_or_else(|| Refused(format!("No finding #{n} on this order.")))?;
        finding.status = status;
        finding.note = (!note.trim().is_empty()).then(|| note.trim().to_string());
        let kind = if status == FindingStatus::Fixed {
            "fix"
        } else {
            "waive"
        };
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station,
            kind: kind.into(),
            actor: station.agent().into(),
            note: format!("#{n} {}", note.trim()),
        });
        Ok(())
    }

    /// Stop the line. Every command that moves work is refused until a
    /// human clears it.
    pub fn pull_andon(&mut self, id: &str, reason: &str, now: &str) -> Result<(), Refused> {
        if reason.trim().is_empty() {
            return refuse("Say why the line is stopping with --reason.");
        }
        let order = self.order_mut(id)?;
        order.status = Status::Blocked;
        order.updated_at = now.into();
        let station = order.station;
        order.events.push(Event {
            at: now.into(),
            station,
            kind: "andon".into(),
            actor: station.agent().into(),
            note: reason.trim().to_string(),
        });
        let order_id = order.id.clone();
        self.andon = Some(Andon {
            order: order_id,
            station,
            reason: reason.trim().to_string(),
            at: now.into(),
        });
        Ok(())
    }

    pub fn clear_andon(&mut self, note: &str, now: &str) -> Result<String, Refused> {
        let Some(andon) = self.andon.take() else {
            return refuse("The line is running; there is no andon to clear.");
        };
        if let Ok(order) = self.order_mut(&andon.order) {
            if order.status == Status::Blocked {
                order.status = Status::Waiting;
            }
            order.updated_at = now.into();
            let station = order.station;
            order.events.push(Event {
                at: now.into(),
                station,
                kind: "clear".into(),
                actor: "human".into(),
                note: note.trim().to_string(),
            });
        }
        Ok(andon.order)
    }

    /// Check every approval gate. `reports` says which report files exist;
    /// `current_seal` is the tree fingerprint right now.
    pub fn gates(
        &self,
        id: &str,
        reports: &[Station],
        current_seal: Option<&str>,
    ) -> Result<GateReport, Refused> {
        let order = self.order(id)?;
        let mut g = GateReport::default();
        g.check(
            order.station == Station::Approve,
            format!(
                "Order is at the approval station (now: {})",
                order.station.label()
            ),
        );
        for station in [
            Station::Plan,
            Station::Build,
            Station::Design,
            Station::Security,
        ] {
            g.check(
                order.passed.contains(&station),
                format!("{} passed in this cycle", station.label()),
            );
            g.check(
                reports.contains(&station),
                format!("{} report written", station.label()),
            );
        }
        let blocking: Vec<_> = order
            .findings
            .iter()
            .filter(|f| f.status == FindingStatus::Open && f.severity.blocks_approval())
            .map(|f| format!("#{}", f.n))
            .collect();
        g.check(
            blocking.is_empty(),
            if blocking.is_empty() {
                "No open findings of medium severity or worse".to_string()
            } else {
                format!(
                    "Open findings must be fixed or waived: {}",
                    blocking.join(", ")
                )
            },
        );
        match (&order.seal, current_seal) {
            (Some(seal), Some(now)) => g.check(
                seal.hash == now,
                if seal.hash == now {
                    "Code is unchanged since the build passed".to_string()
                } else {
                    "Code changed after the build passed. Send it back to Build with `factory fail`.".to_string()
                },
            ),
            (None, _) => g.check(false, "No build seal recorded. The build station must pass first."),
            (Some(_), None) => g.check(false, "Could not fingerprint the working tree (is this a git repository?)"),
        }
        g.check(self.andon.is_none(), "The line is running (no andon)");
        Ok(g)
    }

    /// Approve: close the order if every gate passes.
    pub fn approve(
        &mut self,
        id: &str,
        note: &str,
        reports: &[Station],
        current_seal: Option<&str>,
        now: &str,
    ) -> Result<GateReport, Refused> {
        let report = self.gates(id, reports, current_seal)?;
        if !report.passed() {
            return Ok(report);
        }
        let order = self.order_mut(id)?;
        order.passed.push(Station::Approve);
        order.station = Station::Done;
        order.status = Status::Closed;
        order.closed_at = Some(now.into());
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station: Station::Approve,
            kind: "approve".into(),
            actor: "approver".into(),
            note: note.trim().to_string(),
        });
        Ok(report)
    }

    pub fn scrap(&mut self, id: &str, reason: &str, now: &str) -> Result<(), Refused> {
        if reason.trim().is_empty() {
            return refuse("Say why with --reason.");
        }
        let order = self.order_mut(id)?;
        let station = order.station;
        order.station = Station::Scrapped;
        order.status = Status::Closed;
        order.closed_at = Some(now.into());
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station,
            kind: "scrap".into(),
            actor: "human".into(),
            note: reason.trim().to_string(),
        });
        let order_id = order.id.clone();
        if self.andon.as_ref().is_some_and(|a| a.order == order_id) {
            self.andon = None;
        }
        Ok(())
    }

    /// Record a free-form event, used by hooks when an agent finishes.
    pub fn note(
        &mut self,
        id: &str,
        kind: &str,
        actor: &str,
        note: &str,
        now: &str,
    ) -> Result<(), Refused> {
        let order = self.order_mut(id)?;
        let station = order.station;
        order.updated_at = now.into();
        order.events.push(Event {
            at: now.into(),
            station,
            kind: kind.into(),
            actor: actor.into(),
            note: note.trim().to_string(),
        });
        Ok(())
    }

    /// Orders that are not closed, oldest first.
    pub fn open_orders(&self) -> impl Iterator<Item = &Order> {
        self.orders.iter().filter(|o| o.status != Status::Closed)
    }
}

/// Accept `7`, `wo-7`, `WO-0007` as the same id.
pub fn normalise_id(id: &str) -> String {
    let digits: String = id
        .trim()
        .trim_start_matches(['W', 'w'])
        .trim_start_matches(['O', 'o'])
        .trim_start_matches('-')
        .to_string();
    match digits.parse::<u32>() {
        Ok(n) => format!("WO-{n:04}"),
        Err(_) => id.trim().to_uppercase(),
    }
}

/// Order ids are used in file paths, so only accept exactly `WO-` and digits.
pub fn is_valid_id(id: &str) -> bool {
    id.len() >= 4 && id.starts_with("WO-") && id[3..].bytes().all(|b| b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: &str = "2026-10-23T09:00:00Z";

    fn seal(h: &str) -> Option<Seal> {
        Some(Seal {
            hash: h.into(),
            files: 1,
            at: T.into(),
        })
    }

    fn through_build(board: &mut Board) -> String {
        let id = board.new_order("Add RSS link", T).expect("new");
        board.start(&id, T).expect("start");
        board.pass(&id, "planned", true, None, T).expect("plan");
        board.start(&id, T).expect("start");
        board
            .pass(&id, "built", true, seal("abc"), T)
            .expect("build");
        id
    }

    #[test]
    fn ids() {
        assert_eq!(normalise_id("7"), "WO-0007");
        assert_eq!(normalise_id("wo-12"), "WO-0012");
        assert_eq!(normalise_id("WO-0003"), "WO-0003");
        assert!(is_valid_id("WO-0003"));
        assert!(!is_valid_id("WO-../x"));
        assert!(!is_valid_id("../WO-1"));
    }

    #[test]
    fn happy_path_reaches_done() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        b.pass(&id, "looks right", true, None, T).expect("design");
        b.pass(&id, "no issues", true, None, T).expect("security");
        let all = [
            Station::Plan,
            Station::Build,
            Station::Design,
            Station::Security,
        ];
        let report = b
            .approve(&id, "ship it", &all, Some("abc"), T)
            .expect("approve");
        assert!(report.passed(), "{report:?}");
        let o = b.order(&id).expect("order");
        assert_eq!(o.station, Station::Done);
        assert_eq!(o.status, Status::Closed);
        assert_eq!(o.rework, 0);
    }

    #[test]
    fn report_required_before_pass() {
        let mut b = Board::default();
        let id = b.new_order("x", T).expect("new");
        assert!(b.pass(&id, "", false, None, T).is_err());
    }

    #[test]
    fn review_failure_sends_back_to_build_and_clears_seal() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        assert_eq!(b.fail(&id, "contrast too low", T), Ok(Station::Build));
        let o = b.order(&id).expect("order");
        assert_eq!(o.rework, 1);
        assert_eq!(o.passed, vec![Station::Plan]);
        assert!(o.seal.is_none());
    }

    #[test]
    fn open_findings_block_pass_and_approval() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        b.pass(&id, "", true, None, T).expect("design");
        b.add_finding(&id, Severity::High, "Token logged", None, T)
            .expect("finding");
        assert!(
            b.pass(&id, "", true, None, T).is_err(),
            "security cannot pass with its own open high finding"
        );
        b.resolve_finding(&id, 1, FindingStatus::Waived, "Only in local dev logs", T)
            .expect("waive");
        b.pass(&id, "", true, None, T)
            .expect("security passes after waiver");
        let all = [
            Station::Plan,
            Station::Build,
            Station::Design,
            Station::Security,
        ];
        assert!(b
            .approve(&id, "", &all, Some("abc"), T)
            .expect("gates")
            .passed());
    }

    #[test]
    fn critical_finding_stops_the_line() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        b.add_finding(&id, Severity::Critical, "Secret committed", None, T)
            .expect("finding");
        assert!(b.andon.is_some());
        assert_eq!(b.order(&id).expect("o").status, Status::Blocked);
        assert!(
            b.pass(&id, "", true, None, T).is_err(),
            "nothing moves while the line is stopped"
        );
        let other = b
            .new_order("other", T)
            .expect("new orders can still be logged");
        assert!(b.start(&other, T).is_err());
        b.clear_andon("rotated the key", T).expect("clear");
        assert_eq!(b.order(&id).expect("o").status, Status::Waiting);
    }

    #[test]
    fn changed_code_fails_the_seal_gate() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        b.pass(&id, "", true, None, T).expect("design");
        b.pass(&id, "", true, None, T).expect("security");
        let all = [
            Station::Plan,
            Station::Build,
            Station::Design,
            Station::Security,
        ];
        let report = b
            .approve(&id, "", &all, Some("different"), T)
            .expect("gates");
        assert!(!report.passed());
        assert_eq!(b.order(&id).expect("o").status, Status::Waiting);
    }

    #[test]
    fn missing_reports_fail_gates() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        b.pass(&id, "", true, None, T).expect("design");
        b.pass(&id, "", true, None, T).expect("security");
        let report = b
            .approve(&id, "", &[Station::Plan], Some("abc"), T)
            .expect("gates");
        assert!(!report.passed());
    }

    #[test]
    fn rework_limit_pulls_andon() {
        let mut b = Board {
            max_rework: 1,
            ..Board::default()
        };
        let id = through_build(&mut b);
        b.fail(&id, "first", T).expect("fail 1");
        b.start(&id, T).expect("start");
        b.pass(&id, "", true, seal("x"), T).expect("build again");
        b.fail(&id, "second", T).expect("fail 2");
        assert!(b.andon.is_some());
    }

    #[test]
    fn fail_needs_a_reason_and_waiver_needs_a_note() {
        let mut b = Board::default();
        let id = through_build(&mut b);
        assert!(b.fail(&id, "  ", T).is_err());
        b.add_finding(&id, Severity::Medium, "x", None, T)
            .expect("finding");
        assert!(b
            .resolve_finding(&id, 1, FindingStatus::Waived, "", T)
            .is_err());
    }

    #[test]
    fn json_round_trip() {
        let mut b = Board::default();
        through_build(&mut b);
        let json = serde_json::to_string(&b).expect("ser");
        let back: Board = serde_json::from_str(&json).expect("de");
        assert_eq!(b, back);
    }
}
