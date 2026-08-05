use crate::data::Users;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::SaltString,
};
use rand_core::OsRng;
use std::io;
use validator::ValidateEmail;
use crate::signup::read_input;


pub fn signup_users() -> Users {
    println!("\n========== USER SIGNUP ==========\n");

    println!("Enter First Name:");
    let first_name = read_input().0;

    println!("Enter Email:");
    let email = read_input().0;

    if !email.validate_email() {
        println!("❌ Invalid email format.");
    }

    println!("Enter Phone Number (10 digits):");
    let phone_number = read_input().0;

    if phone_number.len() != 10 {
        println!("❌ Phone number must be 10 digits.");
    }

    println!("Enter Password:");
    let password = read_input().0;

    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("\nCreating user account...");
    println!("--------------------------------");
    println!("First Name : {}", first_name);
    println!("Email      : {}", email);
    println!("Phone      : {}", phone_number);
    println!("--------------------------------");

    Users {
        first_name,
        email,
        hashed_password,
        phone_number,
    }
}