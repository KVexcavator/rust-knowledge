// 6.1. Хранение данных и варианта IpAddrKind IP-адресас использованием структуры

enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1")
};

// таже самая идея но корояе

enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("123.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

// или например, у IP-адресов версии четыре всегда будет четыре числовых компонента

enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// или применить типы

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr {
  V4(Ipv4Addr),
  V5(Ipv6Addr),
}
