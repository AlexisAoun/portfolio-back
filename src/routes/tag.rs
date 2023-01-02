use crate::handlers::tag;
use crate::models::tag::Tag;
use rocket::serde::json::Json;
use rocket::State;

#[get("/tag/all")]
pub async fn get_all_tags(uri: &State<String>) -> Json<Vec<Tag>> {
    Json(tag::get_all_tags(uri.to_string()).await)
}
