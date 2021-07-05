enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    _kind: IpAddrKind, //unimportant for example
    address: String,
}

// You can put almost any kind of data within an enum
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

// We can implement traites like ToString for an enum
impl ToString for IpAddress {
    fn to_string(&self) -> String {
        let string: String = match self {
            IpAddress::V4(x, y, z, a) => format!("{}.{}.{}.{}", x, y, z, a),
            IpAddress::V6(address) => address.to_string(),
        };
        format!("The ip adress is {}", string)
    }
}

enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    _Quarter,
}

impl Coin {
    fn value(&self ) -> u8 {
        match self {
            Coin::_Penny => 1,
            Coin::_Nickel => 5,
            Coin::_Dime => 10,
            Coin::_Quarter => 25,
        }
    }
}

/*
 Use of option for no values or a value.
 Shows that matches are exhaustive and all cases must be handled
*/
fn plusOne(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
// We can implement this with either a struct of simply implementing it in an enum

pub fn enums() {
    let home = IpAddr {
        _kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        _kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{}", home.address);
    println!("{}", loopback.address);

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    println!("{}", home.to_string());
    println!("{}", loopback.to_string());

    let coin:Coin = Coin::_Nickel;
    println!("The coin is worth {}", coin.value());

    let five = Some(5);
    let six = plusOne(five);
    let none = plusOne(None);
    println!("{:?} {:?}", six, none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // A placeholder if we do not care about any other value
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // An if let allows us to to perform a quick match like above easier
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Dont care for that value");
    }
}