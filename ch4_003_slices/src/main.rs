fn main() {
    let s = String::from("hello world");

    let slice = find_first_word(&s);

    println!("slice {} in {}", slice, s);
}

fn find_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
