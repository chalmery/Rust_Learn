#[derive(Debug)]
enum IP_ADDRESS{
    V4(String,u32),
    V6,
}

fn main() {
    println!("{:?}",IP_ADDRESS::V6)
}
