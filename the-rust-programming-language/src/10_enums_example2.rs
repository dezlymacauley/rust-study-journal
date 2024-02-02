// NOTE: Enums - Example 2

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);

}

fn route(ip_kind: IpAddrKind) {

}
