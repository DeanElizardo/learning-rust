use std::io;

fn main() {

    println!("****************************");
    println!("*        TUPLE TIME        *");
    println!("****************************\n");
    // tuples do not have the ability to grow or
    // shrink after they have been declared
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    
    // tuples can be destructured
    let (_x, y, _z) = tup;
    
    println!("y is {y}");
    
    // We can access the elements of a tuple with the
    // '.' operator
    let q = tup.0;
    let r = tup.1;
    let s = tup.2;
    
    println!("q: {q}\nr: {r}\ns: {s}\n\n");
    
    println!("****************************");
    println!("*       ARRAY ALL DAY      *");
    println!("****************************\n");

    // Arrays are very similar to Vectors, but they
    // are allocated on the stack instead of the heap.
    // Arrays do not grow or shrink in size after allocation.
    let a: [i32; 4] = [1, 2, 3, 4];

    // Elements of an array are accessed with square brackets
    let _b: i32 = a[0];
    let _c: i32 = a[2];

    // Attempting to access an element outside of the index
    // range of an array causes a runtime error
    println!("Give an index between 0 - 3");
    let mut index: String = String::new();
    io::stdin().read_line(&mut index).expect("Unable to read line");
    
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };

    let value = a[index];
    println!("If you entered a number between 0 and 3, you got this: {value}");
}
