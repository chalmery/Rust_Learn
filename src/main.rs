fn main(){
    let  s = String::from("Hello World");
    let h1 = &s[0..5];
    let w1 = &s[6..11];
    //语法糖
    let h2 = &s[..5];
    let w2 = &s[6..];
    let all = &s[..];

}

