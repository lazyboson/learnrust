mod print;
mod vars;
mod types;
mod tuples;
mod strings;
mod arr;
mod vec;
use std::{io};

fn main() {
    println!("Guess the number");
    vec::usevec();
    print::run();
    vars::var();
    strings::stringmethods();
    types::datatype();
    tuples::tuples();
    arr::usearray();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read the line");
    println!("You guessed: {guess}");
    let x = 5; 
    let y = 6;
    println!("x= {x} and y+2 = {}", y);
}
