fn main() {
    /*
     ****************************************************************************
     *                              ENUMS                                       *
     ****************************************************************************
     *
     * Structs make it possible to group related fields and data (see Rectangle
     * in chapter 11).
     *
     * Enums give you the ability to insist that a value must be one of a
     * possible set of values. For example, we may want to say that `Rectangle`
     * is one of a set of possible shapes that also includes `Circle`, and
     * `Triangle`. We can do this in Rust by encoding the possible values
     * with an Enum.
     *
     * As a practical example, consider working with IPv4 and IPv6 addresses
     */

    // We create intances of IpAddrType variants like so
    let four: IpAddrType = IpAddrType::V4(Ipv4Addr::new(127, 0, 0, 1));
    let six: IpAddrType = IpAddrType::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    // Since route() takes the enum type, there's not problems here:
    route(&four);
    route(&six);

    println!("Dropped the reference from {:?}", four);
    println!("Dropped the reference from {:?}", six);
}

// We can express the two types of IP address with an enum
//
// #[derive(Debug)]
// enum IpAddrType {
//     V4(String),
//     V6(String),
// }

// Even better, we can encode different types of data for each variant
//
// #[derive(Debug)]
// enum IpAddrType {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// How's this for a slice of fried gold: You can put STRUCTS into enums!
#[derive(Debug)]
enum IpAddrType {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// We can define a function that takes any of the variants of an enum
fn route(ip_addr: &IpAddrType) {
    let route_string = match ip_addr {
        IpAddrType::V4(addr) => &addr.full_string,
        IpAddrType::V6(addr) => &addr.full_string,
    };

    println!("Routing through {}", route_string);
}

#[derive(Debug)]
struct Ipv4Addr {
    first: u8,
    second: u8,
    third: u8,
    fourth: u8,
    full_numeric: (u8, u8, u8, u8),
    full_string: String,
}

impl Ipv4Addr {
    fn new(one: u8, two: u8, three: u8, four: u8) -> Self {
        Ipv4Addr {
            first: one,
            second: two,
            third: three,
            fourth: four,
            full_numeric: (one, two, three, four),
            full_string: format!("{}.{}.{}.{}", one, two, three, four),
        }
    }
}

#[derive(Debug)]
struct Ipv6Addr {
    first: u16,
    second: u16,
    third: u16,
    fourth: u16,
    fifth: u16,
    sixth: u16,
    seventh: u16,
    eighth: u16,
    full_numeric: (u16, u16, u16, u16, u16, u16, u16, u16),
    full_string: String,
}

impl Ipv6Addr {
    fn new(
        one: u16,
        two: u16,
        three: u16,
        four: u16,
        five: u16,
        six: u16,
        seven: u16,
        eight: u16,
    ) -> Self {
        Ipv6Addr {
            first: one,
            second: two,
            third: three,
            fourth: four,
            fifth: five,
            sixth: six,
            seventh: seven,
            eighth: eight,
            full_numeric: (one, two, three, four, five, six, seven, eight),
            full_string: format!(
                "{}:{}:{}:{}:{}:{}:{}:{}",
                one, two, three, four, five, six, seven, eight
            ),
        }
    }
}
