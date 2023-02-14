fn main() {
    let email = String::from("a@gmail.com");
    let username = String::from("a");
    let user1 = build_user(email, username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
