# RustViz Web - Learning Rust Ownership & Borrowing

Welcome! This interactive tool visualizes Rust's ownership, borrowing, and copying concepts as animated SVG timelines.

## Topics Covered

1. **Variables & Mutability** – Understand `let` (immutable) vs `let mut` (mutable)
2. **Copy** – Learn which types copy instead of move (scalar types like integers)
3. **Ownership** – See how values are moved between variables and functions
4. **Borrowing** – Explore immutable references (`&T`) and mutable references (`&mut T`)

## How to Use These Examples

1. **Select a topic** from the left sidebar (organized from basics to advanced)
2. **Read the Rust source** in the diagram on each page (the same code appears inside the visualization)
3. **Hover over the timeline** to understand:
   - **Dots** – Events (variable declarations, moves, borrows)
   - **Vertical lines** – Variable lifetimes
   - **Arrows** – Ownership transfers and references

## Try the Playground

Use the **Playground** (last item in the sidebar) to write Rust beside a live **ownership & timeline** diagram. Experiment with different patterns to build intuition!

## Learning Path

Start from the top of the sidebar and work your way down:

- Begin with **Immutable Variables** and **Mutable Variables**
- Then explore **Stack-Only Data: Copy** to understand what makes types copyable
- Move on to **Ownership** examples (moves and functions)
- Finally, study **References and Borrowing**

---

*Project forked from [rustviz/rustviz](https://github.com/rustviz/rustviz)*
