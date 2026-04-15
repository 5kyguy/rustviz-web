## Theory

When you need to update a value, Rust requires an explicit mutable binding with `let mut`. This makes mutation intentional and visible to readers of the code.

Rust keeps immutability as the default, while `mut` is the opt-in mechanism for state changes.

> **Official Rust Book**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## Code Example

```rust
{{#rustdoc_include assets/mutable_variables/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="mutable_variables code_panel" data="assets/mutable_variables/vis_code.svg"></object>
<object type="image/svg+xml" class="mutable_variables tl_panel" data="assets/mutable_variables/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('mutable_variables')"></object>
</div>
