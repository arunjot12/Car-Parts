pub mod db;
pub mod models;
pub mod schema;
pub mod signup;
pub mod cli;

use std::net::TcpListener;

use axum::{Json, serve};
use axum::routing::Route;

use crate::db::establish_connection;
use crate::signup::handler::{handle_customer_signup, handle_shopkeeper_signup};
use crate::signup::signup_users::signup_user;
use crate::signup::signup_shopkeeper::signup_shopkeeper;

#[tokio::main]
async fn main() {
    let mut connection = establish_connection();

    let app = Router::new().route("./signup",post(signup));
    let listener = TcpListener::bind("127.0.0.1:3000").await;

    serve(listener, app).await.unwrap();

}

pub async fn signup(Json(request: Json<NewUsers>)) -> StatusCode {

}