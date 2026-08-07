pub mod db;
pub mod models;
pub mod schema;
pub mod signup;
pub mod cli;

use tokio::net::TcpListener;
use crate::models::NewUsers;
use axum::{Json, serve};
use axum::Router;
use axum::routing::get;
use axum::http::StatusCode;

use crate::db::establish_connection;
use crate::signup::signup_users::signup_user;
use crate::signup::signup_shopkeeper::signup_shopkeeper;

#[tokio::main]
async fn main() {
    // let mut connection = establish_connection();
    let app = Router::new().route("/signup",get(signup));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

pub async fn signup() -> String {
    "Hello World".to_string()
}