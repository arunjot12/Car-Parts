pub mod connection;
pub mod data;
pub mod schema;
pub mod signup;

use crate::connection::establish_connection;
use crate::signup::database::{handle_customer_signup, handle_shopkeeper_signup};
use crate::signup::read_input;

#[tokio::main]
async fn main() {
    let mut connection = establish_connection();
    println!("Let's start the database entry. Choose the right option:");
    println!("1 for customer");
    println!("2 for shopkeeper");

    let choice = read_input().1;
    match choice {
        1 => handle_customer_signup(&mut connection),
        2 => handle_shopkeeper_signup(&mut connection),
        _ => println!("❌ Invalid choice"),
    }
}
