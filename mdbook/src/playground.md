# Playground

Write Rust in the editor — the ownership and timeline on the right update shortly after you stop typing (or at most every second or so while you keep typing). **Compile** runs the code through the Rust compiler. Hover diagram dots, arrows, and labels for explanations. Line height matches the diagram so rows stay aligned when you scroll.

<div id="rv-playground" class="rv-playground rv-playground-full" data-compile-endpoint="">
  <div class="rv-playground-shell">
    <section class="rv-playground-pane rv-pane-editor" aria-label="Source editor">
      <div class="rv-playground-toolbar">
        <span class="rv-toolbar-title">Editor</span>
        <div class="rv-playground-actions">
          <button type="button" id="rv-copy-btn" class="rv-playground-btn" title="Copy to clipboard" aria-label="Copy to clipboard">
            <i class="fa fa-copy"></i> Copy
          </button>
          <button type="button" id="rv-reset-btn" class="rv-playground-btn" title="Reset to default" aria-label="Reset to default">
            <i class="fa fa-undo"></i> Reset
          </button>
          <button type="button" id="rv-compile-btn" class="rv-playground-btn rv-playground-btn-primary rv-playground-btn-wide" title="Compile with rustc" aria-label="Compile with rustc">
            <i class="fa fa-cog"></i> Compile
          </button>
        </div>
      </div>
      <div class="rv-editor-shell">
        <div class="rv-line-numbers" id="rv-line-numbers" aria-hidden="true">1</div>
        <textarea id="rv-source-input" class="rv-playground-ta" spellcheck="false" rows="1" wrap="off"></textarea>
      </div>
      <div id="rv-diagnostics" class="rv-diagnostics" hidden>
        <div class="rv-diagnostics-header">Compiler output</div>
        <pre id="rv-diagnostics-body" class="rv-diagnostics-body"></pre>
      </div>
      <p id="rv-error" class="rv-playground-error rv-inference-error" style="display:none;" role="alert"></p>
    </section>
    <div class="rv-pane-resizer" id="rv-pane-resizer" role="separator" aria-orientation="vertical" aria-label="Resize panes" tabindex="0"></div>
    <section class="rv-playground-pane rv-pane-visual" aria-label="Visualization">
      <div class="rv-visual-header">
        <span class="rv-toolbar-title">Ownership &amp; timeline</span>
      </div>
      <div id="rv-out-timeline" class="rv-playground-svg rv-playground-svg-timeline-only"></div>
    </section>
  </div>
</div>
