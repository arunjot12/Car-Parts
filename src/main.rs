use std::env;
use axum::{
    routing::get,
    Router
};
use diesel::ExpressionMethods;
use diesel::{RunQueryDsl, associations::HasTable, dsl::select, query_dsl::methods::FilterDsl};
use schema::users::dsl::*;
use argon2::{Argon2, PasswordHasher, password_hash::{PasswordHash, SaltString}};
pub mod data;
use rand_core::OsRng;
use diesel::{Connection, insert_into, mysql::MysqlConnection};
use crate::data::{NewUser, Users};
use dotenv::dotenv;
pub mod schema; 


fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}

#[tokio::main]
async fn main() {
   let new_email = "arunjotsingh@gmail.com";
   let password = "arunjot";
   let mut connection = establish_connection();

   let argon = Argon2::default();
   let salt_string = SaltString::generate(&mut OsRng);
   let new_hashed_password = argon.hash_password(password.as_bytes(), &salt_string).unwrap().to_string();
   let store_data = NewUser{
    email : new_email.to_string(),
    hashed_password : new_hashed_password
   };


   let check = users.filter(email.eq(new_email)).first::<Users>(&mut connection);

//    let insertion = insert_into(users).values(store_data).execute(&mut connection);
//     match insertion {
//         Ok(_) => println!("Insertion is done"),
//         Err(err) => println!("{}",err)
//     }
}
    