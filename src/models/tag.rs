use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum TagType {
    Tech,
    Experience,
    Domaine,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    tag_id: u32,
    name: String,
    tag_type: TagType,
    color: String, //in hex format
}
