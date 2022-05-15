use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    // 处理Result
    let f=  match f {
        Ok(file) => file,
        Err(error) => panic!("Error open File {:?}",error),
    };



}