pub mod connection;
pub mod data;
pub mod schema;
pub mod signup;

use crate::connection::establish_connection;
use crate::data::{SignupShopkeepers, Users};
use crate::signup::{read_input, signup_shopkeeper, signup_users};
use diesel::insert_into;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

fn handle_customer_signup(connection: &mut MysqlConnection) {
    let customer = signup_users::signup_users();

    let check_customer_number: Result<Option<Users>, _> = schema::users::table
        .select(Users::as_select())
        .filter(schema::users::phone_number.eq(&customer.phone_number))
        .first(connection)
        .optional();

    match check_customer_number {
        Ok(Some(_)) => {
            println!("❌ Phone number already registered as customer.");
            return;
        }
        Err(err) => {
            println!("❌ Database error during validation check: {}", err);
            return;
        }
        Ok(None) => {}
    }

    let insert_result = insert_into(schema::users::table)
        .values(&customer)
        .execute(connection);

    match insert_result {
        Ok(_) => println!("✅ Customer registered successfully!"),
        Err(err) => println!("❌ Failed to insert customer: {}", err),
    }
}

fn handle_shopkeeper_signup(connection: &mut MysqlConnection) {
    let shopkeeper = signup_shopkeeper::signup_shopkeeper();

    if let Some(ref phone) = shopkeeper.phone_number {
        let check_shopkeeper_number: Result<Option<SignupShopkeepers>, _> =
            schema::signup_shopkeepers::table
                .select(SignupShopkeepers::as_select())
                .filter(schema::signup_shopkeepers::phone_number.eq(phone))
                .first(connection)
                .optional();

        match check_shopkeeper_number {
            Ok(Some(_)) => {
                println!("❌ Phone number already registered as shopkeeper.");
                return;
            }
            Err(err) => {
                println!("❌ Database error during validation check: {}", err);
                return;
            }
            Ok(None) => {}
        }
    }

    let insert_result = insert_into(schema::signup_shopkeepers::table)
        .values(&shopkeeper)
        .execute(connection);

    match insert_result {
        Ok(_) => println!("✅ Shopkeeper registered successfully!"),
        Err(err) => println!("❌ Failed to insert shopkeeper: {}", err),
    }
}

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
