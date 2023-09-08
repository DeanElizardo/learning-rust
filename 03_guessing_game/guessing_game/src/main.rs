use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play a guessing game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Guess a number... (Hint: {secret_number})");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read line.");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please, type a number next time.");

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low..."),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Just right; you win!");
                break;
            }
        }
    }
}
