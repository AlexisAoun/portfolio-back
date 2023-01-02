use crate::models::article::Article;
use crate::models::tag::Tag;
use mongodb::{Client, Collection, Database};

async fn get_db(uri: String) -> Database {
    let client = match Client::with_uri_str(uri).await {
        Ok(client) => client,
        Err(error) => panic!("Error : Failed to create a db client : {:?}", error),
    };

    client.database("portfolio")
}

pub async fn get_article_collection(uri: String) -> Collection<Article> {
    let db = get_db(uri).await;
    db.collection::<Article>("article")
}

pub async fn get_tag_collection(uri: String) -> Collection<Tag> {
    let db = get_db(uri).await;
    db.collection::<Tag>("tag")
}

pub async fn check_db_credentials(uri: &String) -> bool {
    println!("{uri}");
    let client = match Client::with_uri_str(uri).await {
        Ok(client) => client,
        Err(_) => return false,
    };
    let col: Collection<Article> = client.database("portfolio").collection::<Article>("article");
    match col.find(None,None).await {
        Ok(_) => true,
        Err(_) => false,
    }
}
