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

fn main() {
    let v4 = IpVersion::V4;
    println!("Hello, {:?}", v4);
    println!("Hello, {:?}", IpVersion::V6);

    let v4_address = IpAddress::V4(0, 0, 0, 0);
    println!("Hello, {:?}", v4_address);

    let v6_address = IpAddress::V6(String::from("::1"));
    println!("Hello, {:?}", v6_address);
}
