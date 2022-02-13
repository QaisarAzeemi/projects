// page # 203 of new book
// Trais bonds are use with Generic type parameters
#![allow(non_snake_case)]
#![allow(unused_variables)]
#[derive(Debug)]
struct Tweet {
    username: String,
    contant: String,
}

#[derive(Debug)]
struct NewsArticle {
    author: String,
    contant: String,
}

trait Summarizable {
    fn summary(&self) -> String;
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("@{} tweeted {}", self.username, self.contant)
    }
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("@{} wrote that {}", self.author, self.contant)
    }
}

fn notify <T:Summarizable> (item:T) -> String {  // this function is bound for trait summarizable
    format!("Breaking News: {} ", item.summary())
}

fn main() {
   
    let tweet_1 = Tweet {
        username : String::from("Qaisar"),
        contant : String::from("people are sensless... "),
    };

    let NewsArticle_1 = NewsArticle {
        author : String::from("PRC"),
        contant : String::from("COVID-19 is about to finish from this world... "),
    };

    println!("\n{}", notify(tweet_1));
    println!("\n{}\n", notify(NewsArticle_1));

}
