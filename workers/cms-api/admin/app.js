// Admin editor for the Rust CMS worker. Plain JavaScript, no build step.
// Every request that changes data sends X-CMS-Request: 1, which the worker
// requires as part of its cross-site request checks.

const $ = (id) => document.getElementById(id);

const state = {
  articles: [],
  current: null, // the article as last saved
  dirty: false,
  busy: false,
  previewTimer: 0,
};

// ---------- API ----------

class ApiError extends Error {
  constructor(status, body) {
    super(body?.error?.message || `Request failed with status ${status}.`);
    this.status = status;
    this.fields = body?.error?.fields || [];
  }
}

async function api(method, path, body) {
  const options = { method, credentials: "same-origin", headers: { Accept: "application/json" } };
  if (method !== "GET") options.headers["X-CMS-Request"] = "1";
  if (body !== undefined) {
    options.headers["Content-Type"] = "application/json";
    options.body = JSON.stringify(body);
  }
  const response = await fetch(path, options);
  if (response.status === 204) return null;
  let data = null;
  try {
    data = await response.json();
  } catch {
    // Non-JSON reply, handled below.
  }
  if (!response.ok) {
    if (response.status === 401) showSignIn("Your session ended. Sign in again.");
    throw new ApiError(response.status, data);
  }
  return data;
}

// ---------- small helpers ----------

function el(tag, attrs = {}, ...children) {
  const node = document.createElement(tag);
  for (const [key, value] of Object.entries(attrs)) {
    if (key === "class") node.className = value;
    else if (key.startsWith("data-") || key.startsWith("aria-")) node.setAttribute(key, value);
    else node[key] = value;
  }
  for (const child of children) node.append(child);
  return node;
}

const relative = new Intl.RelativeTimeFormat(undefined, { numeric: "auto" });
function ago(unixSeconds) {
  const diff = unixSeconds - Date.now() / 1000;
  const steps = [
    [60, "second"],
    [3600, "minute"],
    [86400, "hour"],
    [604800, "day"],
    [2629800, "week"],
    [31557600, "month"],
    [Infinity, "year"],
  ];
  let unit = "second";
  let divisor = 1;
  for (const [limit, name] of steps) {
    if (Math.abs(diff) < limit) {
      unit = name;
      break;
    }
    divisor = limit;
  }
  return relative.format(Math.round(diff / divisor), unit);
}

