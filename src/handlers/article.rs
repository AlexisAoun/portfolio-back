use crate::utils::db;
use crate::models::article::Article;
use mongodb::Collection;

pub async fn add_article(article: Article) {
    let col: Collection<Article>= db::get_article_collection().await;
    col.insert_one(article, None).await.unwrap();
}
