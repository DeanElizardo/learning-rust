fn main() {
    /*
    ****************************************************************************
    *             THE Option ENUM AND ITS ADVANTAGES OVER null                 *
    ****************************************************************************
    *
    * Option is an enum defined by the standard library, and it encodes the
    * concept that a value could be something or nothing.
    *
    * As an example, if you request the first value in a non-empty list you
    * get something. If you request the first value of an empty list, you get
    * nothing. This is different from have a null type, in that you can't
    * work with a reference that is nothing (does not exist), whereas in non-
    * Rust languages you _can_ refer to a null type and try to work with it
    * as non-null, often with disasterous consequences.
    *
    * Rust uses
    > --------------------------------------------------------------------------
    > enum Option<T> {
    >   None,
    >   Some(T),
    > }
    > --------------------------------------------------------------------------
    *
    * The Option<T> is a part of the prelude and doesn't need to be brought
    * into scope explicitly. The Some and None variants can also be used
    * without the Option:: prefix.
    *
    * The <T> syntax is a feature of rust called a generic type parameter,
    * generics are discussed in their own module.
    */

    // Option<T> is it's own type, so there are limitations to what we
    // can try to do between T and Some(T):
    let x: i8 = 5;
    let y: Option<i8> = Some(6);

    // Watch this break against the compiler!
    //
    // let sum = x + y;

    // The error that gets returned from x + y indicates that Rust doesn't
    // know how to add between an i8 and an Option<i8>. The problem here is
    // that Option may actually be Option::None. Look what happens when
    // you try to add with None
    //
    // let broken = 1 + None;

    // See the similarity? To use an Option<T>, we want to have code that
    // handles each variant. You'll need code that will only run when you have a
    // Some(T) value, and this code gets to use the inner T. You'll also want
    // code that will run only if we have a None value.
    // ...
    // How? MATCH. Check out the module on the match expression.
}
