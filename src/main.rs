fn main() {
    let mut s = String::from("hello");
    s.push_str("aaa");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let string = format!("{}-{}", s1, s2);
    println!("{}", string);
}