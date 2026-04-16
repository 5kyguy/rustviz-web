# Ownership: Take and Return

## Overview

Functions can take ownership of values and then return ownership back to the caller. This follows the same ownership transfer rules as assignment and returns.

While valid, this pattern is often verbose and motivates borrowing when you only need temporary access.

> **Official Rust Book**: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions) and [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

---

## Code example

<div class="flex-container vis_block rv-vis-block">
<object type="image/svg+xml" class="func_take_return_ownership rv-viz-combined" data="assets/func_take_return_ownership/vis_combined.svg" onmouseenter="helpers('func_take_return_ownership')"></object>
</div>
