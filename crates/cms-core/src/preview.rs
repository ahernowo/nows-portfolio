//! Markdown to HTML for the admin preview pane.
//!
//! The preview is shown inside the admin page, so it must never run
//! script. Raw HTML in the Markdown is shown as text, and links or images
//! with `javascript:` and similar schemes are neutralised. The published site
//! is rendered by Zola, not by this function.

use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag};

fn options() -> Options {
    let mut o = Options::empty();
    o.insert(Options::ENABLE_TABLES);
    o.insert(Options::ENABLE_FOOTNOTES);
    o.insert(Options::ENABLE_STRIKETHROUGH);
    o.insert(Options::ENABLE_TASKLISTS);
    o.insert(Options::ENABLE_SMART_PUNCTUATION);
    o
}

/// Only these URL schemes survive. Relative links (no scheme) are kept too.
fn is_safe_url(url: &str) -> bool {
    let lower: String = url
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace() && !c.is_control())
        .collect::<String>()
        .to_ascii_lowercase();
    match lower.split_once(':') {
        Some((scheme, _))
            if !scheme.contains('/') && !scheme.contains('?') && !scheme.contains('#') =>
        {
            matches!(scheme, "http" | "https" | "mailto")
        }
        _ => true,
    }
}

fn safe(url: CowStr<'_>) -> CowStr<'_> {
    if is_safe_url(&url) {
        url
    } else {
        CowStr::Borrowed("#unsafe-link-removed")
    }
}

/// Render untrusted Markdown to HTML that is safe to insert into the admin page.
pub fn render_safe(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, options()).map(|event| match event {
        Event::Html(raw) | Event::InlineHtml(raw) => Event::Text(raw),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Link {
            link_type,
            dest_url: safe(dest_url),
            title,
            id,
        }),
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Image {
            link_type,
            dest_url: safe(dest_url),
            title,
            id,
        }),
        other => other,
    });
    let mut out = String::with_capacity(markdown.len() * 3 / 2);
    html::push_html(&mut out, parser);
    out
}

/// Rough reading time in minutes at 220 words a minute, at least 1.
pub fn reading_minutes(markdown: &str) -> u32 {
    let words = markdown.split_whitespace().count() as u32;
    words.div_ceil(220).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_markdown() {
        let html =
            render_safe("# Title\n\nSome *text* and `code`.\n\n| a | b |\n|---|---|\n| 1 | 2 |");
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<em>text</em>"));
        assert!(html.contains("<table>"));
    }

    #[test]
    fn raw_html_becomes_text() {
        let html = render_safe("<script>alert(1)</script>\n\nHi <img src=x onerror=alert(1)>");
        assert!(!html.contains("<script>"));
        assert!(!html.contains("<img src=x"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn dangerous_link_schemes_removed() {
        for md in [
            "[x](javascript:alert(1))",
            "[x](JaVaScRiPt:alert(1))",
            "[x](java\tscript:alert(1))",
            "[x](data:text/html,hi)",
            "![x](vbscript:msgbox)",
        ] {
            let html = render_safe(md);
            // Either the link is neutralised, or Markdown did not treat it as a link at all.
            let neutralised = html.contains("#unsafe-link-removed");
            let no_link = !html.contains("<a ") && !html.contains("<img ");
            assert!(neutralised || no_link, "{md} -> {html}");
            assert!(
                !html.to_ascii_lowercase().contains("href=\"javascript"),
                "{md} -> {html}"
            );
        }
    }

    #[test]
    fn normal_links_kept() {
        for md in [
            "[x](https://a.b)",
            "[x](/articles/)",
            "[x](#top)",
            "[x](mailto:a@b.c)",
            "[x](page?x=a:b)",
        ] {
            assert!(!render_safe(md).contains("unsafe"), "{md}");
        }
    }

    #[test]
    fn reading_time() {
        assert_eq!(reading_minutes(""), 1);
        assert_eq!(reading_minutes(&"word ".repeat(221)), 2);
    }
}
