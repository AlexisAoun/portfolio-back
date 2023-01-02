use crate::utils::db;
use crate::models::article::Article;
use mongodb::{Collection, bson::doc};
use futures::stream::TryStreamExt;

pub async fn get_all_articles(uri: String) -> Vec<Article> {
    let col: Collection<Article> = db::get_article_collection(uri).await;
    let cursor = col.find(None,None).await.unwrap();
    cursor.try_collect().await.unwrap()
}

pub async fn get_article_by_id(id: i64, uri: String) -> Option<Article> {
    let col: Collection<Article> = db::get_article_collection(uri).await;
    let filter = doc! { "_id":  id};
    let output = col.find_one(filter,None).await.unwrap();
    output
}
