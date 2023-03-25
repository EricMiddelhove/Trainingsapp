pub mod apparatus;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use std::{any::Any, env::args_os};

use mongodb::{bson::doc, results::InsertOneResult, Client};

pub struct User {
    // User Base Data
    name: String,
    email: String,
    password: String,

    // User Profile Data
    apparatus: Box<[apparatus::Apparatus]>,
}

impl User {
    fn hash_password(password: &String) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password");

        password_hash.to_string()
    }

    fn verify_password(password: &String, hashed_password: &String) -> bool {
        let parsed_hash = PasswordHash::new(hashed_password).unwrap();

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn new(name: &String, email: &String, password: &String) -> User {
        let hashed_password = User::hash_password(password);

        User {
            name: name.to_string(),
            email: email.to_string(),
            password: hashed_password,
            apparatus: Box::new([]),
        }
    }

    pub async fn database_insert(self: User, client: &Client) -> InsertOneResult {
        let users = client.database("prod").collection("users");

        let insert_result = users.insert_one(
            doc! {
                "name": self.name,
                "email": self.email,
                "password": self.password,
                "apparatus": [],
            },
            None,
        );

        insert_result.await.unwrap()
    }
}
