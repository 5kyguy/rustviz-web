## Theory

`String` owns heap data, so assigning or passing it by value moves ownership unless you explicitly borrow or clone.

Understanding this move behavior explains why some bindings become invalid after transfer and why Rust prevents later use.

> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

---

## Code Example

```rust
{{#rustdoc_include assets/string_from_move_print/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="string_from_move_print code_panel" data="assets/string_from_move_print/vis_code.svg"></object>
<object type="image/svg+xml" class="string_from_move_print tl_panel" data="assets/string_from_move_print/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('string_from_move_print')"></object>
</div>
