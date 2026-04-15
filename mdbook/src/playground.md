# Playground

Paste or type plain Rust in one editor. RustViz infers ownership, borrowing, and shadowing events and renders SVGs live in your browser via WebAssembly (no server).

Inference is best-effort. Unsupported constructs will show an inline error instead of crashing the page.

<div id="rv-playground" class="rv-playground">
  <div class="rv-playground-col rv-playground-col-full">
    <div class="rv-playground-header">
      <label for="rv-source-input"><strong>Rust Source</strong></label>
      <div class="rv-playground-actions">
        <button id="rv-copy-btn" class="rv-playground-btn" title="Copy to clipboard" aria-label="Copy to clipboard">
          <i class="fa fa-copy"></i> Copy
        </button>
        <button id="rv-reset-btn" class="rv-playground-btn" title="Reset to default" aria-label="Reset to default">
          <i class="fa fa-undo"></i> Reset
        </button>
      </div>
    </div>
    <textarea id="rv-source-input" class="rv-playground-ta" spellcheck="false"></textarea>
  </div>
  <p id="rv-error" class="rv-playground-error" style="display:none;"></p>
  <div class="rv-playground-output">
    <div class="rv-playground-out">
      <strong>Code Visualization</strong>
      <div id="rv-out-code" class="rv-playground-svg rv-playground-svg-code"></div>
    </div>
    <div class="rv-playground-out">
      <strong>Timeline Visualization</strong>
      <div id="rv-out-timeline" class="rv-playground-svg rv-playground-svg-timeline"></div>
    </div>
  </div>
</div>

<!-- Note: This file is a duplicate of src/playground.md for reference. The actual source file used by mdbook is in the src/ directory. -->
