fn main(){
    let s1 = String::from("Hello");
    let len = get_lenth(&s1);

    println!("{} length is {}",s1,len);
}

fn get_lenth(s: &String) -> usize {
    s.len()
}