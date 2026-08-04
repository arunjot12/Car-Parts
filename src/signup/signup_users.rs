use std::io;
use argon2::password_hash;
use validator::ValidateEmail;
use rand_core::OsRng;
use crate::{data::Users, schema::signup_shopkeepers::hashed_password};
use argon2::{Argon2, PasswordHasher, password_hash::{PasswordHash, SaltString}};

pub fn signup_users() -> Users {
    let first_name = read_input();
    let email = read_input();

    if email.validate_email() {
    println!("Valid email");
    } else {
    println!("Invalid email");
    }
    let mut phone_number = String::new();
    io::stdin().read_line(&mut phone_number).expect("type something");
    let phone_number : i32 = email.trim().parse().expect("Please type a correct email");

    if phone_number < 10 {
        println!("Invalid Number")

    }

   let password = read_input();    
   let argon = Argon2::default();
   let salt_string = SaltString::generate(&mut OsRng);
   let new_hashed_password = argon.hash_password(password.as_bytes(), &salt_string).unwrap().to_string();

    Users { first_name: first_name, email: email, hashed_password: password, phone_number }

}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    input.trim().to_owned()
}