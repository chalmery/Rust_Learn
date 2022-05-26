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

//因为存在默认实现，这里不重写表示使用默认的逻辑
impl Summary for News {
    fn summarize_author(&self) -> String {
        format!("@{}",self.author)
    }
}

