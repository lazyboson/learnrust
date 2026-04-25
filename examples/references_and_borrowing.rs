//! TRPL Ch 4.2 — References and Borrowing
//!
//! A REFERENCE (&T) lets a function USE a value without taking ownership.
//! The reference doesn't own the data; when the reference goes out of scope,
//! the underlying value is NOT dropped.
//!
//! The two rules of references (enforced by the borrow checker):
//!   1. At any time you can have EITHER ONE  &mut T  OR  any number of  &T.
//!   2. References must always be valid (Rust prevents dangling refs at compile time).

fn main() {
    // --- immutable reference: borrow without moving ---
    let s = String::from("hello");
    let len = calculate_length(&s);    // pass &s — a reference. s stays valid.
    println!("'{}' has length {}", s, len);

    // --- references are immutable by default ---
    // you cannot mutate through a `&T`. this would NOT compile:
    //   fn change(s: &String) { s.push_str("!"); }
    //   ERROR: cannot borrow `*s` as mutable, as it is behind a `&` reference

    // --- mutable reference: &mut T ---
    let mut s = String::from("hello");
    change(&mut s);                    // pass &mut so the function can mutate s
    println!("after change: {}", s);

    // --- rule 1a: only ONE mutable reference at a time ---
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;                // ERROR: cannot borrow `s` as mutable more than once
    r1.push_str(", world");
    println!("r1 = {}", r1);
    //
    // why? two simultaneous &mut references could mutate the same data
    // concurrently. Rust prevents the data race at compile time.

    // --- rule 1b: cannot mix &mut with & at overlapping times ---
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;                       // OK: many shared (&) borrows are fine
    println!("r1 = {}, r2 = {}", r1, r2);
    // r1, r2 last used above. their borrows END here (NLL — non-lexical lifetimes).

    // ...so a fresh &mut is now allowed:
    let r3 = &mut s;
    r3.push_str(", world");
    println!("r3 = {}", r3);

    // --- rule 2: dangling references are rejected at compile time ---
    // this would NOT compile:
    //   fn dangle() -> &String {
    //       let s = String::from("hello");
    //       &s   // ERROR: returns reference to data owned by current function
    //   }
    //
    // fix: return the String itself, transferring ownership.
    let good = no_dangle();
    println!("ownership returned: {}", good);

    // --- borrows release at last use, freeing the value for new borrows ---
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("first borrow = {}", first);
    // first last used above. borrow ends here.

    v.push(4);                          // OK: no live borrow remains
    println!("vec after push = {:?}", v);
}

// taking &String is fine but &str is more general — see slices.rs.
fn calculate_length(s: &String) -> usize {
    s.len()    // reading through a shared reference is allowed
} // when s goes out of scope, only the REFERENCE is dropped — not the value.

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s   // ownership moves out — caller now owns the String, no dangling ref
}
