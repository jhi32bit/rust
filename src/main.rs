use std::io;
//use rand::Rng;

fn main() {
    let err_read = "Failed to read line.";

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Guess the number!\nPlease insert your guess");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect(err_read);

    println!("You guessed: {guess}");
}

# [test]
fn should_fail() {
    unimplemented!();
}
