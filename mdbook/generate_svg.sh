#!/bin/bash
red=$'\e[1;31m'
end=$'\e[0m'

cd mdbook

# copy book.js to theme/
mkdir -p "./theme"
cp mdbook_plugin/book.js theme/book.js

if ! [[ -d "src" ]]; then
    mkdir src
fi

# clear assets and md files to mdbook directory
rm -f src/*md

if [[ -d "src/assets" ]]; then
    rm -r src/assets
fi

# SUMMARY.md is written after examples are built (see end of script)

# Home page content with usage guidance.
cat > src/index.md <<'EOF'
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
EOF

printf "Generating visualizations for the following examples: \n"

# Core examples for teaching ownership, borrowing, and copying in Rust
# Aligned with curriculum: Variables, Ownership, Borrowing, Copy trait
declare -a targetExamples=(
    "immutable_variable"
    "mutable_variables"
    "copy"
    "move_assignment"
    "move_different_scope"
    "move_func_return"
    "func_take_ownership"
    "func_take_return_ownership"
    "string_from_move_print"
    "immutable_borrow"
    "mutable_borrow"
    "multiple_immutable_borrow"
    "immutable_borrow_method_call"
    "mutable_borrow_method_call"
)

EX="../src/examples"

# Theory content for each example type
write_theory() {
    local target="$1"
    case "$target" in
        "immutable_variable")
            printf "# Immutable Variables\n\n## Overview\n\n"
            printf "Rust variables are immutable by default. Once a value is bound to a name, reassigning it is not allowed unless the binding is marked mutable with \`mut\`.\n\n"
            printf "This default helps prevent accidental state changes and makes code easier to reason about, because the compiler enforces when values can and cannot change.\n\n"
            printf "> **Official Rust Book**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)\n\n"
            ;;
        "mutable_variables")
            printf "# Mutable Variables\n\n## Overview\n\n"
            printf "When you need to update a value, Rust requires an explicit mutable binding with \`let mut\`. This makes mutation intentional and visible to readers of the code.\n\n"
            printf "Rust keeps immutability as the default, while \`mut\` is the opt-in mechanism for state changes.\n\n"
            printf "> **Official Rust Book**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)\n\n"
            ;;
        "copy")
            printf "# Stack-Only Data: Copy\n\n## Overview\n\n"
            printf "Some stack-only types (such as integers) implement the \`Copy\` trait. Assigning them duplicates the value, so both variables remain valid after assignment.\n\n"
            printf "Because no heap allocation ownership is transferred, this operation is cheap and does not trigger move semantics.\n\n"
            printf "> **Official Rust Book**: [Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)\n\n"
            ;;
        "move_assignment")
            printf "# Move Semantics: Assignment\n\n## Overview\n\n"
            printf "For heap-owning values like \`String\`, assignment moves ownership instead of copying heap data. After \`let s2 = s1;\`, \`s1\` is no longer valid.\n\n"
            printf "This prevents double-free bugs by ensuring only one owner is responsible for dropping the value.\n\n"
            printf "> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)\n\n"
            ;;
        "move_different_scope")
            printf "# Move Semantics: Different Scope\n\n## Overview\n\n"
            printf "Ownership is always tied to scope. When a value is moved, the previous binding becomes invalid, and cleanup happens when the current owner goes out of scope.\n\n"
            printf "This rule is especially important across nested scopes, where validity and drops depend on who owns the value at each point.\n\n"
            printf "> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)\n\n"
            ;;
        "move_func_return")
            printf "# Move Semantics: Function Return\n\n## Overview\n\n"
            printf "Returning a value transfers ownership to the caller. Function boundaries follow the same move rules as assignment.\n\n"
            printf "This means ownership can flow out of one scope and into another through return values, controlling exactly where data remains valid.\n\n"
            printf "> **Official Rust Book**: [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)\n\n"
            ;;
        "string_from_move_print")
            printf "# String Ownership and Move\n\n## Overview\n\n"
            printf "\`String\` owns heap data, so assigning or passing it by value moves ownership unless you explicitly borrow or clone.\n\n"
            printf "Understanding this move behavior explains why some bindings become invalid after transfer and why Rust prevents later use.\n\n"
            printf "> **Official Rust Book**: [Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)\n\n"
            ;;
        "func_take_ownership")
            printf "# Ownership: Function Parameters\n\n## Overview\n\n"
            printf "Passing a value into a function transfers ownership when the parameter takes the value by type (for example, \`String\` rather than \`&String\`).\n\n"
            printf "After that call, the original binding is no longer valid unless ownership is returned in some form.\n\n"
            printf "> **Official Rust Book**: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)\n\n"
            ;;
        "func_take_return_ownership")
            printf "# Ownership: Take and Return\n\n## Overview\n\n"
            printf "Functions can take ownership of values and then return ownership back to the caller. This follows the same ownership transfer rules as assignment and returns.\n\n"
            printf "While valid, this pattern is often verbose and motivates borrowing when you only need temporary access.\n\n"
            printf "> **Official Rust Book**: [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions) and [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)\n\n"
            ;;
        "immutable_borrow")
            printf "# Immutable References\n\n## Overview\n\n"
            printf "A reference (\`&T\`) lets you borrow a value without taking ownership. Borrowing allows functions to read data while the original owner keeps control.\n\n"
            printf "Because ownership does not move, the borrowed value remains usable after the call.\n\n"
            printf "> **Official Rust Book**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)\n\n"
            ;;
        "mutable_borrow")
            printf "# Mutable References\n\n## Overview\n\n"
            printf "A mutable reference (\`&mut T\`) allows mutation through a borrow, but Rust enforces exclusivity: at a given time, either one mutable reference or any number of immutable references.\n\n"
            printf "These rules prevent data races and aliasing bugs at compile time.\n\n"
            printf "> **Official Rust Book**: [Mutable References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)\n\n"
            ;;
        "multiple_immutable_borrow")
            printf "# Multiple Immutable Borrows\n\n## Overview\n\n"
            printf "Rust allows multiple immutable references to the same value at the same time, because read-only access does not create conflicting mutations.\n\n"
            printf "This enables safe shared access while still enforcing strict rules around mutation.\n\n"
            printf "> **Official Rust Book**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)\n\n"
            ;;
        "immutable_borrow_method_call")
            printf "# Method Calls: Immutable Borrow\n\n## Overview\n\n"
            printf "Methods commonly take \`&self\`, which means calling the method immutably borrows the receiver instead of moving it.\n\n"
            printf "This allows read-only behavior on a value while keeping ownership with the caller.\n\n"
            printf "> **Official Rust Book**: [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)\n\n"
            ;;
        "mutable_borrow_method_call")
            printf "# Method Calls: Mutable Borrow\n\n## Overview\n\n"
            printf "Methods that take \`&mut self\` borrow the receiver mutably, allowing the method to change internal state without taking ownership.\n\n"
            printf "Rust's method-call syntax automatically handles the needed referencing while preserving borrowing rules.\n\n"
            printf "> **Official Rust Book**: [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)\n\n"
            ;;
    esac
    printf -- "---\n\n## Code example\n\n"
}

# Loop through the specified examples
for target in ${targetExamples[@]}; do
    printf "building %s..." $target
    
    # Check if required files are there
    if [[ -f  "$EX/$target/input/annotated_source.rs" && -f "$EX/$target/source.rs" ]]
    then
        # Check if file headers exist
        if ! [[ -f "$EX/$target/main.rs" ]]
        then
            printf "\ngenerating header for %s..." $target
            cd ../parse
            cargo run "$EX/$target/source.rs" >/dev/null 2>&1
        fi

        cd ../src # switch to appropriate folder
        # Run svg generation for example (keep stderr so build errors are visible)
        cargo run $target >/dev/null

        # If if the svg generation exited with an error or the combined SVG wasn't created, report failure and continue
        if [[ $? -ne 0 || !(-f "examples/$target/vis_combined.svg") ]]; then
            printf "${red}FAILED${end} on SVG generation.\n"
            cd ../mdbook
            continue
        fi
        cd ../mdbook
        
        # Copy files to mdbook directory
        mkdir -p "./src/assets/$target"
        cp "$EX/$target/source.rs" "./src/assets/$target/source.rs"
        cp "$EX/$target/vis_combined.svg" "./src/assets/$target/vis_combined.svg"
        
        echo "done"

        # Write theory section first, then visualization (source is embedded in the SVG)
        write_theory "$target" >> src/$target.md

        printf '<div class="flex-container vis_block rv-vis-block">\n' >> src/$target.md
        printf '<object type="image/svg+xml" class="%s rv-viz-combined" data="assets/%s/vis_combined.svg" onmouseenter="helpers('"'"'%s'"'"')"></object>\n' "$target" "$target" "$target">> src/$target.md
        printf "</div>\n" >> src/$target.md
    else
        # Not Necessary (file double check)
        printf "${red}FAILED${end}. The required files are not in the examples dir.\n"
    fi
done

# Navigation: sections and readable titles (mdBook supports # headings between items)
cat > src/SUMMARY.md <<'SUMMARY_EOF'
# Summary

- [Home](./index.md)

# Variables and mutability

- [Immutable Variables](./immutable_variable.md)
- [Mutable Variables](./mutable_variables.md)

# The Copy trait

- [Stack-Only Data: Copy](./copy.md)

# Ownership

- [Move: Assignment](./move_assignment.md)
- [Move: Different Scope](./move_different_scope.md)
- [Move: Function Return](./move_func_return.md)
- [Ownership: Function Parameters](./func_take_ownership.md)
- [Ownership: Take and Return](./func_take_return_ownership.md)
- [String and Move](./string_from_move_print.md)

# References and borrowing

- [Immutable Borrow](./immutable_borrow.md)
- [Mutable Borrow](./mutable_borrow.md)
- [Multiple Immutable Borrows](./multiple_immutable_borrow.md)
- [Method Calls: Immutable Borrow](./immutable_borrow_method_call.md)
- [Method Calls: Mutable Borrow](./mutable_borrow_method_call.md)

---

- [Playground](./playground.md)
SUMMARY_EOF

# Playground chapter (WASM; run wasm/build.sh first for theme/pkg/*.wasm)
cp playground.md src/playground.md

# Copy wasm-pack output into mdBook assets so it is published with the book
if ! ls theme/pkg/*.js theme/pkg/*.wasm >/dev/null 2>&1; then
    printf "${red}FAILED${end}: missing wasm assets in mdbook/theme/pkg.\n"
    printf "Run ./wasm/build.sh from repository root before ./mdbook/view_examples.sh.\n"
    exit 1
fi
mkdir -p src/assets/pkg
cp -f theme/pkg/*.js theme/pkg/*.wasm src/assets/pkg/

# Build mdbook (output: ./book/)
mdbook build

echo ""
echo "Book built at $(pwd)/book/"
echo "Preview locally: node host-book.mjs"
