use std::env;
use axum::{
    routing::get,
    Router
};
use argon2::{Argon2, PasswordHasher, password_hash::{PasswordHash, SaltString}};
pub mod data;
use rand_core::OsRng;
use diesel::{mysql::MysqlConnection, Connection};
use crate::data::Username;
use dotenv::dotenv;
pub mod schema; 


fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}

#[tokio::main]
async fn main() {
   let email = "arunjotsingh@gmail.com";
   let password = "arunjot";
   establish_connection();

   let argon = Argon2::default();
   let salt_string = SaltString::generate(&mut OsRng);
   let hashed_password = argon.hash_password(password.as_bytes(), &salt_string).unwrap().to_string();
   let store_data = Username{
    email : email.to_string(),
    hashed_password : hashed_password
   };

}
