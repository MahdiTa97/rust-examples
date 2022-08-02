struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Rust doesn’t allow us to mark only certain fields as mutable
    let mut user1 = User {
        active: true,
        username: String::from("someone123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    user1.email = String::from("test@example.com")
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit Structs
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
