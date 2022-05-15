fn main() {
    let x : i8 = 5;
    let y : Option<i8> = Some(5);

    let z = y.expect("i8");

    let sum = x + z;

    println!("{}",sum);
}
