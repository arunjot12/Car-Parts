use std::env;
pub mod data;
pub mod signup;
use diesel::{Connection,RunQueryDsl, insert_into, mysql::MysqlConnection};
use dotenv::dotenv;
use crate::schema::users::dsl::users;
use crate::signup::{read_input, signup_users};
pub mod schema;

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
        let insert_customer = insert_into(users)
            .values(customer)
            .execute(&mut connection);
        match insert_customer {
            Ok(_) => println!("Insertion is completed for customer"),
            Err(err) => println!("{}", err),
        }
    } else {
           let insert_shopkeeper = insert_into(users).values(store_data).execute(&mut connection);
            match insert_shopkeeper {
                Ok(_) => println!("Insertion is completed for shopkeeper"),
                Err(err) => println!("{}",err)
            }
    }
}
