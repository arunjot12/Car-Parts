use crate::models::NewUsers;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::SaltString,
};
use rand_core::OsRng;
use validator::ValidateEmail;

pub fn check_signup_user(req: NewUsers) -> Result<NewUsers, String> {
    if !req.email.validate_email() {
        return Err("Invalid email".into());
    }
    if req.phone_number.len() != 10 {
        return Err("Phone number should be 10 digits".into());
    }

    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password = argon
        .hash_password(req.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    Ok(NewUsers {
        first_name: req.first_name,
        email: req.email,
        phone_number: req.phone_number,
        password,
    })
}