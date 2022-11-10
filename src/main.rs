// author : Alexis Aoun
// Note : My first ever Rust project

//after further reading I understood that importing the crate with #macro_use makes it global to the hole project
//one could simply import rocket with the 'use' keyword
#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod utils;
mod handlers;

use crate::routes::article::{add_article, get_all_articles, index};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![add_article, get_all_articles, index])
}
