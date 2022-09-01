use crate::models::article::Article;
use crate::utils::counter::{get_counter_value, increment_counter_value};
use crate::utils::db::get_article_collection;
use mongodb::Client;

// TODO routes to implement :
// - post addArticle

// - get getArticleById
// - get getArticlesByTags
// - get getArticlesByKeyword
// - get getAllArticles

// - put updateArticleById

// - delete deleteArticleById

//TODO figure out how form data works && create an article form data
#[get("/article/add")]
pub async fn add_article() {
    let article_collection = get_article_collection().await;
}

#[get("/")]
pub async fn index() -> String {
    String::from("Hello World !")
}
