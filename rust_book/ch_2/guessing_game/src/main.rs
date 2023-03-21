use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()  // returns an instance of std::io::Stdin
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("You guessed: {guess}.");
    // pin - https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html#generating-a-secret-number
}
