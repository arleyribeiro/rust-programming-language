struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = build_user(String::from("a@a.com"), String::from("a a"));
    show_user(&user);
    user.active = false;
    println!("Active is {}", user.active);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };
    show_user(&user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn show_user(user: &User) {
    println!("Username is {}", user.username);
    println!("E-mail is {}", user.email);
    println!("Active is {}", user.active);
    println!("Sign in count is {}", user.sign_in_count);
}
