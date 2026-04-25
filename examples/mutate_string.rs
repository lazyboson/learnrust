//! Two ways to "pass a mutable String to a function that modifies it":
//!
//!   A) BORROW MUTABLY  fn f(s: &mut String)      — caller keeps ownership;
//!                                                   function mutates in place;
//!                                                   no return value needed.
//!
//!   B) MOVE + RETURN   fn f(mut s: String) -> String  — function takes
//!                                                   OWNERSHIP, mutates the
//!                                                   owned value, and returns
//!                                                   it. caller must re-bind.
//!
//! (A) is idiomatic — less ceremony, and the borrow checker still guarantees
//! you can't have two simultaneous mutators. Reach for (B) only when the
//! function needs to OWN the value (e.g. builder chains, conditional consume).

fn main() {
    // --- pattern A: &mut String ----------------------------------------
    let mut greeting = String::from("hello");
    println!("before A: {}", greeting);

    append_world(&mut greeting);     // pass a mutable reference
    // greeting is STILL OWNED here. same binding, same allocation, longer.
    println!("after  A: {}", greeting);

    // call it again — borrow ends after each call returns, so it's fine
    append_world(&mut greeting);
    println!("twice  A: {}", greeting);

    // --- pattern B: move + return --------------------------------------
    let greeting = String::from("hello");
    println!("before B: {}", greeting);

    let greeting = append_world_owned(greeting);   // move in, return back
    // we re-bind via shadowing. the OLD `greeting` no longer exists —
    // its ownership was transferred INTO the function and back OUT.
    // the new `greeting` is immutable (no `mut`).
    println!("after  B: {}", greeting);

    // calling it again requires moving the new binding in too
    let greeting = append_world_owned(greeting);
    println!("twice  B: {}", greeting);
}

// PATTERN A — takes &mut, mutates in place, returns ().
//   - caller writes:   append_world(&mut s);
//   - caller still owns `s` after the call.
//   - the `&mut` borrow lasts only for the duration of THIS call, so the
//     caller can borrow again afterwards (or read again, etc.).
fn append_world(s: &mut String) {
    s.push_str(", world");
}

// PATTERN B — takes ownership, mutates the owned value, returns it.
//   - `mut s` in the parameter list makes the OWNED parameter mutable
//     inside this function. it's NOT the same as `&mut s`. the caller's
//     binding doesn't need to be `mut`; only this function's does.
//   - caller writes:   let s = append_world_owned(s);
fn append_world_owned(mut s: String) -> String {
    s.push_str(", world");
    s   // final expression — return the (now-modified) String
}
