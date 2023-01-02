// author : Alexis Aoun
// Note : My first ever Rust project

//after further reading I understood that importing the crate with #macro_use makes it global to the hole project
//one could simply import rocket with the 'use' keyword
#[macro_use]
extern crate rocket;

mod handlers;
mod models;
mod routes;
mod utils;

use crate::routes::article::{get_all_articles, get_article_by_id};
use crate::routes::tag::get_all_tags;
use crate::utils::db::check_db_credentials;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::{Request, Response};
use std::io::stdin;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

async fn compute_uri_from_user() -> String {
    let mut uri: String = String::new();
    while !check_db_credentials(&uri).await {
        let mut mongo_username: String = String::new();
        let mut mongo_password: String = String::new();
        println!("Enter DB credentials");
        println!("Username:");
        stdin()
            .read_line(&mut mongo_username)
            .expect("failed to read username");
        println!("Password:");
        stdin()
            .read_line(&mut mongo_password)
            .expect("failed to read password");
        uri.push_str("mongodb://");
        uri.push_str(&mongo_username.trim_end());
        uri.push_str(":");
        uri.push_str(&mongo_password.trim_end());
        uri.push_str("@0.0.0.0:27017/");
    }
    uri
}

#[launch]
async fn rocket() -> _ {
    let uri: String = compute_uri_from_user().await;
    rocket::build()
        .manage(uri)
        .mount(
            "/",
            routes![get_all_articles, get_all_tags, get_article_by_id],
        )
        .mount("/media/", FileServer::from(relative!("assets")))
        .attach(CORS)
}
