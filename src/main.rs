use std::fs::File;
use std::io;
use std::io::{ Read};

fn main() {
    match read_username() {
        Ok(name) => println!("{}", name),
        Err(e) => println!("{}", e),
    }
}

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}