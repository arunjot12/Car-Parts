use crate::models::SignupShopkeepers;
use crate::schema::signup_shopkeepers;
use crate::schema::signup_shopkeepers::dsl::*;
use crate::{db::establish_connection, models::Login};
use axum::{Json, http::StatusCode};
use diesel::prelude::*;

pub async fn login_shopkeeper(Json(payload): Json<Login>) {
    let mut connection = establish_connection();
    let shopkeeper = signup_shopkeepers::table
        .filter(
            email
                .eq(&payload.username_or_email)
                .or(signup_shopkeepers::username.eq(&payload.username_or_email)),
        )
        .first::<SignupShopkeepers>(&mut connection)
        .optional();
}
