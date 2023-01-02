use crate::handlers::article;
use crate::models::article::Article;
use rocket::serde::json::Json;
use rocket::State;

#[get("/article/all")]
pub async fn get_all_articles(uri: &State<String>) -> Json<Vec<Article>> {
    Json(article::get_all_articles(uri.to_string()).await)
}

#[get("/article/<id>")]
pub async fn get_article_by_id(id: i64, uri: &State<String>) -> Option<Json<Article>> {
    let res = article::get_article_by_id(id, uri.to_string()).await;
    match res {
        Some(res) => Some(Json(res)),
        None => None,
    }
}

