# String Ownership and Move

## Overview

`String` owns heap data, so assigning or passing it by value moves ownership unless you explicitly borrow or clone.

Understanding this move behavior explains why some bindings become invalid after transfer and why Rust prevents later use.

> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="string_from_move_print rv-viz-combined" data="assets/string_from_move_print/vis_combined.svg" onmouseenter="helpers('string_from_move_print')"></object>
</div>
