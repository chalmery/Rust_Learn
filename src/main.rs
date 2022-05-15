use std::collections::HashMap;

fn main() {
    let text = "Hello world This is Rust world";
    let mut map = HashMap::new();

    //根据空格切分
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1; //对值解引用然后+1
    }
    println!("{:#?}", map);

    //HashMap默认使用了安全的Hash算法，能抵御DDOS等攻击，如果觉得太慢可以切换Hasher

}