# Immutable References

## Overview

A reference (`&T`) lets you borrow a value without taking ownership. Borrowing allows functions to read data while the original owner keeps control.

Because ownership does not move, the borrowed value remains usable after the call.

> **Official Rust Book**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="immutable_borrow rv-viz-combined" data="assets/immutable_borrow/vis_combined.svg" onmouseenter="helpers('immutable_borrow')"></object>
</div>
