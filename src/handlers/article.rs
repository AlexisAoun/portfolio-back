use crate::utils::db;
use crate::models::article::Article;
use mongodb::{Collection, options::FindOptions, bson::doc};
use futures::stream::TryStreamExt;

pub async fn add_article(article: Article) {
    let col: Collection<Article> = db::get_article_collection().await;
    col.insert_one(article, None).await.unwrap();
}

pub async fn get_all_articles() -> Vec<Article> {
    let col: Collection<Article> = db::get_article_collection().await;
    let cursor = col.find(None,None).await.unwrap();
    cursor.try_collect().await.unwrap()
}
