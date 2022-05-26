use rust_learn::{News, Summary,Tweet};

fn main() {
    let n = News {
        headline: String::from("he"),
        location: String::from("lo"),
        author: String::from("ycc"),
    };

    println!("{}", n.summarize());

    let t = Tweet {};

    println!("{}", t.summarize());
}
