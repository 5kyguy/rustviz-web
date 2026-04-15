## Theory

A reference (`&T`) lets you borrow a value without taking ownership. Borrowing allows functions to read data while the original owner keeps control.

Because ownership does not move, the borrowed value remains usable after the call.

> **Official Rust Book**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

## Code Example

```rust
{{#rustdoc_include assets/immutable_borrow/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="immutable_borrow code_panel" data="assets/immutable_borrow/vis_code.svg"></object>
<object type="image/svg+xml" class="immutable_borrow tl_panel" data="assets/immutable_borrow/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('immutable_borrow')"></object>
</div>
