# Playground

Paste or type plain Rust in one editor. RustViz infers ownership, borrowing, and shadowing events and renders SVGs live in your browser via WebAssembly (no server).

Inference is best-effort. Unsupported constructs will show an inline error instead of crashing the page.

<div id="rv-playground" class="rv-playground">
  <div class="rv-playground-col rv-playground-col-full">
    <label for="rv-source-input"><strong>Rust Source</strong></label>
    <textarea id="rv-source-input" class="rv-playground-ta" spellcheck="false"></textarea>
  </div>
  <p id="rv-error" class="rv-playground-error" style="display:none;"></p>
  <div class="rv-playground-output">
    <div class="rv-playground-out">
      <strong>vis_code.svg</strong>
      <div id="rv-out-code" class="rv-playground-svg"></div>
    </div>
    <div class="rv-playground-out">
      <strong>vis_timeline.svg</strong>
      <div id="rv-out-timeline" class="rv-playground-svg"></div>
    </div>
  </div>
</div>
