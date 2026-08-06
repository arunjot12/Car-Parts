use crate::models::NewSignupShopkeepers;
use argon2::{
    password_hash::SaltString,
    Argon2,
    PasswordHasher,
};
use rand_core::OsRng;
use validator::ValidateEmail;

pub fn signup_shopkeeper(req: NewSignupShopkeepers) -> Result<NewSignupShopkeepers, String> {
    if let Some(ref email) = req.email {
        if !email.validate_email() {
            return Err("Invalid email format".into());
        }
    }
    if let Some(ref phone) = req.phone_number {
        if phone.len() != 10 {
            return Err("Phone number should be 10 digits".into());
        }
    }

    let mut shopkeeper = req;

    if let Some(password) = shopkeeper.password {
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = argon
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        shopkeeper.password = Some(hashed_password);
    }

    Ok(shopkeeper)
}