use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateApparatus {
    pub userid: String,
    pub name: String,
    pub description: String,

    pub repetitions: u8,
    pub sets: u8,
    pub notes: String,
}

#[derive(Deserialize)]
pub struct PatchApparatus {
    pub userid: String,
    pub apparatusid: String,

    pub name: Option<String>,
    pub description: Option<String>,
    pub repetitions: Option<u8>,
    pub sets: Option<u8>,
    pub notes: Option<String>,
}
