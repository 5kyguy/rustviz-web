## Theory

Returning a value transfers ownership to the caller. Function boundaries follow the same move rules as assignment.

This means ownership can flow out of one scope and into another through return values, controlling exactly where data remains valid.

> **Official Rust Book**: [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

---

## Code Example

```rust
{{#rustdoc_include assets/move_func_return/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="move_func_return code_panel" data="assets/move_func_return/vis_code.svg"></object>
<object type="image/svg+xml" class="move_func_return tl_panel" data="assets/move_func_return/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('move_func_return')"></object>
</div>
