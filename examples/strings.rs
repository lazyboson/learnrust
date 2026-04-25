fn main() {
    let hello: &str = "hello";
    let mut hel = String::from("Hello");
    let hell2 = String::from("Hello2");
    hel = hell2;
    println!("changed {}", hel);

    hel.push('a');
    hel.push_str("pandey");
    println!("string1 {} and string2 {}", hello, hel);

    //capacity in bytes
    println!("hel capacity: {}", hel.capacity());

    //loop through string by whitespace
    for word in hel.split(' ')  {
        println!("{}", word)
    }

    assert_eq!(hel, "Helloapandey");
}