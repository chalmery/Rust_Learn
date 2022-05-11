fn main(){
    let mut s = String::from("Hello");
    let m1 = & s;
    let m2 = & s;
    let m3 = &mut s;

    println!("{} {} {}",m1,m2,m3);
}
