use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // unwrap处理，如果是正常的就返回文件对象，如果是错误的，抛出错误信息
    // let f = File::open("hello.txt").unwrap();

    // exception，替换默认的错误信息
    let f = File::open("hello.txt").expect("无法打开文件");

}