
struct User{
    name : String,
    email : String,
    sign_in_count : u64,
    archive : bool,
}

fn build_user(name:String,email:String) ->User{
    User{
        // email : email,
        // name : name,
        email,
        name,
        sign_in_count : 1,
        archive : false,
    };
}

fn fun()-> User{
    User{
        email : String::from("ycc@123.com"),
        name : String::from("ycc"),
        sign_in_count : 1,
        archive : false,
    };
}