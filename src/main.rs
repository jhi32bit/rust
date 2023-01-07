extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("########### GAME ##########");
    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // initialize empty String
    let mut guess = String::new();
    println!("Insert a number (1-100):");
    // get user input String
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // parse String to i32
    let num_guess: i32 = guess.trim().parse().expect("Please type a number!");
    // check result
    match num_guess.cmp(&secret_number) {
        Ordering::Less => print!("Sorry, your number {guess} is smaller than {secret_number}!"),
        Ordering::Greater => print!("Sorry, your number {guess} is bigger than {secret_number}!"),
        Ordering::Equal => print!("Yeah! You guessed the right number..."),
    }
}