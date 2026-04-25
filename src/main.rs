fn main() {
    println!("learnrust — pick a topic to run:");
    println!();
    println!("  Rust by Example");
    println!("    cargo run --example formatted_print            // println! formatting");
    println!();
    println!("  TRPL Ch 3: Common Programming Concepts");
    println!("    cargo run --example variables_and_mutability   // 3.1 let, mut, const, shadowing");
    println!("    cargo run --example data_types                 // 3.2 integers, floats, bool, char, unit");
    println!("    cargo run --example tuples                     // 3.2 compound: tuples + destructuring");
    println!("    cargo run --example arrays                     // 3.2 compound: arrays + slices");
    println!("    cargo run --example functions                  // 3.3 fn, params, return, expr vs stmt");
    println!("    cargo run --example comments                   // 3.4 // and /* */ and doc comments");
    println!("    cargo run --example control_flow               // 3.5 if, loop, while, for, ranges");
    println!();
    println!("  TRPL Ch 4: Understanding Ownership");
    println!("    cargo run --example ownership                  // 4.1 ownership rules, move, clone, Copy");
    println!("    cargo run --example references_and_borrowing   // 4.2 &T, &mut T, borrowing rules, NLL");
    println!("    cargo run --example slices                     // 4.3 &str, &[T], slice as parameter");
    println!("    cargo run --example mutate_string              // patterns: &mut String vs move+return");
    println!();
    println!("  TRPL Ch 8: Common Collections");
    println!("    cargo run --example strings                    // 8.2 String, &str");
    println!("    cargo run --example vectors                    // 8.1 Vec<T>");
    println!("    cargo run --example common_collections         // HashMap, HashSet, BTreeMap, BTreeSet");
    println!();
    println!("Run all tests with: cargo test");
}
