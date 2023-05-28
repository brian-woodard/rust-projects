
#[derive(Debug)]
enum UsState {
    Florida,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn main() {
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("Max configured to {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("Max configured to {}", max);
    }

    let coin = Coin::Penny;
    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println!("State of quarter: {:?}", state),
        _ => count += 1,
    }

    let coin2 = Coin::Quarter(UsState::Florida);

    if let Coin::Quarter(state) = coin2 {
        println!("State of quarter: {:?}", state);
    } else {
        count += 1;
    }

    println!("Other coins: {}", count);

    println!("Hello, world!");
}
