#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    println!("test {}",value_in_cents(coin));
    let five = Some(5);
    let six = plus(five);
    let none = plus(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}

fn plus(x:Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn value_in_cents(coin:Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        }
    }
}
