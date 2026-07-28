// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct TaskPriorityEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct TaskStatusEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct UsersRoleEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaskStatusEnum;
    use super::sql_types::TaskPriorityEnum;

    task (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 11]
        status -> Nullable<TaskStatusEnum>,
        #[max_length = 6]
        priority -> Nullable<TaskPriorityEnum>,
        #[max_length = 255]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 255]
        assigned_to_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UsersRoleEnum;

    users (id) {
        id -> Integer,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 10]
        role -> UsersRoleEnum,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(task, users,);
