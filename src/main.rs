fn main(){
    let mut s = String::from("Hello");
    {
        let m1 = &mut s;
    }

    let m2 = &mut s;
}
