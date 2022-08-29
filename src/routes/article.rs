use mongodb::Client;
use crate::models::article::Article;
// TODO routes to implement :
// - post addArticle
// - post addTag

// - get getArticleById
// - get getArticlesByTags
// - get getArticlesByKeyword
// - get getAllArticles

// - get getAllTags
// - get getTagById
// - get getTagName

// - put updateArticleById
// - put updateTagById

// - delete deleteArticleById
// - delete deleteTagById

#[get("/article/<id>")]
pub async fn add_article(id: u32) -> String {
    String::from("helloo boi here s the id ")+&id.to_string()
}

#[get("/")]
pub async fn index() -> String {
    let client = match Client::with_uri_str("mongodb://root:root@0.0.0.0:27017/").await {
        Ok(client) => client,
        Err(error) => panic!("Failed to create a db client : {:?}", error),
    };

    let db = client.database("portfolio");
    let article_collection = db.collection::<Article>("article");

    // let python_tag = Tag {
    //
    //     name: String::from("Python"),
    //     tag_type: TagType::Tech,
    //     color: String::from("blue"),
    // };
    // let my_first_article = Article {
    //     title: String::from("A python project"),
    //     content: String::from("this a super nice project using python"),
    //     tags: vec![python_tag],
    //     keywords: vec!["python".to_string(), "project".to_string()],
    //     value: 2,
    // };

    // article_collection
    //     .insert_one(my_first_article, None)
    //     .await
    //     .unwrap();

    String::from("hi")
}
