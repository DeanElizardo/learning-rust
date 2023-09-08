use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play a guessing game!");

    let exit_keyword: String = String::from("quit");
    let mut exit_semaphore: bool = false;
    let mut number_of_guesses: u32 = 0;
    let mut guess: String = String::new();
    loop {
        if exit_semaphore {
            break;
        }
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        loop {
            if exit_semaphore {
                break;
            }


            println!("Guess a number... ");

            guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Unable to read line.");

            guess = guess.chars().filter(|&c| !c.is_whitespace()).collect();

            if guess == exit_keyword {
                exit_semaphore = true;
                continue;
            }

            let guess: u32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed {guess}");

            number_of_guesses += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too low..."),
                Ordering::Greater => println!("Too high!"),
                Ordering::Equal => {
                    println!("Just right; you win!");
                    exit_semaphore = true;
                }
            }
        }
    }

    if guess != exit_keyword {
        println!("You made {number_of_guesses} guesses.");
    }
}
