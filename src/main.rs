fn main() {
    // let v : Vec<i32> = Vec::new();
    //指定初始值的方式创建,这样v不需要指定类型，因为可以推断出来
    // let v = vec![1,2,3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("{:?}",v);
}