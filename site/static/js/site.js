// Small enhancements. The site works without this file: every feature here
// adds to pages that are already complete.

(function () {
  "use strict";

  // ---------- Theme: light by default, dark by choice ----------

  var THEMES = ["light", "dark"];

  // The theme showing now. head.js has already applied a saved "dark" before
  // the first paint, so the attribute is the truth; reading it (rather than
  // storage) keeps the button working when storage is blocked.
  function readTheme() {
    return document.documentElement.getAttribute("data-theme") === "dark" ? "dark" : "light";
  }

  function showLabel(theme) {
    var labels = document.querySelectorAll("[data-theme-label]");
    for (var i = 0; i < labels.length; i++) labels[i].textContent = "Theme: " + theme;
  }

  function applyTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme);
    try {
      window.localStorage.setItem("theme", theme);
    } catch (e) {
      // Not saved; the choice lasts for this page only.
    }
    showLabel(theme);
  }

  function initTheme() {
    var buttons = document.querySelectorAll("[data-theme-toggle]");
    if (!buttons.length) return;
    // A first visit stores nothing: only the label is set.
    showLabel(readTheme());
    for (var i = 0; i < buttons.length; i++) {
      buttons[i].addEventListener("click", function () {
        var next = THEMES[(THEMES.indexOf(readTheme()) + 1) % THEMES.length];
        applyTheme(next);
      });
    }
  }

  // ---------- Search ----------

  var searchIndex = null;
  var searchLoading = null;

  function loadIndex(url) {
    if (searchIndex) return Promise.resolve(searchIndex);
    if (!searchLoading) {
      searchLoading = fetch(url, { credentials: "same-origin" })
        .then(function (r) {
          if (!r.ok) throw new Error("Search index failed to load");
          return r.json();
        })
        .then(function (data) {
          searchIndex = (Array.isArray(data) ? data : []).map(function (d) {
            return {
              title: d.title || "",
              description: d.description || "",
              // Drop the "#" of heading anchor links that end up in the text.
              body: (d.body || "").replace(/(^|\s)#(\s|$)/g, " "),
              url: d.permalink || d.url || d.path || "#",
              titleLower: (d.title || "").toLowerCase(),
              descriptionLower: (d.description || "").toLowerCase(),
              bodyLower: "",
            };
          });
          searchIndex = searchIndex.filter(function (d) {
            d.bodyLower = d.body.toLowerCase();
            return d.title;
          });
          return searchIndex;
        });
    }
    return searchLoading;
  }

  function escapeHtml(text) {
    return text.replace(/[&<>"']/g, function (c) {
      return { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c];
    });
  }

  function highlight(text, terms) {
    var html = escapeHtml(text);
    terms.forEach(function (term) {
      var safe = escapeHtml(term).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      html = html.replace(new RegExp("(" + safe + ")", "gi"), "<mark>$1</mark>");
    });
    return html;
  }

  function snippet(doc, terms) {
    var at = -1;
    for (var i = 0; i < terms.length && at < 0; i++) at = doc.bodyLower.indexOf(terms[i]);
    if (at < 0) return doc.description || doc.body.slice(0, 140);
    var start = Math.max(0, at - 60);
    var text = doc.body.slice(start, start + 160).replace(/\s+/g, " ").trim();
    return (start > 0 ? "…" : "") + text + "…";
  }

  function search(docs, query) {
    var terms = query
      .toLowerCase()
      .split(/\s+/)
      .filter(function (t) {
        return t.length > 1;
      });
    if (!terms.length) return { terms: terms, results: [] };
    var results = [];
    docs.forEach(function (doc) {
      var score = 0;
      for (var i = 0; i < terms.length; i++) {
        var t = terms[i];
        var s = 0;
        if (doc.titleLower.indexOf(t) >= 0) s += 10;
        if (doc.descriptionLower.indexOf(t) >= 0) s += 4;
        if (doc.bodyLower.indexOf(t) >= 0) s += 1;
        if (!s) return; // every word must appear somewhere
        score += s;
      }
      results.push({ doc: doc, score: score });
    });
    results.sort(function (a, b) {
      return b.score - a.score;
    });
    return { terms: terms, results: results.slice(0, 10) };
  }

  function initSearch() {
    var dialog = document.getElementById("search");
    if (!dialog || typeof dialog.showModal !== "function") return;
    var input = document.getElementById("search-input");
    var list = dialog.querySelector("[data-search-results]");
    var status = dialog.querySelector("[data-search-status]");
    var selected = -1;

    function links() {
      return list.querySelectorAll("a");
    }

    function select(index) {
      var all = links();
      if (!all.length) return;
      selected = (index + all.length) % all.length;
      for (var i = 0; i < all.length; i++) all[i].setAttribute("aria-selected", String(i === selected));
      all[selected].scrollIntoView({ block: "nearest" });
    }

    function render() {
      var query = input.value.trim();
      selected = -1;
      list.replaceChildren();
      if (query.length < 2) {
        status.textContent = "Type at least two letters.";
        return;
      }
      loadIndex(dialog.getAttribute("data-index"))
        .then(function (docs) {
          if (input.value.trim() !== query) return;
          var found = search(docs, query);
          status.textContent = found.results.length
            ? found.results.length + (found.results.length === 1 ? " result" : " results")
            : "Nothing matches “" + query + "”. Try fewer or different words.";
          found.results.forEach(function (r) {
            var li = document.createElement("li");
            var a = document.createElement("a");
            a.href = r.doc.url;
            a.innerHTML =
              '<span class="result-title">' + highlight(r.doc.title, found.terms) + "</span>" +
              '<span class="result-snippet">' + highlight(snippet(r.doc, found.terms), found.terms) + "</span>";
            li.appendChild(a);
            list.appendChild(li);
          });
        })
        .catch(function () {
          status.textContent = "Search isn't available right now. Use the menu to browse instead.";
        });
    }

    function open() {
      dialog.showModal();
      input.focus();
      input.select();
      loadIndex(dialog.getAttribute("data-index")).catch(function () {});
    }

    var openers = document.querySelectorAll("[data-search-open]");
    for (var i = 0; i < openers.length; i++) openers[i].addEventListener("click", open);

    input.addEventListener("input", render);
    input.addEventListener("keydown", function (e) {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        select(selected + 1);
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        select(selected - 1);
      } else if (e.key === "Enter") {
        var all = links();
        var target = all[selected >= 0 ? selected : 0];
        if (target) {
          e.preventDefault();
          window.location.href = target.href;
        }
      }
    });
    dialog.addEventListener("click", function (e) {
      if (e.target === dialog) dialog.close();
    });

    document.addEventListener("keydown", function (e) {
      var typing = /^(INPUT|TEXTAREA|SELECT)$/.test(e.target.tagName) || e.target.isContentEditable;
      if (dialog.open) return;
      if ((e.key === "/" && !typing) || (e.key.toLowerCase() === "k" && (e.metaKey || e.ctrlKey))) {
        e.preventDefault();
        open();
      }
    });
  }

  // ---------- Copy buttons on code blocks ----------

  function initCopy() {
    if (!navigator.clipboard) return;
    var blocks = document.querySelectorAll(".prose pre");
    for (var i = 0; i < blocks.length; i++) {
      (function (pre) {
        var button = document.createElement("button");
        button.type = "button";
        button.className = "code-copy";
        button.textContent = "Copy";
        button.addEventListener("click", function () {
          var code = pre.querySelector("code") || pre;
          navigator.clipboard.writeText(code.innerText.replace(/\n$/, "")).then(
            function () {
              button.textContent = "Copied";
              button.setAttribute("data-state", "done");
              setTimeout(function () {
                button.textContent = "Copy";
                button.removeAttribute("data-state");
              }, 1800);
            },
            function () {
              button.textContent = "Copy failed";
            }
          );
        });
        pre.appendChild(button);
      })(blocks[i]);
    }
  }

  function ready() {
    initTheme();
    initSearch();
    initCopy();
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", ready);
  else ready();
})();
