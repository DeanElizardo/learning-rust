/*
What if I want to read some string twice, like in the example below?

Rust has a mechanism for passing pointers around without changing ownership: References
 */
fn main() {
    /*
    ***************************************************************************
    *                     REFERENCES (A.K.A. BORROWING)                       *
    ***************************************************************************
    */
    let hello: String = String::from("Hello ");
    let world: String = String::from("world!");
    
    // greet(hello, world); // will move both variables to greet()'s arguments
    
    greet(&hello, &world); // we pass references, which give access to the value and don't transfer ownership
    
    println!("{}{}", hello, world);
    
    /*
    ***************************************************************************
    *                      DEREFERENCING A POINTER                            *
    ***************************************************************************
    */
    let mut x: Box<i32> = Box::new(1);

    // *x reads the heap value, so a = 1
    let a:i32 = *x;
    
    // we modify the value on the heap; does not change a
    *x += 1;
    
    println!("a = {a}\nb = {}", *x);
    
    // r1 contains a reference to a Box with an i32 value. r1 does not own the box.
    let r1: &Box<i32> = &x;
    
    // To get the value in the box through r1, we need to double de-reference
    // 1) The first dereference gets us to the box
    // 2) The second dereference gets us _in_ the box to the value
    let b2: i32 = **r1;
    
    *x += 1;
    
    // Here we can create a reference directly to the value on the heap
    // so we only need to dereference once to get the value off the heap.
    let r2: &i32 = &*x;
    let c: i32 = *r2;
    
    println!("b2 = {}\nc = {}", b2, c);
    
    /*
    ***************************************************************************
    *                           IMPLICIT DEREFERENCE                          *
    ***************************************************************************
    */
    let x: Box<i32> = Box::new(-1);
    let x_abs1: i32 = i32::abs(*x);
    let x_abs2: i32 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = x.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = String::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);

    /*
    ***************************************************************************
    *                           BORROWING TROUBLE                             *
    ***************************************************************************
    */

    //
    // IMMUTABLE REFERENCES ///////////////////////////////////////////////////
    //
    // The compiler has a module called the Borrow Checker. The BC's job is to
    // make sure that mutable and immutable references do not occur concurrently
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];

    // This will not pass the borrow checker, because the push will cause the
    // original vector allocated on the heap to be copied to a new location and
    // then modified. The old pointer on the heap is gone, but `num` still point
    // to that region of memory!
    // v.push(4);

    // THIS is also not allowed, because while `num` is a reference that's in use
    // modifying the vector owned by `v` could confuse anything that relies on
    // `num
    // v[0] = 9;

    // If the push on line 88 is allowed, then when this print macro tries to
    // read the value out of the `num` reference it will be attempting to access
    // a deallocated portion of the heap. Not good!
    println!("The third element of v is {}", *num);

    // THIS push is okay, because we don't try to use `num` in any way AFTER it's
    // called. Uncomment the
    v.push(4);

    //
    // MUTABLE REFERENCES /////////////////////////////////////////////////////
    //
    let mut v: Vec<i32> = vec![4, 5, 6];
    let num: &mut i32 = &mut v[2];

    // println!("{}, {}, {}", v[0], v[1], v[2]);

    *num = 0;
    v[1] = 9;

    println!("{}, {}, {}", v[0], v[1], v[2]);
}

//
// THIS greet takes ownership of whatever you pass to it. Sometimes inconvenient.
//
// fn greet(word1: String, word2: String) {
//     println!("{}{}", word1, word2);
// }

// THIS greet takes a reference, but does not transfer ownership to its arguments.
fn greet(word1: &String, word2: &String) {
    println!("{}{}", word1, word2);
}