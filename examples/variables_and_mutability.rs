fn main() {
    let name = "Ashutosh";
    let mut age = 29;

    println!("my name is {} and I am {}", name, age);
     age = 45;
    println!("my name is {} and I am {}", name, age);

    //define a const
    const ID:i32 = 001;
    println!("ID: {}", ID);

    //assign multiple variable
    let (my_name, my_age) = ("ashutosh", 35);
    println!("my name {} age{}", my_name, my_age);

    // --- shadowing (TRPL 3.1) ---
    // re-declaring with `let` creates a NEW binding that shadows the old one.
    // unlike `mut`, shadowing can change the *type*, and the new binding is
    // still immutable inside its scope.
    let x = 5;
    let x = x + 1;          // x is now 6 (a new binding; the old 5 is shadowed)
    let x = x * 2;          // x is now 12
    println!("shadowed x = {}", x);

    // shadowing across types: same name, different type each step
    let spaces = "   ";        // spaces is &str
    let spaces = spaces.len(); // spaces is now usize (3)
    println!("spaces became a number: {}", spaces);

    // contrast with mut: `mut` keeps the SAME type. The line below would NOT compile:
    //   let mut y: &str = "  ";
    //   y = y.len();  // ERROR: expected &str, found usize

    // shadowing is scoped — an inner shadow disappears when the scope ends.
    let z = 5;
    {
        let z = z * 10;     // inner z = 50
        println!("inner z = {}", z);
    }
    println!("outer z = {}", z);  // still 5
}