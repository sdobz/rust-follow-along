/*
// naw
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

enum IpAddr {
    V4(i8, i8, i8, i8),
    V6(String),
}

impl IpAddr {
    fn distinguish(&self) {
        match self {
            IpAddr::V4(v0, v1, v2, v3) => {
                println!("V4 {}.{}.{}.{}", v0, v1, v2, v3);
            },
            IpAddr::V6(addr) => {
                println!("V6 {}", addr);
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                Some(state) => {
                    println!("Quarter from {:?}!", state);
                    26
                }
                None => 25,
            }
        }
    }
}

fn optional_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None
    }
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    four.distinguish();
    six.distinguish();

    let some_number = Some(5);
    let some_number = some_number.expect("Thisll unbork it");
    println!("Now some_number is safe: {}", some_number);

    let _absent_number: Option<i32> = None;

    let coin = Coin::Quarter(None);
    let cents = value_in_cents(&coin);
    println!("A {:?} is worth {}", coin, cents);

    let coin = Coin::Quarter(Some(UsState::Alabama));
    let cents = value_in_cents(&coin);
    println!("A {:?} is worth {}", coin, cents);

    let optional_spooky_six = optional_plus_one(Some(6));
    let none_plus_one = optional_plus_one(None);

    println!("Optional spooky six {:?} none+1 {:?}", optional_spooky_six, none_plus_one);

    if let Some(6) = optional_spooky_six {
        println!("The six isn't that spooky");
    }
    if let Some(7) = optional_spooky_six {
        println!("The six really is spooky");
    }

    if let Coin::Quarter(state) = coin {
        println!("If let the coin is from {:?}", state);
    } else {
        println!("No state coin");
    }
}
