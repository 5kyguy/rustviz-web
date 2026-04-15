## Theory

Ownership is always tied to scope. When a value is moved, the previous binding becomes invalid, and cleanup happens when the current owner goes out of scope.

This rule is especially important across nested scopes, where validity and drops depend on who owns the value at each point.

> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

---

## Code Example

```rust
{{#rustdoc_include assets/move_different_scope/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="move_different_scope code_panel" data="assets/move_different_scope/vis_code.svg"></object>
<object type="image/svg+xml" class="move_different_scope tl_panel" data="assets/move_different_scope/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('move_different_scope')"></object>
</div>
