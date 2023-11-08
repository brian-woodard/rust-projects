use std::collections::HashMap;

fn main() {
   let mut scores = HashMap::new();

   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("Yellow"), 50);

   let team_name = String::from("Blue");
   let mut score = scores.get(&team_name).copied().unwrap_or(0);

   println!("score {}", score);

   for (key, value) in &scores {
      println!("{key}: {value}");
   }

   // overwrite value with insert again
   scores.insert(String::from("Blue"), 25);
   score = scores.get("Blue").copied().unwrap_or(0);
   println!("score {}", score);

   // add value only if it doesn't exist
   scores.entry(String::from("Blue")).or_insert(50);
   println!("{:?}", scores);

   // update values in hash map based on previous value
   let text = "hello world wonderful world";

   let mut map = HashMap::new();

   for word in text.split_whitespace() {
      println!("{word}");
      let count = map.entry(word).or_insert(0);
      *count += 1;
   }

   println!("{:?}", map);
}
