use std::io;

fn main() {
    println!("Let's play a guessing game!");
    println!("Guess a number...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line.");

    println!("You guessed {guess}");
}
