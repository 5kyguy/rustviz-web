## Theory

Rust variables are immutable by default. Once a value is bound to a name, reassigning it is not allowed unless the binding is marked mutable with `mut`.

This default helps prevent accidental state changes and makes code easier to reason about, because the compiler enforces when values can and cannot change.

> **Official Rust Book**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## Code Example

```rust
{{#rustdoc_include assets/immutable_variable/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="immutable_variable code_panel" data="assets/immutable_variable/vis_code.svg"></object>
<object type="image/svg+xml" class="immutable_variable tl_panel" data="assets/immutable_variable/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('immutable_variable')"></object>
</div>
