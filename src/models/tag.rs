use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum TagType {
    Tech,
    Experience,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    _id: u32,
    name: String,
    tag_type: TagType,
    color: String, //in hex format
}
