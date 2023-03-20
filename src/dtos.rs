use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
