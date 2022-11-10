use crate::handlers::article;
use crate::models::article::Article;
use rocket::serde::json::Json;
// TODO routes to implement :
// - post addArticle

// - get getArticleById
// - get getArticlesByTags
// - get getArticlesByKeyword
// - get getAllArticles

// - put updateArticleById

// - delete deleteArticleById

#[post("/article/add", format = "json", data = "<article_wrapper>")]
pub async fn add_article(article_wrapper: Json<Article>) {
    let article: Article = article_wrapper.into_inner();
    article::add_article(article).await;
}

#[get("/")]
pub async fn index() -> String {
    String::from("Hello World !")
}
