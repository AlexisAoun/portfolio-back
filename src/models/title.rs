use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Title {
    en: String,
    fr: String
}
