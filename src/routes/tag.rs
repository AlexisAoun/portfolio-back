use crate::handlers::tag;
use crate::models::tag::Tag;
use rocket::serde::json::Json;

#[get("/tag/all")]
pub async fn get_all_tags() -> Json<Vec<Tag>> {
    Json(tag::get_all_tags().await)
}
