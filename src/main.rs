#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
fn get_area(r : &Rectangle) -> u32 {
    r.length * r.width
}
fn main() {
    let r = Rectangle{
        length:10,
        width:10,
    };
    let area = get_area(&r);
    println!("{}",area);

    println!("{:#?}",r)
}
