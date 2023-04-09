pub fn tuples() {
    //tuples max 12 elements are allowed
    let person : ( &str, &str, i8) = ("ashu", "pandey", 13);
    println!("my name is {} and last name is {} and age is {}", person.0, person.1, person.2);
}