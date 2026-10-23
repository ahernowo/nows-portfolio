//! Turning titles into URL slugs, and checking slugs typed by hand.

use unicode_normalization::UnicodeNormalization;

/// Longest slug we produce or accept.
pub const MAX_SLUG_LEN: usize = 80;

/// Make a URL slug from any title.
///
/// Accents are removed (`Café` becomes `cafe`), anything that is not a
/// letter or digit becomes a single hyphen, and the result is cut at a word
/// boundary so it is never longer than [`MAX_SLUG_LEN`].
///
/// ```
/// assert_eq!(cms_core::slug::slugify("Hello, Rust & Zola!"), "hello-rust-zola");
/// ```
pub fn slugify(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut pending_dash = false;

    // NFKD splits "é" into "e" plus a combining accent, which we then drop.
    for ch in title.nfkd() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(ch.to_ascii_lowercase());
        } else if is_combining_mark(ch) || ch == '\'' || ch == '\u{2019}' {
            // Accents and apostrophes vanish without leaving a gap: "don't" -> "dont".
        } else {
            pending_dash = true;
        }
    }

    truncate_at_word(&out, MAX_SLUG_LEN)
}

/// True when `slug` is already a clean slug: lowercase ASCII letters and
/// digits in groups joined by single hyphens.
pub fn is_valid_slug(slug: &str) -> bool {
    if slug.is_empty() || slug.len() > MAX_SLUG_LEN {
        return false;
    }
    slug.split('-').all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
    })
}

fn is_combining_mark(ch: char) -> bool {
    matches!(ch as u32, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

fn truncate_at_word(slug: &str, max: usize) -> String {
    if slug.len() <= max {
        return slug.to_string();
    }
    let cut = &slug[..max];
    match cut.rfind('-') {
        Some(idx) if idx > 0 => cut[..idx].to_string(),
        _ => cut.trim_end_matches('-').to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_titles() {
        assert_eq!(slugify("How this site is built"), "how-this-site-is-built");
        assert_eq!(slugify("  Spaces   everywhere  "), "spaces-everywhere");
        assert_eq!(slugify("Rust 1.97: what's new?"), "rust-1-97-whats-new");
    }

    #[test]
    fn accents_and_apostrophes() {
        assert_eq!(slugify("Café Société"), "cafe-societe");
        assert_eq!(slugify("Don’t panic"), "dont-panic");
    }

    #[test]
    fn non_latin_only_gives_empty_slug() {
        assert_eq!(slugify("日本語"), "");
    }

    #[test]
    fn long_titles_cut_at_word() {
        let title = "word ".repeat(40);
        let slug = slugify(&title);
        assert!(slug.len() <= MAX_SLUG_LEN);
        assert!(!slug.ends_with('-'));
        assert!(is_valid_slug(&slug));
    }

    #[test]
    fn validity() {
        assert!(is_valid_slug("hello-world-2"));
        assert!(!is_valid_slug("Hello"));
        assert!(!is_valid_slug("double--dash"));
        assert!(!is_valid_slug("-leading"));
        assert!(!is_valid_slug("trailing-"));
        assert!(!is_valid_slug("../etc/passwd"));
        assert!(!is_valid_slug(""));
    }
}
