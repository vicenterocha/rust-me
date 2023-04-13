use rand::{Rng, random}; // Rng is a trait that defines methods that random number generators implement
use std::{cmp::Ordering, io};


fn main() {
    println!("Guess the number!");
    // `thread_rng` gives a random number generator local to the current
    // thread of execution and seeded by the operating system
    // `gen_range` is defined by the Rng trait and generates a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin() // returns an instance of std::io::Stdin
            .read_line(&mut guess)
            .expect("Failed to read line");

        // u32 tells parse to convert guess to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
