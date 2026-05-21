use anyhow:: Result;
use argon2 :: {
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        SaltString,
    },
    Argon2
};

pub fn hash_password(password:&str)-> Result<String>{
    let salt = SaltString::generatd(&mut OsRng);

    let argon = Argon2::default();

    let password_hash = argon2
    .hash_password(password.as_bytes(),&salt)?
    .to_string();

    ok(password_hash)
}