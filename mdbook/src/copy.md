# Stack-Only Data: Copy

## Overview

Some stack-only types (such as integers) implement the `Copy` trait. Assigning them duplicates the value, so both variables remain valid after assignment.

Because no heap allocation ownership is transferred, this operation is cheap and does not trigger move semantics.

> **Official Rust Book**: [Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="copy rv-viz-combined" data="assets/copy/vis_combined.svg" onmouseenter="helpers('copy')"></object>
</div>
