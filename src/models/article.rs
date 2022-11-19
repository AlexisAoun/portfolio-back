use crate::models::{tag, title, content};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    _id: u32,
    title: title::Title,
    content: content::Content,
    images_url: Vec<String>,
    tags: Vec<tag::Tag>,
    value: u32,            // for ranking articles
}
