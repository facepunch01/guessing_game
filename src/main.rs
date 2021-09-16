use std::io;

fn main() {
    // prints to console
    println!("Guess the number!");
    println!("Please input your guess.");
    // creates empty string
    let mut guess = String::new();
    // gets input in full name would be standard_input
    io::stdin()
        // reads what you typed
        .read_line(&mut guess)
        // just ends this method
        .expect("Failed to read line");
    // prints out what info it gathered
    println!("You guessed: {}", guess);
}