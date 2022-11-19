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

use crate::routes::article::get_all_articles;
use crate::routes::tag::get_all_tags;
use rocket::fs::{relative, FileServer};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_all_articles, get_all_tags])
        .mount("/media/", FileServer::from(relative!("assets")))
}
