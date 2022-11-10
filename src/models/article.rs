use crate::models::tag::Tag;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    _id: u32,
    title: String,
    content: String,
    tags: Vec<Tag>,
    keywords: Vec<String>, // for the search feature
    value: u32,            // for ranking articles
}
