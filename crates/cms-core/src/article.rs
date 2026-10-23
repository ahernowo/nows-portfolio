//! Article input from the editor, and the rules it must pass before we store it.

use serde::{Deserialize, Serialize};

use crate::date::Date;
use crate::slug::{is_valid_slug, slugify, MAX_SLUG_LEN};

pub const MAX_TITLE_CHARS: usize = 120;
pub const MAX_DESCRIPTION_CHARS: usize = 300;
pub const MAX_TAGS: usize = 8;
pub const MAX_TAG_CHARS: usize = 30;
/// 200 KB of Markdown is far more than any article needs, and keeps a single
/// D1 row and a single GitHub commit small.
pub const MAX_BODY_BYTES: usize = 200_000;

/// What the admin editor sends. Every field is untrusted.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArticleInput {
    pub title: String,
    /// Optional. When empty we make one from the title.
    pub slug: String,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
    /// Optional `YYYY-MM-DD`. When empty the publish date is used.
    pub date: String,
}

/// An article that passed [`validate`]. Fields are trimmed and normalised.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ValidArticle {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
    pub date: Option<Date>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Published,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Published => "published",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(Status::Draft),
            "published" => Some(Status::Published),
            _ => None,
        }
    }
}

/// One problem with one field, shown next to that field in the editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FieldError {
    pub field: &'static str,
    pub message: String,
}

impl FieldError {
    fn new(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            field,
            message: message.into(),
        }
    }
}

/// Check and normalise editor input. Returns every problem at once so the
/// editor can show them all, rather than one per save.
pub fn validate(input: ArticleInput) -> Result<ValidArticle, Vec<FieldError>> {
    let mut errors = Vec::new();

    let title = collapse_spaces(&input.title);
    let title_chars = title.chars().count();
    if title_chars < 3 {
        errors.push(FieldError::new(
            "title",
            "Title needs at least 3 characters.",
        ));
    } else if title_chars > MAX_TITLE_CHARS {
        errors.push(FieldError::new(
            "title",
            format!("Title can be at most {MAX_TITLE_CHARS} characters."),
        ));
    }
    if has_control_chars(&title) {
        errors.push(FieldError::new(
            "title",
            "Title contains invisible control characters.",
        ));
    }

    let slug = if input.slug.trim().is_empty() {
        slugify(&title)
    } else {
        input.slug.trim().to_string()
    };
    if slug.is_empty() {
        errors.push(FieldError::new(
            "slug",
            "Add a slug using letters a to z and digits, because the title has none.",
        ));
    } else if !is_valid_slug(&slug) {
        errors.push(FieldError::new(
            "slug",
            format!("Use lowercase letters, digits and single hyphens, at most {MAX_SLUG_LEN} characters."),
        ));
    }

    let description = collapse_spaces(&input.description);
    if description.chars().count() > MAX_DESCRIPTION_CHARS {
        errors.push(FieldError::new(
            "description",
            format!("Description can be at most {MAX_DESCRIPTION_CHARS} characters."),
        ));
    }
    if has_control_chars(&description) {
        errors.push(FieldError::new(
            "description",
            "Description contains invisible control characters.",
        ));
    }

    let mut tags: Vec<String> = Vec::new();
    for raw in &input.tags {
        let tag = raw.trim().to_lowercase();
        if tag.is_empty() || tags.contains(&tag) {
            continue;
        }
        if tag.chars().count() > MAX_TAG_CHARS
            || !tag
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == ' ')
        {
            errors.push(FieldError::new(
                "tags",
                format!("Tag \"{tag}\" can only use a to z, digits, spaces and hyphens, up to {MAX_TAG_CHARS} characters."),
            ));
            continue;
        }
        tags.push(tag);
    }
    if tags.len() > MAX_TAGS {
        errors.push(FieldError::new(
            "tags",
            format!("Use at most {MAX_TAGS} tags."),
        ));
    }

    let body = input.body.replace("\r\n", "\n");
    if body.len() > MAX_BODY_BYTES {
        errors.push(FieldError::new(
            "body",
            "The article is too long. Split it into two articles.",
        ));
    }

    let date = if input.date.trim().is_empty() {
        None
    } else {
        match Date::parse(&input.date) {
            Some(d) => Some(d),
            None => {
                errors.push(FieldError::new(
                    "date",
                    "Use a real date in the form YYYY-MM-DD.",
                ));
                None
            }
        }
    };

    if errors.is_empty() {
        Ok(ValidArticle {
            title,
            slug,
            description,
            tags,
            body,
            date,
        })
    } else {
        Err(errors)
    }
}

