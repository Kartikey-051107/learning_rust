#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Invalid,
}

fn value(coin: Coin) {
    match coin {
        Coin::Penny => println!("Penny -> 1 cent"),
        Coin::Nickel => println!("Nickel -> 5 cents"),
        Coin::Dime => println!("Dime -> 10 cents"),
        Coin::Quarter => println!("Quarter -> 25 cents"),
        Coin::Invalid => println!("Invalid Coin"),
    }
}

fn main() {
    value(Coin::Penny);
    value(Coin::Quarter);
    value(Coin::Invalid);
}