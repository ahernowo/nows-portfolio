// The donor journey on case studies that switch it on (work-page.html).
// The page is complete without this file: both scenes are drawn in their
// finished state by the journey_scene component, and the steps are a list.
// This file adds the motion:
//   1. each scene is drawn from one progress number, 0 to 1 (bindScene);
//   2. the hero plays its first beat once, with a Pause animation button;
//   3. the walkthrough is pinned and driven by scrolling;
//   4. the recording plays while it is on screen, with a Play/Pause button
//      (it does nothing while the recording is switched off in
//      partials/journey.html, because there is no <video> to find).
// Style changes go through element.style only, which the CSP allows.

(function () {
  "use strict";

  var rooms = document.querySelectorAll(".jn-room");
  if (!rooms.length) return;

  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)");
  function motionAllowed() {
    return !reduce.matches;
  }

  // ---------- Small maths ----------

  function clamp(x, a, b) {
    return Math.min(b, Math.max(a, x));
  }
  // How far p is through the span a..b, from 0 to 1.
  function seg(p, a, b) {
    return clamp((p - a) / (b - a), 0, 1);
  }
  function ease(x) {
    return x < 0.5 ? 4 * x * x * x : 1 - Math.pow(-2 * x + 2, 3) / 2;
  }
  function lerp(a, b, u) {
    return a + (b - a) * u;
  }

  // ---------- 1. The scene ----------

  // Where the thumb rests and types, in the hands drawing's units.
  var TARGETS = { rest: [178, 326], key: [104, 238] };
  // Thumb waypoints: [progress, target]. A pause on one target is two entries.
  var WAYPOINTS = [
    [0, "rest"], [0.07, "rest"], [0.088, "notif"], [0.1, "notif"], [0.125, "rest"],
    [0.18, "rest"], [0.2, "chip"], [0.214, "chip"], [0.24, "rest"], [0.262, "cont1"], [0.288, "cont1"], [0.312, "rest"],
    [0.322, "fname"], [0.338, "fname"], [0.35, "key"], [0.415, "key"], [0.43, "femail"], [0.445, "femail"], [0.455, "key"], [0.515, "key"],
    [0.525, "fphone"], [0.538, "fphone"], [0.55, "key"], [0.59, "key"], [0.612, "rest"],
    [0.622, "chk"], [0.636, "chk"], [0.652, "cont2"], [0.672, "cont2"], [0.7, "rest"],
    [0.715, "ewallet"], [0.73, "ewallet"], [0.76, "pay"], [0.786, "pay"], [0.82, "rest"], [1, "rest"],
  ];
  var TAPS = [0.095, 0.207, 0.278, 0.333, 0.44, 0.532, 0.63, 0.662, 0.724, 0.782];
  var NAME = "Rina Pratama";
  var EMAIL = "rina@example.com";
  var PHONE = "0812 3456 7890";
  // The shoulder the arm grows from, in the hands drawing's units.
  var SHOULDER = [196, 470];

  // Fade a screen in over a..b and out over c..d (null: no fade that side).
  function show(node, p, a, b, c, d) {
    var fadeIn = a === null ? 1 : seg(p, a, b);
    var o = fadeIn * (1 - (c === null ? 0 : seg(p, c, d)));
    node.style.opacity = o;
    node.style.visibility = o > 0.001 ? "visible" : "hidden";
    node.style.transform = "translateY(" + (1 - fadeIn) * 5 + "cqw)";
  }

  // The first part of text, as if typed between a and b.
  function typed(text, p, a, b) {
    return text.slice(0, Math.round(text.length * seg(p, a, b)));
  }

  // Bind the server-rendered nodes of one scene. Returns render(p) and layout().
  function bindScene(root) {
    function $(s) {
      return root.querySelector(s);
    }
    function all(s) {
      return [].slice.call(root.querySelectorAll(s));
    }

    var phone = $(".jn-phone");
    var off = $(".jn-off");
    var person = $(".jn-hop");
    var head = $(".jn-head");
    var pupils = $(".jn-pupils");
    var brows = $(".jn-brows");
    var mouths = [$(".jn-m0"), $(".jn-m1"), $(".jn-m2")];
    var cheeks = all(".jn-cheek");
    var pages = [".jn-p0", ".jn-p1", ".jn-p2", ".jn-p3", ".jn-p4"].map($);
    var chip = $(".jn-c150");
    var b1 = $(".jn-b1");
    var b2 = $(".jn-b2");
    var b3 = $(".jn-b3");
    var payLabel = $(".jn-paylabel");
    var check = $(".jn-chk");
    var fields = [$(".jn-fn"), $(".jn-fe"), $(".jn-fp")];
    var keyboard = $(".jn-kb");
    var methods = all(".jn-pm");
    var tick = $(".jn-ring path");
    var burst = all(".jn-burst line");
    var arm = $(".jn-arm");
    var handLines = [$(".jn-hand-o"), $(".jn-hand")];
    var tap = $(".jn-tap");
    var rows = all(".jn-back li");
    var stamp = $(".jn-stamp");
    var points = {};

    // Where each tap target sits on the phone, in the hands drawing's units
    // (the phone is 160 wide there). Layout offsets ignore transforms, so the
    // fades and slides never move these points.
    function layout() {
      var k = 160 / (phone.offsetWidth || 160);
      all("[data-t]").forEach(function (n) {
        var x = 0;
        var y = 0;
        for (var o = n; o && o !== phone; o = o.offsetParent) {
          x += o.offsetLeft;
          y += o.offsetTop;
        }
        var name = n.getAttribute("data-t");
        var across = name === "chk" ? 0.12 : 0.5;
        points[name] = [(x + n.offsetWidth * across) * k, (y + n.offsetHeight / 2) * k];
      });
    }

    function target(name) {
      return points[name] || TARGETS[name] || TARGETS.rest;
    }

    // The thumb's tip at progress p: eased between the two waypoints around p.
    function thumb(p) {
      var j = 0;
      while (j < WAYPOINTS.length - 2 && p > WAYPOINTS[j + 1][0]) j++;
      var from = WAYPOINTS[j];
      var to = WAYPOINTS[j + 1];
      var u = ease(seg(p, from[0], to[0]));
      var a = target(from[1]);
      var b = target(to[1]);
      var x = lerp(a[0], b[0], u);
      var y = lerp(a[1], b[1], u);
      if (from[1] === "key" && to[1] === "key") y -= Math.abs(Math.sin(p * 1500)) * 5; // typing
      return [x, y];
    }

    function render(p) {
      // The phone comes up and the screen wakes.
      phone.style.setProperty("--lift", ease(seg(p, 0, 0.06)));
      off.style.opacity = 1 - seg(p, 0.045, 0.065);

      // Screens
      show(pages[0], p, null, 0, 0.118, 0.122);
      show(pages[1], p, 0.105, 0.118, 0.318, 0.322);
      show(pages[2], p, 0.305, 0.318, 0.708, 0.712);
      show(pages[3], p, 0.695, 0.708, 0.843, 0.847);
      show(pages[4], p, 0.835, 0.848, null, 0);
      pages[0].style.transform = "";

      // Screen 1: the amount, then Continue
      chip.classList.toggle("on", p > 0.207);
      b1.classList.toggle("dis", p <= 0.207);

      // Screen 2: typing, the focused field, the keyboard
      fields[0].querySelector(".jn-tx").textContent = typed(NAME, p, 0.35, 0.415);
      fields[1].querySelector(".jn-tx").textContent = typed(EMAIL, p, 0.455, 0.515);
      fields[2].querySelector(".jn-tx").textContent = typed(PHONE, p, 0.55, 0.59);
      fields[0].classList.toggle("act", p > 0.333 && p < 0.44);
      fields[1].classList.toggle("act", p >= 0.44 && p < 0.532);
      fields[2].classList.toggle("act", p >= 0.532 && p < 0.61);
      var keys = ease(seg(p, 0.318, 0.34)) * (1 - ease(seg(p, 0.592, 0.614)));
      keyboard.style.transform = "translateY(" + (1 - keys) * 100 + "%)";
      check.classList.toggle("on", p > 0.63);
      b2.classList.toggle("dis", p <= 0.63);

      // Screen 3: the method, Pay, the wait
      methods[0].classList.toggle("on", p > 0.724);
      b3.classList.toggle("dis", p <= 0.724);
      var busy = p > 0.782 && p < 0.85;
      b3.classList.toggle("busy", busy);
      payLabel.textContent = busy ? "Confirming" : "Pay Rp150.000";

      // Screen 4: the tick draws, a small burst
      tick.style.strokeDashoffset = 1 - ease(seg(p, 0.85, 0.885));
      var bu = seg(p, 0.875, 0.92);
      burst.forEach(function (line) {
        line.style.opacity = bu > 0 && bu < 1 ? 1 - bu : 0;
        line.style.transform = "scale(" + (1 + bu * 0.25) + ")";
        line.style.transformOrigin = "30px 30px";
      });

      // The arm runs from the shoulder to the wrist, the thumb from the wrist
      // to the tip.
      var tip = thumb(p);
      var dx = tip[0] - SHOULDER[0];
      var dy = tip[1] - SHOULDER[1];
      var len = Math.sqrt(dx * dx + dy * dy);
      var wx = tip[0] - (dx / len) * 48;
      var wy = tip[1] - (dy / len) * 48;
      setLine(arm, SHOULDER[0], SHOULDER[1], wx, wy);
      handLines.forEach(function (n) {
        setLine(n, wx, wy, tip[0], tip[1]);
      });
      // A ripple where the thumb touches.
      var phase = 0;
      TAPS.forEach(function (t) {
        var e = (p - t) / 0.016;
        if (e >= 0 && e < 1) phase = e;
      });
      tap.setAttribute("cx", tip[0]);
      tap.setAttribute("cy", tip[1]);
      tap.setAttribute("r", 5 + phase * 14);
      tap.style.opacity = phase > 0 ? (1 - phase) * 0.9 : 0;

      // The donor: looks down at the phone, smiles, beams at the end.
      var look = 1 - seg(p, 0.03, 0.07);
      var joy = ease(seg(p, 0.85, 0.9));
      var smile = ease(seg(p, 0.29, 0.33)) * 0.9;
      head.style.transform = "rotate(" + (lerp(7, 0, 1 - look) - 4 * joy) + "deg)";
      var lx = clamp((tip[0] - 80) / 90, -1, 1) * 1.6 * (1 - joy);
      var ly = lerp(1.6, 3, look) - joy * 3;
      pupils.style.transform = "translate(" + lx + "px," + ly + "px)";
      brows.style.transform = "translateY(" + -3.2 * joy + "px)";
      mouths[0].style.opacity = (1 - smile) * (1 - joy);
      mouths[1].style.opacity = smile * (1 - joy);
      mouths[2].style.opacity = joy;
      cheeks.forEach(function (c) {
        c.style.opacity = joy * 0.32;
      });
      person.style.transform = "translateY(" + Math.sin(Math.PI * seg(p, 0.85, 0.93)) * -9 + "px)";

      // Behind the screen: the panel slides in, each record saves in turn.
      root.style.setProperty("--sh", ease(seg(p, 0.9, 0.955)));
      rows.forEach(function (li, n) {
        var r = seg(p, 0.935 + n * 0.016, 0.955 + n * 0.016);
        li.querySelector(".jn-meter i").style.width = r * 100 + "%";
        li.classList.toggle("saving", r > 0 && r < 1);
        li.classList.toggle("done", r >= 1);
        var status = r >= 1 ? (n === 1 ? "Confirmed" : "Saved") : r > 0 ? "Saving" : "Waiting";
        li.querySelector(".jn-st").textContent = status;
      });
      var sp = seg(p, 0.992, 1);
      stamp.style.opacity = Math.min(1, sp * 8);
      stamp.style.transform = "rotate(-8deg) scale(" + (1.7 - 0.7 * ease(sp)) + ")";
    }

    function setLine(line, x1, y1, x2, y2) {
      line.setAttribute("x1", x1);
      line.setAttribute("y1", y1);
      line.setAttribute("x2", x2);
      line.setAttribute("y2", y2);
    }

    root.classList.add("is-live");
    layout();
    if (window.ResizeObserver) new ResizeObserver(layout).observe(root);
    if (document.fonts && document.fonts.ready) document.fonts.ready.then(layout);
    return { render: render, layout: layout };
  }

  // ---------- 2. The hero's first beat, and its pause ----------

  var HERO_REST = 0.25; // the amount is chosen; the scene rests here
  var BEAT_DELAY = 700; // ms before the beat starts
  var BEAT_LENGTH = 5200; // ms the beat takes

  var heroRoom = document.querySelector(".jn-room-hero");
  var pauseButton = document.querySelector(".jn-pause");
  var hero = heroRoom ? bindScene(heroRoom) : null;
  var paused = false;
  var beatDone = false;
  var elapsed = 0; // ms of beat played, not counting time spent paused
  var lastFrame = null;
  var frame = 0;

  function beat(now) {
    if (lastFrame !== null) elapsed += now - lastFrame;
    lastFrame = now;
    var u = clamp((elapsed - BEAT_DELAY) / BEAT_LENGTH, 0, 1);
    hero.render(HERO_REST * u);
    if (u < 1) {
      frame = window.requestAnimationFrame(beat);
    } else {
      beatDone = true;
      frame = 0;
    }
  }

  function startBeat() {
    if (!hero || beatDone || frame || paused) return;
    lastFrame = null;
    frame = window.requestAnimationFrame(beat);
  }

  function stopBeat() {
    if (frame) window.cancelAnimationFrame(frame);
    frame = 0;
  }

  // Pausing holds the beat where it is, and (through .jn-paused) every CSS
  // animation in the journey: breathing, blinking, the title, the caret.
  function setPaused(on) {
    paused = on;
    document.documentElement.classList.toggle("jn-paused", on);
    if (pauseButton) {
      pauseButton.setAttribute("data-paused", String(on));
      pauseButton.querySelector(".jn-pause-label").textContent = on ? "Play animation" : "Pause animation";
    }
    if (on) stopBeat();
    else startBeat();
  }

  if (pauseButton) {
    pauseButton.addEventListener("click", function () {
      setPaused(!paused);
    });
  }

  // ---------- 3. The pinned walkthrough ----------

  var story = document.getElementById("journey-story");
  var storyRoom = story ? story.querySelector(".jn-room") : null;
  var scene = storyRoom ? bindScene(storyRoom) : null;
  var stepCount = story ? story.querySelector(".jn-stepcount") : null;
  var stepButtons = story ? [].slice.call(story.querySelectorAll(".jn-step")) : [];
  var CUTS = [0.14, 0.31, 0.69, 0.85]; // where step 2, 3, 4 and 5 begin
  var lastStep = -1;

  function renderStory(p) {
    var step = 1 + CUTS.filter(function (c) {
      return p >= c;
    }).length;
    if (step !== lastStep) {
      lastStep = step;
      stepButtons.forEach(function (b, i) {
        if (i + 1 === step) b.setAttribute("aria-current", "step");
        else b.removeAttribute("aria-current");
      });
      if (stepCount) stepCount.textContent = "Step " + step + " of 5";
    }
    scene.render(p);
  }

  // How far the visitor has scrolled through the tall story, 0 to 1.
  function progress() {
    if (!motionAllowed()) return 1;
    var total = story.offsetHeight - window.innerHeight;
    return total > 0 ? clamp(-story.getBoundingClientRect().top / total, 0, 1) : 1;
  }

  // At most one redraw per frame, however often the scroll event fires.
  var ticking = false;
  function onScroll() {
    if (ticking || !scene) return;
    ticking = true;
    window.requestAnimationFrame(function () {
      ticking = false;
      renderStory(progress());
    });
  }

  // A step button scrolls to its part of the story, so the keyboard can drive it.
  stepButtons.forEach(function (b) {
    b.addEventListener("click", function () {
      if (!motionAllowed()) return;
      var total = story.offsetHeight - window.innerHeight;
      var top = story.getBoundingClientRect().top + window.scrollY;
      var at = parseFloat(b.getAttribute("data-target")) || 0;
      window.scrollTo({ top: top + total * at, behavior: "smooth" });
    });
  });

  if (scene) {
    window.addEventListener("scroll", onScroll, { passive: true });
    window.addEventListener("resize", onScroll);
  }

  // ---------- 4. The recording ----------

  var video = document.querySelector(".jn-video video");
  var videoButton = document.querySelector(".jn-vbtn");
  var userPaused = false;

  function paintVideoButton() {
    var playing = !video.paused;
    videoButton.setAttribute("aria-pressed", String(playing));
    videoButton.querySelector(".jn-vlabel").textContent = playing ? "Pause" : "Play";
  }

  // play() returns a promise that rejects when the browser refuses (or the
  // file can't be decoded); the poster then stays, which is fine.
  function playVideo() {
    var attempt = video.play();
    if (attempt && attempt.catch) attempt.catch(function () {});
  }

  if (video && videoButton) {
    videoButton.hidden = false;
    video.addEventListener("play", paintVideoButton);
    video.addEventListener("pause", paintVideoButton);
    videoButton.addEventListener("click", function () {
      if (video.paused) {
        userPaused = false;
        playVideo();
      } else {
        userPaused = true;
        video.pause();
      }
    });
    // Plays while at least half of it is on screen; the button always wins.
    if ("IntersectionObserver" in window) {
      new IntersectionObserver(
        function (entries) {
          if (!motionAllowed() || userPaused) return;
          if (entries[0].isIntersecting) playVideo();
          else video.pause();
        },
        { threshold: 0.5 }
      ).observe(video);
    }
    paintVideoButton();
  }

  // ---------- Motion on or off ----------

  // Runs at load and whenever the visitor's reduced-motion setting changes.
  function applyMotionPreference() {
    var on = motionAllowed();
    if (hero) {
      if (pauseButton) pauseButton.hidden = !on;
      if (on) {
        startBeat();
      } else {
        // Nothing moves, so no pause is needed: rest on the chosen amount.
        stopBeat();
        beatDone = true;
        setPaused(false);
        hero.render(HERO_REST);
      }
    }
    if (scene) {
      story.classList.toggle("is-live", on);
      story.classList.toggle("is-static", !on);
      // Enabled only when they can do something: scroll to their step.
      stepButtons.forEach(function (b) {
        b.disabled = !on;
      });
      lastStep = -1;
      scene.layout();
      renderStory(progress());
    }
    if (video && !on) video.pause();
  }

  if (hero) hero.render(motionAllowed() ? 0 : HERO_REST);
  if (reduce.addEventListener) reduce.addEventListener("change", applyMotionPreference);
  applyMotionPreference();
})();
