# Method Calls: Mutable Borrow

## Overview

Methods that take `&mut self` borrow the receiver mutably, allowing the method to change internal state without taking ownership.

Rust's method-call syntax automatically handles the needed referencing while preserving borrowing rules.

> **Official Rust Book**: [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="mutable_borrow_method_call rv-viz-combined" data="assets/mutable_borrow_method_call/vis_combined.svg" onmouseenter="helpers('mutable_borrow_method_call')"></object>
</div>
