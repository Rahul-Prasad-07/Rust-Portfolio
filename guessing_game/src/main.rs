use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //generate random number between 1 and 100
    let secrate_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secrate_number);

    loop {
        println!("Please input your guess.");
        //create a mutable variable to store the user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /**
         * here we have convert guess input string to number, it's called shadowing
         * to handle the error if the user input is not a number
         * we need to use match expression to handle the error & possible variants
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secrate_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
