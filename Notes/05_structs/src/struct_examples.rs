// Structs and enums are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile-time type checking.

#[derive(Debug)] // this line and "{:?}" are both used to be able to use the formatter provided by std::fmt::Display and thus let me use println!() for test_user.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("test"),
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test2@gmail.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("test3@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3_shorthand = User {
        email: String::from("test4@gmail.com"),
        ..user2
    };

    let test_user = build_user(String::from("test@gmail.com"), String::from("kirby"));

    println!("{:?}", test_user);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
