fn main() {
    let mut v = vec![1,2,3,4];
    //获取值的引用,来访问值，这样不太好，可能越界
    let third  = &v[2];
    v.push(5);
    println!("{}", third);
    //因为在push之后，thired指向的堆地址可能是老的
}