struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername1"),
        active: true,
        sign_in_count: 1,
    };

    // user1.email = String::from("anotheremail@example.com");

    build_user(String::from("scott@email.com"), String::from("scott"));
}
