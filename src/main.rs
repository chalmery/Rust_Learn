use rust_learn::{News, Summary};

fn main() {
    let n = News {
        headline: String::from("he"),
        location: String::from("lo"),
    };

    println!("{}", n.summarize());
}
