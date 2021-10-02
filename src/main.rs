#![feature(decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod post;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rocket::{
    response::{NamedFile, Redirect},
};
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", post::retrieve_all().unwrap())
}

#[get("/")]
fn about() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("about", &context)
}

#[get("/blog")]
fn blog() -> Redirect {
    Redirect::to("/")
}

#[get("/img/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/img").join(file)).ok()
}

#[get("/favicon/<file..>")]
fn favicon(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/favicon").join(file)).ok()
}

#[get("/css/<file..>")]
fn css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/css").join(file)).ok()
}

#[get("/<file>")]
fn post(file: String) -> Option<Template> {
    Some(Template::render("post", post::get(file)?))
}

#[catch(404)]
fn not_found() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("404", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, blog])
        .mount("/about", routes![about])
        .mount("/assets/", routes![css, images, favicon])
        .mount("/blog/", routes![post])
        .register(catchers![not_found])
        .launch();
}
