use crate::handlers::article;
use crate::models::article::Article;
use rocket::serde::json::Json;

#[get("/article/all")]
pub async fn get_all_articles() -> Json<Vec<Article>> {
    Json(article::get_all_articles().await)
}

#[get("/article/<id>")]
pub async fn get_article_by_id(id: i64) -> Option<Json<Article>> {
    let res = article::get_article_by_id(id).await;
    match res {
        Some(res) => Some(Json(res)),
        None => None,
    }
}

