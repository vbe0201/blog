#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

mod post;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}