function slugify(text) {
  return text
    .normalize("NFKD")
    .replace(/[̀-ͯ]/g, "")
    .replace(/['’]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 80)
    .replace(/-+$/, "");
}

let toastTimer = 0;
function toast(message, bad = false) {
  const t = $("toast");
  t.textContent = message;
  t.classList.toggle("bad", bad);
  t.hidden = false;
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => (t.hidden = true), bad ? 7000 : 3500);
}

function confirmAction(text, okLabel) {
  return new Promise((resolve) => {
    const dialog = $("confirm");
    $("confirm-text").textContent = text;
    $("confirm-ok").textContent = okLabel;
    dialog.addEventListener("close", () => resolve(dialog.returnValue === "ok"), { once: true });
    dialog.showModal();
  });
}

// ---------- sign-in ----------

const signinMessages = {
  "not-allowed": "That GitHub account is not on the editor list. Ask the site owner to add it to ADMIN_LOGINS.",
  expired: "Sign-in took too long or was started in another tab. Try again.",
  cancelled: "Sign-in was cancelled on GitHub.",
  failed: "GitHub sign-in failed. Try again in a minute.",
};

function showSignIn(message) {
  $("app").hidden = true;
  $("signin").hidden = false;
  for (const id of ["who", "activity-button", "signout-button"]) $(id).hidden = true;
  if (message) $("signin-message").textContent = message;
}

async function boot() {
  const params = new URLSearchParams(location.search);
  const signin = params.get("signin");
  if (signin) history.replaceState(null, "", location.pathname);

  try {
    const me = await api("GET", "/api/me");
    $("who").textContent = `Signed in as ${me.login}`;
    for (const id of ["who", "activity-button", "signout-button"]) $(id).hidden = false;
    $("signin").hidden = true;
    $("app").hidden = false;
    await loadList();
    const fromHash = location.hash.slice(1);
    if (fromHash) openArticle(fromHash);
  } catch (error) {
    if (error.status === 401 || error.status === 403) {
      showSignIn(signinMessages[signin] || (error.status === 403 ? error.message : undefined));
    } else {
      showSignIn("The CMS could not start. Check that the worker is deployed and configured.");
    }
  }
}

// ---------- list ----------

async function loadList() {
  const data = await api("GET", "/api/admin/articles");
  state.articles = data.articles;
  renderList();
}

function renderList() {
  const list = $("article-list");
  list.replaceChildren();
  $("list-empty").hidden = state.articles.length > 0;
  for (const article of state.articles) {
    const button = el(
      "button",
      { type: "button", "aria-current": String(state.current?.id === article.id) },
      article.title,
      el("span", { class: "meta" }, `${article.status === "published" ? "Published" : "Draft"}, edited ${ago(article.updated_at)}`),
    );
    button.addEventListener("click", () => openArticle(article.id));
    list.append(el("li", {}, button));
  }
}

// ---------- editor ----------

const fields = ["title", "slug", "date", "description", "tags", "body"];

function formValues() {
  return {
    title: $("title").value,
    slug: $("slug").value.trim(),
    date: $("date").value,
    description: $("description").value,
    tags: $("tags").value.split(",").map((t) => t.trim()).filter(Boolean),
    body: $("body").value,
  };
}

function fillForm(article) {
  $("title").value = article?.title ?? "";
  $("slug").value = article?.slug ?? "";
  $("date").value = article?.date ?? "";
  $("description").value = article?.description ?? "";
  $("tags").value = (article?.tags ?? []).join(", ");
  $("body").value = article?.body ?? "";
  clearErrors();
  updateDerived();
}

function updateDerived() {
  const slug = $("slug").value.trim() || slugify($("title").value);
  $("slug-preview").textContent = slug || "…";
  $("description-count").textContent = `${$("description").value.length} of 300`;
  const locked = Boolean(state.current?.published_slug);
  $("slug").readOnly = locked;
  $("slug-hint").lastChild.textContent = locked ? "/ (fixed: this article has been published)" : "/";
}

function setDirty(dirty) {
  state.dirty = dirty;
  const s = $("save-state");
  s.classList.toggle("dirty", dirty);
  if (dirty) s.textContent = "Unsaved changes";
  else if (state.current) s.textContent = `Saved ${ago(state.current.updated_at)} by ${state.current.updated_by}`;
  else s.textContent = "";
}

function showStatus() {
  const article = state.current;
  const pill = $("status-pill");
  const published = article?.status === "published";
  pill.textContent = article ? (published ? "Published" : "Draft") : "New";
  pill.className = `pill ${published ? "published" : "draft"}`;
  $("save-button").textContent = published ? "Save" : "Save draft";
  $("publish-button").textContent = published ? "Publish changes" : "Publish";
  $("publish-button").hidden = !article;
  $("unpublish-button").hidden = !published;
  $("delete-button").hidden = !article || published;
}

function clearErrors() {
  for (const node of document.querySelectorAll("[data-error]")) node.textContent = "";
  for (const id of fields) $(id).removeAttribute("aria-invalid");
}

function showErrors(errors) {
  clearErrors();
  let first = null;
  for (const { field, message } of errors) {
    const slot = document.querySelector(`[data-error="${field}"]`);
    if (slot) slot.textContent = slot.textContent ? `${slot.textContent} ${message}` : message;
    const input = $(field);
    if (input) {
      input.setAttribute("aria-invalid", "true");
      first ??= input;
    }
  }
  first?.focus();
}

async function guardUnsaved() {
  if (!state.dirty) return true;
  return confirmAction("You have unsaved changes. Leave them behind?", "Discard changes");
}

async function openArticle(id) {
  if (!(await guardUnsaved())) return;
  try {
    const data = await api("GET", `/api/admin/articles/${encodeURIComponent(id)}`);
    state.current = data.article;
    location.hash = id;
    showEditor();
  } catch (error) {
    toast(error.message, true);
  }
}

async function newArticle() {
  if (!(await guardUnsaved())) return;
  state.current = null;
  history.replaceState(null, "", location.pathname);
  showEditor();
  $("title").focus();
}

function showEditor() {
  $("editor-placeholder").hidden = true;
  $("form").hidden = false;
  fillForm(state.current);
  showStatus();
  setDirty(false);
  selectTab("write");
  renderList();
}

async function save(event) {
  event?.preventDefault();
  if (state.busy) return;
  await run($("save-button"), async () => {
    const values = formValues();
    const data = state.current
      ? await api("PUT", `/api/admin/articles/${state.current.id}`, { ...values, version: state.current.version })
      : await api("POST", "/api/admin/articles", values);
    state.current = data.article;
    location.hash = data.article.id;
    fillForm(state.current);
    showStatus();
    setDirty(false);
    await loadList();
    toast(data.note || "Saved.");
  });
}

async function publish() {
  if (state.dirty) {
    toast("Save your changes before publishing.", true);
    return;
  }
  await run($("publish-button"), async () => {
    const data = await api("POST", `/api/admin/articles/${state.current.id}/publish`);
    state.current = data.article;
    showStatus();
    updateDerived();
    setDirty(false);
    await loadList();
    toast(data.note || "Published.");
  });
}

async function unpublish() {
  const ok = await confirmAction("Hide this article from the site? The file stays in git, marked as a draft.", "Unpublish");
  if (!ok) return;
  await run($("unpublish-button"), async () => {
    const data = await api("POST", `/api/admin/articles/${state.current.id}/unpublish`);
    state.current = data.article;
    showStatus();
    setDirty(false);
    await loadList();
    toast(data.note || "Unpublished.");
  });
}

async function remove() {
  const ok = await confirmAction(`Delete "${state.current.title}"? This cannot be undone.`, "Delete");
  if (!ok) return;
  await run($("delete-button"), async () => {
    await api("DELETE", `/api/admin/articles/${state.current.id}`);
    state.current = null;
    state.dirty = false;
    history.replaceState(null, "", location.pathname);
    $("form").hidden = true;
    $("editor-placeholder").hidden = false;
    await loadList();
    toast("Deleted.");
  });
}

async function run(button, task) {
  state.busy = true;
  button.disabled = true;
  try {
    await task();
  } catch (error) {
    if (error.fields?.length) showErrors(error.fields);
    toast(error.message, true);
  } finally {
    state.busy = false;
    button.disabled = false;
  }
}

// ---------- preview ----------

function selectTab(which) {
  const write = which === "write";
  $("tab-write").setAttribute("aria-selected", String(write));
  $("tab-preview").setAttribute("aria-selected", String(!write));
  $("body").hidden = !write;
  $("preview").hidden = write;
  if (!write) refreshPreview();
}

async function refreshPreview() {
  try {
    const data = await api("POST", "/api/admin/preview", { markdown: $("body").value });
    // The worker escapes raw HTML and unsafe links, and the page's Content
    // Security Policy blocks inline scripts, so this cannot run code.
    $("preview").innerHTML = data.html;
    $("minutes").textContent = `${data.minutes} minute read`;
  } catch (error) {
    $("preview").textContent = error.message;
  }
}

// ---------- activity ----------

async function showActivity() {
  try {
    const data = await api("GET", "/api/admin/audit");
    const list = $("activity-list");
    list.replaceChildren();
    for (const entry of data.entries) {
      list.append(
        el(
          "li",
          {},
          `${entry.actor} ${entry.action}d ${entry.detail}`,
          el("time", { dateTime: new Date(entry.at * 1000).toISOString() }, ago(entry.at)),
        ),
      );
    }
    if (!data.entries.length) list.append(el("li", {}, "Nothing yet."));
    $("activity").showModal();
  } catch (error) {
    toast(error.message, true);
  }
}

// ---------- wiring ----------

function wire() {
  $("new-button").addEventListener("click", newArticle);
  $("form").addEventListener("submit", save);
  $("publish-button").addEventListener("click", publish);
  $("unpublish-button").addEventListener("click", unpublish);
  $("delete-button").addEventListener("click", remove);
  $("tab-write").addEventListener("click", () => selectTab("write"));
  $("tab-preview").addEventListener("click", () => selectTab("preview"));
  $("activity-button").addEventListener("click", showActivity);
  $("signout-button").addEventListener("click", async () => {
    await api("POST", "/auth/logout").catch(() => {});
    showSignIn("Signed out.");
  });

  $("form").addEventListener("input", () => {
    setDirty(true);
    updateDerived();
  });

  // Tab inserts two spaces in the body instead of leaving the field.
  $("body").addEventListener("keydown", (event) => {
    if (event.key === "Tab" && !event.shiftKey && !event.altKey && !event.ctrlKey && !event.metaKey) {
      event.preventDefault();
      const area = event.target;
      area.setRangeText("  ", area.selectionStart, area.selectionEnd, "end");
      setDirty(true);
    }
    if (event.key === "Escape") area_blur(event.target);
  });

  document.addEventListener("keydown", (event) => {
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
      event.preventDefault();
      if (!$("form").hidden) save();
    }
  });

  window.addEventListener("beforeunload", (event) => {
    if (state.dirty) event.preventDefault();
  });
}

// Escape leaves the body field, so keyboard users are never trapped by Tab.
function area_blur(area) {
  area.blur();
  $("save-button").focus();
}

wire();
boot();
