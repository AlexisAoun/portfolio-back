use crate::handlers::article;
use crate::models::article::Article;
use rocket::serde::json::Json;

#[get("/article/all")]
pub async fn get_all_articles() -> Json<Vec<Article>> {
    Json(article::get_all_articles().await)
}

