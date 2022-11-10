use crate::models::tag::Tag;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub article_id: u32,
    pub title: String,
    pub content: String,
    pub tags: Vec<Tag>,
    pub keywords: Vec<String>, // for the search feature
    pub value: u32,            // for ranking articles
}
