# Immutable Variables

## Overview

Rust variables are immutable by default. Once a value is bound to a name, reassigning it is not allowed unless the binding is marked mutable with `mut`.

This default helps prevent accidental state changes and makes code easier to reason about, because the compiler enforces when values can and cannot change.

> **Official Rust Book**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="immutable_variable rv-viz-combined" data="assets/immutable_variable/vis_combined.svg" onmouseenter="helpers('immutable_variable')"></object>
</div>
