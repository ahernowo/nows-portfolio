# Learning Rust with this repo

The Rust in this project is about 5,000 lines, written to be read. This is a path through it, from the smallest pure functions to async code running on Cloudflare, with an exercise at each stop. Do the exercises by hand first; ask Claude to explain, not to write. When you want a review, run your change through `/factory`, and the reviewers will tell you what they'd change.

Keep [The Rust Book](https://doc.rust-lang.org/book/) open alongside; each stop names the chapters it leans on.

## Tools for reading

```powershell
cargo test -p cms-core           # run one crate's tests
cargo test -p cms-core slug      # only tests with "slug" in the name
cargo doc -p cms-core --open     # the crate's documentation in your browser
cargo clippy --workspace         # Rust's own reviewer
```

In VS Code, the **rust-analyzer** extension shows types on hover and lets you jump to any definition with F12. It's the best teacher in the room.

## 1. Strings and tests: `crates/cms-core/src/slug.rs`

`slugify` turns a title into a web address. Read how a `&str` (a borrowed view of text) becomes a `String` (text you own), how `.chars()` walks Unicode correctly, and how `#[cfg(test)] mod tests` keeps tests next to the code.

Book: chapters 4 (ownership), 8.2 (strings), 11 (tests).

*Exercise*: titles like "Rust & Zola" become `rust-zola`. Make `&` become `and` instead. Write the test first, watch it fail (`cargo test -p cms-core slug`), then make it pass.

## 2. Types that make wrong states impossible: `date.rs`

A `Date` is a struct with a constructor that can fail (`parse` returns `Option<Date>`), a `Display` implementation so it prints as `2026-10-23`, and a hand-written `Serialize`.

Book: chapters 5 (structs), 6 (enums and `Option`), 10.2 (traits).

*Exercise*: make `Date::parse` accept `2026/10/23` as well as `2026-10-23`, with tests for both and for nonsense like `2026/13/40`. Then look at why `Date` can be compared with `>`: which derive gives it that, and why does the order of the fields matter?

## 3. Errors as values: `article.rs`

`validate` collects every problem with an article into a `Vec<FieldError>` instead of stopping at the first, and returns `Result<ValidArticle, Vec<FieldError>>`. A `ValidArticle` can only be made by passing validation: the type itself is the proof.

Book: chapter 9 (errors).

*Exercise*: titles must not end with a full stop. Add the rule, a message that says how to fix it, and tests. Notice that `newpost --check`, the stage B CMS and the CI all pick it up without further changes.

## 4. Serde and formats: `frontmatter.rs`

Front matter is a struct with `#[derive(Serialize, Deserialize)]`; the `toml` crate turns it into text and back. No string concatenation, so a title with quotes can't break the file.

Book: chapter 10 (generics, traits); the [serde guide](https://serde.rs/).

*Exercise*: add an optional `series` field ("Learning Rust, part 2") to articles: an `Option<String>` with `#[serde(skip_serializing_if = "Option::is_none")]`, a validation rule, and a line on the article page. See how `updated` is already done the same way.

## 5. Bytes and cryptography: `session.rs` and `oauth.rs`

A session token is the claims (login, issued, expires) as base64 JSON, a dot, and an HMAC-SHA256 signature of that text. Read `verify` slowly: check the signature *before* trusting anything inside, compare in constant time, then check expiry. `oauth.rs` builds URLs with the `url` crate and HTML with careful escaping.

*Exercise*: write a test showing that changing one character of a signed token makes `verify` fail. Then explain to Claude, in your own words, why `constant_time_eq` exists, and ask it to poke holes in your explanation.

## 6. A command-line tool: `tools/newpost/src/main.rs`

`clap` turns a struct into a full CLI with `--help`. The tool finds the site folder by walking up from the current directory, uses `cms-core` for everything that matters, and reports errors without panicking.

Book: chapter 12 (an I/O project).

*Exercise*: add `newpost --list` that prints every article's date, draft status and title, newest first.

## 7. State machines: `tools/factory/src/board.rs`

The factory's rules: an `enum Station`, an `Order` that moves between stations, and methods that return `Err(Refused(...))` when a move breaks a rule. Over 800 lines, but each method is short, and the tests at the bottom read like a specification.

Book: chapters 6 (enums and `match`), 8 (collections).

*Exercise*: add a `max_wip` rule: refuse `factory new` when three orders are already open. Read `new_order`, add the field to `Board` with `#[serde(default)]` so old boards still load, and test it.

## 8. Files, processes and a web server: `store.rs`, `seal.rs`, `server.rs`

Atomic saves (write a temp file, then rename), running `git` as a child process and hashing its output, and a tiny HTTP server on `std::net::TcpListener` with no framework, so you can see what a request really is.

Book: chapter 21 (the final project, a web server), which `server.rs` resembles.

*Exercise*: add a `/api/health` route that returns `{"ok":true}`, and a test for the routing function.

## 9. Async Rust on the edge: `workers/`

Worker code is `async`: each `.await` hands control back while waiting for D1 or GitHub. `workers/cms-api/src/lib.rs` routes with one `match` on method and path; `db.rs` binds every value; `github.rs` talks to a JSON API; `http.rs` turns errors into responses.

Book: chapter 17 (async); [workers-rs](https://github.com/cloudflare/workers-rs).

*Exercise*: add `GET /api/tags` to the CMS: a public list of tags and how many published articles use each. Put the counting in `cms-core` (testable), the query in `db.rs`, and the route in `lib.rs`. Run it with `npx wrangler dev`.

## After that

- Make `newpost` interactive (prompt for title and tags) with the `dialoguer` crate.
- Give the factory a performance station that runs Lighthouse; see "A new station" in [factory.md](factory.md).
- Rewrite `tools/shots/serve.mjs` in Rust as a `factory preview` command.
