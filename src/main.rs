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
}