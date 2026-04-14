(() => {
  const root = document.getElementById("rv-playground");
  if (!root) {
    return;
  }

  const input = document.getElementById("rv-source-input");
  const errorEl = document.getElementById("rv-error");
  const outCode = document.getElementById("rv-out-code");
  const outTimeline = document.getElementById("rv-out-timeline");
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

  const debouncedRender = debounce(render, 250);
  input.addEventListener("input", debouncedRender);
  render();
})();
