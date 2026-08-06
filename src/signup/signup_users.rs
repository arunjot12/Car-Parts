use crate::data::NewUsers;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::SaltString,
};
use rand_core::OsRng;
use validator::ValidateEmail;
use crate::signup::read_input;

pub fn signup_users() -> NewUsers {
    println!("\n========== USER SIGNUP ==========\n");

    println!("Enter First Name:");
    let first_name  : String = read_input();

    println!("Enter Email:");
    let email : String = read_input();

    if !email.validate_email() {
        println!("❌ Invalid email format.");
    }

    println!("Enter Phone Number (10 digits):");
    let phone_number: String = read_input();

    if phone_number.len() != 10 {
        println!("❌ Phone number must be 10 digits.");
    }

    println!("Enter Password:");
    let password : String = read_input();

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

    NewUsers {
        first_name,
        email,
        hashed_password,
        phone_number,
    }
}