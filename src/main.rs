fn main(){
    let mut s1 = String::from("Hello");
    let len = get_lenth(&mut s1);

    println!("{} length is {}",s1,len);
}

fn get_lenth(s: &mut String) -> usize {
    s.push_str(", World");
    s.len()
}