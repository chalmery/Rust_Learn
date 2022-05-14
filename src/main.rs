fn main(){
    let  s = String::from("Hello World");
    let str = first_world(&s[..]);
    println!("{}",str);
}

fn first_world(s: &str) ->&str{
    let bytes = s.as_bytes();
    // iter会返回一个迭代器
    // enumerate会将迭代器包装为一个元组，1 索引， 2 元素的引用
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  &s[..i];
        }
    }
    &s[..]
}
