
struct User{
    name : String,
    email : String,
    sign_in_count : u64,
    archive : bool,
}

fn main(){
    let trple = (1,"a",3);
    let (a,b,c) = trple;
    println!("{}",a);

    // 元组结构体
    let white = Color(0,0,0);
    let orange = Color(2,5,1);
}

struct Color(i32,i32,i32);