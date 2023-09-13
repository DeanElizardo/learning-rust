fn main() {
    let mut s: String = String::from("Hello, world!");

    // Taking a slice causes an immutable borrow
    let first_word: &str = &s[0..5];
    let secnd_word: &str = &s[7..12];

    // Implicit mutable borrow is not allowed here!
    // s.push_str(" howdy.");

    println!("{} {}", first_word, secnd_word);

    s.push_str(" Now that the slices are no longer alive, this push is legal!");

    println!("{}", s);

    /*
    ***************************************************************************
    *                             RANGE SYNTAX                                *
    ***************************************************************************
    */

    // get the first 3 characters of s
    let slice_1: &str = &s[..3];

    // get the the characters after the 8th character
    let slice_2: &str = &s[9..];

    // get a slice of a slice
    let slice_3: &str = &s[0..15][3..9];

    println!("{}", slice_1);
    println!("{}", slice_2);
    println!("{}", slice_3);
}
