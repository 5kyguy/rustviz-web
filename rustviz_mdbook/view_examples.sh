#!/bin/bash
red=$'\e[1;31m'
end=$'\e[0m'

cd rustviz_mdbook

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

# Write the first line of SUMMARY.md. This clears anything that was there previously
printf "# Summary\n\n" > src/SUMMARY.md
echo "- [Home](./index.md)" >> src/SUMMARY.md

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
            cd ../RustvizParse
            cargo run "$EX/$target/source.rs" >/dev/null 2>&1
        fi

        cd ../src # switch to appropriate folder
        # Run svg generation for example (keep stderr so build errors are visible)
        cargo run $target >/dev/null

        # If if the svg generation exited with an error or the required SVGs weren't created, report failure and continue
        if [[ $? -ne 0 || !(-f "examples/$target/vis_code.svg") || !(-f "examples/$target/vis_timeline.svg") ]]; then
            printf "${red}FAILED${end} on SVG generation.\n"
            cd ../rustviz_mdbook
            continue
        fi
        cd ../rustviz_mdbook
        
        # Copy files to mdbook directory
        mkdir -p "./src/assets/$target"
        cp "$EX/$target/source.rs" "./src/assets/$target/source.rs"
        cp "$EX/$target/vis_code.svg" "./src/assets/$target/vis_code.svg"
        cp "$EX/$target/vis_timeline.svg" "./src/assets/$target/vis_timeline.svg"
        
        # Add append corresponding line to SUMMARY.md
        echo "- [$target](./$target.md)" >> src/SUMMARY.md
        echo "done"

        # Write into .md files (no in-page heading to avoid duplicate nested TOC entries)
        printf "\`\`\`rust\n" >> src/$target.md
        printf "{{#rustdoc_include assets/%s/source.rs}}\n" "$target" >> src/$target.md
        printf "\`\`\`\n" >> src/$target.md
        printf '<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;">\n' >> src/$target.md
        printf '\t<object type="image/svg+xml" class="%s code_panel" data="assets/%s/vis_code.svg"></object>\n' "$target" "$target">> src/$target.md
        printf '\t<object type="image/svg+xml" class="%s tl_panel" data="assets/%s/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('"'"'%s'"'"')"></object>\n' "$target" "$target" "$target">> src/$target.md
        printf "</div>" >> src/$target.md
    else
        # Not Necessary (file double check)
        printf "${red}FAILED${end}. The required files are not in the examples dir.\n"
    fi
done

# Playground chapter (WASM; run wasm/build.sh first for theme/pkg/*.wasm)
echo "- [Playground](./playground.md)" >> src/SUMMARY.md
cp playground.md src/playground.md

# Copy wasm-pack output into mdBook assets so it is published with the book
if ! ls theme/pkg/*.js theme/pkg/*.wasm >/dev/null 2>&1; then
    printf "${red}FAILED${end}: missing wasm assets in rustviz_mdbook/theme/pkg.\n"
    printf "Run ./wasm/build.sh from repository root before ./rustviz_mdbook/view_examples.sh.\n"
    exit 1
fi
mkdir -p src/assets/pkg
cp -f theme/pkg/*.js theme/pkg/*.wasm src/assets/pkg/

# Build mdbook (output: ./book/)
mdbook build

echo ""
echo "Book built at $(pwd)/book/"
echo "Preview locally: node host-book.mjs"
