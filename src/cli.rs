use std::{io, str::FromStr};
use crate::models::{NewUsers, NewSignupShopkeepers};
use validator::ValidateEmail;

pub fn read_input<T>() -> T 
where 
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<T>().expect("No input found")
}

pub fn signup_users_cli() -> NewUsers {
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

    NewUsers {
        first_name,
        email,
        password,
        phone_number,
    }
}

pub fn signup_shopkeeper_cli() -> NewSignupShopkeepers {
    println!("\n========== SHOPKEEPER SIGNUP ==========\n");

    println!("Enter First Name:");
    let first_name: String = read_input();

    println!("Enter Email:");
    let email: String = read_input();

    if email.validate_email() {
        println!("✅ Valid email.");
    } else {
        println!("❌ Invalid email.");
    }

    println!("Enter Phone Number (10 digits):");
    let phone_number: String = read_input();

    if phone_number.len() != 10 {
        println!("❌ Invalid phone number.");
    }

    println!("Enter Password:");
    let password:String = read_input();

    println!("Enter Shop Name:");
    let shop_name: String = read_input();

    println!("Enter Shop Address:");
    let shop_address: String = read_input();

    println!("Enter City:");
    let city: String = read_input();

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
        password: Some(password),
        phone_number: Some(phone_number),
        shop_name: Some(shop_name),
        shop_address: Some(shop_address),
        city: Some(city),
    };

    println!("✅ Shopkeeper data collected successfully.");

    shopkeeper
}
