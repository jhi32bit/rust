extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("########## GAME - Guess the number ###########");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess(1-100):");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} is too big!"),
            Ordering::Equal => {
                println!("Your guess was correct. You win!");
                break;
            }
        }
    }
}