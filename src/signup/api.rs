use crate::signup::{signup_shopkeeper::check_signup_shopkeeper};
use crate::signup::handler::{handle_customer_signup, handle_shopkeeper_signup};
use crate::db::establish_connection;
use crate::NewSignupShopkeepers;
use crate::models::NewUsers;
use axum::Json;
use crate::signup::signup_users::check_signup_user;
use axum::http::StatusCode;

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

pub async fn signup_users(Json(payload): Json<NewUsers>) -> Result<(StatusCode,String), (StatusCode,String)> {
         let user =    match check_signup_user(payload) {
                Ok(shopkeeper) => shopkeeper,           
                Err(err) => {
                    return Err ((
                        StatusCode::BAD_REQUEST,
                        err.to_string()
                    ));
                }
            };  
            let mut connection = establish_connection();
            match handle_customer_signup(&mut connection, &user) {
                Ok(_) => Ok((StatusCode::CREATED,"successfully created shopkeeper".to_string())),
                Err(err) => Err((StatusCode::BAD_REQUEST,err.to_string()))
            }
}
