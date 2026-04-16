# Multiple Immutable Borrows

## Overview

Rust allows multiple immutable references to the same value at the same time, because read-only access does not create conflicting mutations.

This enables safe shared access while still enforcing strict rules around mutation.

> **Official Rust Book**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="multiple_immutable_borrow rv-viz-combined" data="assets/multiple_immutable_borrow/vis_combined.svg" onmouseenter="helpers('multiple_immutable_borrow')"></object>
</div>
