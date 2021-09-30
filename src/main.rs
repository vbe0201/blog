#![feature(decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod post;

use std::path::{Path, PathBuf};

use rocket::response::{NamedFile, Redirect};
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", post::retrieve_all().unwrap())
}

#[get("/blog")]
fn blog() -> Redirect {
    Redirect::to("/")
}

#[get("/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/<file>")]
fn post(file: String) -> Option<Template> {
    Some(Template::render("post", post::get(file)?))
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, blog])
        .mount("/assets/", routes![assets])
        .mount("/blog/", routes![post])
        .launch();
}
