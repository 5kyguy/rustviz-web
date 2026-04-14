(() => {
  const root = document.getElementById("rv-playground");
  if (!root) {
    return;
  }

  const input = document.getElementById("rv-source-input");
  const errorEl = document.getElementById("rv-error");
  const outCode = document.getElementById("rv-out-code");
  const outTimeline = document.getElementById("rv-out-timeline");
  const copyBtn = document.getElementById("rv-copy-btn");
  const resetBtn = document.getElementById("rv-reset-btn");
  if (!input || !errorEl || !outCode || !outTimeline) {
    return;
  }

  const defaultSource = `fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    let s2 = s;
}`;

  input.value = input.value.trim() ? input.value : defaultSource;

  // Tab key support - inserts 2 spaces instead of changing focus
  input.addEventListener("keydown", (e) => {
    if (e.key === "Tab") {
      e.preventDefault();
      const start = input.selectionStart;
      const end = input.selectionEnd;
      const spaces = "  ";
      input.value = input.value.substring(0, start) + spaces + input.value.substring(end);
      input.selectionStart = input.selectionEnd = start + spaces.length;
      // Trigger input event for debounced render
      input.dispatchEvent(new Event("input"));
    }
  });

  // Copy button functionality
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
        showError("Failed to copy to clipboard");
      }
    });
  }

  // Reset button functionality
  if (resetBtn) {
    resetBtn.addEventListener("click", () => {
      input.value = defaultSource;
      render();
    });
  }

  let wasmApi = null;
  let renderToken = 0;

  function showError(message) {
    errorEl.style.display = "block";
    errorEl.textContent = message;
  }

  function clearError() {
    errorEl.style.display = "none";
    errorEl.textContent = "";
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
    return wasmApi;
  }

  // Apply current theme colors to SVG elements since embedded SVG styles
  // don't automatically inherit CSS custom properties from parent
  function applyThemeToSvg(svgElement) {
    if (!svgElement) return;

    const html = document.documentElement;
    const computedStyle = getComputedStyle(html);
    const bg = computedStyle.getPropertyValue("--sidebar-bg").trim() || "#f1f1f1";
    const fg = computedStyle.getPropertyValue("--sidebar-fg").trim() || "#6e6b5e";

    // Set CSS variables on the SVG element itself so embedded styles can use them
    svgElement.style.setProperty("--sidebar-bg", bg);
    svgElement.style.setProperty("--sidebar-fg", fg);

    // Apply background color directly to SVG
    svgElement.style.backgroundColor = bg;

    // Only update plain text elements without classes - elements with classes
    // (like .code, .label, .functionIcon) should get their colors from CSS
    const textElements = svgElement.querySelectorAll("text:not([class]):not([data-hash])");
    textElements.forEach((text) => {
      text.style.fill = fg;
    });
  }

  async function render() {
    const token = ++renderToken;
    const source = input.value;

    try {
      const api = await ensureWasm();
      const result = api.render_rustviz_source_wasm(source);
      if (token !== renderToken) {
        return;
      }
      outCode.innerHTML = result.vis_code;
      outTimeline.innerHTML = result.vis_timeline;

      // Apply theme to the newly rendered SVGs
      const codeSvg = outCode.querySelector("svg");
      const timelineSvg = outTimeline.querySelector("svg");
      applyThemeToSvg(codeSvg);
      applyThemeToSvg(timelineSvg);

      clearError();
    } catch (err) {
      if (token !== renderToken) {
        return;
      }
      const message = err && err.message ? err.message : String(err);
      showError(message);
    }
  }

  function debounce(fn, ms) {
    let handle = null;
    return () => {
      if (handle) {
        clearTimeout(handle);
      }
      handle = setTimeout(() => {
        handle = null;
        fn();
      }, ms);
    };
  }

  // Listen for theme changes and re-apply theme to SVGs
  function handleThemeChange() {
    const codeSvg = outCode.querySelector("svg");
    const timelineSvg = outTimeline.querySelector("svg");
    applyThemeToSvg(codeSvg);
    applyThemeToSvg(timelineSvg);
  }

  // Observe theme changes by watching for class changes on html element
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === "class") {
        handleThemeChange();
      }
    });
  });
  observer.observe(document.documentElement, { attributes: true });

  const debouncedRender = debounce(render, 250);
  input.addEventListener("input", debouncedRender);
  render();
})();
