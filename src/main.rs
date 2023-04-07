use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read the line");
    println!("You guessed: {guess}");
    let x = 5; 
    let y = 6;
    println!("x= {x} and y+2 = {}", y);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret number is: {}", secret_number);
}
