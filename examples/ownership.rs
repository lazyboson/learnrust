//! TRPL Ch 4.1 — What is Ownership?
//!
//! The three rules:
//!   1. Each value has an owner.
//!   2. There can only be ONE owner at a time.
//!   3. When the owner goes out of scope, the value is dropped (freed).
//!
//! These rules are enforced at COMPILE TIME — no garbage collector, no manual
//! free, and no runtime cost.

fn main() {
    // --- variable scope ---
    {
        let s = "hello";
        println!("scoped: {}", s);
    } // s goes out of scope here. for heap-owning types, memory is freed now.

    // --- move semantics for heap-owning types ---
    // String owns its bytes on the heap. Assigning a String MOVES ownership;
    // the source binding becomes invalid.
    let s1 = String::from("hello");
    let s2 = s1;                 // ownership of the heap data MOVES from s1 -> s2
    // println!("{}", s1);       // ERROR: borrow of moved value: `s1`
    println!("after move, s2 = {}", s2);
    //
    // why? if both s1 and s2 pointed at the same heap memory, dropping both
    // at scope end would double-free. moving prevents that — only s2 owns it.

    // --- explicit deep copy with .clone() ---
    let a = String::from("hi");
    let b = a.clone();           // allocates new heap memory and copies the bytes
    println!("clone: a = {}, b = {}", a, b);   // both bindings still valid

    // --- the Copy trait: stack-only types are COPIED, not moved ---
    // Copy types include: all integers, f32/f64, bool, char,
    // and tuples whose elements are all Copy (e.g. (i32, bool)).
    // Copy is NOT implemented for String, Vec<T>, Box<T>, etc.
    let x = 5;
    let y = x;                   // x is COPIED (cheap stack value, just bytes)
    println!("copy: x = {}, y = {}", x, y);    // both still valid

    // --- ownership and functions: passing arguments moves (or copies) ---
    let owned = String::from("owned");
    takes_ownership(owned);       // owned is MOVED into the function
    // println!("{}", owned);     // ERROR: owned is no longer valid here

    let n = 42;
    makes_copy(n);                // n is COPIED into the function (i32 is Copy)
    println!("n still usable: {}", n);

    // --- returning ownership ---
    let s = gives_ownership();    // function transfers a String OUT to s
    println!("got: {}", s);

    let mut s2 = String::from("round trip");
    let s2 = takes_and_gives_back(s2);  // moves in, then moves out via return
    println!("returned: {}", s2);

    // taking ownership and giving it back is verbose; this is exactly what
    // motivates references in 4.2 — let a function USE a value temporarily
    // without taking ownership of it.
}

fn takes_ownership(some_string: String) {
    println!("inside takes_ownership: {}", some_string);
} // some_string drops here; its heap memory is freed.

fn makes_copy(n: i32) {
    println!("inside makes_copy: {}", n);
} // n drops here. nothing on the heap to free.

fn gives_ownership() -> String {
    let s = String::from("hello from inside");
    s   // s is moved out to the caller (no copy)
}

fn takes_and_gives_back(s: String) -> String {
    s   // moved in, then moved out — the caller gets ownership back
}
