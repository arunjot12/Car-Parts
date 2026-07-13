// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct UsersRoleEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UsersRoleEnum;

    users (id) {
        id -> Integer,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        hashed_password -> Nullable<Varchar>,
        #[max_length = 5]
        role -> UsersRoleEnum,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
