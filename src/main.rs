use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please input you guess.");

    //random number between 1 and 101
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        //creates empty string to store guess in
        let mut guess = String::new();

        println!("You guessed:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //converts input from string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        };

        //cmp = compare operator, refrencing secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        };
    }
}
