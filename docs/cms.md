# Writing and the two editors

There are three ways to write for the site. All three produce the same thing in the end: a Markdown file in `site/content/`, committed to GitHub, which Cloudflare Pages builds into a page.

| Way | Best for | Where drafts live |
|---|---|---|
| Decap CMS (stage A) at `<your-site>/admin/` | Writing in the browser, images, case studies | Pull requests on GitHub |
| Your Rust CMS (stage B) at `<cms-worker>/admin` | Articles, from any device, with your own code doing the work | Cloudflare D1 |
| The command line, `newpost` | Writing in your editor, alongside code | Files in your working copy |

## Decap CMS (stage A)

1. Open `<your-site>/admin/` and sign in with GitHub.
2. **Articles → New Article.** Title, description (shown in lists, search and link previews), date, topics, and the body. New articles start with **Draft** on.
3. **Save.** Decap opens a pull request. Cloudflare Pages builds a preview of it; the preview link appears in the editor.
4. Move the entry through **Drafts → In review → Ready**. **Publish** merges the pull request; the site rebuilds in about a minute.
5. To make it public, turn **Draft** off before publishing (drafts stay out of the build even after merging).

Images uploaded in the editor go to `site/static/images/uploads/`. Keep them under 300 KB, and write alt text that says what the image shows.

**Try it without GitHub**: in one terminal `npx decap-server`, in another `zola --root site serve`, then open http://127.0.0.1:1111/admin/. Changes go straight to the files in your folder.

**Case studies** live under **Work**: year, your role, status, what it was built with, and links. Lower **Order** numbers show first.

## Your Rust CMS (stage B)

Open `https://portfolio-cms.<your-subdomain>.workers.dev/admin` and sign in with GitHub. Only logins in `ADMIN_LOGINS` (in `workers/cms-api/wrangler.toml`) get in.

- **New article**, then write in the **Write** tab; **Preview** shows it rendered. Ctrl+S saves the draft.
- **Publish** commits the article to `site/content/articles/<slug>.md` through the GitHub API; the site rebuilds. From then on the slug is fixed, because links point to it.
- **Unpublish** rewrites the file with `draft = true`, so the page disappears on the next build but the history stays.
- **Delete** removes an article for good. A published article must be unpublished first; deleting it then also removes its file from the repository.
- **Activity** lists who did what, and when.

If two tabs edit the same article, the second save is refused with a message, so neither overwrites the other.

How it works inside: drafts are rows in D1, validated by the same Rust rules (`crates/cms-core`) that `newpost --check` and the CI use, and the front matter is written by the same code. The worker is in `workers/cms-api/`.

## From the command line

```powershell
newpost "Why reconciliation fails at month end" --tags "reconciliation,payments" --description "Three causes I keep finding, and the checks that catch them."
newpost "Donor portal" --work
newpost --check
```

The first creates `site/content/articles/why-reconciliation-fails-at-month-end.md` as a draft; `--work` creates a case study; `--check` checks every file the way CI does. Inside the repo you can also run `cargo run -q -p newpost -- ...`.

Formatting you can use (alerts, footnotes, figures, code) is all shown on one draft page: `site/content/articles/writing-reference.md`. Preview it with `zola --root site serve --drafts`.

## Upgrading Decap

Decap is loaded from unpkg, pinned to one version with an integrity hash, so the browser refuses the file if it ever changes. To move to a new version (say 3.18.0):

1. Compute the new hash. In Git Bash:

   ```bash
   curl -sL https://unpkg.com/decap-cms@3.18.0/dist/decap-cms.js | openssl dgst -sha384 -binary | openssl base64 -A
   ```

2. In `site/static/admin/index.html`, change the version in the `src` and put `sha384-<the hash>` in `integrity`.
3. In `site/static/_headers`, change the version in the `/admin/*` policy, in both `script-src` and `connect-src`.
4. Test with `zola --root site serve` and sign in at `/admin/` before pushing.

Or: `/factory Upgrade Decap CMS to 3.18.0` and let the line do it; the security checker verifies the hash.

## Keys and tokens

| Secret | Where | Rotate when |
|---|---|---|
| OAuth app client secrets | GitHub → Developer settings → OAuth Apps; `npx wrangler secret put GITHUB_CLIENT_SECRET` in each worker | Yearly, or if exposed |
| `GITHUB_CONTENT_TOKEN` | Fine-grained token, Contents read/write on this repo only | Before it expires (90 days) |
| `SESSION_KEY` | `openssl rand -base64 32`, then `npx wrangler secret put SESSION_KEY` | Any time; everyone signs in again |

If a secret ever lands in a commit, treat it as public: rotate it first, then remove it from the history.
