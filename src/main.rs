
struct User{
    name : String,
    email : String,
    sign_in_count : u64,
    archive : bool,
}

fn main(){
    let user1 = User{
        email : String::from("ycc@123.com"),
        name : String::from("ycc"),
        sign_in_count : 1,
        archive : false,
    };

    let user2 = User{
        email : String::from("233@gmail.com"),
        ..user1
    };
}