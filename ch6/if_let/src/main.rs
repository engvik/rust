#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    _Penny,
    _Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    println!("coin: {:?}", coin);

    match coin {
        Coin::Quarter(state) => println!("State quarter from: {:?}", state),
        _ => count += 1,
    }

    println!("count: {}", count);

    let coin2 = Coin::Dime;

    println!("coin: {:?}", coin2);

    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from: {:?}", state);
    } else {
        count += 1;
    }

    println!("count: {}", count);
}
