#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

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

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/",
                          routes![index,
                          name_and_age_u8,
                          name_and_age_usize])
}

//Note no "main" function.