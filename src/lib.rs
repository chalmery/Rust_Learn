pub trait Summary {
    //接口方法的默认实现
    fn summarize(&self) -> String{
        format!("Read more....and author : {}",self.summarize_author())
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
        format!("@{}",self.author)
    }
}


pub struct Tweet{
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        String::from("Tweet....")
    }
}
// 以trait为入参传递
pub fn notify(item: impl Summary){
    print!("news!...{}",item.summarize())
}


