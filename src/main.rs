pub mod db;
pub mod models;
pub mod schema;
pub mod signup;
pub mod cli;

use tokio::net::TcpListener;
use crate::models::NewSignupShopkeepers;
use axum::{Json, serve};
use axum::Router;
use axum::routing::post;
use axum::http::StatusCode;

use crate::signup::signup_shopkeeper::check_signup_shopkeeper;
use crate::signup::handler::handle_shopkeeper_signup;
use crate::db::establish_connection;


#[tokio::main]
async fn main() {
    // let mut connection = establish_connection();
    let app = Router::new().route("/signup",post(signup_shopkeeper));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

pub async fn signup_shopkeeper(Json(payload): Json<NewSignupShopkeepers>) -> Result<(StatusCode,String), (StatusCode,String)> {
         let shopkeeper =    match check_signup_shopkeeper(payload) {
                Ok(shopkeeper) => shopkeeper,           
                Err(err) => {
                    return Err ((
                        StatusCode::BAD_REQUEST,
                        err.to_string()
                    ));
                }
            };  
            let mut connection = establish_connection();
            match handle_shopkeeper_signup(&mut connection, &shopkeeper) {
                Ok(_) => Ok((StatusCode::CREATED,"successfully created shopkeeper".to_string())),
                Err(err) => Err((StatusCode::BAD_REQUEST,err.to_string()))
            }
}