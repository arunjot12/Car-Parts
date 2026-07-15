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
    password : hashed_password
   };

//    let app = Router::new().route("/",get(root)).route("/hello",get(run));
//    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//    axum::serve(listener,app).await.unwrap();
}

// async fn root() -> &'static str{
//     "Hello Arun bhai. Naukri lag jayegi"
// }

// async fn run() -> &'static str{
//     "Hello from the another server"
// }