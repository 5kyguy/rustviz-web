# RustViz Web - Learning Rust Ownership & Borrowing

Welcome! This interactive tool visualizes Rust's ownership, borrowing, and copying concepts as animated SVG timelines.

## Topics Covered

1. **Variables & Mutability** – Understand `let` (immutable) vs `let mut` (mutable)
2. **Copy** – Learn which types copy instead of move (scalar types like integers)
3. **Ownership** – See how values are moved between variables and functions
4. **Borrowing** – Explore immutable references (`&T`) and mutable references (`&mut T`)

## How to Use These Examples

1. **Select a topic** from the left sidebar (organized from basics to advanced)
2. **Read the Rust source code** at the top of each page
3. **Hover over the timeline** to understand:
   - **Dots** – Events (variable declarations, moves, borrows)
   - **Vertical lines** – Variable lifetimes
   - **Arrows** – Ownership transfers and references

## Try the Playground

Use the **Playground** (last item in sidebar) to write your own Rust code and see the visualization instantly. Experiment with different patterns to build intuition!

## Learning Path

Start from the top of the sidebar and work your way down:
- Begin with `immutable_variable` and `mutable_variables`
- Then explore `copy` to understand what makes types copyable
- Move on to ownership examples (`move_*`, `func_take_*`)
- Finally, study borrowing with `immutable_borrow` and `mutable_borrow`

---

*Project forked from [rustviz/rustviz](https://github.com/rustviz/rustviz)*
