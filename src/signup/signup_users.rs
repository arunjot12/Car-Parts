use crate::{data::Users, schema::signup_shopkeepers::hashed_password};
use argon2::password_hash;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{PasswordHash, SaltString},
};
use rand_core::OsRng;
use std::io;
use validator::ValidateEmail;
use crate::signup::read_input;

pub fn signup_users() -> Users {
    let first_name = read_input().0;
    let email = read_input().0;

    if email.validate_email() {
        println!("Valid email");
    } else {
        println!("Invalid email");
    }
    let mut phone_number = String::new();
    io::stdin()
        .read_line(&mut phone_number)
        .expect("type something");
    let phone_number: i32 = email.trim().parse().expect("Please type a correct email");

    if phone_number < 10 {
        println!("Invalid Number")
    }
    let password = read_input().0;
    let argon = Argon2::default();
    let salt_string = SaltString::generate(&mut OsRng);
    let new_hashed_password = argon
        .hash_password(password.as_bytes(), &salt_string)
        .unwrap()
        .to_string();

    Users {
        first_name: first_name,
        email: email,
        hashed_password: new_hashed_password,
        phone_number,
    }
}