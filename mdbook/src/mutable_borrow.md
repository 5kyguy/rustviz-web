## Theory

A mutable reference (`&mut T`) allows mutation through a borrow, but Rust enforces exclusivity: at a given time, either one mutable reference or any number of immutable references.

These rules prevent data races and aliasing bugs at compile time.

> **Official Rust Book**: [Mutable References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)

---

## Code Example

```rust
{{#rustdoc_include assets/mutable_borrow/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="mutable_borrow code_panel" data="assets/mutable_borrow/vis_code.svg"></object>
<object type="image/svg+xml" class="mutable_borrow tl_panel" data="assets/mutable_borrow/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('mutable_borrow')"></object>
</div>
