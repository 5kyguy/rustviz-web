## Theory

Passing a value into a function transfers ownership when the parameter takes the value by type (for example, `String` rather than `&String`).

After that call, the original binding is no longer valid unless ownership is returned in some form.

> **Official Rust Book**: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)

---

## Code Example

```rust
{{#rustdoc_include assets/func_take_ownership/source.rs}}
```

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">
<object type="image/svg+xml" class="func_take_ownership code_panel" data="assets/func_take_ownership/vis_code.svg"></object>
<object type="image/svg+xml" class="func_take_ownership tl_panel" data="assets/func_take_ownership/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('func_take_ownership')"></object>
</div>
