fn main() {
    let x = 5;
    let x = x + 1;

    println!("Outer scope, entering inner scope...");
    {
        let x = x * 2;
        println!("In the inner scope, x = {x}");
    }

    println!("In the outer scope, x = {x}");

    let guess: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    println!("Guess {guess}");

    let guess: u32 = match "flerp".parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    println!("Guess again: {guess}");
}
