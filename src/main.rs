use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // 闭包处理Result
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() ==ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Error creating file: {:?}",error);
            })
        }else {
            panic!("Error notfind file: {:?}",error);
        }
    });

}