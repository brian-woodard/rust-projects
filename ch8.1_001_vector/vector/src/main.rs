fn main() {
   //let v: Vec<i32> = Vec::new();
   //let v = vec![1, 2, 3];

   // let mut v = Vec::new();

   // v.push(5);
   // v.push(6);
   // v.push(7);
   // v.push(8);

   // let v = vec![1, 2, 3, 4, 5];

   // let third: &i32 = &v[2];
   // println!("The third element is {third}");

   // let third: Option<&i32> = v.get(3);
   // match third {
   //    Some(third) => println!("The third element is {third}"),
   //    None => println!("There is no third element."),
   // }

   // for i in &v {
   //    println!("Value in vec {i}");
   // }

   enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
   }

   let v = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
   ];

   println!("vector size {}", v.len());
}