use crate::data::NewSignupShopkeepers;
use argon2::{
    password_hash::SaltString,
    Argon2,
    PasswordHasher,
};
use rand_core::OsRng;
use validator::ValidateEmail;
use crate::signup::read_input;


pub fn signup_shopkeeper() -> NewSignupShopkeepers {
    println!("\n========== SHOPKEEPER SIGNUP ==========\n");

    println!("Enter First Name:");
    let first_name = read_input().0;

    println!("Enter Email:");
    let email = read_input().0;

    if email.validate_email() {
        println!("✅ Valid email.");
    } else {
        println!("❌ Invalid email.");
    }

    println!("Enter Phone Number (10 digits):");
    let phone_number = read_input().0;

    if phone_number.len() != 10 {
        println!("❌ Invalid phone number.");
    }

    println!("Enter Password:");
    let password = read_input().0;

    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("Enter Shop Name:");
    let shop_name = read_input().0;

    println!("Enter Shop Address:");
    let shop_address = read_input().0;

    println!("Enter City:");
    let city = read_input().0;

    println!("\nCreating shopkeeper account...");
    println!("--------------------------------------");
    println!("First Name   : {}", first_name);
    println!("Email        : {}", email);
    println!("Phone Number : {}", phone_number);
    println!("Shop Name    : {}", shop_name);
    println!("Shop Address : {}", shop_address);
    println!("City         : {}", city);
    println!("--------------------------------------");

    let shopkeeper = NewSignupShopkeepers {
        first_name: Some(first_name),
        email: Some(email),
        hashed_password: Some(hashed_password),
        phone_number: Some(phone_number),
        shop_name: Some(shop_name),
        shop_address: Some(shop_address),
        city: Some(city),
    };

    println!("✅ Shopkeeper data collected successfully.");

    shopkeeper
}