fn collapse_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn has_control_chars(s: &str) -> bool {
    s.chars().any(|c| c.is_control())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(title: &str) -> ArticleInput {
        ArticleInput {
            title: title.into(),
            body: "Hello".into(),
            ..Default::default()
        }
    }

    #[test]
    fn makes_slug_from_title() {
        let a = validate(input("  How   this site is built ")).expect("valid");
        assert_eq!(a.title, "How this site is built");
        assert_eq!(a.slug, "how-this-site-is-built");
    }

    #[test]
    fn rejects_short_title_and_bad_slug_together() {
        let mut i = input("Hi");
        i.slug = "Not A Slug".into();
        let errs = validate(i).expect_err("invalid");
        let fields: Vec<_> = errs.iter().map(|e| e.field).collect();
        assert_eq!(fields, vec!["title", "slug"]);
    }

    #[test]
    fn rejects_path_tricks_in_slug() {
        let mut i = input("Valid title");
        i.slug = "../../.github/workflows/evil".into();
        assert!(validate(i).is_err());
    }

    #[test]
    fn title_without_latin_letters_needs_manual_slug() {
        let errs = validate(input("日本語のタイトル")).expect_err("invalid");
        assert_eq!(errs[0].field, "slug");
        let mut i = input("日本語のタイトル");
        i.slug = "japanese-title".into();
        assert!(validate(i).is_ok());
    }

    #[test]
    fn tags_are_normalised_and_deduplicated() {
        let mut i = input("Tags test");
        i.tags = vec!["Rust".into(), " rust ".into(), "web dev".into(), "".into()];
        let a = validate(i).expect("valid");
        assert_eq!(a.tags, vec!["rust", "web dev"]);
    }

    #[test]
    fn rejects_odd_tags_and_too_many() {
        let mut i = input("Tags test");
        i.tags = vec!["<script>".into()];
        assert!(validate(i).is_err());

        let mut i = input("Tags test");
        i.tags = (0..9).map(|n| format!("tag{n}")).collect();
        assert!(validate(i).is_err());
    }

    #[test]
    fn control_chars_rejected() {
        assert!(validate(input("Bad\u{0007}title")).is_err());
    }

    #[test]
    fn dates() {
        let mut i = input("Dated");
        i.date = "2026-10-23".into();
        assert_eq!(
            validate(i)
                .expect("valid")
                .date
                .map(|d| d.to_string())
                .as_deref(),
            Some("2026-10-23")
        );

        let mut i = input("Dated");
        i.date = "yesterday".into();
        assert!(validate(i).is_err());
    }

    #[test]
    fn body_line_endings_normalised_and_size_limited() {
        let mut i = input("Body");
        i.body = "a\r\nb".into();
        assert_eq!(validate(i).expect("valid").body, "a\nb");

        let mut i = input("Body");
        i.body = "x".repeat(MAX_BODY_BYTES + 1);
        assert!(validate(i).is_err());
    }

    #[test]
    fn status_round_trip() {
        for s in [Status::Draft, Status::Published] {
            assert_eq!(Status::parse(s.as_str()), Some(s));
        }
        assert_eq!(Status::parse("deleted"), None);
    }
}
