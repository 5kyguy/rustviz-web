# Mutable References

## Overview

A mutable reference (`&mut T`) allows mutation through a borrow, but Rust enforces exclusivity: at a given time, either one mutable reference or any number of immutable references.

These rules prevent data races and aliasing bugs at compile time.

> **Official Rust Book**: [Mutable References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="mutable_borrow rv-viz-combined" data="assets/mutable_borrow/vis_combined.svg" onmouseenter="helpers('mutable_borrow')"></object>
</div>
