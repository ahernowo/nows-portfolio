// Factory board. Polls /api/state every two seconds and redraws when the
// board changes. Report HTML comes from the server already sanitised, and
// the page's Content Security Policy blocks inline scripts.

(function () {
  "use strict";

  var STATIONS = [
    { key: "plan", label: "Plan", agent: "planner", file: "plan.md" },
    { key: "build", label: "Build", agent: "builder", file: "build.md" },
    { key: "design", label: "Design review", agent: "ui-ux-reviewer", file: "design.md" },
    { key: "security", label: "Security check", agent: "security-checker", file: "security.md" },
    { key: "approve", label: "Approval", agent: "approver", file: "approval.md" },
    { key: "done", label: "Done", agent: "recent", file: null },
  ];

  var state = null;
  var lastJson = "";
  var openId = null;
  var openTab = "request.md";
  var lastOk = 0;

  function $(id) {
    return document.getElementById(id);
  }

  function el(tag, attrs) {
    var node = document.createElement(tag);
    if (attrs) {
      Object.keys(attrs).forEach(function (k) {
        if (k === "text") node.textContent = attrs[k];
        else if (k === "class") node.className = attrs[k];
        else node.setAttribute(k, attrs[k]);
      });
    }
    for (var i = 2; i < arguments.length; i++) {
      var child = arguments[i];
      if (child == null) continue;
      node.append(child);
    }
    return node;
  }

  function ms(iso) {
    var t = Date.parse(iso);
    return isNaN(t) ? 0 : t;
  }

  function duration(msecs) {
    var m = Math.round(msecs / 60000);
    if (m < 1) return "under a minute";
    if (m < 60) return m + " min";
    var h = Math.floor(m / 60);
    var rest = m % 60;
    if (h < 24) return h + " h" + (rest ? " " + rest + " min" : "");
    var d = Math.floor(h / 24);
    return d + " d " + (h % 24) + " h";
  }

  function ago(iso) {
    var diff = Date.now() - ms(iso);
    if (diff < 45000) return "just now";
    return duration(diff) + " ago";
  }

  function clock(iso) {
    var d = new Date(ms(iso));
    return d.toLocaleString(undefined, { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" });
  }

  function openFindings(order) {
    return order.findings.filter(function (f) {
      return f.status === "open";
    });
  }

  // ---------- rendering ----------

  function renderTop() {
    $("project").textContent = state.project || "";
    var lamp = $("lamp");
    var board = state.board;
    var working = board.orders.some(function (o) {
      return o.status === "working";
    });
    var open = board.orders.filter(function (o) {
      return o.status !== "closed";
    }).length;
    if (board.andon) {
      lamp.dataset.state = "stopped";
      $("lamp-text").textContent = "Line stopped";
    } else if (working || open) {
      lamp.dataset.state = "running";
      $("lamp-text").textContent = working ? "Line running" : "Line running, orders waiting";
    } else {
      lamp.dataset.state = "idle";
      $("lamp-text").textContent = "Line idle";
    }
    $("andon").hidden = !board.andon;
    if (board.andon) {
      $("andon-text").textContent =
        board.andon.order + " at " + board.andon.station + ": " + board.andon.reason + " (since " + ago(board.andon.at) + ").";
    }
  }

  function renderShift() {
    var orders = state.board.orders;
    var done = orders.filter(function (o) {
      return o.station === "done";
    });
    var wip = orders.filter(function (o) {
      return o.status !== "closed";
    });
    var firstPass = done.filter(function (o) {
      return o.rework === 0;
    }).length;
    var lead = done.map(function (o) {
      return ms(o.closed_at) - ms(o.created_at);
    });
    var avgLead = lead.length
      ? duration(
          lead.reduce(function (a, b) {
            return a + b;
          }, 0) / lead.length
        )
      : "none yet";
    var sev = { critical: 0, high: 0, medium: 0, low: 0 };
    wip.forEach(function (o) {
      openFindings(o).forEach(function (f) {
        sev[f.severity] += 1;
      });
    });
    var stops = orders.reduce(function (n, o) {
      return (
        n +
        o.events.filter(function (e) {
          return e.kind === "andon";
        }).length
      );
    }, 0);

    var findings = el("dd", {});
    var any = false;
    ["critical", "high", "medium", "low"].forEach(function (k) {
      if (sev[k]) {
        any = true;
        findings.append(el("span", { class: "sev sev-" + k, text: sev[k] + " " + k }));
      }
    });
    if (!any) findings.textContent = "none";

    function item(label, value) {
      return el("div", {}, el("dt", { text: label }), typeof value === "string" ? el("dd", { text: value }) : value);
    }

    $("shift").replaceChildren(
      item("In progress", String(wip.length)),
      item("Done", String(done.length)),
      item("First-pass yield", done.length ? Math.round((firstPass / done.length) * 100) + "%" : "none yet"),
      item("Average lead time", avgLead),
      item("Line stops", String(stops)),
      item("Open findings", findings)
    );
  }

  function card(order) {
    var findings = openFindings(order);
    var since = order.events.length ? order.events[order.events.length - 1].at : order.updated_at;
    var button = el(
      "button",
      { type: "button", class: "card", "data-status": order.status, "aria-haspopup": "dialog" },
      el("span", { class: "card-id" }, el("span", { text: order.id }), el("span", { text: order.status === "blocked" ? "Blocked" : order.status })),
      el("span", { class: "card-title", text: order.title }),
      el(
        "span",
        { class: "card-meta" },
        el("span", { text: order.status === "closed" ? "closed " + ago(order.closed_at || order.updated_at) : "last move " + ago(since) }),
        order.rework ? el("span", { class: "warn", text: "rework " + order.rework }) : null,
        findings.length && order.status !== "closed"
          ? el("span", { class: "warn", text: findings.length + " open finding" + (findings.length > 1 ? "s" : "") })
          : null
      )
    );
    button.addEventListener("click", function () {
      openDrawer(order.id);
    });
    return el("li", {}, button);
  }

  function renderLine() {
    var orders = state.board.orders;
    var columns = STATIONS.map(function (s) {
      var here = orders.filter(function (o) {
        return o.station === s.key;
      });
      if (s.key === "done") {
        here = here
          .slice()
          .sort(function (a, b) {
            return ms(b.closed_at) - ms(a.closed_at);
          })
          .slice(0, 6);
      }
      var list = el("ul", { class: "station-cards" });
      if (!here.length) list.append(el("li", { class: "station-empty", text: s.key === "done" ? "Nothing finished yet." : "Empty" }));
      here.forEach(function (o) {
        list.append(card(o));
      });
      return el(
        "section",
        { class: "station", "aria-label": s.label },
        el(
          "div",
          { class: "station-head" },
          el("h2", {}, el("span", { text: s.label }), el("span", { class: "station-count", text: String(here.length) })),
          el("p", { class: "station-agent", text: s.key === "done" ? "Most recent first" : "Agent: " + s.agent })
        ),
        list
      );
    });
    $("line").replaceChildren.apply($("line"), columns);
  }

  function renderLower() {
    var lessons = $("lessons");
    lessons.replaceChildren();
    (state.lessons || []).forEach(function (l) {
      lessons.append(el("li", { text: l.replace(/^- /, "") }));
    });
    if (!lessons.children.length) lessons.append(el("li", { class: "empty-note", text: "No lessons yet. The approver adds one after each order." }));

    var events = [];
    state.board.orders.forEach(function (o) {
      o.events.forEach(function (e, i) {
        events.push({ order: o, e: e, seq: i });
      });
    });
    // Newest first. Events in the same second keep their recorded order.
    events.sort(function (a, b) {
      return ms(b.e.at) - ms(a.e.at) || (a.order.id === b.order.id ? b.seq - a.seq : 0);
    });
    var feed = $("feed");
    feed.replaceChildren();
    events.slice(0, 40).forEach(function (x) {
      feed.append(
        el(
          "li",
          {},
          el("time", { datetime: x.e.at, text: clock(x.e.at) }),
          el("span", { class: "kind", text: x.order.id + " " + x.e.kind }),
          document.createTextNode(" at " + x.e.station + " by " + x.e.actor + (x.e.note ? ": " + x.e.note : ""))
        )
      );
    });
    if (!feed.children.length) feed.append(el("li", { class: "empty-note", text: "Nothing has happened yet. Start an order with /factory in Claude Code." }));
  }

  // ---------- drawer ----------

  function findOrder(id) {
    return state.board.orders.find(function (o) {
      return o.id === id;
    });
  }

  function openDrawer(id) {
    openId = id;
    openTab = "request.md";
    renderDrawer();
    if (!$("drawer").open) $("drawer").showModal();
  }

  function renderDrawer() {
    var order = openId && findOrder(openId);
    if (!order) return;
    $("drawer-id").textContent = order.id + ", opened " + clock(order.created_at) + (order.rework ? ", rework " + order.rework : "");
    $("drawer-title").textContent = order.title;

    var track = $("drawer-track");
    track.replaceChildren();
    STATIONS.slice(0, 5).forEach(function (s) {
      var st = "todo";
      if (order.passed.indexOf(s.key) >= 0) st = "passed";
      if (order.station === s.key) st = order.status === "blocked" ? "blocked" : "current";
      track.append(el("li", { "data-state": st, text: s.label }));
    });

    var have = (state.reports && state.reports[order.id]) || [];
    var tabs = [{ id: "request.md", label: "Request", enabled: true }];
    STATIONS.slice(0, 5).forEach(function (s) {
      tabs.push({ id: s.file, label: s.label, enabled: have.indexOf(s.key) >= 0 });
    });
    tabs.push({ id: "findings", label: "Findings (" + order.findings.length + ")", enabled: true });
    tabs.push({ id: "history", label: "History", enabled: true });

    var tabRow = $("drawer-tabs");
    tabRow.replaceChildren();
    tabs.forEach(function (t) {
      var b = el("button", { type: "button", role: "tab", "aria-selected": String(openTab === t.id), text: t.label });
      b.disabled = !t.enabled;
      b.addEventListener("click", function () {
        openTab = t.id;
        renderDrawer();
      });
      tabRow.append(b);
    });

    var body = $("drawer-body");
    if (openTab === "findings") {
      body.replaceChildren(findingsTable(order));
    } else if (openTab === "history") {
      body.replaceChildren(historyList(order));
    } else {
      loadReport(order.id, openTab);
    }
  }

  function findingsTable(order) {
    if (!order.findings.length) return el("p", { text: "No findings logged." });
    var rows = order.findings.map(function (f) {
      return el(
        "tr",
        {},
        el("td", { text: "#" + f.n }),
        el("td", {}, el("span", { class: "sev sev-" + f.severity, text: f.severity })),
        el("td", { class: f.status === "open" ? "status-open" : "", text: f.status }),
        el("td", { text: f.station }),
        el("td", {}, document.createTextNode(f.title), f.file ? el("div", { class: "card-meta", text: f.file }) : null, f.note ? el("div", { class: "card-meta", text: "Note: " + f.note }) : null)
      );
    });
    var head = el("tr", {}, el("th", { text: "" }), el("th", { text: "Severity" }), el("th", { text: "Status" }), el("th", { text: "Station" }), el("th", { text: "Finding" }));
    var table = el("table", { class: "table" }, el("thead", {}, head), el("tbody", {}));
    rows.forEach(function (r) {
      table.tBodies[0].append(r);
    });
    return table;
  }

  function historyList(order) {
    var list = el("ol", { class: "history" });
    order.events
      .slice()
      .reverse()
      .forEach(function (e) {
        list.append(
          el(
            "li",
            {},
            el("time", { datetime: e.at, text: clock(e.at) }),
            el("span", { class: "kind", text: e.kind + " (" + e.station + ")" }),
            el("span", { text: e.actor + (e.note ? ": " + e.note : "") })
          )
        );
      });
    if (order.seal) {
      list.append(el("li", {}, el("time", { text: clock(order.seal.at) }), el("span", { class: "kind", text: "seal" }), el("span", { text: order.seal.hash.slice(0, 16) + "… over " + order.seal.files + " files" })));
    }
    return list;
  }

  function loadReport(id, file) {
    var body = $("drawer-body");
    body.replaceChildren(el("p", { text: "Loading…" }));
    fetch("/api/orders/" + encodeURIComponent(id) + "/" + encodeURIComponent(file))
      .then(function (r) {
        if (!r.ok) throw new Error("missing");
        return r.json();
      })
      .then(function (data) {
        if (openId !== id || openTab !== file) return;
        var div = el("div", { class: "report" });
        div.innerHTML = data.html;
        body.replaceChildren(div);
      })
      .catch(function () {
        if (openId !== id || openTab !== file) return;
        body.replaceChildren(el("p", { text: "This report hasn't been written yet." }));
      });
  }

  // ---------- polling ----------

  function render() {
    renderTop();
    renderShift();
    renderLine();
    renderLower();
    if ($("drawer").open) {
      var keepTab = openTab;
      renderDrawer();
      openTab = keepTab;
    }
  }

  function tick() {
    fetch("/api/state", { cache: "no-store" })
      .then(function (r) {
        return r.json();
      })
      .then(function (data) {
        lastOk = Date.now();
        if (data.error) {
          $("lamp").dataset.state = "stopped";
          $("lamp-text").textContent = data.error;
          return;
        }
        var json = JSON.stringify(data.board) + JSON.stringify(data.lessons) + JSON.stringify(data.reports);
        state = data;
        if (json !== lastJson) {
          lastJson = json;
          render();
        }
      })
      .catch(function () {
        $("lamp").removeAttribute("data-state");
        $("lamp-text").textContent = "Lost connection to `factory serve`";
      })
      .finally(function () {
        $("updated").textContent = lastOk ? "Updated " + ago(new Date(lastOk).toISOString()) : "";
        setTimeout(tick, 2000);
      });
  }

  $("drawer-close").addEventListener("click", function () {
    $("drawer").close();
  });
  $("drawer").addEventListener("close", function () {
    openId = null;
  });
  $("drawer").addEventListener("click", function (e) {
    if (e.target === $("drawer")) $("drawer").close();
  });

  tick();
})();
