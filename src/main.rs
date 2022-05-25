
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct News {
    pub headline: String,
    pub location: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{},{}", self.location, self.headline)
    }
}

fn main() {
    let n = News {
        headline: String::from("he"),
        location: String::from("lo"),
    };

    println!("{}", n.summarize());
}
