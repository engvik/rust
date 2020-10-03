#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Ipv4Addr {}

#[derive(Debug)]
struct Ipv6Addr {}

#[derive(Debug)]
enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home);
    println!("{:?}", loopback);

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("{:?}", home2);
    println!("{:?}", loopback2);

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    println!("{:?}", home3);
    println!("{:?}", loopback3);

    let home4 = IpAddr4::V4(Ipv4Addr {});
    let loopback4 = IpAddr4::V6(Ipv6Addr {});

    println!("{:?}", home4);
    println!("{:?}", loopback4);

    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Quit;
    let m3 = Message::Move { x: 5, y: -2 };
    let m4 = Message::ChangeColor(0, 0, 0);

    m1.call();
    m2.call();
    m3.call();
    m4.call();

    let some_number = Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_string, absent_number);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
