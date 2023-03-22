struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("User1"),
        email: String::from("dummy@email.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.username = String::from("User1new");

    let user2 = build_user(String::from("dummy2@email.com"), String::from("user2"));

    let user3 = User {
        username: String::from("User3"),
        email: String::from("dumm3@email.com"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
