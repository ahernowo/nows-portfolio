+++
title = "Writing reference"
template = "article.html"
description = "Every formatting option the site supports, in one place. This page stays a draft and is never published."
date = 2026-10-23
draft = true

[taxonomies]
tags = ["reference"]
+++

Use this page to check how content looks. Run `zola serve --drafts` to see it.

## Headings

Use `##` for sections and `###` for subsections. The page title is the only `#` heading.

### A subsection

Paragraphs are plain text. You can use *emphasis*, **strong text**, `inline code`, and [links to other pages](@/about.md). Keyboard keys look like <kbd>Ctrl</kbd> + <kbd>S</kbd>.

## Lists

- Bullet lists for things in no particular order
- Each item short
  - Nested items are fine, but rarely needed

1. Numbered lists for steps
2. Where the order matters
3. Like a procedure

## Quotes and callouts

> A quotation from someone else sits in a block like this.

> [!NOTE]
> A note adds useful context.

> [!TIP]
> A tip suggests a better way.

> [!IMPORTANT]
> Important marks something the reader must not miss.

> [!WARNING]
> A warning points out a risk.

> [!CAUTION]
> Caution marks something that can cause damage.

## Code

```rust
fn main() {
    let total: u64 = [150_000, 75_000, 250_000].iter().sum();
    println!("Rp{total}");
}
```

```toml
[extra]
name = "Nows"
```

```bash
zola serve --drafts
```

## Tables

| Station | Agent | Can stop the line |
| --- | --- | --- |
| Plan | planner | No |
| Build | builder | No |
| Design review | ui-ux-reviewer | Yes |
| Security | security-checker | Yes |
| Approval | approver | Yes |

## Images

{{ <figure src="/images/og.png" alt="The site's link preview card: the name Nows on carbon-copy paper colours" caption="Use the figure component for images with captions." /> }}

## Footnotes

Footnotes are collected at the bottom of the page.[^1]

[^1]: Like this one.
