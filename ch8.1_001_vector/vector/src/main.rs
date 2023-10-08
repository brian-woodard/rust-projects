fn main() {
   //let v: Vec<i32> = Vec::new();
   //let v = vec![1, 2, 3];

   // let mut v = Vec::new();

   // v.push(5);
   // v.push(6);
   // v.push(7);
   // v.push(8);

   let v = vec![1, 2, 3, 4, 5];

   let third: &i32 = &v[2];
   println!("The third element is {third}");

   let seventh: Option<&i32> = v.get(6);
   match seventh {
      Some(seventh) => println!("The 7th element is {seventh}"),
      None => println!("There is no 7th element."),
   }

   // for i in &v {
   //    println!("Value in vec {i}");
   // }

   enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
   }

   let mut v = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
   ];

   v.push(SpreadsheetCell::Int(5));

   println!("vector size {}", v.len());

   for i in &v {
      match i {
         SpreadsheetCell::Int(value) => println!("Int val {value}"),
         SpreadsheetCell::Text(value) => println!("Text val {value}"),
         SpreadsheetCell::Float(value) => println!("Float val {value}"),
      }
   }
}