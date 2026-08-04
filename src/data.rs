use diesel::prelude::*;

use crate::schema::sql_types::UsersRoleEnum;

#[derive(Debug)]
pub enum Role{
    Admin, 
    Shopkeeper, 
    Customer
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
    pub role: UsersRoleEnum,
    pub 
}