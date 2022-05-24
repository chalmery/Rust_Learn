
struct Point<T> {
    x: T,
    y: T,
}
//表示在类型T上实现方法
impl<T> Point<T> {
    //相当于get，获取x的引用
    fn x(&self) -> &T {
        &self.x
    }
}
//可以根据具体的类型实现方法,这样，只有i32的类型才你能调用这个
impl Point<i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point{
        x : 100,
        y : 100,
    };

    println!("{}", p.x());

}
