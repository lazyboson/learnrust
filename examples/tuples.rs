fn main() {
    //tuples max 12 elements are allowed
    let person : ( &str, &str, i8) = ("ashu", "pandey", 13);
    println!("my name is {} and last name is {} and age is {}", person.0, person.1, person.2);

    // --- destructuring (TRPL 3.2) ---
    // bind each element to its own name in one statement.
    let (first, last, age) = person;
    println!("destructured: {} {} {}", first, last, age);

    // ignore parts you don't need with `_`
    let (only_first, _, _) = person;
    println!("only first = {}", only_first);

    // --- the unit tuple () ---
    // a tuple with no elements. its sole value is also `()`. it's the
    // "no meaningful value" return type — e.g. functions that exist only
    // for side effects (like println!) return ().
    let nothing: () = ();
    println!("unit = {:?}", nothing);
}