use crate::utils::db;
use crate::models::article::Article;
use mongodb::Collection;
use futures::stream::TryStreamExt;

pub async fn get_all_articles() -> Vec<Article> {
    let col: Collection<Article> = db::get_article_collection().await;
    let cursor = col.find(None,None).await.unwrap();
    cursor.try_collect().await.unwrap()
}