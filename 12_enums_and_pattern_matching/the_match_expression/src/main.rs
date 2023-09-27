fn main() {
    /*
     ****************************************************************************
     *                  THE match CONTROL FLOW CONSTRUCT                        *
     ****************************************************************************
     *
     * The match express is a control flow construct that allows the developer
     * to compare a value agains a series of patterns and then execute code
     * based on which pattern matches. Patterns can be literal values, variable
     * names, wildcards, and a lot of other things. Patterns are discussed in
     * their own module.
     *
     * An analogy for match is that it's like a coin-sorting machine: Coins
     * slide down a track and as soon as a coin finds a slot that it can fit
     * in it falls through. Match works the same way: the first pattern that
     * works executes.
     */

    coin_example();
    option_matching_example();
}

/*
********************************************************************************
*                           COIN EXAMPLES                                      *
********************************************************************************
*/
fn coin_example() {
    let p: Coin = Coin::Penny;
    let n: Coin = Coin::Nickle;
    let d: Coin = Coin::Dime;
    let q: Coin = Coin::Quarter(UsState::Mordor);

    println!("{}", value_in_cents(p));
    println!("{}", value_in_cents(n));
    println!("{}", value_in_cents(d));
    println!("{}", value_in_cents(q));
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

#[derive(Debug)] //We use this so we can inspect the value later
enum UsState {
    Alabama,
    Alaska,
    Massachusetts,
    Texas,
    Mordor,
}

/*
********************************************************************************
*                       OPTION MATCHING EXAMPLES                               *
********************************************************************************
*/
fn option_matching_example() {
    let five = Some(5);
    let six = plus_one(five);
    let nada = plus_one(None);

    let got_five = unwrap_num(&five);
    let got_six = unwrap_num(&six);
    let got_nada = unwrap_num(&nada);

    println!("{}", got_five);
    println!("{}", got_six);
    println!("{}", got_nada);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn unwrap_num(thingy: &Option<i32>) -> i32 {
    match thingy {
        Some(i) => {
            println!("Got {} from a thingy", i);
            *i
        }
        None => {
            println!("Got nothing!");
            0
        }
    }
}
