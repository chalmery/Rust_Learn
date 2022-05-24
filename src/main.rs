mod food;//声明模块
use food::cake::Cake;
fn main() {
    let cake = Cake{
        size :100
    };
    println!("#{:?}",cake);
}