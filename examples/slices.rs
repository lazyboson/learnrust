//! TRPL Ch 4.3 — The Slice Type
//!
//! A SLICE is a reference to a contiguous sequence of elements in a
//! collection. It's a "fat pointer": a pointer + a length.
//!
//!   &str  is a slice into UTF-8 bytes (a String, or a string literal).
//!   &[T]  is a slice into an array or Vec.
//!
//! Slices borrow — they don't own. The compiler tracks the borrow so the
//! underlying data can't be invalidated while a slice exists.

fn main() {
    // --- string slices ---
    let s = String::from("hello world");

    let hello: &str = &s[0..5];        // bytes 0..5  (5 is exclusive)
    let world: &str = &s[6..11];       // bytes 6..11
    println!("hello = '{}', world = '{}'", hello, world);

    // shorthand range forms
    let from_start = &s[..5];          // == &s[0..5]
    let to_end     = &s[6..];          // == &s[6..s.len()]
    let whole      = &s[..];           // == &s[0..s.len()] (entire string)
    println!("{} | {} | {}", from_start, to_end, whole);

    // --- string literals ARE slices ---
    // a literal's type is &'static str — a slice into the program's binary.
    // that's why literals are immutable.
    let literal: &str = "I'm a slice baked into the executable";
    println!("{}", literal);

    // --- &str as parameter is more general than &String ---
    // a function taking &str accepts BOTH a slice of a String and a literal.
    // (deref coercion turns &String into &str automatically.)
    let owned = String::from("hello world");
    let word1 = first_word(&owned);            // pass a slice of an owned String
    let word2 = first_word("foo bar baz");     // pass a string literal directly
    println!("first_word(owned) = '{}', first_word(literal) = '{}'", word1, word2);

    // --- array slices: &[T] ---
    let arr = [1, 2, 3, 4, 5];
    let middle: &[i32] = &arr[1..4];          // [2, 3, 4]
    println!("array slice middle = {:?}", middle);
    println!("middle.len() = {}, sum = {}", middle.len(), sum(middle));

    // --- a slice keeps a borrow alive — borrow checker prevents misuse ---
    let mut v = vec![10, 20, 30];
    let view = &v[..];
    // v.push(40);  // ERROR: cannot borrow `v` as mutable while `view` is alive
    println!("view = {:?}", view);
    // view last used above; borrow ends here (NLL)

    v.push(40);                                // OK now: no live borrow
    println!("v after push = {:?}", v);
}

// idiomatic: take &str (slice) instead of &String — accepts both forms.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];     // slice up to (not including) the first space
        }
    }
    s   // no space found — the whole string is one word
}

fn sum(xs: &[i32]) -> i32 {
    let mut total = 0;
    for &x in xs {              // `&x` destructures &i32 -> i32 (since i32 is Copy)
        total += x;
    }
    total
}
