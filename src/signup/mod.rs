pub mod signup_shopkeeper;
pub mod signup_users;

use std::io;
pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_owned()
}
