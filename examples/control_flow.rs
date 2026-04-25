// --- control flow: if, loop, while, for (TRPL 3.5) ---

fn main() {
    let n = 7;

    // if/else if/else. the condition MUST be a bool — Rust has no
    // truthy/falsy: `if 1 { ... }` would NOT compile.
    if n < 5 {
        println!("{} is small", n);
    } else if n < 10 {
        println!("{} is medium", n);
    } else {
        println!("{} is large", n);
    }

    // `if` is an EXPRESSION — both arms must produce the same type.
    // (Rust has no ternary `?:` because `if` is already one.)
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, label);

    // `loop` runs forever until a `break`. uniquely, `break value;`
    // returns a value FROM the loop expression. while/for cannot do this.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;       // result becomes 20
        }
    };
    println!("loop result = {}", result);

    // loop labels — break or continue a SPECIFIC outer loop by name.
    let mut count = 0;
    'outer: loop {
        loop {
            count += 1;
            if count == 5 {
                break 'outer;        // exits the OUTER loop, not just the inner
            }
        }
    }
    println!("count after labelled break = {}", count);

    // `while` — like other languages.
    let mut k = 3;
    while k != 0 {
        println!("while k = {}", k);
        k -= 1;
    }

    // `for x in <iterator>` — the idiomatic loop. safer than indexing
    // because the iterator can't go out of bounds.
    let arr = [10, 20, 30];
    for v in arr.iter() {
        println!("for elem = {}", v);
    }

    // ranges: `start..end` is EXCLUSIVE of end. `start..=end` is inclusive.
    for i in 1..4 {
        println!("range i = {}", i);    // prints 1, 2, 3
    }

    // .rev() reverses an iterator.
    for i in (1..4).rev() {
        println!("rev i = {}", i);      // prints 3, 2, 1
    }
}
