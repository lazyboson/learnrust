pub fn run() {
    println!("hi from print");

    //base formatting
    println!("{} says hello, from {}", "ashu", "mars");

    //position formatting
    println!("Hi this is {0}, Welcome to {1}", "ashutosh", "India");

    //named parameter
    println!("{name} likes to play {game}", name="ashutosh", game="football");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Oct: {:o}", 10,10,10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}