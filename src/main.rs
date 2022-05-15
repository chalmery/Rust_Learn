fn main() {
    let v = vec![1,2,3,4];
    //获取值的引用,来访问值，这样不太好，可能越界
    let third  = &v[2];
    println!("{}", third);
    // get方法返回一个Option
    match v.get(5) {
        None => println!("Null"),
        Some(num) => println!("num is :{}",num),
    }
}