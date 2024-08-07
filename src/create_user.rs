use bcrypt::{hash, DEFAULT_COST};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    hashed_password: String,
    handle: String,
    email: String,
    token: String,
}

impl User {
    fn new(username: &str, password: &str, handle: &str, email: &str) -> User {
        // Hash the password using bcrypt
        let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

        // Generate a random token
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        User {
            username: username.to_string(),
            hashed_password,
            handle: handle.to_string(),
            email: email.to_string(),
            token,
        }
    }
}

fn main() {
    let user = User::new(
        "user123",
        "password123",
        "user_handle",
        "user@example.com",
    );
    println!("User created: {:?}", user);
}
