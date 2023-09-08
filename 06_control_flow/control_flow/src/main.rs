use std::io;

fn main() {
    let mut input = String::new();

    println!("Make input.");

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

    if input < 5 {
        println!("Input {input} is less than 5");
    } else {
        println!("Input {input} is 5 or greater");
    }

    countdown_divisible(input);

    // You can combine if-else with expressions
    // to perform conditional assignment.
    let even_or_odd: &str = if input % 2 == 0 { "even" } else { "odd" };

    println!("{input} is an {even_or_odd} number.");

    // Note that conditional compilation requires that all arms of the
    // branching logic evaluate to the same type. The following
    // statement would not compile if left uncommented.
    // let _fail_to_compile = if input % 17 == 0 { "foo" } else { 17 };

    /*
    *************************************************************************
    * LOOPS ARE FREAKIN' GREAT                                              *
    *************************************************************************
     */

    // we can return a value from a loop. This is great for retries where you
    // definitely need a value from a successful attempt.
    let mut counter = 0;
    let multiple_of_17: i32 = (17 * input).abs();
    let difference_to_multiple_of_17: i32 = loop {
        if counter == multiple_of_17 {
            break counter;
        }

        counter += 1;
    };

    println!("{}", difference_to_multiple_of_17);
}

fn countdown_divisible(x: i32) {
    let mut d: i32 = x;

    while d > 0 {
        if x % d == 0 {
            println!("{x} is divisble by {d}");
        }

        d -= 1;
    }
}