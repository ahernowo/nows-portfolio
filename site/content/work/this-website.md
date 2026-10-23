+++
title = "This website and its Rust CMS"
template = "work-page.html"
description = "A portfolio that costs nothing to host, with a CMS I wrote in Rust to learn the language."
date = 2026-10-23
weight = 1
include_in_feeds = false

[extra]
year = "2026"
role = "Design, build and writing"
status = "Live, still growing"
stack = ["Rust", "Zola", "Cloudflare Workers", "Cloudflare D1", "Decap CMS"]
source = ""
+++

## The problem

I wanted a place to show my work and write about it, with three conditions. It had to cost nothing to run. It had to have a proper editor for articles, so writing a post doesn't mean opening a code editor. And building it had to teach me Rust, because I want Rust in my toolkit for the data tools I make at work.

## What I built

The public site is static HTML built by [Zola](https://www.getzola.org/), a site generator written in Rust. Static pages are fast, cheap to host and have almost nothing to attack. Cloudflare Pages serves them for free.

The editing side came in two stages.

**Stage one** uses Decap CMS, an editor that runs in the browser and saves each article as a Markdown file in the site's GitHub repository. Decap needs a small server for GitHub sign-in, and I wrote that server in Rust as a Cloudflare Worker.

**Stage two** is my own CMS, also a Rust worker. Drafts live in Cloudflare D1, a SQLite database. When I press publish, the worker writes the article into the repository through the GitHub API, and the site rebuilds. Git stays the record of what is public, and the database is only the editing desk.

| Part | Runs on | Language |
| --- | --- | --- |
| Public site | Cloudflare Pages | Zola templates, Sass, a little JavaScript |
| Sign-in helper | Cloudflare Workers | Rust |
| CMS API and editor | Cloudflare Workers and D1 | Rust, plain JavaScript for the editor page |
| Shared logic | Compiled into both workers and the CLI tools | Rust, with unit tests |

## Decisions worth explaining

**Logic lives in a plain Rust library.** Validation, slugs, front matter, session signing and the safe Markdown preview are in one crate that never touches the network or the clock. That makes every rule testable with `cargo test`, and the workers stay thin.

**The sign-in helper only hands a token to my own site.** A common shortcut posts the GitHub token to whichever page asks. Mine checks a random `state` value against a cookie, then sends the token only to the exact origin I configured.

**Published files are written with a TOML library, not string formatting.** A title with quotes or a description containing `+++` can't break out of its field and change the page's settings. There is a test for exactly that.

**No database rows for sessions.** The editor's session is a signed, expiring cookie. Removing a GitHub login from the allowlist signs that person out on their next request.

## What I'd do next

Image uploads in the Rust CMS, and a scheduled check that compares what D1 thinks is published with what is actually in git, which is reconciliation again, just for articles.
