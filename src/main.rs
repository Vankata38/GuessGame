// External crates
extern crate rand;

// Use
use std::cmp::Ordering;
use std::io;
use std::io::Write;

use rand::Rng;

// Main function
fn main() {

    // Constants
    let range_bot: u32 = 1;
    let range_top: u32 = 101;

    // Variables
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(range_bot..range_top);

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap(); // Forces the print

        // Create the string
        let mut guess = String::new();

        // Read the input as a string - new line
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the variable to a u8, used for matching
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Show guess
        println!("You guessed {}", guess);

        // Compare the guess to the secret number
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