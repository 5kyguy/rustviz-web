## Theory

Some stack-only types (such as integers) implement the `Copy` trait. Assigning them duplicates the value, so both variables remain valid after assignment.

Because no heap allocation ownership is transferred, this operation is cheap and does not trigger move semantics.

> **Official Rust Book**: [Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)

---

## Code Example

```rust
{{#rustdoc_include assets/copy/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="copy code_panel" data="assets/copy/vis_code.svg"></object>
<object type="image/svg+xml" class="copy tl_panel" data="assets/copy/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('copy')"></object>
</div>
