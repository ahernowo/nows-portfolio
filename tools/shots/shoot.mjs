// Screenshots and accessibility scans of the built site, for the UI/UX
// reviewer and for Nows.
//
//   node tools/shots/shoot.mjs --out target/shots/WO-0007 --pages /,/articles/
//
// Options (all optional):
//   --pages   comma-separated paths            default: every top-level page
//   --widths  comma-separated viewport widths  default: 390,1440
//   --themes  light,dark (dark is set as the visitor's saved choice)  default: light,dark
//   --out     folder for the PNG files         default: target/shots/latest
//   --drafts  include draft pages
//   --site    use an already built folder instead of building
//   --root    the Zola site folder to build (for a test copy)  default: site
//   --reduced also take reduced-motion shots
//   --focus   tab through each page and photograph the focus ring
//   --viewport-only   capture the first screen instead of the whole page
//   --scroll "<selector>:<f1>,<f2>,..."   after each normal shot, scroll
//             through the element (0 = its top at the top of the screen,
//             1 = its end at the bottom) and save a viewport shot at each
//             fraction, e.g. --scroll "#journey-story:0.05,0.5,0.97"
//   --no-axe  skip the accessibility scan
//   --strict  exit with code 1 when anything is reported (for CI)
//
// The script builds the site itself with Zola (which must be on PATH) into
// target/shots-site, with the base URL set to the local server, so every
// link and asset resolves locally.
// First run on a new machine: cd tools/shots && npm install && npx playwright install chromium
//
// The report goes to standard output as Markdown, ready to paste into a
// design review. Screenshots never change the repo: target/ is git-ignored.

import fs from "node:fs";
import net from "node:net";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { startServer } from "./serve.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(here, "..", "..");

function parseArgs(argv) {
  const opts = { flags: new Set() };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (!a.startsWith("--")) continue;
    const key = a.slice(2);
    const next = argv[i + 1];
    if (next !== undefined && !next.startsWith("--")) {
      opts[key] = next;
      i++;
    } else {
      opts.flags.add(key);
    }
  }
  return opts;
}

const opts = parseArgs(process.argv.slice(2));
const out = path.resolve(repo, opts.out || "target/shots/latest");
const widths = (opts.widths || "390,1440").split(",").map(Number).filter(Boolean);
const themes = (opts.themes || "light,dark").split(",").filter((t) => t === "light" || t === "dark");
const runAxe = !opts.flags.has("no-axe");
const fullPage = !opts.flags.has("viewport-only");

// --scroll "<selector>:<fractions>": split at the last colon, so selectors
// with colons (like :nth-child) still work.
function parseScroll(spec) {
  if (!spec) return null;
  const at = spec.lastIndexOf(":");
  if (at < 1) return null;
  const fractions = spec
    .slice(at + 1)
    .split(",")
    .map(Number)
    .filter((f) => Number.isFinite(f) && f >= 0 && f <= 1);
  return fractions.length ? { selector: spec.slice(0, at), fractions } : null;
}
const scroll = parseScroll(opts.scroll);
if (opts.scroll && !scroll) {
  console.error('--scroll needs "<selector>:<fractions>", for example "#journey-story:0.05,0.5".');
  process.exit(2);
}

function freePort() {
  return new Promise((resolve, reject) => {
    const probe = net.createServer();
    probe.once("error", reject);
    probe.listen(0, "127.0.0.1", () => {
      const { port } = probe.address();
      probe.close(() => resolve(port));
    });
  });
}

const port = await freePort();
let site;
if (opts.site) {
  site = path.resolve(repo, opts.site);
} else {
  site = path.join(repo, "target", "shots-site");
  // An argument array, never a shell string, so --root is passed as is.
  const args = ["--root", opts.root || "site", "build", "--base-url", `http://127.0.0.1:${port}`, "--output-dir", site, "--force"];
  if (opts.flags.has("drafts")) args.push("--drafts");
  const build = spawnSync("zola", args, { cwd: repo, encoding: "utf8" });
  if (build.error || build.status !== 0) {
    console.error("Zola could not build the site for screenshots:");
    console.error(build.error ? String(build.error.message) : (build.stderr || build.stdout).trim());
    process.exit(2);
  }
}

if (!fs.existsSync(path.join(site, "index.html"))) {
  console.error(`No built site at ${path.relative(repo, site)}.`);
  process.exit(2);
}

