#[derive(Debug)]
enum IpVersion {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

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
    let v4 = IpVersion::V4;
    println!("Hello, {:?}", v4);
    println!("Hello, {:?}", IpVersion::V6);

    let v4_address = IpAddress::V4(0, 0, 0, 0);
    println!("Hello, {:?}", v4_address);

    let v6_address = IpAddress::V6(String::from("::1"));
    println!("Hello, {:?}", v6_address);

    let quarter = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(us_state) = quarter {
        println!("State quarter from {:?}!", us_state);
    }

    let five = Some(5);
    let six = plus_one(five);
    let mut none = None;
    none = plus_one(none);

    println!("Number, {:?}", five);
    println!("Number, {:?}", six);
    println!("Number, {:?}", none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State {:?} quarter!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
