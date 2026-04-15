## Theory

Functions can take ownership of values and then return ownership back to the caller. This follows the same ownership transfer rules as assignment and returns.

While valid, this pattern is often verbose and motivates borrowing when you only need temporary access.

> **Official Rust Book**: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions) and [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

---

## Code Example

```rust
{{#rustdoc_include assets/func_take_return_ownership/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="func_take_return_ownership code_panel" data="assets/func_take_return_ownership/vis_code.svg"></object>
<object type="image/svg+xml" class="func_take_return_ownership tl_panel" data="assets/func_take_return_ownership/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('func_take_return_ownership')"></object>
</div>
