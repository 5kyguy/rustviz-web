# Move Semantics: Different Scope

## Overview

Ownership is always tied to scope. When a value is moved, the previous binding becomes invalid, and cleanup happens when the current owner goes out of scope.

This rule is especially important across nested scopes, where validity and drops depend on who owns the value at each point.

> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="move_different_scope rv-viz-combined" data="assets/move_different_scope/vis_combined.svg" onmouseenter="helpers('move_different_scope')"></object>
</div>
