fn main(){
    let mut s = String::from("Hello World");
    let index = first_world(&s);

    s.clear();
    println!("{}",&index);
}

fn first_world(str : &String) ->usize{
    let bytes = str.as_bytes();
    // iter会返回一个迭代器
    // enumerate会将迭代器包装为一个元组，1 索引， 2 元素的引用
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  i;
        }        
    }
    str.len()
}
