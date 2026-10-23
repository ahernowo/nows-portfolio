# Setup on Windows 11

From an empty laptop to a live site with a working editor and the agent factory. Each step ends with a **checkpoint**: something you can see that proves the step worked. If a checkpoint fails, stop there and ask Claude Code about that step; the later steps build on it.

Time: about an hour for steps 1 to 5 (the site live), and another hour for the two CMS stages.

Everything here is free: GitHub, Cloudflare Pages, Cloudflare Workers and D1 all have free plans that cover a personal site many times over.

---

## 1. Install the tools

Unzip the project somewhere without spaces in the path, for example `C:\code\nows-portfolio`. Open **PowerShell** in that folder and run:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\setup.ps1
```

The script installs Git, the C++ build tools Rust needs, Rust, Zola 0.23.6 and Node.js (each only if missing), then builds the two project tools, `factory` and `newpost`, prepares the factory board and runs the tests. The C++ build tools are a large download; let it finish.

Then **open a new terminal**, so the new programs are on your PATH.

**Checkpoint**: each of these prints a version:

```powershell
git --version; rustc --version; zola --version; node --version; factory --version; newpost --version
```

## 2. Open the project in Claude Code

In the project folder, run `claude`. Claude Code asks whether you trust this folder: say yes. The project's settings, hooks, agents and skills only switch on after that.

**Checkpoint**: under the prompt you see the status line `factory: idle, 0 done`, and typing `/` lists `/factory`, `/factory-resume`, `/release` and `/kaizen`.

## 3. Preview the site

```powershell
zola --root site serve
```

Open http://127.0.0.1:1111. Edits to content, templates and styles reload the page by themselves. Stop the server with Ctrl+C.

**Checkpoint**: the home page shows the headline and the receipt stack landing, then the "Reconciled" stamp.

Before going public, make the site yours:

- `site/zola.toml`: in `[extra]`, set `email`, `github`, `linkedin` (or leave `""` to hide), `source`, and the `now_line`.
- `site/content/about.md` and `site/content/now.md`: your words.
- `site/content/work/`: two case studies are drafts waiting for your facts (`bank-reconciliation.md`, `donation-dashboard.md`). Fill the `<!-- Nows: ... -->` comments, then set `draft = false`.

Or ask the factory: `/factory Fill in the bank reconciliation case study. Here are the facts: ...`

## 4. Put the code on GitHub

1. On github.com, create a new **public** repository named `nows-portfolio`, with no README (the project has one).
2. In the project folder:

   ```powershell
   git init -b main
   git add .
   git commit -m "Start the portfolio"
   git remote add origin https://github.com/<your-login>/nows-portfolio.git
   git push -u origin main
   ```

3. Replace `your-github-login` with your login in `site/static/admin/config.yml`, `workers/cms-api/wrangler.toml` and `site/zola.toml`, then commit and push again.

**Checkpoint**: the repository page shows your files, and the **Actions** tab shows the CI workflow running. Its three jobs (Rust, Workers, Site) turn green within about ten minutes.

## 5. Publish with Cloudflare Pages

1. Sign up at dash.cloudflare.com (free plan).
2. **Workers & Pages → Create → Pages → Connect to Git**, and pick `nows-portfolio`.
3. Build settings:

   | Setting | Value |
   |---|---|
   | Framework preset | None |
   | Build command | `zola build` (production) |
   | Build output directory | `public` |
   | Root directory | `site` |
   | Environment variable | `ZOLA_VERSION` = `0.23.6` |

4. Save and deploy. Pages gives you an address like `nows-portfolio.pages.dev`.
5. Put that address in `site/zola.toml` as `base_url`, in `site/static/admin/config.yml` as `site_url` and `display_url`, and in both workers' `wrangler.toml` (`ALLOWED_ORIGIN`, `SITE_ORIGIN`). Commit and push.
6. Preview builds for branches and pull requests need their own address: under **Settings → Builds → Build command**, set the preview command to `zola build --base-url "$CF_PAGES_URL"`.

**Checkpoint**: your `pages.dev` address shows the site, and https://securityheaders.com rates it A or A+.

## 6. Stage A editor: Decap CMS

Decap is an editor in the browser at `/admin/`. It saves each article to GitHub as a pull request. GitHub sign-in goes through a small Rust worker in `workers/decap-oauth`.

1. **GitHub OAuth app**: github.com → Settings → Developer settings → OAuth Apps → New.
   - Homepage URL: your site address.
   - Authorization callback URL: `https://decap-oauth.<your-subdomain>.workers.dev/callback` (your workers subdomain is under Workers & Pages → Overview in Cloudflare).
   - Copy the Client ID, and generate a Client secret.
