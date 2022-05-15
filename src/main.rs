use std::cmp::Ordering;
//Ordering是一个枚举类型
use std::io;

use rand::Rng;

//Rng是一个trait 可以认为是接口
//rust默认会倒入prelude这个模块
fn main() {
    println!("猜一个数字");
    //i32 u32 i64 都是整形
    let number = rand::thread_rng().gen_range(1..101);
    // loop表示无限循环
    loop {
        let mut guess = String::new();
        // io函数读取输  使用expect方法，表示如果result是err则执行
        io::stdin().read_line(&mut guess).expect("无法读取行");

        //将字符串转为整形 parse方法返回中为Result
        // 隐藏shadow 隐藏重名的旧变量，从18行开始，变量guess类型就变了
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            //_：此通配符表示不关心这个值
            Err(_) => continue,
        };
        // 改造的地方，加入校验
        if guess < 1 || guess > 100 {
            print!("this number will between 1 and 100");
            continue;
        }

        println!("你猜测的数是：{}", guess);

        //match表达式：可以让我们根据 枚举值选择不同的操作，就相当于if else
        //cmp就是compere，比较的意思
        match guess.cmp(&number) {
            //三个枚举分别表示小大等
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}