use diesel::insert_into;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::data::{SignupShopkeepers, Users};
use crate::schema::{signup_shopkeepers, users};
use crate::signup::{signup_shopkeeper, signup_users};

pub fn handle_customer_signup(connection: &mut MysqlConnection) {
    let customer = signup_users::signup_users();

    let check_customer_number: Result<Option<Users>, _> = users::table
        .select(Users::as_select())
        .filter(users::phone_number.eq(&customer.phone_number))
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

    let insert_result = insert_into(users::table)
        .values(&customer)
        .execute(connection);

    match insert_result {
        Ok(_) => println!("✅ Customer registered successfully!"),
        Err(err) => println!("❌ Failed to insert customer: {}", err),
    }
}

pub fn handle_shopkeeper_signup(connection: &mut MysqlConnection) {
    let shopkeeper = signup_shopkeeper::signup_shopkeeper();

    if let Some(ref phone) = shopkeeper.phone_number {
        let check_shopkeeper_number: Result<Option<SignupShopkeepers>, _> =
            signup_shopkeepers::table
                .select(SignupShopkeepers::as_select())
                .filter(signup_shopkeepers::phone_number.eq(phone))
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

    let insert_result = insert_into(signup_shopkeepers::table)
        .values(&shopkeeper)
        .execute(connection);

    match insert_result {
        Ok(_) => println!("✅ Shopkeeper registered successfully!"),
        Err(err) => println!("❌ Failed to insert shopkeeper: {}", err),
    }
}
