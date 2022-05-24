
fn main() {
    let number_list = vec![];
    let largest = get_vec_largest(&number_list);
    println!("{}", largest);
}

fn get_vec_largest(number_list :&[i32])-> i32{
    if number_list.len()==0 {
        return 0;
    }
    let mut largest = number_list[0];
    for &num in number_list {
        if num>largest {
            largest = num;
        }
    }
    //上面的引用方式，或者下面的解引用方式
    // for num in number_list {
    //     if *num>largest {
    //         largest = *num;
    //     }
    // }
    largest
}