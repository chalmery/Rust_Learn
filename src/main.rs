fn main() {
    let mut  v = vec![1,2,3,4];
    // 遍历
    for num in &v {
        print!("{} ", num);
    }
    println!(" ");
    // 遍历并修改，使用解引用的方式，表示：值的修改
    for num in &mut v {
        *num += 10;
    }
    for num in &v {
        print!("{} ", num);
    }
}