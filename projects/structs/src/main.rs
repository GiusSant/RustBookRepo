fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("johndoe@example.com"),
        username: String::from("JohnDoe"),
        active: true,
        sign_in_count: 1,
    };
    let email = &user1.email;
    println!("user1 email is {email}");
    
    let mut user2 = build_user(String::from("mariorossi@esempio.it"), String::from("MRossi"));
    let user2_username = &user2.username;
    println!("user2 user is {user2_username}");

    let mut user3 = User {
        email: String::from("mrossi@esempio.it"),
        ..user2
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count:1
    }
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
struct AlwaysEqual;