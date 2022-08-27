// author : Alexis Aoun
// Note : My first ever Rust project

//after further reading I understood that importing the crate with #macro_use makes it global to the hole project
//one could simply import rocket with the 'use' keyword
#[macro_use]
extern crate rocket;

use rocket_db_pools::mongodb::Client;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

//TODO refactor
#[derive(Debug, Serialize, Deserialize)]
enum TagType {
    Tech,
    Experience,
    Domaine,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    name: String,
    tag_type: TagType,
    color: String, //in hex format
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    title: String,
    content: String,
    tags: Vec<Tag>,
    keywords: Vec<String>, // for the search feature
    value: u8,             // for ranking articles
}

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
async fn add_article(id: u32) {
     
}

#[get("/")]
async fn index() -> String {
    let client = match Client::with_uri_str("mongodb://root:root@0.0.0.0:27017/").await {
        Ok(client) => client,
        Err(error) => panic!("Failed to create a db client : {:?}", error),
    };

    let db = client.database("portfolio");
    let article_collection = db.collection::<Article>("article");

    let python_tag = Tag {
        name: String::from("Python"),
        tag_type: TagType::Tech,
        color: String::from("blue"),
    };
    let my_first_article = Article {
        title: String::from("A python project"),
        content: String::from("this a super nice project using python"),
        tags: vec![python_tag],
        keywords: vec!["python".to_string(), "project".to_string()],
        value: 2,
    };

    article_collection
        .insert_one(my_first_article, None)
        .await
        .unwrap();

    String::from("hi")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
