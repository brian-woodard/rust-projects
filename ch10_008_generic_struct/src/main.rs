struct Point<T> {
   x: T,
   y: T,
}

impl<T> Point<T> {
   fn x(&self) -> &T {
      &self.x
   }
}

impl Point<f32> {
   fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
   }
}

struct Point2<T, U> {
   x: T,
   y: U,
}

fn main() {
   let both_integer = Point { x: 5, y: 10 };
   let both_float = Point { x: 1.0, y: 4.0 };
   //let integer_and_float = Point { x: 5, y: 4.0 }; // compile error
   let integer_and_float = Point2 { x: 5, y: 4.0 };

   println!("both_integer.x = {}", both_integer.x());
   println!("both_float dist = {}", both_float.distance_from_origin());
}
