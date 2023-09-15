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

    let word_1: &str = frst_wrd(&s);
    let word_2: &str = scnd_wrd(&s);

    // THIS is not legal, because the references (slices) returned from the calls
    // to frst_wrd and scnd_wrd are still live
    // s.clear();

    println!("First word from slicing: {}", word_1);
    println!("Second word from slicing: {}", word_2);
}


fn frst_wrd(input: &String) -> &str {
    let bytes: &[u8] = input.as_bytes();

    for (i, &element) in bytes.iter().enumerate() {
        if element == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}

fn scnd_wrd(input: &String) -> &str {
    let bytes: &[u8] = input.as_bytes();
    let mut start_idx: usize = 0;
    let mut stop_idx: usize = 0;

    for (i, &element) in bytes.iter().enumerate() {
        start_idx = i;
        if element == b' ' {
            break;
        }
    }

    if start_idx == bytes.len() {
        return &input[..];
    }

    start_idx += 1;
    for (i, &element) in bytes.iter().enumerate() {
        stop_idx = i;

        if stop_idx > start_idx && element == b' ' {
            break;
        }
    }

    if start_idx < stop_idx {
        return &input[start_idx..stop_idx];
    }

    &input[..]
}