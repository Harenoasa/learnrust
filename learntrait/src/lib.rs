use std::fmt::{Debug, Display};

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
     fn summarize(&self) -> String{
         format!("{}, by {} ({})",self.headline,self.author,self.location)
     }
}

pub struct Tweet {
    pub username: String,
    pub contect: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String{
        format!("{}: {}",self.username,self.contect)
    }
}

// pub fn notify1(item1: impl Summary + Display) {
//     println!("Breaking news! {}", item1.summarize());
// }
// pub fn notify<T: Summary + Display>(item:& T) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T, U>(a: T,b: U)
where T : Summary + Display,
      U : Clone + Debug,
{
    println!("Breaking news! {}", a.summarize());
}
pub fn notify1(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguin win the Stanley Cup Championship!"),
        content: String::from(
            "The Pittsburgh Penguins once agin are th best hockey team in the NHL.",
        ),
        author: String::from("Subin"),
        location: String::from("Pittsburgh, PA, USA"),
    }
}