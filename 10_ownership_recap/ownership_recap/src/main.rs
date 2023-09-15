fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
}

/*
*******************************************************************************
*                                 EXAMPLE 1                                   *
*******************************************************************************
 */
fn example_1() {
    // Create a value on the stack
    let mut a_num: i32 = 0;

    // Create a mutable reference to a_num
    // and pass it to inner()
    inner(&mut a_num);

    // All of the memory on the heap allocated inside inner()
    // is deallocated here

    println!("{}", a_num); // Expect this to be `5`
}

fn inner(x: &mut i32) {
    // place an i32 on the stack frame for the call to inner()
    let another_num: i32 = 2;

    // place a reference on the stack frame that points to the
    // variable another_num
    let a_stack_ref: &i32 = &another_num;

    // Place a value on the heap, in this case the contents
    // of the stack variable pointed to by a_stack_ref.
    // a_box is a pointer to the heap, and this pointer lives
    // on this stack frame.
    let a_box: Box<i32> = Box::new(*a_stack_ref);

    // Place a pointer on this stack frame that points
    // to the pointer that is a_box
    let a_box_stack_ref: &Box<i32> = &a_box;

    // Place a pointer on this stack frame that points to
    // the i32 on the heap that is also pointed to by a_box
    let a_box_heap_ref: &i32 = &*a_box;

    *x += 5;

    assert_eq!(*a_box_heap_ref, **a_box_stack_ref);
}

/*
*******************************************************************************
*                                 EXAMPLE 2                                   *
*******************************************************************************
 */
fn example_2() {
    let mut x = String::from("abcdefg");

    // Ownership of the string gets transferred from x to consume_a_string
    //
    // consume_a_string(x);  // X X X X X X X X X X X X X X X X X X X X X X X X

    // Some alternatives:
    borrow_without_taking_ownership(&x);

    x = take_ownership_but_return(x);

    consume_a_copy(x.clone());

    // Since we didn't return anything from consume_a_string,
    // the string that x _used_ to own is deallocated. X can't get to the string
    // again!
    println!("{}", x);
}

fn consume_a_string(s: String) {
    // om nom nom nom!
    drop(s);
}

fn borrow_without_taking_ownership(s: &String) {
    println!("Nice string you've got there: {}. Have it back.", s);
}

fn take_ownership_but_return(s: String) -> String {
    s
}

fn consume_a_copy(s: String) {
    consume_a_string(s);
}

/*
*******************************************************************************
*                                 EXAMPLE 3                                   *
*******************************************************************************
 */
fn example_3() {
    let mut s = String::from("Hello");

    let s_ref = &s;
    
    // As long as the immutable reference s_ref
    // is live, we can't perform any sort of mutable borrow
    //
    // s.push_str(" World"); // X X X X X X X X X X X X X X X X X X X X X X X X

    // Trying to mutate through an immutable reference is also
    // something you can't do
    //
    // s_ref.push_str("this isn't going to work, duder."); // X X X X X X X X X

    // Also now that there's an immutable reference to s that's live
    // we can't create a mutable reference here
    //
    // let s_mut_ref: &mut String = &mut s;  // X X X X X X X X X X X X X X X X 
    
    println!("{}", s_ref);
    
    // but now that s_ref is no longer live
    // the implicit borrow in push_str is legal
    s.push_str(" World.");

    println!("{}", s);

    // Again, the death of the immutable reference s_ref
    // allows us to create some mutable references now
    let s_clone: String = s.clone();
    let s_mut_ref: &mut String = &mut s;

    // Now see what broke? We can't perform any immutable borrows
    // as long as s_mut_ref is live. Since the print macro here
    // implicitly borrows s immutably, we're SOL until s_mut_ref dies.
    // If you _really_ wanted to print the string, you need to either
    // do it through the reference -OR- through a clone that was made
    // prior to the mutable borrow.
    //
    // println!("This won't work: {}", s); // X X X X X X X X X X X X X X X X X

    println!("Printing through s_mut_ref: {}", s_mut_ref);
    println!("Printing through a clone: {}", s_clone);
    drop(s_clone);

    s_mut_ref.push_str(" How you doin'?");

    println!("{}", s);
}

/*
*******************************************************************************
*                                 EXAMPLE 1                                   *
*******************************************************************************
 */
fn example_4() {
    let mut v = vec![1,2,3];

    let n = &v[0]; // The reference here dies instantly

    // v.push(4);  // X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

    // This is dorked! Either `n` is dead and you're trying to access freed mem,
    // OR `n` is alive and you can't perform the push because of competing borrow types.
    //
    println!("{n}");  // X X X X X X X X X X X X X X X X X X X X X X X X X

    // Safe to push here.
    v.push(4);

    for (i, &item) in v.iter().enumerate() {
        println!("v[{i}] = {item}");
    }
}