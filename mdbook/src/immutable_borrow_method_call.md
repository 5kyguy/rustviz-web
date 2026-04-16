# Method Calls: Immutable Borrow

## Overview

Methods commonly take `&self`, which means calling the method immutably borrows the receiver instead of moving it.

This allows read-only behavior on a value while keeping ownership with the caller.

> **Official Rust Book**: [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="immutable_borrow_method_call rv-viz-combined" data="assets/immutable_borrow_method_call/vis_combined.svg" onmouseenter="helpers('immutable_borrow_method_call')"></object>
</div>
