#[derive(Debug)]
pub enum IpAddrKind {
    V4(String),
    V6(String),
}

pub fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move{x: i32, y: i32}, // Data associated with enum variant
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state of the coin is: {:?}", state);
            25
        },
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn test() {
    // Enums and match examples
    route(IpAddrKind::V4(String::from("127.0.0.1")));
    route(IpAddrKind::V6(String::from("::1")));

    let m = Message::Write(String::from("hello"));
    m.call();

    println!("the coin is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}