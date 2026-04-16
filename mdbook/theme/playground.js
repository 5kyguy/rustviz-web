(() => {
  const root = document.getElementById("rv-playground");
  if (!root) {
    return;
  }

  const input = document.getElementById("rv-source-input");
  const errorEl = document.getElementById("rv-error");
  const outTimeline = document.getElementById("rv-out-timeline");
  const lineNumbersEl = document.getElementById("rv-line-numbers");
  const copyBtn = document.getElementById("rv-copy-btn");
  const resetBtn = document.getElementById("rv-reset-btn");
  const compileBtn = document.getElementById("rv-compile-btn");
  const diagnosticsEl = document.getElementById("rv-diagnostics");
  const diagnosticsBody = document.getElementById("rv-diagnostics-body");
  const resizer = document.getElementById("rv-pane-resizer");
  const shell = root.querySelector(".rv-playground-shell");

  if (!input || !errorEl || !outTimeline || !lineNumbersEl) {
    return;
  }

  const defaultSource = `fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    let s2 = s;
}`;

  input.value = input.value.trim() ? input.value : defaultSource;

  const LIVE_VIZ_WAIT_MS = 450;
  const LIVE_VIZ_MAX_WAIT_MS = 1200;

  function compileEndpoint() {
    const fromAttr = root.getAttribute("data-compile-endpoint");
    if (fromAttr && fromAttr.trim()) {
      return fromAttr.trim();
    }
    return "https://play.rust-lang.org/execute";
  }

  function updateLineNumbers() {
    const n = input.value.split("\n").length || 1;
    const lines = [];
    for (let i = 1; i <= n; i += 1) {
      lines.push(String(i));
    }
    lineNumbersEl.textContent = lines.join("\n");
  }

  function syncLineNumberScroll() {
    lineNumbersEl.scrollTop = input.scrollTop;
  }

  input.addEventListener("scroll", syncLineNumberScroll);
  updateLineNumbers();

  input.addEventListener("keydown", (e) => {
    if (e.key === "Tab") {
      e.preventDefault();
      const start = input.selectionStart;
      const end = input.selectionEnd;
      const spaces = "  ";
      input.value = input.value.substring(0, start) + spaces + input.value.substring(end);
      input.selectionStart = input.selectionEnd = start + spaces.length;
      input.dispatchEvent(new Event("input"));
    }
  });

  if (copyBtn) {
    copyBtn.addEventListener("click", async () => {
      try {
        await navigator.clipboard.writeText(input.value);
        const originalText = copyBtn.innerHTML;
        copyBtn.innerHTML = '<i class="fa fa-check"></i> Copied!';
        setTimeout(() => {
          copyBtn.innerHTML = originalText;
        }, 1500);
      } catch (err) {
        showInferenceError("Failed to copy to clipboard");
      }
    });
  }

  if (resetBtn) {
    resetBtn.addEventListener("click", () => {
      input.value = defaultSource;
      updateLineNumbers();
      clearDiagnostics();
      visualize();
    });
  }

  let wasmApi = null;
  let renderToken = 0;
  let compileInFlight = false;
  /** @type {HTMLElement | null} */
  let timelineScrollEl = null;
  let scrollSyncLock = false;
  /** @type {AbortController | null} */
  let scrollSyncAbort = null;

  function showInferenceError(message) {
    errorEl.style.display = "block";
    errorEl.textContent = message;
  }

  function clearInferenceError() {
    errorEl.style.display = "none";
    errorEl.textContent = "";
  }

  function showDiagnostics(text) {
    if (!diagnosticsEl || !diagnosticsBody) return;
    const t = (text || "").trim();
    if (!t) {
      diagnosticsEl.hidden = true;
      diagnosticsBody.textContent = "";
      return;
    }
    diagnosticsBody.textContent = text;
    diagnosticsEl.hidden = false;
  }

  function clearDiagnostics() {
    showDiagnostics("");
  }

  async function ensureWasm() {
    if (wasmApi) {
      return wasmApi;
    }
    const wasmUrl = new URL("./assets/pkg/wasm.js", window.location.href).toString();
    const mod = await import(wasmUrl);
    if (typeof mod.default === "function") {
      await mod.default();
    }
    wasmApi = mod;
    if (typeof mod.line_step_px === "function") {
      const px = mod.line_step_px();
      root.style.setProperty("--rv-line-step", `${px}px`);
    }
    return wasmApi;
  }

  function applyThemeToSvg(svgElement) {
    if (!svgElement) return;

    const html = document.documentElement;
    const computedStyle = getComputedStyle(html);
    const bg = computedStyle.getPropertyValue("--sidebar-bg").trim() || "#f1f1f1";
    const fg = computedStyle.getPropertyValue("--sidebar-fg").trim() || "#6e6b5e";

    svgElement.style.setProperty("--sidebar-bg", bg);
    svgElement.style.setProperty("--sidebar-fg", fg);
    svgElement.style.backgroundColor = bg;

    const textElements = svgElement.querySelectorAll("text:not([class]):not([data-hash])");
    textElements.forEach((text) => {
      text.style.fill = fg;
    });
  }

  function layoutTimelineScrollHeight() {
    if (!timelineScrollEl) return;
    timelineScrollEl.style.height = `${input.clientHeight}px`;
  }

  function bindScrollSync(scrollEl, textEl) {
    if (scrollSyncAbort) {
      scrollSyncAbort.abort();
    }
    scrollSyncAbort = new AbortController();
    const { signal } = scrollSyncAbort;
    function fromText() {
      if (scrollSyncLock) return;
      scrollSyncLock = true;
      scrollEl.scrollTop = textEl.scrollTop;
      scrollSyncLock = false;
    }
    function fromScroll() {
      if (scrollSyncLock) return;
      scrollSyncLock = true;
      textEl.scrollTop = scrollEl.scrollTop;
      scrollSyncLock = false;
    }
    textEl.addEventListener("scroll", fromText, { signal });
    scrollEl.addEventListener("scroll", fromScroll, { signal });
  }

  function ensurePlaygroundTooltipEl() {
    let el = document.getElementById("rv-playground-tooltip");
    if (!el) {
      el = document.createElement("div");
      el.id = "rv-playground-tooltip";
      el.className = "rv-playground-tooltip";
      el.setAttribute("role", "tooltip");
      el.hidden = true;
      document.body.appendChild(el);
    }
    return el;
  }

  /** Word-wrap long tooltips (same idea as mdbook_plugin/helpers.js `breakText`). */
  function breakPlaygroundTooltipText(text, tooltipEl) {
    const splitText = text.split(" ");
    const words = [];
    let last = 0;
    let span = false;
    for (const elt of splitText) {
      if (elt.startsWith("<")) {
        span = true;
        words.push(elt);
        last = words.length - 1;
      } else if (elt.startsWith("!important")) {
        span = false;
        words[last] += elt;
      } else if (span) {
        words[last] = `${words[last]} ${elt}`;
      } else {
        words.push(elt);
      }
    }
    tooltipEl.innerHTML = "";
    for (const word of words) {
      tooltipEl.innerHTML += `${word} `;
      const left = tooltipEl.getBoundingClientRect().left;
      if (left + tooltipEl.clientWidth > document.documentElement.clientWidth - 20) {
        const idx = tooltipEl.innerHTML.lastIndexOf(" ", tooltipEl.innerHTML.length - 2);
        const temp = tooltipEl.innerHTML.substring(0, idx);
        const other = tooltipEl.innerHTML.substring(idx + 1);
        tooltipEl.innerHTML = `${temp}<br />${other}`;
      }
    }
  }

  function positionPlaygroundTooltip(tooltipEl, clientX, clientY) {
    const margin = 10;
    const offset = 12;
    tooltipEl.hidden = false;
    tooltipEl.style.left = `${clientX + offset}px`;
    tooltipEl.style.top = `${clientY + offset}px`;
    const r = tooltipEl.getBoundingClientRect();
    let x = clientX + offset;
    let y = clientY + offset;
    if (r.right > window.innerWidth - margin) {
      x = window.innerWidth - r.width - margin;
    }
    if (r.bottom > window.innerHeight - margin) {
      y = window.innerHeight - r.height - margin;
    }
    if (x < margin) x = margin;
    if (y < margin) y = margin;
    tooltipEl.style.left = `${Math.round(x)}px`;
    tooltipEl.style.top = `${Math.round(y)}px`;
  }

  /**
   * Hover text for `.tooltip-trigger` elements (same markup as chapter SVGs; book.js uses
   * helpers.js on `<object>` — playground injects inline SVG so we bind here).
   */
  function bindPlaygroundTimelineTooltips(containerEl) {
    const tooltipEl = ensurePlaygroundTooltipEl();
    const svg = containerEl.querySelector("svg");
    if (!svg) return;

    const triggers = svg.getElementsByClassName("tooltip-trigger");

    function showTooltip(e) {
      const raw = e.currentTarget.getAttribute("data-tooltip-text");
      const text = raw != null ? String(raw) : "";
      tooltipEl.innerHTML = text || "";
      positionPlaygroundTooltip(tooltipEl, e.clientX, e.clientY);
      if (tooltipEl.getBoundingClientRect().right >= document.documentElement.clientWidth) {
        breakPlaygroundTooltipText(text, tooltipEl);
        positionPlaygroundTooltip(tooltipEl, e.clientX, e.clientY);
      }
    }

    function hideTooltip() {
      tooltipEl.hidden = true;
      tooltipEl.innerHTML = "";
    }

    for (let i = 0; i < triggers.length; i += 1) {
      const t = triggers[i];
      if (t.getAttribute("data-rv-tip-bound") === "1") continue;
      t.setAttribute("data-rv-tip-bound", "1");
      t.addEventListener("mousemove", showTooltip);
      t.addEventListener("mouseleave", hideTooltip);
    }
  }

  function debounceWithMaxWait(fn, waitMs, maxWaitMs) {
    let waitId = null;
    let maxId = null;
    let started = 0;
    return function schedule() {
      const now = Date.now();
      if (!started) started = now;
      if (waitId) clearTimeout(waitId);
      if (now - started >= maxWaitMs) {
        started = 0;
        if (maxId) {
          clearTimeout(maxId);
          maxId = null;
        }
        fn();
        return;
      }
      waitId = setTimeout(() => {
        started = 0;
        if (maxId) {
          clearTimeout(maxId);
          maxId = null;
        }
        waitId = null;
        fn();
      }, waitMs);
      if (!maxId) {
        maxId = setTimeout(() => {
          started = 0;
          if (waitId) clearTimeout(waitId);
          waitId = null;
          maxId = null;
          fn();
        }, maxWaitMs);
      }
    };
  }

  async function visualize() {
    const token = ++renderToken;
    const source = input.value;

    try {
      const api = await ensureWasm();
      if (typeof api.render_rustviz_source_wasm !== "function") {
        throw new Error(
          "WASM bundle is outdated: rebuild with ./wasm/build.sh (missing render_rustviz_source_wasm)"
        );
      }
      const result = api.render_rustviz_source_wasm(source);
      if (token !== renderToken) {
        return;
      }

      outTimeline.innerHTML = "";
      const wrap = document.createElement("div");
      wrap.id = "rv-timeline-scroll";
      wrap.className = "rv-timeline-scroll-inner";
      wrap.innerHTML = result.vis_timeline;
      outTimeline.appendChild(wrap);
      timelineScrollEl = wrap;

      const svg = wrap.querySelector("svg");
      applyThemeToSvg(svg);

      layoutTimelineScrollHeight();
      bindScrollSync(wrap, input);
      bindPlaygroundTimelineTooltips(wrap);
      requestAnimationFrame(() => {
        layoutTimelineScrollHeight();
      });

      clearInferenceError();
    } catch (err) {
      if (token !== renderToken) {
        return;
      }
      const message = err && err.message ? err.message : String(err);
      showInferenceError(message);
    }
  }

  const scheduleLiveVisualize = debounceWithMaxWait(visualize, LIVE_VIZ_WAIT_MS, LIVE_VIZ_MAX_WAIT_MS);

  input.addEventListener("input", () => {
    updateLineNumbers();
    syncLineNumberScroll();
    scheduleLiveVisualize();
  });

  if (compileBtn) {
    compileBtn.addEventListener("click", async () => {
      if (compileInFlight) return;
      compileInFlight = true;
      compileBtn.disabled = true;
      clearDiagnostics();
      const code = input.value;
      try {
        const url = compileEndpoint();
        const payload = {
          channel: "stable",
          mode: "debug",
          edition: "2021",
          crateType: "bin",
          tests: false,
          code,
          backtrace: false,
        };
        const res = await fetch(url, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(payload),
        });
        const raw = await res.text();
        let data;
        try {
          data = JSON.parse(raw);
        } catch {
          showDiagnostics(`HTTP ${res.status}\n${raw}`);
          compileInFlight = false;
          compileBtn.disabled = false;
          return;
        }
        const stderr = data.stderr != null ? String(data.stderr) : "";
        const stdout = data.stdout != null ? String(data.stdout) : "";
        if (data.success) {
          showDiagnostics(stdout ? stdout : "Build succeeded (no output).");
        } else {
          const out = [stderr, stdout].filter(Boolean).join("\n");
          showDiagnostics(out || JSON.stringify(data, null, 2));
        }
      } catch (e) {
        const msg = e && e.message ? e.message : String(e);
        showDiagnostics(`Compile request failed: ${msg}`);
      }
      compileInFlight = false;
      compileBtn.disabled = false;
    });
  }

  function handleThemeChange() {
    const wrap = outTimeline.querySelector("#rv-timeline-scroll");
    const svg = wrap && wrap.querySelector("svg");
    applyThemeToSvg(svg);
  }

  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === "class") {
        handleThemeChange();
      }
    });
  });
  observer.observe(document.documentElement, { attributes: true });

  if (shell && typeof ResizeObserver !== "undefined") {
    const ro = new ResizeObserver(() => {
      layoutTimelineScrollHeight();
    });
    ro.observe(shell);
  }

  if (resizer) {
    function onPointerMove(ev) {
      if (!shell) return;
      const r = shell.getBoundingClientRect();
      const w = r.width - 6;
      if (w <= 0) return;
      let pct = ((ev.clientX - r.left) / w) * 100;
      pct = Math.min(78, Math.max(22, pct));
      shell.style.setProperty("--rv-editor-pct", `${pct}%`);
      requestAnimationFrame(() => {
        layoutTimelineScrollHeight();
      });
    }

    function onPointerUp() {
      document.removeEventListener("pointermove", onPointerMove);
      document.removeEventListener("pointerup", onPointerUp);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    }

    resizer.addEventListener("pointerdown", (ev) => {
      if (!shell) return;
      ev.preventDefault();
      document.body.style.cursor = "col-resize";
      document.body.style.userSelect = "none";
      document.addEventListener("pointermove", onPointerMove);
      document.addEventListener("pointerup", onPointerUp);
    });
  }

  window.addEventListener("resize", () => {
    layoutTimelineScrollHeight();
  });

  visualize();
})();