2. **Deploy the worker** (first time, `npx wrangler login` opens a browser to connect to Cloudflare):

   ```powershell
   cd workers\decap-oauth
   npx wrangler secret put GITHUB_CLIENT_ID
   npx wrangler secret put GITHUB_CLIENT_SECRET
   npx wrangler deploy
   cd ..\..
   ```

   The first deploy compiles Rust to WebAssembly and takes a few minutes.
3. In `site/static/admin/config.yml`, set `base_url` to the worker's address (`https://decap-oauth.<your-subdomain>.workers.dev`). Commit and push.

**Checkpoint**: on `<your-site>/admin/`, "Login with GitHub" opens a GitHub window, and after approving you see the Articles and Work collections.

Writing with Decap: [cms.md](cms.md).

## 7. Stage B editor: your Rust CMS

Stage B is the CMS you own end to end: drafts in Cloudflare D1, publishing straight into the repository. It can run alongside Decap.

1. **Database**:

   ```powershell
   cd workers\cms-api
   npx wrangler d1 create portfolio-cms
   ```

   Paste the printed `database_id` into `workers/cms-api/wrangler.toml`, then:

   ```powershell
   npx wrangler d1 migrations apply portfolio-cms --remote
   ```

2. **A second GitHub OAuth app**, callback `https://portfolio-cms.<your-subdomain>.workers.dev/auth/callback`.
3. **A fine-grained personal access token**: github.com → Settings → Developer settings → Fine-grained tokens. Repository access: only `nows-portfolio`. Permissions: Contents, read and write. Expiry: 90 days (set a calendar reminder to rotate it).
4. **Secrets and deploy**:

   ```powershell
   npx wrangler secret put GITHUB_CLIENT_ID
   npx wrangler secret put GITHUB_CLIENT_SECRET
   npx wrangler secret put GITHUB_CONTENT_TOKEN
   npx wrangler secret put SESSION_KEY
   npx wrangler deploy
   cd ..\..
   ```

   For `SESSION_KEY`, paste 32 random bytes in base64. In Git Bash: `openssl rand -base64 32`.
5. In `wrangler.toml`, `ADMIN_LOGINS` must be your GitHub login: only listed logins can edit.

**Checkpoint**: `https://portfolio-cms.<your-subdomain>.workers.dev/admin` lets you sign in, create a draft, preview it, and publish it. A minute after publishing, the article is on your site.

## 8. Your first factory order

In Claude Code:

```
/factory Add a "Back to all writing" link at the end of every article.
```

In a second terminal, run `factory serve` and open http://127.0.0.1:4600 to watch the order move through the stations. At the end, read `git diff`, commit and push.

**Checkpoint**: the dashboard shows the order in **Done**, and `.factory/orders/WO-0001/` holds the five reports.

How the line works, and how to tune it: [factory.md](factory.md).

## 9. Optional: your own domain

Cloudflare Pages → your project → Custom domains. Then update `base_url`, `site_url`, `ALLOWED_ORIGIN` and `SITE_ORIGIN`, the OAuth apps' homepage URLs, and redeploy both workers.

---

## When something goes wrong

| Symptom | Likely cause and fix |
|---|---|
| `factory` not found in Claude Code | New terminal needed after setup; or run `cargo install --path tools/factory --locked` again |
| `link.exe not found` during cargo builds | C++ build tools missing: rerun `scripts\setup.ps1` |
| Zola error `Variable ... not found` | Tera 2 treats missing values as errors: use `value or ""` (ask Claude, the `zola-site` skill knows) |
| Decap's GitHub window opens but you never get signed in | `ALLOWED_ORIGIN` in `workers/decap-oauth/wrangler.toml` must equal your site address exactly (scheme and host, no trailing slash); redeploy the worker after changing it |
| Pages build fails on `zola` | `ZOLA_VERSION` environment variable missing in Pages settings |
| Agents ask permission for every command | The folder isn't trusted yet: restart `claude` in the project and accept the trust dialog |
