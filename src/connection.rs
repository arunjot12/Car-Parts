use dotenv::dotenv;
use std::env;
use diesel::mysql::MysqlConnection;
use diesel::Connection;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}