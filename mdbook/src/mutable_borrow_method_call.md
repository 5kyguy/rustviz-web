## Theory

Methods that take `&mut self` borrow the receiver mutably, allowing the method to change internal state without taking ownership.

Rust's method-call syntax automatically handles the needed referencing while preserving borrowing rules.

> **Official Rust Book**: [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)

---

## Code Example

```rust
{{#rustdoc_include assets/mutable_borrow_method_call/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="mutable_borrow_method_call code_panel" data="assets/mutable_borrow_method_call/vis_code.svg"></object>
<object type="image/svg+xml" class="mutable_borrow_method_call tl_panel" data="assets/mutable_borrow_method_call/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('mutable_borrow_method_call')"></object>
</div>
