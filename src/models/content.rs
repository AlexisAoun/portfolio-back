use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Content {
    en: String,
    fr: String
}
