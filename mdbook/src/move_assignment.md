# Move Semantics: Assignment

## Overview

For heap-owning values like `String`, assignment moves ownership instead of copying heap data. After `let s2 = s1;`, `s1` is no longer valid.

This prevents double-free bugs by ensuring only one owner is responsible for dropping the value.

> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="move_assignment rv-viz-combined" data="assets/move_assignment/vis_combined.svg" onmouseenter="helpers('move_assignment')"></object>
</div>
