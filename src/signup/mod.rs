pub mod signup_shopkeeper;
pub mod signup_users;

use std::io;
pub fn read_input() -> (String,i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let string : String = input.trim().parse().expect("nothing found");
    let number : i32= input.trim().parse().expect("nothing found");
    (string,number)
}
