fn main(){
    let v = Some(1);
    match v {
        Some(3)=> println!("3"),
        _ => (),
    }
    //针对一种情况，可以使用if let
    if let Some(3) = v {
        println!("3")
    }

}