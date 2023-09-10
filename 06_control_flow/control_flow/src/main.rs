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

    // You can label loops to provide finer control over breaks and continues:
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // forces a break in the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While loops
    let mut outter_idx: i32 = 0;
    let mut inner_idx: i32 = 0;
    'outter_loop: loop {
        println!("Outter loop {outter_idx}");
        outter_idx += 1;

        while inner_idx < 5 {
            println!("Inner while {inner_idx}");
            inner_idx += 1;
            if outter_idx == 3 {
                break 'outter_loop;
            }
        }
        inner_idx = 0;
    }

    println!("Broke while loop demo early");

    // For loops are a lot more precise in controlling
    // overall iteration and generally better for
    // avoiding panics when iterating over a collection
    let arr: [i32; 10] = [0; 10];

    for index in (0..input) {
        let _element = arr[index as usize];
        println!("Safely accessed the {}th element of an array from a for loop", index);
    }
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