//! Where the factory keeps its files, and how it reads and writes them.
//!
//! ```text
//! .factory/
//!   board.json              the production board (only this tool writes it)
//!   orders/WO-0001/         one folder per work order
//!     request.md            what was asked for
//!     plan.md build.md design.md security.md approval.md
//!   memory/lessons.md       the kaizen log: what each order taught us
//! ```

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::board::{is_valid_id, normalise_id, Board, Station};

/// Largest report we accept, so a runaway agent can't fill the disk.
pub const MAX_REPORT_BYTES: usize = 200_000;

pub struct Store {
    pub root: PathBuf,
}

pub fn now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    cms_core::date::rfc3339(secs)
}

impl Store {
    /// Find the project root: `CLAUDE_PROJECT_DIR` when Claude Code runs us,
    /// otherwise the nearest folder upwards that has `.factory` or `.git`.
    pub fn locate() -> Result<Self, String> {
        if let Ok(dir) = std::env::var("CLAUDE_PROJECT_DIR") {
            let p = PathBuf::from(dir);
            if p.is_dir() {
                return Ok(Self { root: p });
            }
        }
        let start = std::env::current_dir().map_err(|e| e.to_string())?;
        let mut dir = start.clone();
        loop {
            if dir.join(".factory").is_dir() || dir.join(".git").exists() {
                return Ok(Self { root: dir });
            }
            if !dir.pop() {
                return Ok(Self { root: start });
            }
        }
    }

    pub fn factory_dir(&self) -> PathBuf {
        self.root.join(".factory")
    }

    fn board_path(&self) -> PathBuf {
        self.factory_dir().join("board.json")
    }

    pub fn lessons_path(&self) -> PathBuf {
        self.factory_dir().join("memory").join("lessons.md")
    }

    pub fn order_dir(&self, id: &str) -> Result<PathBuf, String> {
        let id = normalise_id(id);
        if !is_valid_id(&id) {
            return Err(format!(
                "{id} is not a work order id. Use the form WO-0001."
            ));
        }
        Ok(self.factory_dir().join("orders").join(id))
    }

    pub fn is_initialised(&self) -> bool {
        self.board_path().is_file()
    }

    /// Create the folders and an empty board. Safe to run twice.
    pub fn init(&self) -> Result<bool, String> {
        fs::create_dir_all(self.factory_dir().join("orders")).map_err(|e| e.to_string())?;
        fs::create_dir_all(self.factory_dir().join("memory")).map_err(|e| e.to_string())?;
        if !self.lessons_path().exists() {
            fs::write(
                self.lessons_path(),
                "# Lessons\n\nOne line per lesson, newest at the bottom. Written by `factory learn`. Read this before planning or reviewing.\n\n",
            )
            .map_err(|e| e.to_string())?;
        }
        if self.is_initialised() {
            return Ok(false);
        }
        self.save(&Board::default())?;
        Ok(true)
    }

    pub fn load(&self) -> Result<Board, String> {
        let path = self.board_path();
        let text = fs::read_to_string(&path).map_err(|_| {
            format!(
                "No board at {}. Run `factory init` in the project folder first.",
                path.display()
            )
        })?;
        serde_json::from_str(&text).map_err(|e| {
            format!("{} is damaged ({e}). Restore it from git with `git checkout -- .factory/board.json`.", path.display())
        })
    }

    /// Write the board atomically: a crash mid-write leaves the old board,
    /// never half a file.
    pub fn save(&self, board: &Board) -> Result<(), String> {
        let path = self.board_path();
        let tmp = path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(board).map_err(|e| e.to_string())?;
        {
            let mut f = fs::File::create(&tmp).map_err(|e| e.to_string())?;
            f.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
            f.write_all(b"\n").map_err(|e| e.to_string())?;
            f.sync_all().map_err(|e| e.to_string())?;
        }
        fs::rename(&tmp, &path).map_err(|e| e.to_string())
    }

    pub fn write_doc(&self, id: &str, file: &str, text: &str) -> Result<PathBuf, String> {
        if text.len() > MAX_REPORT_BYTES {
            return Err(format!(
                "The report is over {} KB. Summarise it.",
                MAX_REPORT_BYTES / 1000
            ));
        }
        if text.trim().is_empty() {
            return Err("The report is empty. Pipe the report text into this command.".into());
        }
        let dir = self.order_dir(id)?;
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = dir.join(file);
        fs::write(&path, text).map_err(|e| e.to_string())?;
        Ok(path)
    }

    pub fn has_report(&self, id: &str, station: Station) -> bool {
        match (self.order_dir(id), station.report_file()) {
            (Ok(dir), Some(file)) => dir.join(file).is_file(),
            _ => false,
        }
    }

    pub fn reports(&self, id: &str) -> Vec<Station> {
        Station::LINE
            .iter()
            .copied()
            .filter(|s| self.has_report(id, *s))
            .collect()
    }

    pub fn read_doc(&self, id: &str, file: &str) -> Option<String> {
        let allowed = [
            "request.md",
            "plan.md",
            "build.md",
            "design.md",
            "security.md",
            "approval.md",
        ];
        if !allowed.contains(&file) {
            return None;
        }
        fs::read_to_string(self.order_dir(id).ok()?.join(file)).ok()
    }

    /// After a rework, earlier build and review reports no longer describe
    /// the code. Keep them for the record as `design.rework-1.md` and so on,
    /// so each station has to write a fresh one.
    pub fn archive_reports(&self, id: &str, rework: u32) -> Result<(), String> {
        let dir = self.order_dir(id)?;
        for station in [
            Station::Build,
            Station::Design,
            Station::Security,
            Station::Approve,
        ] {
            let Some(file) = station.report_file() else {
                continue;
            };
            let from = dir.join(file);
            if from.is_file() {
                let to = dir.join(format!(
                    "{}.rework-{rework}.md",
                    file.trim_end_matches(".md")
                ));
                fs::rename(&from, &to).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub fn append_lesson(&self, line: &str) -> Result<(), String> {
        let path = self.lessons_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| e.to_string())?;
        writeln!(f, "{line}").map_err(|e| e.to_string())
    }

    pub fn lessons(&self) -> Vec<String> {
        fs::read_to_string(self.lessons_path())
            .unwrap_or_default()
            .lines()
            .filter(|l| l.starts_with("- "))
            .map(str::to_string)
            .collect()
    }
}

/// Path relative to the project root, with forward slashes, for messages.
pub fn display_rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
