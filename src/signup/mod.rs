pub mod signup_shopkeeper;
pub mod signup_users;
pub mod database;

use std::{io, str::FromStr};
pub fn read_input<T>() -> T 
where 
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<T>().expect("No input found")
}
