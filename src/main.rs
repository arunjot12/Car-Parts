use std::env;
pub mod data;
pub mod signup;
use crate::data::Users;
use crate::signup::signup_shopkeeper::signup_shopkeeper;
use crate::signup::{read_input, signup_users, signup_shopkeeper};
use diesel::prelude::*;
use diesel::{insert_into, mysql::MysqlConnection};
use dotenv::dotenv;
pub mod schema;
pub use schema::users::dsl::*;
pub use schema::signup_shopkeepers::dsl::*;


fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}

#[tokio::main]
async fn main() {
    let mut connection = establish_connection();
    println!("Let's start the database entry. Choice the right option");
    println!("1 for customer ");
    println!("2 for shopkeeper");

    let choice = read_input().1;
    if choice == 0 {
        let customer = signup_users::signup_users();
        let customer_phone_number = customer.phone_number.clone();

        let _check_email = users
            .select(Users::as_select())
            .filter(schema::users::dsl::phone_number.eq(&customer_phone_number))
            .first(&mut connection)
            .optional();

        let insert_customer = insert_into(users).values(customer).execute(&mut connection);
        match insert_customer {
            Ok(_) => println!("Insertion is completed for customer"),
            Err(err) => println!("{}", err),
        }
    } else {
            let shopkeeper = signup_shopkeeper::signup_shopkeeper();
           let insert_shopkeeper = insert_into(signup_shopkeepers).values(shopkeeper).execute(&mut connection);
            match insert_shopkeeper {
                Ok(_) => println!("Insertion is completed for shopkeeper"),
                Err(err) => println!("{}",err)
            }
    }
}
