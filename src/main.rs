use std::env;
use diesel::{RunQueryDsl, associations::HasTable, dsl::select, query_dsl::methods::FilterDsl};
pub mod data;
pub mod signup;
use diesel::{Connection, insert_into, mysql::MysqlConnection};
use dotenv::dotenv;

use crate::signup::read_input;
pub mod schema; 

fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}

#[tokio::main]
async fn main() {
   establish_connection();
   println!("Let's start the database entry. Choice the right option");
   println!("1 for customer ");
   println!("2 for shopkeeper");

   let choice = read_input().1;
   if choice == 0 {
//    let insertion = insert_into(users).values(store_data).execute(&mut connection);
//     match insertion {
//         Ok(_) => println!("Insertion is done"),
//         Err(err) => println!("{}",err)
//     }
   }
   else{

//    let insertion = insert_into(users).values(store_data).execute(&mut connection);
//     match insertion {
//         Ok(_) => println!("Insertion is done"),
//         Err(err) => println!("{}",err)
//     }
   }

}
    