## Theory

Methods commonly take `&self`, which means calling the method immutably borrows the receiver instead of moving it.

This allows read-only behavior on a value while keeping ownership with the caller.

> **Official Rust Book**: [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)

---

## Code Example

```rust
{{#rustdoc_include assets/immutable_borrow_method_call/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="immutable_borrow_method_call code_panel" data="assets/immutable_borrow_method_call/vis_code.svg"></object>
<object type="image/svg+xml" class="immutable_borrow_method_call tl_panel" data="assets/immutable_borrow_method_call/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('immutable_borrow_method_call')"></object>
</div>
