enum IpAddrKind{
    V4,
    V6
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum Message{
    Quit,
    Move{x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn addresses(){
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
