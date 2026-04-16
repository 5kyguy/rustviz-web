# Mutable Variables

## Overview

When you need to update a value, Rust requires an explicit mutable binding with `let mut`. This makes mutation intentional and visible to readers of the code.

Rust keeps immutability as the default, while `mut` is the opt-in mechanism for state changes.

> **Official Rust Book**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="mutable_variables rv-viz-combined" data="assets/mutable_variables/vis_combined.svg" onmouseenter="helpers('mutable_variables')"></object>
</div>
