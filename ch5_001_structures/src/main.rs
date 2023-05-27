
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn main() {
    println!("Hello, world!");

    let user1: User;

    user1 = build_user(String::from("someuser"), String::from("someemail@dom.com"));

    println!("User {} ({})", user1.username, user1.email);

    let user2 = User {
        email: String::from("adifferentemail@somepl.com"),
        ..user1
    };

    println!("New user email {} active {} signin {}", user2.email, user2.active, user2.sign_in_count);
}
