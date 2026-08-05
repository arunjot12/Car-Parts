use crate::data::SignupShopkeepers;
use argon2::{
    password_hash::SaltString,
    Argon2,
    PasswordHasher,
};
use rand_core::OsRng;
use validator::ValidateEmail;
use crate::signup::read_input;
pub fn signup_shopkeeper() -> SignupShopkeepers {
    println!("Enter first name:");
    let first_name = read_input();

    println!("Enter email:");
    let email = read_input();

    if email.validate_email() {
        println!("Valid email");
    } else {
        println!("Invalid email");
    }

    println!("Enter phone number:");
    let phone_number = read_input();

    if phone_number.len() != 10 {
        println!("Invalid phone number");
    }

    println!("Enter password:");
    let password = read_input();

    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("Enter shop name:");
    let shop_name = read_input();

    println!("Enter shop address:");
    let shop_address = read_input();

    println!("Enter city:");
    let city = read_input();

    SignupShopkeepers {
        first_name: Some(first_name),
        email: Some(email),
        hashed_password: Some(hashed_password),
        phone_number: Some(phone_number),
        shop_name: Some(shop_name),
        shop_address: Some(shop_address),
        city: Some(city),
    }
}