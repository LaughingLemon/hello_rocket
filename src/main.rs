#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

//Note no "main" function.