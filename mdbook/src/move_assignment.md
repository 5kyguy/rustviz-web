## Theory

For heap-owning values like `String`, assignment moves ownership instead of copying heap data. After `let s2 = s1;`, `s1` is no longer valid.

This prevents double-free bugs by ensuring only one owner is responsible for dropping the value.

> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

---

## Code Example

```rust
{{#rustdoc_include assets/move_assignment/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="move_assignment code_panel" data="assets/move_assignment/vis_code.svg"></object>
<object type="image/svg+xml" class="move_assignment tl_panel" data="assets/move_assignment/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('move_assignment')"></object>
</div>
