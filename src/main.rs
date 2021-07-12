#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use std::fs;
use rocket::form::Form;
use rocket::response::content;
use rocket::response::content::Html;

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

//Up to age 255
#[get("/person/<name>/<age>")]
fn name_and_age_u8(name: &str, age: u8) -> String {
    format!("Person {} is {}", name, age)
}

//Ages > 255
#[get("/person/<name>/<age>", rank = 2)]
fn name_and_age_usize(name: &str, age: usize) -> String {
    format!("{} is {}", name, age)
}

//Everything else
#[get("/<_..>", rank = 3)]
fn everything_else() -> String {
    format!("Default")
}

#[get("/form")]
fn get_form() -> Html<String> {
    content::Html(fs::read_to_string("form.html").unwrap())
}

#[derive(FromForm)]
struct Task<'r> {
    fname: &'r str,
    lname: &'r str,
}

#[post("/form", data = "<task>")]
fn submit_form(task: Form<Task<'_>>) -> Html<String> {
    content::Html(fs::read_to_string("accept.html").unwrap())
}


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/",
                          routes![index,
                          name_and_age_u8,
                          name_and_age_usize,
                          everything_else,
                          get_form,
                          submit_form])
}

//Note no "main" function.