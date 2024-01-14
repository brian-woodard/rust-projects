
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {result}");
}

// this function won't compile without lifetimes specified
//fn longest(s1: &str, s2: &str) -> &str {
//    if s1.len() > s2.len() {
//        s1
//    } else {
//        s2
//    }
//}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
