use std::fmt::{Debug, Display};

pub trait Summary {
    //接口方法的默认实现
    fn summarize(&self) -> String {
        format!("Read more....and author : {}", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct News {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for News {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        String::from("Tweet....")
    }
}
// // 以trait为入参传递
// pub fn notify(item: impl Summary){
//     print!("news!...{}",item.summarize())
// }

// 第二种方式
pub fn notify<T: Summary>(item: T) {
    print!("news!...{}", item.summarize())
}

pub fn notify1<T: Summary>(item1: T, item2: T) {
    print!("news!...{}", item1.summarize())
}

//限定入参必须实现多个接口
pub fn notify2(item: impl Summary + Display) {}

pub fn notify3<T: Summary + Display>(item: T) {}

//当限定语句太多的时候，方法就显得很臃肿
pub fn sc<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("sc....")
}
//使用where简化
pub fn sc2<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("sc....")
}

//trait作为返回值：
//tips: 必须只返回一个具体的实现，不能：if返回实现a ，else 返回实现2
pub fn ret() -> impl Summary {
    Tweet {}
}


// 使用 tarit bound 实现有条件的实现方法
struct Pair<T>{
    x:T,
    y:T,
}
//任何泛型都有此函数
impl <T> Pair<T> {
    fn new (x: T, y:T) -> Self{
        Self{x,y}
    }
}

//只有实现了这两个接口的泛型，才有此方法
impl <T:Display+PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x>=self.y{
            println!("x>y")
        }
    }
}
