struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = build_user(String::from("usuario"), String::from("email@email.com"));
    println!("Hello, {}", user1.username);
}
