
fn main() {
    let number_list = vec![];
    let largest:i32 = get_vec_largest(&number_list);
    println!("{}", largest);
}

fn get_vec_largest<T: std::cmp::PartialOrd>(number_list :&[T])-> T{
    let mut largest = number_list[0];
    for &num in number_list {
        if num>largest {
            largest = num;
        }
    }
    largest
}