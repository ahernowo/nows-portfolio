//! The build seal: a SHA-256 fingerprint of every file git would track,
//! taken when Build passes and checked again at approval. If a review
//! station (which should only read) changed code, the fingerprints differ
//! and approval is refused.
//!
//! Factory records and agent memory are left out, because reviewers are
//! expected to write those.

use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::Command;

use crate::board::Seal;

const EXCLUDED_PREFIXES: [&str; 3] = [
    ".factory/",
    ".claude/agent-memory/",
    ".claude/agent-memory-local/",
];

/// Fingerprint the working tree. Returns `None` outside a git repository.
pub fn fingerprint(root: &Path, at: &str) -> Option<Seal> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let mut files: Vec<String> = output
        .stdout
        .split(|b| *b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| String::from_utf8_lossy(p).into_owned())
        .filter(|p| !EXCLUDED_PREFIXES.iter().any(|x| p.starts_with(x)))
        .collect();
    files.sort();
    files.dedup();

    let mut hasher = Sha256::new();
    let mut count = 0;
    for file in &files {
        hasher.update(file.as_bytes());
        hasher.update([0]);
        match std::fs::read(root.join(file)) {
            Ok(bytes) => {
                // Normalise line endings so a Windows checkout and a Linux
                // checkout of the same code agree.
                let bytes: Vec<u8> = bytes.into_iter().filter(|b| *b != b'\r').collect();
                hasher.update((bytes.len() as u64).to_le_bytes());
                hasher.update(&bytes);
            }
            // A tracked file that was deleted counts as a change too.
            Err(_) => hasher.update(b"<missing>"),
        }
        count += 1;
    }
    let hash = hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    Some(Seal {
        hash,
        files: count,
        at: at.to_string(),
    })
}
