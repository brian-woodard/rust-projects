fn main() {
   let mut s = String::new();
   let x = 35;

   println!("{s}");

   let data = "initial content";

   s = data.to_string();
   println!("{s}");

   s = String::from(data);
   println!("{s}");

   s = x.to_string();
   println!("{s}");

   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{s}");

   let mut s = String::from("lo");
   s.push('l');
   println!("{s}");

   let s1 = String::from("Hello, ");
   let s2 = String::from("World!");
   let s3 = s1 + &s2; // note s1 no longer valid after this addition
   println!("{s3}");

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");
   let s = format!("{s1}-{s2}-{s3}");
   println!("{s}");

   //let s = String::from("hello");
   // next line won't compile, Strings can't be indexed
   //let h = &s[0];

   let hello = "Здравствуйте";
   println!("{hello}");
   for c in hello.chars() {
      println!("{c}");
   }
   for b in hello.bytes() {
      println!("{b}");
   }

}
