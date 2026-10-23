//! `newpost`: start an article or case study, or check the ones you have.
//!
//! ```text
//! newpost "How I reconcile gateway payouts" --tags payments,excel
//! newpost "PDF merger" --work
//! newpost --check
//! ```
//!
//! New files start as drafts. Run `zola serve --drafts` in `site/` to see them.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use cms_core::frontmatter::{self, FrontMatter};
use cms_core::{validate, ArticleInput, Date};
use serde::Serialize;

#[derive(Parser)]
#[command(
    version,
    about = "Create a new article or case study, or check existing articles."
)]
struct Args {
    /// Title of the new article or case study.
    #[arg(required_unless_present = "check", conflicts_with = "check")]
    title: Option<String>,

    /// One-sentence summary, shown in lists and link previews.
    #[arg(long, short, default_value = "")]
    description: String,

    /// Comma-separated tags, for example "rust,cloudflare".
    #[arg(long, short, value_delimiter = ',')]
    tags: Vec<String>,

    /// Publication date as YYYY-MM-DD. Defaults to today (UTC).
    #[arg(long)]
    date: Option<String>,

    /// Use this slug instead of one made from the title.
    #[arg(long)]
    slug: Option<String>,

    /// Create a case study in content/work instead of an article.
    #[arg(long)]
    work: bool,

    /// Check every article and case study and report problems.
    #[arg(long)]
    check: bool,

    /// Path to the site folder. Found automatically when run inside the repository.
    #[arg(long)]
    site: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let site = match args.site.clone().or_else(find_site) {
        Some(s) => s,
        None => {
            eprintln!("Could not find the site folder (a folder named site with zola.toml). Run this inside the repository or pass --site.");
            return ExitCode::from(2);
        }
    };

    let result = if args.check {
        check(&site)
    } else {
        create(&site, &args)
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// Walk up from the current folder until we find `site/zola.toml`.
fn find_site() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join("site/zola.toml").is_file() {
            return Some(dir.join("site"));
        }
        if dir.join("zola.toml").is_file() && dir.join("content").is_dir() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn today() -> Date {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    Date::from_unix(secs)
}

fn create(site: &Path, args: &Args) -> Result<(), String> {
    let input = ArticleInput {
        title: args.title.clone().unwrap_or_default(),
        slug: args.slug.clone().unwrap_or_default(),
        description: args.description.clone(),
        tags: args.tags.clone(),
        body: String::new(),
        date: args.date.clone().unwrap_or_default(),
    };
    let article = validate(input).map_err(|errors| {
        let lines: Vec<String> = errors
            .iter()
            .map(|e| format!("  {}: {}", e.field, e.message))
            .collect();
        format!("Not created:\n{}", lines.join("\n"))
    })?;
    let date = article.date.unwrap_or_else(today);

    let (folder, contents) = if args.work {
        (
            "work",
            render_work(&article.title, &article.description, date, &article.tags),
        )
    } else {
        let front = FrontMatter::for_article(&article, date, None, true);
        let body = "Write the opening here: the point of the article in two or three sentences.\n\n## First section\n\nKeep paragraphs short. Use `##` for sections.";
        ("articles", frontmatter::render(&front, body))
    };

    let path = site
        .join("content")
        .join(folder)
        .join(format!("{}.md", article.slug));
    if path.exists() {
        return Err(format!(
            "{} already exists. Pick another title or pass --slug.",
            path.display()
        ));
    }
    fs::write(&path, contents).map_err(|e| format!("Could not write {}: {e}", path.display()))?;
    println!("Created {}", path.display());
    println!(
        "It is a draft. Preview it with `zola serve --drafts` in {}.",
        site.display()
    );
    Ok(())
}

#[derive(Serialize)]
struct WorkFront<'a> {
    title: &'a str,
    template: &'a str,
    description: &'a str,
    date: toml::value::Datetime,
    weight: u32,
    draft: bool,
    include_in_feeds: bool,
    extra: WorkExtra<'a>,
}

#[derive(Serialize)]
struct WorkExtra<'a> {
    year: String,
    role: &'a str,
    status: &'a str,
    stack: &'a [String],
}

fn render_work(title: &str, description: &str, date: Date, stack: &[String]) -> String {
    let front = WorkFront {
        title,
        template: "work-page.html",
        description,
        date: frontmatter::toml_date(date),
        weight: 10,
        draft: true,
        include_in_feeds: false,
        extra: WorkExtra {
            year: date.year.to_string(),
            role: "",
            status: "",
            stack,
        },
    };
    let toml = toml::to_string(&front).unwrap_or_default();
    format!(
        "+++\n{toml}+++\n\n## The problem\n\nWhat was hard, slow or risky before?\n\n## What I built\n\nWhat it does, in plain words.\n\n## Decisions\n\nThe choices worth explaining.\n\n## What changed\n\nA before and after you can stand behind.\n"
    )
}

/// Check every article and case study. Problems are listed per file.
fn check(site: &Path) -> Result<(), String> {
    let mut problems = Vec::new();
    let mut count = 0;
    for folder in ["articles", "work"] {
        let dir = site.join("content").join(folder);
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        paths.sort();
        for path in paths {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            if !name.ends_with(".md") || name == "_index.md" {
                continue;
            }
            count += 1;
            let text = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(e) => {
                    problems.push(format!("{}: cannot read: {e}", path.display()));
                    continue;
                }
            };
            let (front, body) = match frontmatter::parse(&text) {
                Ok(parts) => parts,
                Err(e) => {
                    problems.push(format!("{}: {e}", path.display()));
                    continue;
                }
            };
            let expected = if folder == "articles" {
                "article.html"
            } else {
                "work-page.html"
            };
            if front.template.as_deref() != Some(expected) {
                problems.push(format!(
                    "{}: add `template = \"{expected}\"` so the CMS lists this file",
                    path.display()
                ));
            }
            let slug = name.trim_end_matches(".md").to_string();
            let input = ArticleInput {
                title: front.title.clone(),
                slug,
                description: front.description.clone(),
                tags: front.taxonomies.tags.clone(),
                body,
                date: String::new(),
            };
            if let Err(errors) = validate(input) {
                for e in errors {
                    problems.push(format!("{}: {}: {}", path.display(), e.field, e.message));
                }
            }
        }
    }

    if problems.is_empty() {
        println!("Checked {count} files. No problems.");
        Ok(())
    } else {
        Err(format!(
            "Checked {count} files. {} problems:\n{}",
            problems.len(),
            problems.join("\n")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_front_matter_is_valid_toml_and_zola_shaped() {
        let out = render_work(
            "PDF \"merger\"",
            "Merges files",
            Date::parse("2026-10-11").expect("date"),
            &["Rust".into()],
        );
        let toml_src = out
            .trim_start_matches("+++\n")
            .split("+++\n")
            .next()
            .expect("front");
        let value: toml::Table = toml::from_str(toml_src).expect("valid toml");
        assert_eq!(value["title"].as_str(), Some("PDF \"merger\""));
        assert_eq!(value["template"].as_str(), Some("work-page.html"));
        assert_eq!(value["date"].to_string(), "2026-10-11");
        assert_eq!(value["extra"]["year"].as_str(), Some("2026"));
        assert_eq!(value["draft"].as_bool(), Some(true));
    }
}
