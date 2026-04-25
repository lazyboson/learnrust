//! inner doc comments use `//!` and document the *enclosing* item —
//! typically used at the very top of a module or crate root.
//! `cargo doc` reads these and renders them as the crate-level docs.

// `//` starts a line comment — runs to end of line.
// you can stack
// multiple line comments
// like this.

/* block comments use /* ... */ and CAN be nested
   /* like this nested one */
   though `//` is the idiomatic style for most code in Rust. */

/// outer doc comments use `///` and document the NEXT item below them.
/// they support markdown and are picked up by `cargo doc`.
fn main() {
    println!("see the source — comments don't run, they're for humans.");

    let x = 5; // trailing comments work too
    println!("x = {}", x);
}
