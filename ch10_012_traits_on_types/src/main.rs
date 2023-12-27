
pub trait Summary {
   fn summarize(&self) -> String;
}

pub struct NewsArticle {
   pub headline: String,
   pub author: String,
   pub location: String,
}

impl Summary for NewsArticle {
   fn summarize(&self) -> String {
      format!("{}, by {} ({})",
              self.headline,
              self.author,
              self.location)
   }
}

pub struct Tweet {
   pub username: String,
   pub content: String,
   pub reply: bool,
   pub retweet: bool,
}

impl Summary for Tweet {
   fn summarize(&self) -> String {
      format!("{}: {}",
              self.username,
              self.content)
   }
}

fn main() {
   let article: NewsArticle = NewsArticle {
      headline: String::from("Trump is crazy!"),
      author: String::from("Brian Woodard"),
      location: String::from("New York")
   };

   let tweet: Tweet = Tweet {
      username: String::from("Boopitydoop"),
      content: String::from("Worst tweet ever!"),
      reply: false,
      retweet: false,
   };

   println!("article summary: {}", article.summarize());
   println!("tweet summary: {}", tweet.summarize());
}