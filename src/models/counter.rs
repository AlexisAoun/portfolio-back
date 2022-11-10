use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub label: String,
    pub value: u32,
}
