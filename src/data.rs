use diesel::prelude::*;

#[derive(Debug)]
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct Users{
    pub first_name : String,
    pub email: String,
    pub hashed_password: String,
    pub phone_number: String
}

#[derive(Debug)]
#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::signup_shopkeepers)]
pub struct SignupShopkeepers {
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    pub phone_number: Option<String>,
    pub shop_name: Option<String>,
    pub shop_address: Option<String>,
    pub city: Option<String>
}