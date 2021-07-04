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
}