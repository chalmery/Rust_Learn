use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    let blue_str = String::from("Blue");
    let res = map.get(&blue_str);

    match res {
        Some(s) => println!("this is {}", s),
        None => println!("this is None"),
    }
    //遍历
    for (k, v) in &map {
        println!("key: {} , value: {}", k, v);
    }
    //查看key是否存在
    let e = map.entry(String::from("Green"));
    println!("{:?}", e);//返回了一个枚举
    // or instert表示，如果返回的值没有就插入
    let vel = e.or_insert(100);
    println!("{}", vel); // 方法返回值为当前value值的可变的引用 &mut

}