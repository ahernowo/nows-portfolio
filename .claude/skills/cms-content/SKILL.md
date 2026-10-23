---
name: cms-content
description: How articles, case studies and pages are written and stored here (front matter, the newpost CLI, Decap and stage B CMS fields, images, voice, truthfulness). Use when creating or editing anything in site/content/, or changing the CMS fields.
---

# Content

Content is Markdown with TOML front matter in `site/content/`. Three ways write it, and all three must produce the same shape:

1. **By hand or by an agent**, starting from `cargo run -q -p newpost -- "Title"`.
2. **Decap CMS** (stage A) at `/admin/`, configured in `site/static/admin/config.yml`. Each save is a pull request; merging publishes.
3. **The Rust CMS** (stage B), which renders front matter with `cms_core::frontmatter` and commits it through the GitHub API.

The rules live in `crates/cms-core/src/article.rs` (validation) and `frontmatter.rs` (the TOML shape). When a rule changes, change it there first, then mirror it in `config.yml` patterns and hints, so all three paths agree. `cargo run -q -p newpost -- --check` checks every file against those rules.

## Articles (`site/content/articles/<slug>.md`)

```toml
+++
title = "How this site is built"          # 3 to 120 characters
template = "article.html"                 # always; the CMS filters on it
description = "One or two sentences."     # up to 300 characters; lists, search, link previews
date = 2026-10-23
draft = true                              # new pieces start as drafts

[taxonomies]
tags = ["rust", "zola"]                   # up to 8; lowercase letters, digits, spaces, hyphens; 30 characters each
+++
```

The slug is the file name: lowercase ASCII letters, digits and hyphens, up to 80 characters. A published article's slug stays fixed, because links point to it.

## Case studies (`site/content/work/<slug>.md`)

Start with `cargo run -q -p newpost -- "Title" --work`. Front matter adds `template = "work-page.html"`, `weight` (lower shows first), `include_in_feeds = false`, and `[extra]` with `year`, `role`, `status`, `stack` (a list), `link`, `source`.

A case study answers four questions, in this order: the problem, what Nows built, how it works (the interesting technical choices), and what changed because of it. Results are stated only as Nows reported them; when a number is missing, describe the change in words and leave a `<!-- Nows: add the figure -->` comment for Nows.

## Pages

`about.md` and `now.md` render with `page.html`. When the Now page changes, its `updated` date changes with it.

## Markdown that works here

- Headings start at `##` (the title is the `h1`).
- Code fences name their language: ```` ```rust ````, ```` ```toml ````, ```` ```bash ````. An unknown language fails the build.
- Alerts: `> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]`.
- Footnotes: `text[^1]` and `[^1]: the note` at the end.
- Images with captions use the component: `{{ <figure src="/images/uploads/name.png" alt="What it shows" caption="Optional" /> }}`; add `wide={true}` for a wider figure.
- `site/content/articles/writing-reference.md` (a draft) shows every element; keep it in step when a new element is added.

## Images

- Files go in `site/static/images/uploads/` (Decap uploads there too), named in lowercase with hyphens: `reconciliation-flow.png`.
- Export at the size shown, at most 1600 px wide; prefer WebP or AVIF for photos, PNG for screenshots, SVG for diagrams. Keep each file under 300 KB.
- Alt text says what the image shows and why it is there, in one sentence. Screenshots of Nows's work tools hide donor names, emails, amounts tied to people, and internal URLs.

## Voice

- First person, plain and specific: "I wrote a worker that..." over "A solution was implemented...".
- Short paragraphs; one idea each. Explain a technical term the first time it appears.
- Sentence-case headings that say what the section tells you ("Why drafts live in D1").
- Nows works in digital fundraising for a large organisation. Name employers, partners or colleagues only when the request does. Donor data, internal figures and system names stay out unless Nows has said they are public.

## Truthfulness

Every claim about Nows (what they built, their role, results, dates, tools) comes from the request, from Nows's existing content in this repo, or from the code itself. Where a piece needs a fact that isn't available, write the sentence around it and leave a visible `<!-- Nows: ... -->` comment, and mention it in the build report. Invented specifics on a portfolio are worse than a gap.
