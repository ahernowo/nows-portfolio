+++
title = "How this site is built"
template = "article.html"
description = "A static site in Zola, two small Rust services on Cloudflare, and a pipeline of AI agents that check every change. All of it free to run."
date = 2026-10-23

[taxonomies]
tags = ["rust", "zola", "cloudflare", "security"]
+++

This site has three jobs: show my work, hold my writing, and teach me Rust while I build it. It also had to cost nothing to run. Here is how the pieces fit, and why each one is there.

## A static site first

Every page you read here is a plain HTML file, built ahead of time by [Zola](https://www.getzola.org/). There is no server rendering pages on request, which means there is nothing to slow down under load, nothing to patch at night, and very little to attack. Cloudflare Pages hosts the files for free and rebuilds them whenever the repository changes.

Zola is a single program written in Rust. It turns Markdown files into pages, compiles the stylesheet, highlights code and builds the search index you can open with the <kbd>/</kbd> key.

## Writing without a code editor

An article is a Markdown file in the repository. That's great for keeping history, and terrible if every post means opening a terminal. So the site has an editor, built in two stages.

### Stage one: Decap CMS and a Rust sign-in helper

[Decap CMS](https://decapcms.org/) is an editor that runs in the browser at `/admin` and saves articles straight into GitHub. It needs one small server: GitHub won't hand an access token to a web page without a secret that can't live in a browser. I wrote that server as a Cloudflare Worker in Rust.

The sign-in helper does three things in order. It sets a random `state` value in a cookie and sends you to GitHub. When GitHub sends you back, it checks that the `state` in the address matches the cookie. Then it gives the token to the editor window, but only if that window is on my site.

That last check is the one many helpers skip. Here is the comparison that guards the `state` value, written so the time it takes doesn't reveal how many characters matched:

```rust
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}
```

### Stage two: my own CMS in Rust

The second stage replaces Decap with a CMS I wrote. It's another Rust worker, with drafts stored in Cloudflare D1, a SQLite database that runs next to the worker.

Publishing doesn't put the article in the database for the site to read. Instead, the worker writes a Markdown file into the repository through the GitHub API, exactly the kind of file Decap writes, and the site rebuilds. Git stays the single record of what is public. If I ever drop the CMS, every article is still a readable file.

> [!NOTE]
> The free D1 plan allows 5 million row reads and 100,000 row writes a day. An editor used by one person stays far below that, and the public site never touches the database at all.

## The rules live in one tested library

Both workers and two command-line tools share a small Rust library. It does the fiddly, security-sensitive work: checking article fields, making slugs, writing front matter, signing session cookies and rendering a safe preview. It never touches the network or the clock, so all of it runs under `cargo test`.

One test I'm fond of feeds the front matter writer a title full of quotes and a description containing `+++`, then checks that nothing escapes its field. Because the file is written with a TOML library instead of string formatting, a cleverly worded description can't switch a draft to published.

| Concern | How it's handled |
| --- | --- |
| Someone else's page asks for a token | Sign-in hands it only to the configured site address |
| Forged session cookie | Cookie is signed with HMAC-SHA256 and expires after 8 hours |
| Cross-site form posts | `SameSite=Strict` cookie, a required custom header, and an origin check |
| Script in an article preview | Raw HTML is shown as text; `javascript:` links are removed |
| Script injected into the site | A Content Security Policy that only allows the site's own files |

## A factory of agents

I build with Claude Code, and I've set it up like a small production line. Each change becomes a work order that passes through stations: a planner writes the plan, a builder makes the change, a design reviewer checks it against the design system, a security checker looks for problems, and an approver checks the evidence and signs off. Any station can stop the line. A local dashboard shows every order and where it is.

It's more ceremony than a personal site needs, and that's the point. It is practice for working with agents on systems where mistakes cost money.

## What it costs

Nothing, as long as I use the free addresses Cloudflare gives me. A custom domain is the only thing I'd pay for, and the site works without one.
