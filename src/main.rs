mod l_trait;
use l_trait::{News, Summary,Tweet};

fn main() {
    
    let num = vec![29,31,43,66,243];
    let result = largest(&num);


    let chr = vec!['a','b','c'];
    let car_res = largest(&chr);



    let str = vec![String::from("str")];
    let str_res = largest(&str);

}

fn largest<T: PartialOrd+Clone>(number_list :&[T])-> T{
    let mut largest = number_list[0].clone();
    for num in number_list {
        //只有实现了std::cmp::PartialOrd的接口的类型才能使用><来比较
        if num >  &largest {
            largest = num.clone();
        }
    }
    largest
}