function defaultPages() {
  const pages = ["/"];
  for (const entry of fs.readdirSync(site, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    if (["admin", "fonts", "js", "images", "tags"].includes(entry.name)) continue;
    if (fs.existsSync(path.join(site, entry.name, "index.html"))) pages.push(`/${entry.name}/`);
  }
  pages.push("/404.html");
  return pages;
}

const pages = opts.pages ? opts.pages.split(",").map((p) => p.trim()).filter(Boolean) : defaultPages();

function slug(p) {
  const s = p.replace(/\.html$/, "").replace(/^\/+|\/+$/g, "").replace(/[^a-z0-9]+/gi, "-");
  return s || "home";
}

let AxeBuilder = null;
if (runAxe) {
  try {
    ({ default: AxeBuilder } = await import("@axe-core/playwright"));
  } catch {
    console.error("Accessibility scan skipped: @axe-core/playwright is not installed (cd tools/shots && npm install).");
  }
}

fs.mkdirSync(out, { recursive: true });
const { server, url: base } = await startServer(site, port);

let browser;
try {
  browser = await chromium.launch(process.env.CHROMIUM_PATH ? { executablePath: process.env.CHROMIUM_PATH } : {});
} catch (error) {
  server.close();
  console.error("Could not start Chromium. Run once: cd tools/shots && npx playwright install chromium");
  console.error(String(error.message || error).split("\n")[0]);
  process.exit(2);
}

const problems = [];
const shots = [];
const axeResults = [];
const focusNotes = [];

async function openPage({ width, theme, reduced }) {
  const context = await browser.newContext({
    viewport: { width, height: width < 700 ? 844 : 900 },
    deviceScaleFactor: 1,
    colorScheme: theme,
    reducedMotion: reduced ? "reduce" : "no-preference",
  });
  // The site ignores the system setting and is light unless the visitor picked
  // dark with the Theme button. Save that choice before any page script runs,
  // the way the button does, so dark shots and dark axe scans really are dark.
  if (theme === "dark") {
    await context.addInitScript(() => {
      try {
        localStorage.setItem("theme", "dark");
      } catch (e) {}
    });
  }
  const page = await context.newPage();
  return { context, page };
}

function watch(page, label) {
  page.on("console", (m) => {
    if (m.type() === "error" || m.type() === "warning") problems.push(`${label}: console ${m.type()}: ${m.text()}`);
  });
  page.on("pageerror", (e) => problems.push(`${label}: script error: ${e.message}`));
  page.on("requestfailed", (r) => problems.push(`${label}: request failed: ${r.url().replace(base, "")} (${r.failure()?.errorText})`));
  page.on("response", (r) => {
    if (r.status() >= 400 && !r.url().endsWith("/404.html")) problems.push(`${label}: HTTP ${r.status()} for ${r.url().replace(base, "")}`);
  });
}

async function settle(page, reduced) {
  await page.waitForLoadState("networkidle");
  // The home page's slips and stamp finish landing at about 1.6 s.
  await page.waitForTimeout(reduced ? 300 : 1900);
}

for (const p of pages) {
  for (const theme of themes) {
    const motions = opts.flags.has("reduced") ? [false, true] : [false];
    for (const reduced of motions) {
      for (const width of widths) {
        const label = `${p} ${width}px ${theme}${reduced ? " reduced-motion" : ""}`;
        const { context, page } = await openPage({ width, theme, reduced });
        watch(page, label);
        await page.goto(base + p);
        await settle(page, reduced);
        const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
        if (overflow > 1) problems.push(`${label}: page scrolls sideways by ${overflow}px`);
        const file = path.join(out, `${slug(p)}-${width}-${theme}${reduced ? "-reduced" : ""}.png`);
        await page.screenshot({ path: file, fullPage });
        shots.push(path.relative(repo, file).replaceAll("\\", "/"));

        // One accessibility scan per page and theme, at the widest width.
        if (AxeBuilder && !reduced && width === Math.max(...widths)) {
          const result = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
          for (const v of result.violations) {
            axeResults.push({ where: `${p} ${theme}`, id: v.id, impact: v.impact, help: v.help, count: v.nodes.length, target: v.nodes[0]?.target?.join(" ") });
          }
        }

        // Scroll shots run with motion on only: with reduced motion the
        // scroll-driven parts are switched off and every fraction looks alike.
        if (scroll && !reduced) {
          for (const f of scroll.fractions) {
            const found = await page.evaluate(
              ({ selector, f }) => {
                const el = document.querySelector(selector);
                if (!el) return false;
                const top = el.getBoundingClientRect().top + window.scrollY;
                window.scrollTo(0, top + (el.offsetHeight - window.innerHeight) * f);
                return true;
              },
              { selector: scroll.selector, f }
            );
            // Pages without the element (the others in --pages) are skipped.
            if (!found) break;
            await page.waitForTimeout(600);
            const shot = path.join(out, `${slug(p)}-${width}-${theme}-scroll-${String(f).replace(".", "_")}.png`);
            await page.screenshot({ path: shot });
            shots.push(path.relative(repo, shot).replaceAll("\\", "/"));
          }
        }
        await context.close();
      }
    }
  }

  if (opts.flags.has("focus")) {
    const width = Math.max(...widths);
    const { context, page } = await openPage({ width, theme: themes[0] || "light", reduced: true });
    watch(page, `${p} focus`);
    await page.goto(base + p);
    await settle(page, true);
    for (let n = 1; n <= 10; n++) {
      await page.keyboard.press("Tab");
      const info = await page.evaluate(() => {
        const el = document.activeElement;
        if (!el || el === document.body) return null;
        // A ring may sit on the element or on a wrapper (for example a list
        // row styled with :has(a:focus-visible)), so look a few levels up.
        const ringed = (node) => {
          const s = getComputedStyle(node);
          return (s.outlineStyle !== "none" && parseFloat(s.outlineWidth) > 0) || (s.boxShadow && s.boxShadow !== "none");
        };
        let visible = false;
        for (let node = el, i = 0; node && node !== document.body && i < 4; node = node.parentElement, i++) {
          if (ringed(node)) {
            visible = true;
            break;
          }
        }
        const r = el.getBoundingClientRect();
        const name = (el.getAttribute("aria-label") || el.textContent || el.getAttribute("title") || el.tagName).trim().replace(/\s+/g, " ").slice(0, 40);
        return { name, tag: el.tagName.toLowerCase(), visible, x: r.x, y: r.y, w: r.width, h: r.height };
      });
      if (!info) break;
      if (!info.visible) focusNotes.push(`${p}: tab stop ${n} (${info.tag} "${info.name}") shows no outline or ring`);
      if (info.w > 0 && info.h > 0) {
        const pad = 24;
        const clip = { x: Math.max(0, info.x - pad), y: Math.max(0, info.y - pad), width: Math.min(width, info.w + pad * 2), height: info.h + pad * 2 };
        const file = path.join(out, `${slug(p)}-focus-${n}.png`);
        await page.screenshot({ path: file, clip }).catch(() => {});
        shots.push(path.relative(repo, file).replaceAll("\\", "/"));
      }
    }
    await context.close();
  }
}

await browser.close();
server.close();

const lines = [];
lines.push(`## Screenshots (${shots.length})`, "");
lines.push(`Folder: \`${path.relative(repo, out).replaceAll("\\", "/")}\``, "");
for (const s of shots) lines.push(`- ${s}`);
lines.push("", `## Console, network and layout problems (${problems.length})`, "");
lines.push(...(problems.length ? [...new Set(problems)].map((x) => `- ${x}`) : ["- none"]));
if (AxeBuilder) {
  lines.push("", `## Accessibility scan, WCAG 2.2 AA (${axeResults.length} rule failures)`, "");
  lines.push(...(axeResults.length ? axeResults.map((v) => `- ${v.where}: **${v.id}** (${v.impact}, ${v.count} element${v.count === 1 ? "" : "s"}): ${v.help}. First: \`${v.target}\``) : ["- none"]));
}
if (opts.flags.has("focus")) {
  lines.push("", `## Focus (${focusNotes.length} problems)`, "");
  lines.push(...(focusNotes.length ? focusNotes.map((x) => `- ${x}`) : ["- every tab stop shows a focus ring"]));
}
console.log(lines.join("\n"));

const failures = problems.length + axeResults.length + focusNotes.length;
if (opts.flags.has("strict") && failures > 0) {
  console.error(`\n${failures} problem(s) found.`);
  process.exit(1);
}
