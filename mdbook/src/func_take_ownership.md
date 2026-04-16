# Ownership: Function Parameters

## Overview

Passing a value into a function transfers ownership when the parameter takes the value by type (for example, `String` rather than `&String`).

After that call, the original binding is no longer valid unless ownership is returned in some form.

> **Official Rust Book**: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="func_take_ownership rv-viz-combined" data="assets/func_take_ownership/vis_combined.svg" onmouseenter="helpers('func_take_ownership')"></object>
</div>
