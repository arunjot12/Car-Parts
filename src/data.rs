use diesel::prelude::*;

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct Users{
    pub first_name : String,
    pub email: String,
    pub hashed_password: String,
    pub phone_number: i32
}