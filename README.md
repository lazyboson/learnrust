# learnrust

A personal playground for learning Rust by working through
[The Rust Programming Language (TRPL)](https://doc.rust-lang.org/book/) and
[Rust by Example](https://doc.rust-lang.org/rust-by-example/).

Each topic lives in its own runnable example under [`examples/`](examples/),
paired with an integration test under [`tests/`](tests/) that asserts the
concepts the example demonstrates.

## Run an example

```sh
cargo run --example <topic>
```

Or just `cargo run` to print the topic list.

## Run tests

```sh
cargo test                      # all tests
cargo test --test <topic>       # one topic
```

## Topics

### Rust by Example

| Topic | Example | Covers |
| --- | --- | --- |
| Formatted print | [`formatted_print`](examples/formatted_print.rs) | `println!` positional, named, debug, binary/hex/oct |

### TRPL Ch 3 — Common Programming Concepts

| Section | Example | Tests | Covers |
| --- | --- | --- | --- |
| 3.1 Variables | [`variables_and_mutability`](examples/variables_and_mutability.rs) | [tests](tests/variables_and_mutability.rs) | `let`, `mut`, `const`, tuple destructuring, **shadowing** (incl. type-changing and scoped) |
| 3.2 Data types | [`data_types`](examples/data_types.rs) | [tests](tests/data_types.rs) | scalar types, integer literal forms (`0xff`, `0o77`, `0b1111_0000`, `b'A'`, `1_000_000`), overflow with `wrapping_*` / `checked_*`, `f32` vs `f64`, char as 4-byte Unicode, unit `()` |
| 3.2 Tuples | [`tuples`](examples/tuples.rs) | [tests](tests/tuples.rs) | typed literal, `.0/.1/.2` access, destructuring, unit tuple |
| 3.2 Arrays | [`arrays`](examples/arrays.rs) | [tests](tests/arrays.rs) | `[T; N]`, indexing & mutation, slices, repeat syntax `[3; 5]`, runtime bounds-check panic, `.get()` |
| 3.3 Functions | [`functions`](examples/functions.rs) | [tests](tests/functions.rs) | parameters, `-> T` return, **statements vs expressions**, block expressions |
| 3.4 Comments | [`comments`](examples/comments.rs) | — | `//`, `/* */`, `///` outer doc, `//!` inner doc |
| 3.5 Control flow | [`control_flow`](examples/control_flow.rs) | [tests](tests/control_flow.rs) | `if` as expression, `loop` with `break value`, loop labels, `while`, `for`, ranges `..` and `..=`, `.rev()` |

### TRPL Ch 8 — Common Collections

| Section | Example | Tests | Covers |
| --- | --- | --- | --- |
| 8.1 Vectors | [`vectors`](examples/vectors.rs) | [tests](tests/vectors.rs) | `vec![]` macro, `Vec::new()`, `push`, iteration |
| 8.2 Strings | [`strings`](examples/strings.rs) | [tests](tests/strings.rs) | `&str` vs `String`, `push` / `push_str`, `capacity`, `split` |
| 8.3 Hash maps | [`common_collections`](examples/common_collections.rs) | [tests](tests/common_collections.rs) | `HashMap`, `HashSet` deduplication, sorted `BTreeMap` / `BTreeSet` |

## Layout

```
src/main.rs              # landing page printed by `cargo run`
examples/<topic>.rs      # runnable demo per topic
tests/<topic>.rs         # integration tests asserting the topic's concepts
```

## How to study with this repo

1. Pick a topic from the table above.
2. Read the example file — comments call out each concept.
3. Run it: `cargo run --example <topic>`.
4. Read the matching test file — it asserts the same concepts more
   rigorously, and often tests an edge case the example glosses over
   (e.g. `tests/arrays.rs` proves out-of-bounds panics at runtime,
   `tests/data_types.rs` proves `b'A' == 65u8`).
5. Run `cargo test --test <topic>` to confirm nothing has rotted.
