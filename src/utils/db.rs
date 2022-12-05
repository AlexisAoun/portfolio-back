use crate::models::article::Article;
use crate::models::tag::Tag;
use mongodb::{Client, Collection, Database};

async fn get_db() -> Database {
    let client = match Client::with_uri_str("mongodb://root:root@0.0.0.0:27017/").await {
        Ok(client) => client,
        Err(error) => panic!("Error : Failed to create a db client : {:?}", error),
    };

    client.database("portfolio")
}

pub async fn get_article_collection() -> Collection<Article> {
    let db = get_db().await;
    db.collection::<Article>("article")
}

pub async fn get_tag_collection() -> Collection<Tag> {
    let db = get_db().await;
    db.collection::<Tag>("tag")
}
