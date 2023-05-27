fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(str: String) -> (String, usize) {
    let len = str.len();

    (str, len)
}
