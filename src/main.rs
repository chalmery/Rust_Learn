//复合类型
fn main() {
    //元组 tuple
    //let tup: (i32, f64, u8) = (500, 6.4, 1);

    //println!("{},{},{}", tup.0, tup.1, tup.2);

    // 解构 tuple
    //let (x, y, z) = tup;

    //println!("{},{},{}", x, y, z);

    //数组，数组是固定大小的，因此会存放在栈内存因此如果想存放在栈内存则使用数组
    //let a = [1, 2, 3, 4, 5];

    // vector

    //语句表达式和函数
    //函数的返回值默认是最后一行的表达式
    //如果需要提前返回则需要使用return

    //for循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        print!("{} ", element);
    }
    println!(" ");
    for element in a.iter() {
        print!("{} ", element);
    }

    //range
    for number in 1..4 {
        print!("{} ", number);
    }
    println!(" ");
    //反转range
    for number in (1..4).rev() {
        print!("{} ", number)
    }
}
