use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DbValueStyle = "snake_case"]
pub enum Role {
    Admin,
    Shopkeeper,
    Customer,
}

#[derive(Debug)]
#[derive(Queryable,Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct Users{
    pub id : i32,
    pub first_name : Option<String>,
    pub email: String,
    pub hashed_password: String,
    pub role: Role,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}