#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.length * self.width
    }
}

impl Rectangle {
    fn square(size:u32) -> Rectangle {
        Rectangle{
            length :size,
            width :size,
        }
    }
}

fn main() {
    let r = Rectangle{
        length:10,
        width:10,
    };
    let area = r.get_area();
    println!("{}",area);

    let squ = Rectangle::square(10);
    println!("{:#?}",squ)
}
