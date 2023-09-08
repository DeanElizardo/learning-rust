fn main() {
    println!("Hello, world!");

    another_function();

    yet_another_function(11);

    print_some_measure(2, 'g');

    println!("{}", five());

    println!("{}", twelve());
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: u32) {
    println!("YAF. Passed {x}.");
}

fn print_some_measure(x: u32, m: char) {
    println!("The measure is: {x}{m}");
}

// Note the lack of a semicolon here
fn five() -> i32 {
    5
}

fn twelve() -> i32 {
    let _foo: &str = "foo";
    return 12;
}

// This won't compile; either add a return
// or remove the semicolon.
// Implicitly returns the unit type ().
fn deliberately_broken() -> i32 {
    12;
}