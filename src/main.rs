#[macro_use]
extern crate rocket;


#[get("/")]
fn index() -> String {
    let python_tag = Tag {
        name: String::from("Python"),
        tag_type: TagType::Tech,
        color: String::from("blue"),
    };
    let my_first_article = Article {
        title: String::from("A python project"),
        content: String::from("this a super nice project using python"),
        tags: python_tag,
    };
    let output = String::from("My portfolio \n") 
    + &my_first_article.title + &String::from("\n")
    + &my_first_article.content + &String::from("\n")
    + &my_first_article.tags.name + &String::from("\n")
    + &my_first_article.tags.color + &String::from("\n"); 

    return output;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

//TODO refactor
enum TagType{
    Tech,
    Experience,
    Domaine,
}

struct Tag {
    name: String,
    tag_type: TagType,
    color: String, //in hex format
}

struct Article { 
    title: String,
    content: String,
    tags: Tag, //TODO should be an array
}
