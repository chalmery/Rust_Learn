fn main(){
    let s = dangle();
}

fn dangle() -> &String {
    let str = String::from("hello");
    &str
}
