/*
"Safety is the absence of undefined behavior."

Rust provides a novel model for thinking about how memory is managed, and
ownership is a discipline for safely using memory within that model.

1. Variables live on the stack.
2. Boxes live on the heap. `Box` is a construct that Rust uses to house data
   that is pointed to by a variable (usually containing a pointer).
3. Variables that point to a Box "own" the memory on the heap. Only one variable
   may own a box at a time.
4. If a variable's stack frame is deallocated while said variable has ownership of
   a box, then the box is deallocated on the heap.
5. Moving ownership away from a variable without giving it another box means the variable
   can't be used. This is because _after_ a variable is moved, we don't know what it's old
   owner might point to. Trying to use that old owner would be undefined behavior.
 */
fn main() {
    // a owns the box
    let mut a: Box<[i32; 10]> = Box::new([0; 10]);

    // this moves the pointer to the box from a to b.
    // b owns the box.
    let mut b:Box<[i32; 10]> = a; 

    b[0] = 1;

    // a owns the box again.
    a = b;

    println!("{}", a[0]);

    // We can't use b like this, because it's been moved:
    // println!("{}", b[0]); // the compiler will complain about a moved value

    // But if we give b something new to hold...
    b = Box::new([2; 10]);
    println!("{}", b[0]); // tadaa!

    // A way to avoid moving data is to clone it with `.clone()`
    
    let first:String = String::from("Ferris"); // first owns the string we just made

    let first_clone:String = first.clone(); // We clones it!

    let full:String = add_suffix(first_clone); // We move the string from first_clone to the argument of add_suffix()

    // Trying to use first_clone here gets the error about moved values
    // but this is OK:
    println!("{full}, formerly just {first}.");

    // question_3();
}

fn add_suffix(mut name: String) -> String {
    name.push_str("Jr.");
    name
}

// fn question_3() {
//     let s = String::from("hello");
//     let s2;
//     let b = false;

//     // The compiler does not know for certain that b will always be false,
//     // and so it treats this as a change in ownership.
//     if b {
//         s2 = s;
//     }
//     println!("{}", s);
// }