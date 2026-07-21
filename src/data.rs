use diesel::prelude::*;

#[derive(Debug)]
#[derive(Queryable,Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct Username {
    pub email: String,
    pub hashed_password: String
}
