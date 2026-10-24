// Loaded in <head>, before the page paints. Keep this file tiny.

// 1. Apply a saved colour theme, so the page never flashes the wrong one.
// Light is the default: with nothing saved, nothing is set and the light
// :root tokens apply, whatever the system setting.
(function () {
  var theme = null;
  try {
    theme = window.localStorage.getItem("theme");
  } catch (e) {
    // Storage can be blocked; the page then stays light.
  }
  if (theme === "light" || theme === "dark") {
    document.documentElement.setAttribute("data-theme", theme);
  }
})();

// 2. Title morph between a list and the page you open from it. The page
// heading carries data-vt="title" (see _motion.scss); here we give the
// matching list title the same view-transition-name for the duration of the
// transition. `pagereveal` fires before deferred scripts run, so this has to
// live in the head.
(function () {
  function samePath(a, b) {
    try {
      return new URL(a, location.href).pathname === new URL(b, location.href).pathname;
    } catch (e) {
      return false;
    }
  }

  function mark(url) {
    var links = document.querySelectorAll(".index-link");
    for (var i = 0; i < links.length; i++) {
      if (samePath(links[i].href, url)) {
        links[i].style.viewTransitionName = "page-title";
        return links[i];
      }
    }
    return null;
  }

  function clearAfter(transition, el) {
    if (el && transition && transition.finished) {
      transition.finished.finally(function () {
        el.style.viewTransitionName = "";
      });
    }
  }

  window.addEventListener("pageswap", function (e) {
    if (e.viewTransition && e.activation && e.activation.entry) {
      clearAfter(e.viewTransition, mark(e.activation.entry.url));
    }
  });

  window.addEventListener("pagereveal", function (e) {
    // Arriving through a transition: the title morphs in, so CSS skips the
    // case-study title rise (.vt-arrival in journey.scss).
    if (e.viewTransition) document.documentElement.classList.add("vt-arrival");
    var nav = window.navigation;
    if (e.viewTransition && nav && nav.activation && nav.activation.from) {
      clearAfter(e.viewTransition, mark(nav.activation.from.url));
    }
  });
})();
