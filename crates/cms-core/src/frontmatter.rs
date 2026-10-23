//! Writing and reading Zola Markdown files with TOML front matter.
//!
//! We build the front matter with the `toml` crate instead of string
//! formatting. That way a title such as `"quotes" and \ backslashes` or a
//! description containing `+++` can never break out of its field.

use serde::{Deserialize, Serialize};

use crate::article::ValidArticle;
use crate::date::Date;

/// The front matter fields this site uses for articles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    /// Always `article.html` for articles. Decap CMS uses this field to tell
    /// article files apart from the section's `_index.md`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub date: toml::value::Datetime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<toml::value::Datetime>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default, skip_serializing_if = "Taxonomies::is_empty")]
    pub taxonomies: Taxonomies,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Taxonomies {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Taxonomies {
    fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingOpeningFence,
    MissingClosingFence,
    Toml(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingOpeningFence => write!(f, "file does not start with +++"),
            ParseError::MissingClosingFence => write!(f, "front matter has no closing +++"),
            ParseError::Toml(e) => write!(f, "front matter is not valid TOML: {e}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// A calendar date as a TOML date (`2026-10-23`, no time), the form Zola
/// expects in front matter.
pub fn toml_date(d: Date) -> toml::value::Datetime {
    toml::value::Datetime {
        date: Some(toml::value::Date {
            year: d.year as u16,
            month: d.month,
            day: d.day,
        }),
        time: None,
        offset: None,
    }
}

impl FrontMatter {
    /// Front matter for an article. `date` is used when the article has no
    /// date of its own; `updated` is set only when it differs from `date`.
    pub fn for_article(
        article: &ValidArticle,
        date: Date,
        updated: Option<Date>,
        draft: bool,
    ) -> Self {
        let date = article.date.unwrap_or(date);
        Self {
            title: article.title.clone(),
            template: Some("article.html".to_string()),
            description: article.description.clone(),
            date: toml_date(date),
            updated: updated.filter(|u| *u > date).map(toml_date),
            draft,
            taxonomies: Taxonomies {
                tags: article.tags.clone(),
            },
        }
    }
}

/// Render a complete Zola Markdown file.
pub fn render(front: &FrontMatter, body: &str) -> String {
    // Serialising our own struct cannot fail: every field is a plain value.
    let toml = toml::to_string(front).unwrap_or_default();
    let body = body.trim_matches('\n');
    format!("+++\n{}+++\n\n{}\n", toml, body)
}

/// Split a Zola Markdown file into front matter and body.
pub fn parse(file: &str) -> Result<(FrontMatter, String), ParseError> {
    let file = file.strip_prefix('\u{feff}').unwrap_or(file);
    let rest = file
        .strip_prefix("+++\n")
        .or_else(|| file.strip_prefix("+++\r\n"))
        .ok_or(ParseError::MissingOpeningFence)?;

    // The closing fence is the first line that is exactly `+++`.
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end_matches(['\r', '\n']) == "+++" {
            let toml_src = &rest[..offset];
            let body = rest[offset + line.len()..].trim_start_matches(['\r', '\n']);
            let front: FrontMatter =
                toml::from_str(toml_src).map_err(|e| ParseError::Toml(e.to_string()))?;
            return Ok((front, body.to_string()));
        }
        offset += line.len();
    }
    Err(ParseError::MissingClosingFence)
}

/// Where a published article lives in the repository.
pub fn article_path(content_dir: &str, slug: &str) -> String {
    format!("{}/{}.md", content_dir.trim_end_matches('/'), slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article::{validate, ArticleInput};

    fn article(title: &str, description: &str) -> ValidArticle {
        validate(ArticleInput {
            title: title.into(),
            description: description.into(),
            tags: vec!["rust".into(), "zola".into()],
            body: "First paragraph.\n\n+++\n\nStill body.".into(),
            ..Default::default()
        })
        .expect("valid")
    }

    #[test]
    fn renders_zola_front_matter() {
        let a = article("Hello", "A first post");
        let date = Date::parse("2026-10-23").expect("date");
        let out = render(&FrontMatter::for_article(&a, date, None, true), &a.body);
        assert!(out.starts_with("+++\ntitle = \"Hello\"\ntemplate = \"article.html\"\n"));
        assert!(out.contains("date = 2026-10-23\n"));
        assert!(out.contains("draft = true\n"));
        assert!(out.contains("[taxonomies]\ntags = [\"rust\", \"zola\"]\n"));
        assert!(!out.contains("updated"));
    }

    #[test]
    fn hostile_strings_stay_inside_their_field() {
        let a = article("Quote \" and \\ and \n+++", "Ends here\"\ndraft = false");
        let date = Date::parse("2026-10-23").expect("date");
        let out = render(&FrontMatter::for_article(&a, date, None, true), &a.body);
        let (front, body) = parse(&out).expect("parses");
        assert_eq!(front.title, a.title);
        assert_eq!(front.description, a.description);
        assert!(front.draft, "description must not be able to flip draft");
        assert_eq!(body.trim_end(), a.body);
    }

    #[test]
    fn updated_only_when_later() {
        let a = article("Hello", "");
        let date = Date::parse("2026-10-23").expect("date");
        let same = FrontMatter::for_article(&a, date, Some(date), false);
        assert!(same.updated.is_none());
        let later = Date::parse("2026-11-01").expect("date");
        let fm = FrontMatter::for_article(&a, date, Some(later), false);
        assert_eq!(
            fm.updated.map(|d| d.to_string()).as_deref(),
            Some("2026-11-01")
        );
    }

    #[test]
    fn parse_errors() {
        assert_eq!(parse("no fence"), Err(ParseError::MissingOpeningFence));
        assert_eq!(
            parse("+++\ntitle = \"x\"\n"),
            Err(ParseError::MissingClosingFence)
        );
        assert!(matches!(
            parse("+++\ntitle = \n+++\n"),
            Err(ParseError::Toml(_))
        ));
    }

    #[test]
    fn parses_windows_line_endings() {
        let src = "+++\r\ntitle = \"Win\"\r\ndate = 2026-01-02\r\n+++\r\n\r\nBody\r\n";
        let (front, body) = parse(src).expect("parses");
        assert_eq!(front.title, "Win");
        assert_eq!(body.trim_end(), "Body");
    }

    #[test]
    fn path() {
        assert_eq!(
            article_path("site/content/articles/", "hello"),
            "site/content/articles/hello.md"
        );
    }
}
