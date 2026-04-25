// --- functions: parameters, return values, statements vs expressions (TRPL 3.3) ---

fn main() {
    greet("ashutosh");

    let sum = add(3, 4);
    println!("add(3, 4) = {}", sum);

    // STATEMENTS perform an action and do NOT return a value.
    //   `let y = 6;` is a statement.
    //   you cannot do `let x = (let y = 6);` — `let` has no value.

    // EXPRESSIONS evaluate to a value.
    //   `5 + 1`, `add(2, 3)`, and even `{ ... }` blocks are expressions.

    // a block `{ ... }` is an expression — its value is the value of the
    // FINAL expression inside (with NO trailing semicolon).
    let block_value = {
        let a = 2;
        let b = 3;
        a + b               // <-- no semicolon: this is the block's value
    };
    println!("block_value = {}", block_value);

    // adding a semicolon below would turn `a + b` into a statement,
    // and the block would evaluate to () (unit) instead of an i32.

    let five = returns_five();
    println!("returns_five() = {}", five);

    let twice_three = double(3);
    println!("double(3) = {}", twice_three);
}

// parameter types are REQUIRED (Rust does not infer parameter types).
fn greet(name: &str) {
    println!("hello, {}!", name);
}

// `-> i32` declares the return type. the function body's final expression
// (no trailing semicolon) IS the return value. this is idiomatic Rust.
fn add(a: i32, b: i32) -> i32 {
    a + b               // <-- no semicolon: this expression is returned
}

// equivalent using an explicit `return`. less idiomatic but legal.
fn returns_five() -> i32 {
    return 5;
}

// the body is one expression; the implicit return is the standard style.
fn double(x: i32) -> i32 {
    x * 2
}
