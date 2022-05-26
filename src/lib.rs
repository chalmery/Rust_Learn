pub trait Summary {
    //接口方法的默认实现
    fn summarize(&self) -> String{
        String::from("Read more....")
    }
}

pub struct News {
    pub headline: String,
    pub location: String,
}

impl Summary for News {
    
}

