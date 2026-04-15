## Theory

Rust allows multiple immutable references to the same value at the same time, because read-only access does not create conflicting mutations.

This enables safe shared access while still enforcing strict rules around mutation.

> **Official Rust Book**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

## Code Example

```rust
{{#rustdoc_include assets/multiple_immutable_borrow/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="multiple_immutable_borrow code_panel" data="assets/multiple_immutable_borrow/vis_code.svg"></object>
<object type="image/svg+xml" class="multiple_immutable_borrow tl_panel" data="assets/multiple_immutable_borrow/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('multiple_immutable_borrow')"></object>
</div>
