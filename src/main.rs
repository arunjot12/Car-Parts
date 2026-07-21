use std::env;
use axum::{
    routing::get,
    Router
};
use diesel::RunQueryDsl;
use schema::users::dsl::*;
use argon2::{Argon2, PasswordHasher, password_hash::{PasswordHash, SaltString}};
pub mod data;
use rand_core::OsRng;
use diesel::{Connection, insert_into, mysql::MysqlConnection};
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
   let new_email = "arunjotsingh@gmail.com";
   let password = "arunjot";
   let mut connection = establish_connection();

   let argon = Argon2::default();
   let salt_string = SaltString::generate(&mut OsRng);
   let new_hashed_password = argon.hash_password(password.as_bytes(), &salt_string).unwrap().to_string();
   let store_data = Username{
    email : new_email.to_string(),
    hashed_password : new_hashed_password
   };

   let insertion = insert_into(users).values(store_data).execute(&mut connection);

}
    