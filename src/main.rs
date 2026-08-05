use std::env;
pub mod data;
pub mod signup;
use diesel::{ExpressionMethods, OptionalExtension};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{Connection,RunQueryDsl, insert_into, mysql::MysqlConnection};
use dotenv::dotenv;
use crate::data::SignupShopkeepers;
use crate::schema::users::dsl::users;
use crate::signup::signup_users::signup_users;
use crate::signup::{read_input, signup_users};
pub mod schema;
pub use schema::users::dsl::*;

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
        let customer_phone_number = customer.phone_number;

        let check_email = users.filter(phone_number.eq(customer_phone_number)).first::<SignupShopkeepers>(&mut connection).optional();

        let insert_customer = insert_into(users)
            .values(customer)
            .execute(&mut connection);
        match insert_customer {
            Ok(_) => println!("Insertion is completed for customer"),
            Err(err) => println!("{}", err),
        }
    } else {
        //    let insert_shopkeeper = insert_into(users).values(store_data).execute(&mut connection);
        //     match insert_shopkeeper {
        //         Ok(_) => println!("Insertion is completed for shopkeeper"),
        //         Err(err) => println!("{}",err)
        //     }
    }
}
