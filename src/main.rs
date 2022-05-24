
fn main() {
}
//rust自带的枚举类型
enum Option<T> {
    None,
    Some(T),
}
// 多个泛型
enum Result<T, E> {
   Ok(T),
   Err(E),
}