use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    article_id: u32,
    title: String,
    content: String,
    tags: Vec<Tag>,
    keywords: Vec<String>, // for the search feature
    value: u32,            // for ranking articles
}
