use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
   let greeting_file_result = File::open("hello.txt");

   println!("{:?}", greeting_file_result);

   // handle error using match
   // let greeting_file = match greeting_file_result {
   //    Ok(file) => file,
   //    Err(error) => panic!("Problem opening the file: {:?}", error),
   // };

   // handle errors using match, perform different actions on different errors
   // let greeting_file = match greeting_file_result {
   //    Ok(file) => file,
   //    Err(error) => match error.kind() {
   //       ErrorKind::NotFound => match File::create("hello.txt") {
   //          Ok(file) => file,
   //          Err(error) => panic!("Problem creating the file: {:?}", error),
   //       },
   //       other_error => panic!("Problem other error opening file: {:?}", other_error),
   //    }
   // };

   // alternative to handling errors using unwrap_or_else
   // let greeting_file = greeting_file_result.unwrap_or_else(|error| {
   //    if error.kind() == ErrorKind::NotFound {
   //       File::create("hello.txt").unwrap_or_else(|error| {
   //          panic!("Problem creating the file: {:?}", error);
   //       })
   //    } else {
   //       panic!("Problem other error opening file: {:?}", error);
   //    }
   // });

   // using unwrap returns the Ok, or calls panic!
   //let greeting_file = File::open("hello.txt").unwrap();

   // using expect returns the Ok, or panics with the provided message
   //let greeting_file = File::open("hello.txt").expect("File hello.txt must exist!");

   // propagating errors
   let username_result = read_username_from_file();
   println!("username result: {:?}", username_result);

   let last_char = last_char_of_first_line("line1\nline2\nline3");
   println!("last char: {:?}", last_char);
}

// long error propagation
// fn read_username_from_file() -> Result<String, io::Error> {
//    let username_file_result = File::open("hello.txt");

//    let mut username_file = match(username_file_result) {
//       Ok(file) => file,
//       Err(error) => return Err(error),
//    };

//    let mut username = String::new();

//    match username_file.read_to_string(&mut username) {
//       Ok(_) => Ok(username),
//       Err(error) => Err(error),
//    }
// }

// short error propagation using the ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//    let mut username_file = File::open("hello.txt")?;
//    let mut username = String::new();
//    username_file.read_to_string(&mut username)?;
//    Ok(username)
// }

// chaining method calls using the ? operator
fn read_username_from_file() -> Result<String, io::Error> {
   let mut username = String::new();
   File::open("hello.txt")?.read_to_string(&mut username)?;
   Ok(username)
}

// example of using ? operator on an Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
   text.lines().next()?.chars().last()
}