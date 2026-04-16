# Move Semantics: Function Return

## Overview

Returning a value transfers ownership to the caller. Function boundaries follow the same move rules as assignment.

This means ownership can flow out of one scope and into another through return values, controlling exactly where data remains valid.

> **Official Rust Book**: [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="move_func_return rv-viz-combined" data="assets/move_func_return/vis_combined.svg" onmouseenter="helpers('move_func_return')"></object>
</div>
