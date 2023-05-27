
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Florida,
}

enum Coin {
    Nickel,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State: {:?}", state);
            25
        },
    }
}

fn main() {
    println!("Hello, world!");

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    let c = Coin::Nickel;
    println!("coin in cents: {}", value_in_cents(c));

    let q = Coin::Quarter(UsState::Florida);
    println!("coin in cents: {}", value_in_cents(q));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six {:?} none {:?}", six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
